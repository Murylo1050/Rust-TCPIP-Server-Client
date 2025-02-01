use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    loop {
        let mut input = String::new();

        println!("Digite algo:");
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");

        if (input.trim() == "-1") {
            break Ok(());
        };

        stream.write_all(input.as_bytes())?;

        let mut buffer = [0; 512];
        let size = stream.read(&mut buffer)?;

        println!(
            "Resposta do servidor: {}",
            String::from_utf8_lossy(&buffer[..size])
        );
    }
}

