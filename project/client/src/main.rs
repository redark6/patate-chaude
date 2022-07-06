mod hash_cash_challenge;
mod monstrous_maze_challenge;

use crate::hash_cash_challenge::{HashCash, MD5HashCashInput, MD5HashCashOutput};
use crate::monstrous_maze_challenge::{MonstrousMaze, MonstrousMazeInput, MonstrousMazeOutput};

//use crate::hash_cash_challenge::challenge::Challenge;

use std::io::{Read, Write};
use std::mem::transmute;
use std::net::TcpStream;
use std::{env, str};
use clap::builder::TypedValueParser;
use serde::{Serialize, Deserialize};

use crate::hash_cash_challenge::challenge::Challenge;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
      return;
    }
    let ip = String::from(&args[1]);
    let name = String::from(&args[2]);
    let stream = std::net::TcpStream::connect(ip);
    match stream {
        Ok(mut stream ) => {

            let array = [0; 4];
            let hello = Message::Hello;
            let mut nex_target = "".to_string();
            send(&mut stream, hello);

             let subscribe = Message::Subscribe(Subscribe { name: name.parse().unwrap() });
             send(&mut stream, subscribe);

             loop {
                let message = &receive(&mut stream, array); 
                match message {
                    Ok(v) => unsafe {
                        println!("message = {v:?}");
                        if let Message::EndOfGame(..) = v {
                            break;
                        }
                        if let Message::PublicLeaderBoard(board) = v {
                            nex_target = board.0[board.0.len() - 1].name.clone();
                        }

                        if let Message::Challenge(mes) = v {
                            if let Challenges::MD5HashCash(input) = mes {
                                let inputValue = MD5HashCashInput{complexity: input.complexity.clone() ,message: input.message.clone()};
                                let hash = HashCash::new(inputValue);
                                let res = hash.solve();
                                send(&mut stream,Message::ChallengeResult(ChallengeResult{answer: ChallengeAnswer::MD5HashCash(res), next_target: nex_target.clone()}));
                            }

                            if let Challenges::MonstrousMaze(input) = mes {
                                let inputValue = MonstrousMazeInput{grid: input.grid.clone() ,endurance: input.endurance.clone()};
                                let maze = MonstrousMaze::new(inputValue);
                                let res = maze.solve();
                                send(&mut stream,Message::ChallengeResult(ChallengeResult{answer: ChallengeAnswer::MonstrousMaze(res), next_target: nex_target.clone()}));
                            }

                            if let Challenges::RecoverSecret(input) = mes {
                                let res = RecoverSecretOutput{secret_sentence : "".to_string()};
                                send(&mut stream,Message::ChallengeResult(ChallengeResult{answer: ChallengeAnswer::RecoverSecret(res), next_target: nex_target.clone()}));
                            }
                        }
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
    Challenge(Challenges),
    ChallengeResult(ChallengeResult),
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
/*
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
*/
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
pub enum Challenges {
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
