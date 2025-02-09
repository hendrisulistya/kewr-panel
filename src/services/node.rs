use std::process::Command;

pub struct NodeInfo {
    pub installed: bool,
    pub version: Option<String>,
    pub npm_installed: bool,
    pub npm_version: Option<String>,
}

pub fn get_node_info() -> NodeInfo {
    let node_check = Command::new("which")
        .arg("node")
        .output()
        .and_then(|output| {
            if output.status.success() {
                let node_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Command::new(&node_path)
                    .arg("--version")
                    .output()
            } else {
                Ok(output)
            }
        });

    let npm_check = Command::new("which")
        .arg("npm")
        .output()
        .and_then(|output| {
            if output.status.success() {
                let npm_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Command::new(&npm_path)
                    .arg("--version")
                    .output()
            } else {
                Ok(output)
            }
        });

    let node_info = match node_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            (true, Some(version))
        },
        _ => (false, None),
    };

    let npm_info = match npm_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            (true, Some(version))
        },
        _ => (false, None),
    };

    NodeInfo {
        installed: node_info.0,
        version: node_info.1,
        npm_installed: npm_info.0,
        npm_version: npm_info.1,
    }
}