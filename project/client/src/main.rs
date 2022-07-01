use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use serde::{Serialize, Deserialize};

fn main() {
    let stream = std::net::TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream ) => {

            let hello = Message::Hello;
            send(&mut stream, hello);

            let subscribe = Message::Subscribe(Subscribe { name: "Nelson".parse().unwrap() });
            send(&mut stream, subscribe);

            let array = [0; 4];
            receive(&mut stream, array);

            let array_2 = [0; 4];
            receive(&mut stream, array_2);

            let array_3 = [0; 4];
            receive(&mut stream, array_3);


        }
        Err(err) => panic!("Cannot connect: {err}")
    }
}

fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    stream.read( &mut array).unwrap();

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;
    let mut vector = vec![0; size_message];

    println!("{}",size_message);

    stream.read(&mut vector).unwrap();

    let message_received = std::str::from_utf8(&*vector).unwrap();
    println!("received: {}", message_received);
    let welcome_serialized = serde_json::to_string(&message_received).unwrap();
    let a = welcome_serialized.replace("\\", "");


    let first_last_off: &str = &a[1..a.len() - 1];
    let message: Result<Message, _> = serde_json::from_str(&first_last_off);

    match message {
        Ok(m) => println!("message={m:?}"),
        Err(err) => println!("error={err:?}")
    }
}

fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;

    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();

    stream.write_all(&message_to_serialized.as_bytes()).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct Welcome{
    version: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicPlayerLeaderBoard),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),

}

#[derive(Debug, Serialize, Deserialize)]
struct PublicPlayerLeaderBoard {
     Tuple<String,Vec<PublicPlayer>>
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

// pub struct Hello {
//     body: String // il n'y a pas de body ?
// }

// pub struct Welcome {
//     version: u8
// }
// pub struct Subscribe {
//     name: String
// }
// pub enum SubscribeError{
//     AlreadyRegistered, InvalidName
// }

// pub enum BodySubscribeResult {
//     Ok, Err(SubscribeError) 
// }
// pub struct SubscribeResult {
//     a: BodySubscribeResult // pas de a
// }

// pub struct PublicPlayer {
//     name: String,
//     stream_id: String,
//     score: i32,
//     steps: u32,
//     is_active: bool,
//     total_used_time: f64,
// }

// pub struct PublicLeaderBoard {
//   // .0: Vec<PublicPlayer>
// }

//ChallengeOutput

pub struct Challenge {
    // enum {
    //      ChallengeName(ChallengeInput)
    // }
}

pub enum ChallengeAnswer {
   ChallengeName(ChallengeOutput)
}

pub struct ChallengeResult {
    answer: ChallengeAnswer,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String, //"free_patato"
    value: ChallengeValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame{
    leader_board: PublicLeaderBoard
}