use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::server;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


#[derive(Debug)]
pub struct WsChatSession {
    pub id: usize,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>,
    pub addr: Addr<server::ChatServer>,
}








impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    //fn started(&mut self, ctx: &mut Self::Context) {
    //    self.hb(ctx);
//
    //    let addr = ctx.address();
    //    self.addr
    //        .send(server::Connect {
    //            addr: addr.recipient(),
    //        })
    //        .into_actor(self)
    //        .then(|res, act, ctx| {
    //            match res {
    //                Ok(res) => act.id = res,
    //                // something is wrong with chat server
    //                _ => ctx.stop(),
    //            }
    //            fut::ready(())
    //        })
    //        .wait(ctx);
    //}

    //fn stopping(&mut self, _: &mut Self::Context) -> Running {
    //    self.addr.do_send(server::Disconnect { id: self.id });
    //    Running::Stop
    //}
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





