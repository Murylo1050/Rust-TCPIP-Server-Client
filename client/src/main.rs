use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

fn message_thread(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Servidor desconectado.");
                break;
            }
            Ok(size) => {
                let mensagem = String::from_utf8_lossy(&buffer[..size]);
                let decode: Vec<&str> = mensagem.split("\n").collect();

                if decode.len() >= 2 {
                    println!("{}: {}", decode[0], decode[1]);
                }
            }
            Err(e) => {
                println!("Erro ao ler do servidor: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("geraldo-server.ddns.net:25567")?;
    let stream_clone = stream.try_clone()?; // Clonamos para poder usar em outra thread

    println!("Digite o nome de usuário:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Esperado user name!");
    let username = username.trim().to_string(); // Remover espaços extras e "\n"

    // Criar a thread corretamente
    thread::spawn(move || message_thread(stream_clone));

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");

        if input.trim() == "-1" {
            break;
        }

        let mensagem = format!("{}\n{}", username, input);
        stream.write_all(mensagem.as_bytes())?;
    }

    Ok(())
}
