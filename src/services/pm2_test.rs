use super::pm2::*;
use mockall::predicate::*;
use rstest::*;
use test_log::test;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_list_processes() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/list"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!([
                {
                    "name": "test-app",
                    "pm_id": 0,
                    "monit": {
                        "memory": 1024,
                        "cpu": 0.5
                    },
                    "pm2_env": {
                        "status": "online",
                        "version": "1.0.0"
                    }
                }
            ])))
        .mount(&mock_server)
        .await;

    let pm2_client = PM2Client::new(&mock_server.uri());
    let processes = pm2_client.list_processes().await;
    
    assert!(processes.is_ok());
    if let Ok(process_list) = processes {
        assert_eq!(process_list.len(), 1);
        assert_eq!(process_list[0].name, "test-app");
        assert_eq!(process_list[0].status, "online");
    }
}

#[tokio::test]
async fn test_process_status() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/describe/test-app"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "pm_id": 0,
                "name": "test-app",
                "status": "online",
                "version": "1.0.0",
                "uptime": 3600
            })))
        .mount(&mock_server)
        .await;

    let pm2_client = PM2Client::new(&mock_server.uri());
    let status = pm2_client.get_process_status("test-app").await;
    
    assert!(status.is_ok());
    if let Ok(process_status) = status {
        assert_eq!(process_status.name, "test-app");
        assert_eq!(process_status.status, "online");
        assert_eq!(process_status.version, "1.0.0");
    }
}