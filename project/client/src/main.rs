fn main() {
    // // let message = Welcome { version: 1 };
    // // let message = Message { welcome: Welcome { version: 1 }};
    // let message = Message::Welcome(Welcome { version: 1 });
    //
    // let serialized = serde_json::to_string(&message);
    // let serialized =
    //     match serialized {
    //         Ok(s) => s,
    //         Err(err) => panic!("Ça s'est pas bien passé: {err}")
    //     };
    // // let serialized = serialized.unwrap();
    // // let serialized = serialized.expect("Error");
    // // let serialized = serialized?; => try operator

    let stream = std::net::TcpStream::connect("localhost:7676");
    match stream {
        Ok(mut stream ) => {
            let message = "Hello😇";
            stream.write_all(&message.as_bytes()).unwrap();
        }
        Err(err) => panic!("Cannot connect: {err}")
    }

    // println!("{serialized}");
}
