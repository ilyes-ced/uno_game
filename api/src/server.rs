use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};





#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);


#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<usize, HashSet<usize>>,
    rng: ThreadRng,
}


#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub id: usize,
    pub room_id: usize,
    pub addr: Recipient<Message>,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub room_id: usize,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
    pub room_id: usize,
}












impl ChatServer {
    pub fn new(room_id: usize) -> ChatServer {
        // default room
        let pp: HashSet<usize> = HashSet::from([1]);
        let mut rooms = HashMap::new();
        rooms.insert(room_id, pp);


        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
        }
    }

    pub fn add_room(mut rooms: HashMap<usize, HashSet<usize>>,room_name: String) {}

    fn send_message(&self, room: usize, message: &str, skip_id: usize) {}
}


impl Actor for ChatServer {
    type Context = Context<Self>;
}






impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!(" {} has joined {} ", msg.id, msg.room_id );

        // notify all users in same room
        self.send_message(msg.room_id, "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to main room
        self.rooms
            .entry(msg.room_id)
            .or_insert_with(HashSet::new)
            .insert(id);


        println!(" {:?} ", self.rooms);
        println!(" {:?} ", self.sessions);

        id
    }
}











impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}








