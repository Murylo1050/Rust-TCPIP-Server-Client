use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Cliente desconectado: {:?}", stream.peer_addr());
                break; // Cliente fechou a conexão
            }
            Ok(size) => {
                let mensagem = String::from_utf8_lossy(&buffer[..size]);
                let decode: Vec<&str> = mensagem.split("\n").collect();
                println!("{}: {}", decode[0], decode[1]);

                // Enviar mensagem a todos os clientes conectados
                let mut clients_guard = clients.lock().unwrap();
                clients_guard.retain(|mut client| {
                    if let Err(_) = client.write_all(mensagem.as_bytes()) {
                        return false; // Remove clientes desconectados
                    }
                    true
                });
            }
            Err(e) => {
                eprintln!("Erro ao ler do cliente: {}", e);
                break;
            }
        }
    }

    // Remover cliente desconectado da lista
    let mut clients_guard = clients.lock().unwrap();
    clients_guard.retain(|client| client.peer_addr().unwrap() != stream.peer_addr().unwrap());
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25003")?;
    println!("Servidor rodando em 127.0.0.1:25003");

    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Novo cliente conectado: {}", stream.peer_addr().unwrap());

                let clients_clone = Arc::clone(&clients);

                // Adicionar novo cliente à lista
                clients.lock().unwrap().push(stream.try_clone().unwrap());

                thread::spawn(move || handle_client(stream, clients_clone));
            }
            Err(e) => eprintln!("Erro ao aceitar conexão: {}", e),
        }
    }

    Ok(())
}

