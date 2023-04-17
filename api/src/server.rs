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
pub struct GetRooms {
    pub addr: Recipient<Message>,
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
pub struct Join {
    pub id: usize,
    pub room_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Create {
    pub id: usize,
    pub name: String,
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

    fn send_message(&self, room_id: usize, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(&room_id) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
    

    fn send_user_message(&self, user: Recipient<Message> , message: &str) {
        println!("!!! user should recieve rooms");

        user.do_send(Message(message.to_owned()));
    }
}


impl Actor for ChatServer {
    type Context = Context<Self>;
}






impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
     
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





impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}




impl Handler<Create> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Create, ctx: &mut Self::Context) -> Self::Result {
        
    }
}







impl Handler<GetRooms> for ChatServer {
    type Result = usize;

        
        fn handle(&mut self, msg: GetRooms, ctx: &mut Self::Context) -> Self::Result {
        //msg.addr.send(self.rooms);
        println!("!!! user asked for rooms");
        println!("!!! user asked for rooms {:?} ", self.rooms);
        println!("!!! user asked for rooms {:?} ", serde_json::to_string(&self.rooms).unwrap());

        self.send_user_message(msg.addr, &serde_json::to_string(&self.rooms).unwrap());


        let id = self.rng.gen::<usize>();
        id
    }
}


