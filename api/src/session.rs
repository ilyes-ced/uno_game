use std::time::{Duration, Instant};
use rand::{self, rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use actix::prelude::*;
use actix_web_actors::ws;

use crate::server;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageType {
    pub msg_type: String,
    pub content: String
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateRoom {
    pub room_name: String,
    pub content: String
}


#[derive(Debug)]
pub struct WsChatSession {
    pub id: usize,
    pub hb: Instant,
    pub room_id: usize,
    pub name: Option<String>,
    pub addr: Addr<server::ChatServer>,
    pub rng: ThreadRng,
}


impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                act.addr.do_send(server::Disconnect { user_id: act.id });

                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}


impl Handler<server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}


impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {


        
        self.id = self.rng.gen::<usize>();
        self.hb(ctx);
        
        println!("this users_idd is  {:?} ", &self.id);













        //self.addr
        //    .send(server::Connect {
        //        id: self.id,
        //        room_id: self.room_id,
        //        addr: addr.recipient(),
        //    })
        //    .into_actor(self)
        //    .then(|res, act, ctx| {
        //        match res {
        //            Ok(res) => act.id = res,
        //            // something is wrong with chat server
        //            _ => ctx.stop(),
        //        }
        //        fut::ready(())
        //    })
        //    .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { user_id: self.id });
        Running::Stop
    }
}




impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                println!("recieved this////////////////////////////{:?}//////////////// from {:?} ", text, self.id);
                let new_msg: MessageType = serde_json::from_str(&text).unwrap();
                println!("recieved this////////////////////////////{:?}//////////////// from {:?} ", new_msg, self.id);

                match new_msg.msg_type.as_str() {
                    ("create_room") => println!("here we create room"),
                    ("get_rooms") => println!("here we display all rooms and sessions"),
                    ("card_play") => println!("here we play in our room"),
                    (_) => println!("error invalid request"),
                }


            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}



/*

self.addr
.send(server::Create {
    user_id: self.id,
    name: "first created room".to_owned(),
    addr: ctx.address().recipient(),
}).into_actor(self)
.then(|res, act, ctx| {
    match res {
        Ok(res) => act.id = res,
        // something is wrong with chat server
        _ => ctx.stop(),
    }
    fut::ready(())
})
.wait(ctx);


self.addr
.send(server::GetRooms {
    addr: ctx.address().recipient(),
})
.into_actor(self)
.then(|res, act, ctx| {
    //match res {
    //    Ok(res) => act.id = res,
    //    // something is wrong with chat server
    //    _ => ctx.stop(),
    //}
    fut::ready(())
})
.wait(ctx);



*/