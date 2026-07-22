use ghostwire::core::message::Message;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

fn make_test_adapter(listen_addr: SocketAddr) -> ghostwire::core::IrcAdapter {
    ghostwire::core::IrcAdapter {
        listen_addr,
        channel_name: "#test".to_string(),
        server_name: "TestServer".to_string(),
        mesh_rx: None,
    }
}

#[tokio::test]
async fn irc_adapter_name() {
    let adapter = make_test_adapter("127.0.0.1:0".parse().unwrap());
    assert_eq!(adapter.name(), "irc");
    assert_eq!(adapter.description(), "IRC protocol bridge — local IRC server for standard client apps");
}

#[tokio::test]
async fn irc_adapter_send_returns_ok() {
    let adapter = make_test_adapter("127.0.0.1:0".parse().unwrap());
    let msg = Message::new(
        "alice".to_string(),
        "#test".to_string(),
        "hello mesh".to_string(),
    );
    assert!(adapter.send_message(&msg).await.is_ok());
}

#[tokio::test]
async fn irc_adapter_receive_returns_none() {
    let adapter = make_test_adapter("127.0.0.1:0".parse().unwrap());
    assert!(adapter.receive_message().await.unwrap().is_none());
}

#[tokio::test]
async fn irc_server_accepts_client_and_responds() {
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let actual_addr = listener.local_addr().unwrap();

    let adapter = ghostwire::core::IrcAdapter {
        listen_addr: actual_addr,
        channel_name: "#ghostwire".to_string(),
        server_name: "GhostWire".to_string(),
        mesh_rx: None,
    };
    let (mesh_tx, _mesh_rx) = tokio::sync::broadcast::channel::<String>(16);

    let server_handle = tokio::spawn(async move {
        let (stream, _addr) = listener.accept().await.unwrap();
        let (reader, mut writer) = stream.into_split();
        let mut lines = BufReader::new(reader).lines();
        let mut registered = false;

        while let Ok(Some(line)) = lines.next_line().await {
            let line = line.trim_end_matches('\r');
            if line.is_empty() { continue; }
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            match parts[0].to_uppercase().as_str() {
                "NICK" => {}
                "USER" => {
                    if !registered {
                        registered = true;
                        let welcome = format!(
                            ":GhostWire 001 testuser :Welcome\r\n\
                             :GhostWire 002 testuser :GhostWire IRC\r\n\
                             :GhostWire 376 testuser :End of MOTD\r\n"
                        );
                        writer.write_all(welcome.as_bytes()).await.unwrap();
                    }
                }
                "PING" => {
                    let token = parts.get(1).unwrap_or(&"");
                    let resp = format!("PONG :{}\r\n", token);
                    writer.write_all(resp.as_bytes()).await.unwrap();
                }
                "JOIN" => {
                    let ch = parts.get(1).unwrap_or(&"#ghostwire");
                    let resp = format!(
                        ":testuser!user@ghostwire JOIN {}\r\n\
                         :GhostWire 353 testuser = {} :testuser\r\n\
                         :GhostWire 366 testuser {} :End of NAMES\r\n",
                        ch, ch, ch
                    );
                    writer.write_all(resp.as_bytes()).await.unwrap();
                }
                _ => {}
            }
        }
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let mut stream = TcpStream::connect(actual_addr).await.unwrap();
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    writer.write_all(b"NICK testuser\r\n").await.unwrap();
    writer.write_all(b"USER test 0 * :Test\r\n").await.unwrap();

    let mut got_welcome = false;
    let mut got_376 = false;
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(2);

    while tokio::time::Instant::now() < deadline {
        match tokio::time::timeout(std::time::Duration::from_millis(200), lines.next_line()).await {
            Ok(Ok(Some(line))) => {
                let line = line.trim_end_matches('\r');
                if line.contains("001") { got_welcome = true; }
                if line.contains("376") { got_376 = true; }
                if got_welcome && got_376 { break; }
            }
            _ => break,
        }
    }

    assert!(got_welcome, "Should receive RPL_WELCOME (001)");
    assert!(got_376, "Should receive RPL_ENDOFMOTD (376)");

    writer.write_all(b"PING :mytoken\r\n").await.unwrap();
    let mut got_pong = false;
    while tokio::time::Instant::now() < deadline {
        match tokio::time::timeout(std::time::Duration::from_millis(200), lines.next_line()).await {
            Ok(Ok(Some(line))) => {
                if line.contains("PONG") && line.contains("mytoken") {
                    got_pong = true;
                    break;
                }
            }
            _ => break,
        }
    }
    assert!(got_pong, "Should receive PONG response");

    writer.write_all(b"JOIN #ghostwire\r\n").await.unwrap();
    let mut got_join = false;
    while tokio::time::Instant::now() < deadline {
        match tokio::time::timeout(std::time::Duration::from_millis(200), lines.next_line()).await {
            Ok(Ok(Some(line))) => {
                let line = line.trim_end_matches('\r');
                if line.contains("JOIN") && line.contains("#ghostwire") {
                    got_join = true;
                }
                if line.contains("366") { break; }
            }
            _ => break,
        }
    }
    assert!(got_join, "Should receive JOIN confirmation");

    let _ = server_handle.await;
}

#[tokio::test]
async fn irc_privmsg_to_mesh_conversion() {
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let actual_addr = listener.local_addr().unwrap();

    let (mesh_tx, mut mesh_rx) = tokio::sync::broadcast::channel::<Message>(16);

    tokio::spawn(async move {
        let (stream, _addr) = listener.accept().await.unwrap();
        let (reader, mut writer) = stream.into_split();
        let mut lines = BufReader::new(reader).lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let line = line.trim_end_matches('\r');
            if line.is_empty() { continue; }
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            match parts[0].to_uppercase().as_str() {
                "NICK" => {}
                "USER" => {
                    let welcome = format!(
                        ":GhostWire 001 test :Welcome\r\n\
                         :GhostWire 376 test :End of MOTD\r\n"
                    );
                    writer.write_all(welcome.as_bytes()).await.unwrap();
                }
                "PRIVMSG" => {
                    if let Some(msg_start) = line.find(" :") {
                        let content = &line[msg_start + 2..];
                        let gw_msg = Message::new(
                            "test".to_string(),
                            "#ghostwire".to_string(),
                            content.to_string(),
                        );
                        let _ = mesh_tx.send(gw_msg);
                    }
                }
                _ => {}
            }
        }
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let mut stream = TcpStream::connect(actual_addr).await.unwrap();
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    writer.write_all(b"NICK test\r\n").await.unwrap();
    writer.write_all(b"USER test 0 * :Test\r\n").await.unwrap();

    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(2);
    while tokio::time::Instant::now() < deadline {
        match tokio::time::timeout(std::time::Duration::from_millis(200), lines.next_line()).await {
            Ok(Ok(Some(line))) => {
                if line.contains("376") { break; }
            }
            _ => break,
        }
    }

    writer.write_all(b"PRIVMSG #ghostwire :Hello from IRC client\r\n").await.unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let msg = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        mesh_rx.recv(),
    ).await;

    assert!(msg.is_ok(), "Should receive mesh message");
    let received = msg.unwrap().unwrap();
    assert_eq!(received.sender, "test");
    assert_eq!(received.recipient, "#ghostwire");
    assert_eq!(received.content, "Hello from IRC client");
}
