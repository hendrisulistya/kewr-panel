use std::process::Command;

#[derive(Debug)]
pub enum OsType {
    Windows,
    MacOS,
    Linux,
    Unknown
}

pub struct PM2Info {
    pub installed: bool,
    pub version: Option<String>,
    pub os_type: OsType,
}

pub fn get_pm2_info() -> PM2Info {
    let pm2_check = Command::new("which")
        .arg("pm2")
        .output()
        .and_then(|output| {
            if output.status.success() {
                let pm2_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Command::new(&pm2_path)
                    .arg("--version")
                    .output()
            } else {
                Ok(output)
            }
        });

    let os_type = if cfg!(target_os = "windows") {
        OsType::Windows
    } else if cfg!(target_os = "macos") {
        OsType::MacOS
    } else if cfg!(target_os = "linux") {
        OsType::Linux
    } else {
        OsType::Unknown
    };

    match pm2_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            PM2Info {
                installed: true,
                version: Some(version),
                os_type,
            }
        },
        _ => PM2Info {
            installed: false,
            version: None,
            os_type,
        },
    }
}