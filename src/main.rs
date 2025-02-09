use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};
use sysinfo::{System, SystemExt};
use std::sync::Mutex;
use log::info;
use tera::Tera;

mod services;
use services::{system::SystemInfo, node::NodeInfo, pm2::PM2Info, database::DatabaseService};

#[derive(Serialize)]
struct SystemStats {
    cpu_usage: f32,
    memory_total: u64,
    memory_used: u64,
    disk_total: u64,
    disk_used: u64,
    node_installed: bool,
    node_version: Option<String>,
    npm_installed: bool,
    npm_version: Option<String>,
    pm2_installed: bool,
    pm2_version: Option<String>,
    postgres_installed: bool,
    postgres_version: String,
    postgres_running: bool,
}

struct AppState {
    sys: Mutex<System>,
    tera: Tera,
}

async fn get_system_stats(data: web::Data<AppState>) -> impl Responder {
    let mut sys = data.sys.lock().unwrap();
    let system_info: SystemInfo = services::system::get_system_info(&mut sys);
    let node_info: NodeInfo = services::node::get_node_info();
    let pm2_info: PM2Info = services::pm2::get_pm2_info();
    let db_info = DatabaseService::new();

    let stats = SystemStats {
        cpu_usage: system_info.cpu_usage,
        memory_total: system_info.memory_total,
        memory_used: system_info.memory_used,
        disk_total: system_info.disk_total,
        disk_used: system_info.disk_used,
        node_installed: node_info.installed,
        node_version: node_info.version,
        npm_installed: node_info.npm_installed,
        npm_version: node_info.npm_version,
        pm2_installed: pm2_info.installed,
        pm2_version: pm2_info.version,
        postgres_installed: db_info.postgres_installed,
        postgres_version: db_info.postgres_version,
        postgres_running: db_info.postgres_running,
    };

    HttpResponse::Ok().json(stats)
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut sys = data.sys.lock().unwrap();
    let system_info: SystemInfo = services::system::get_system_info(&mut sys);
    let node_info: NodeInfo = services::node::get_node_info();
    let pm2_info: PM2Info = services::pm2::get_pm2_info();
    let db_info = DatabaseService::new();

    let mut ctx = tera::Context::new();
    ctx.insert("cpu_usage", &system_info.cpu_usage);
    ctx.insert("memory_total", &system_info.memory_total);
    ctx.insert("memory_used", &system_info.memory_used);
    ctx.insert("disk_total", &system_info.disk_total);
    ctx.insert("disk_used", &system_info.disk_used);
    ctx.insert("node_installed", &node_info.installed);
    ctx.insert("node_version", &node_info.version);
    ctx.insert("npm_installed", &node_info.npm_installed);
    ctx.insert("npm_version", &node_info.npm_version);
    ctx.insert("pm2_installed", &pm2_info.installed);
    ctx.insert("pm2_version", &pm2_info.version);
    ctx.insert("postgres_installed", &db_info.postgres_installed);
    ctx.insert("postgres_version", &db_info.postgres_version);
    ctx.insert("postgres_running", &db_info.postgres_running);
    ctx.insert("os_type", match pm2_info.os_type {
        services::pm2::OsType::Windows => "Windows",
        services::pm2::OsType::MacOS => "MacOS",
        services::pm2::OsType::Linux => "Linux",
        services::pm2::OsType::Unknown => "Unknown",
    });

    let rendered = data.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

use futures::stream::StreamExt;
use actix_web::web::Bytes;

async fn install_pm2() -> impl Responder {
    use std::process::Command;
    use std::io::{BufRead, BufReader};
    
    let mut child = match Command::new("npm")
        .arg("install")
        .arg("-g")
        .arg("pm2")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn() {
            Ok(child) => child,
            Err(_) => return HttpResponse::InternalServerError().body("Failed to start PM2 installation")
        };

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    
    let reader = BufReader::new(stdout);
    let err_reader = BufReader::new(stderr);

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let tx_err = tx.clone();
    let tx_stdout = tx.clone();

    // Handle stdout
    tokio::spawn(async move {
        for line in reader.lines() {
            if let Ok(line) = line {
                let _ = tx_stdout.send(format!("data: {}\n\n", line)).await;
            }
        }
    });

    // Handle stderr
    tokio::spawn(async move {
        for line in err_reader.lines() {
            if let Ok(line) = line {
                let _ = tx_err.send(format!("data: {}\n\n", line)).await;
            }
        }
    });

    // Wait for the command to complete
    tokio::spawn(async move {
        let status = child.wait().unwrap();
        if status.success() {
            info!("PM2 installation completed successfully");
            let _ = tx.send("data: PM2 installation completed successfully. Refreshing page...\n\n".to_string()).await;
        } else {
            info!("PM2 installation failed");
            let _ = tx.send("data: PM2 installation failed.\n\n".to_string()).await;
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx)
        .map(|s| Ok::<_, actix_web::Error>(Bytes::from(s)));

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut tera = Tera::new("templates/**/*").unwrap();
    tera.autoescape_on(vec!["html", "htm", "xml"]);

    info!("Starting web server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                sys: Mutex::new(System::new_all()),
                tera: tera.clone(),
            }))
            .route("/", web::get().to(index))
            .route("/api/system/stats", web::get().to(get_system_stats))
            .route("/api/install-pm2", web::post().to(install_pm2))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
