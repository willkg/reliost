use std::net::TcpListener;

use reliost::{configuration::ServerSettings, configuration::Settings};

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    let host = "127.0.0.1";
    let listener = TcpListener::bind(format!("{host}:0")).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let settings = Settings {
        server: ServerSettings {
            host: host.to_string(),
            port,
        },
        symbols: None,
    };
    let server = reliost::startup::run(listener, settings).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    format!("{host}:{port}")
}
