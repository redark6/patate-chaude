
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

fn messages(mut stream: TcpStream) {
    let mut count = 0;

    let message = "\"Hello\"";
    let u32_message_size: u32 = message.len().try_into().unwrap();
    let bytes = u32_message_size.to_be_bytes();
    const MESSAGE_SIZE: usize = 4;

    stream.write(&bytes);
    stream.write(message.as_bytes());


    loop {
        let mut rx_bytes = [0u8; MESSAGE_SIZE];
        // Read from the current data in the TcpStream
        stream.read(&mut rx_bytes);
        let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
        println!("{}", received);


        count += 1;
        if count == 50 {
            println!("Je meur");

            break;
        }
    }
}

fn main() {


    println!("Tentative de connexion au serveur...");
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!("Connexion au serveur réussie !");
            messages(stream);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}