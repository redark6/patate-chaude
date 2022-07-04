use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use serde::{Serialize, Deserialize};

fn main() {
    let stream = std::net::TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream ) => {

            let array = [0; 4];
            let hello = Message::Hello;
            send(&mut stream, hello);

            // let subscribe = Message::Subscribe(Subscribe { name: "Paprocki".parse().unwrap() });
            // send(&mut stream, subscribe);

             while 5 != 2 {
                let value = &receive(&mut stream, array); 
                match value {
                    Ok(v) => { 
                        println!("message = {v:?}");
                        println!("{}", std::mem::discriminant(v))
                        //    std::mem::discriminant(a) == std::mem::discriminant(b)

                    },
                    Err(err) => println!("error = {err:?}")
                }
               
             }

             print!("quit");

            
            // receive(&mut stream, array); //challenge

            // let array_2 = [0; 4];
            // receive(&mut stream, array_2); //roundsummary

            // let array_3 = [0; 4];
            // receive(&mut stream, array_3); //edofgame


        }
        Err(err) => panic!("Cannot connect: {err}")
    }
}

fn receive(stream: &mut TcpStream, mut array: [u8; 4]) -> Result<Message, serde_json::Error> {
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

    return message;
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
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeAnswer(ChallengeAnswer),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicLeaderBoard(Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize)]
struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

//pub enum ChallengeOuput {}

//pub enum ChallengeInput {}
#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeOutput {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput),
    RecoverSecret(RecoverSecretInput)
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
   MD5HashCash(MD5HashCashOutput),
   MonstrousMaze(MonstrousMazeOutput),
   RecoverSecret(RecoverSecretOutput)
}

#[derive(Debug, Serialize, Deserialize)]
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