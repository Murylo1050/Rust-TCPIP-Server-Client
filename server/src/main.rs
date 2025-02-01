use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Cliente desconectado");
                break; // Cliente fechou a conexão
            }
            Ok(size) => {
                let mensagem = String::from_utf8_lossy(&buffer[..size]);
                println!("{}: de {}", stream.peer_addr().unwrap(), mensagem.trim());

                // Enviar resposta ao cliente
                if let Err(e) = stream.write_all(b"Mensagem recebida") {
                    eprintln!("Erro ao enviar resposta: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Erro ao ler do cliente: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Servidor rodando em 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "Novo cliente conectado! {} seja bem-vindo",
                    stream.peer_addr().unwrap()
                );
                std::thread::spawn(|| handle_client(stream)); // Criar uma nova thread para cada cliente
            }
            Err(e) => eprintln!("Erro ao aceitar conexão: {}", e),
        }
    }

    Ok(())
}

