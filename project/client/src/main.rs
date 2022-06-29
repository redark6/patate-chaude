
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;


fn to_send_messsage_size(message: &str) -> [u8; 4]{
    let u32_message_size: u32 = message.len().try_into().unwrap();
    return u32_message_size.to_be_bytes();

}

fn messages(mut stream: TcpStream, message: &str) {
    const MESSAGE_PREFIX_SIZE: usize = 4;
    let mut count = 0;

    stream.write(&to_send_messsage_size(message));
    stream.write(message.as_bytes());

    let mut message_prefix_bytes = [0u8; MESSAGE_PREFIX_SIZE];
    stream.read(&mut message_prefix_bytes);
    let received_message_size = u32::from_be_bytes(message_prefix_bytes);
    println!("{}", received_message_size);

    let message_size: usize = received_message_size.try_into().unwrap();

    let mut message_bytes = vec![0u8; message_size];
    stream.read(&mut message_bytes);
    let received_message =  std::str::from_utf8(&message_bytes).expect("valid utf8");
    println!("{}", received_message);
}

fn main() {

    let message = "\"Hello\"";

    println!("Tentative de connexion au serveur...");
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!("Connexion au serveur réussie !");
            messages(stream, message);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}