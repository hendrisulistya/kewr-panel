use std::process::Command;

pub struct DatabaseService {
    pub postgres_installed: bool,
    pub postgres_version: String,
    pub postgres_running: bool
}

impl DatabaseService {
    pub fn new() -> Self {
        let mut service = DatabaseService {
            postgres_installed: false,
            postgres_version: String::new(),
            postgres_running: false
        };
        service.check_postgres_status();
        service
    }

    fn check_postgres_status(&mut self) {
        // Check if PostgreSQL is installed
        if let Ok(output) = Command::new("psql").arg("--version").output() {
            if output.status.success() {
                self.postgres_installed = true;
                let version = String::from_utf8_lossy(&output.stdout);
                if let Some(v) = version.split_whitespace().nth(2) {
                    self.postgres_version = v.to_string();
                }
            }
        }

        // Check if PostgreSQL service is running
        if let Ok(output) = Command::new("ps").args(["-ef"]).output() {
            let processes = String::from_utf8_lossy(&output.stdout);
            self.postgres_running = processes.contains("postgres");
        }
    }
}