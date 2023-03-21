use std::time::{Duration, Instant};
use actix::Running;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer, Responder, get, post, HttpResponse};
use actix_web_actors::ws;

mod server;
mod handlers;


struct WsChatSession {
    id: usize,
    hb: Instant,
    room: String,
    name: Option<String>,
    //addr: Addr<server::ChatServer>,
}



async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    //srv: web::Data<Addr<server::ChatServer>>,
) -> Result<impl Responder, Error> {
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            //addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}


impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
                println!("Websocket Client heartbeat failed, disconnecting!");
                //act.addr.do_send(server::Disconnect { id: act.id });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

//impl Actor for WsChatSession {
//    type Context = ws::WebsocketContext<Self>;
//
//    /// Method is called on actor start.
//    /// We register ws session with ChatServer
//    fn started(&mut self, ctx: &mut Self::Context) {
//        // we'll start heartbeat process on session start.
//        self.hb(ctx);
//
//        // register self in chat server. `AsyncContext::wait` register
//        // future within context, but context waits until this future resolves
//        // before processing any other events.
//        // HttpContext::state() is instance of WsChatSessionState, state is shared
//        // across all routes within application
//        let addr = ctx.address();
//        println!("{:?}", addr);
//
//        //self.addr
//        //    .send(server::Connect {
//        //        addr: addr.recipient(),
//        //    })
//        //    .into_actor(self)
//        //    .then(|res, act, ctx| {
//        //        match res {
//        //            Ok(res) => act.id = res,
//        //            // something is wrong with chat server
//        //            _ => ctx.stop(),
//        //        }
//        //        fut::ready(())
//        //    })
//        //    .wait(ctx);
//    }
//
//    fn stopping(&mut self, _: &mut Self::Context) -> Running {
//        // notify chat server
//        //self.addr.do_send(server::Disconnect { id: self.id });
//        println!("ln");
//        Running::Stop
//    }
//}
//
//
//






#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::hello)
            .service(handlers::echo)
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/", web::get().to(chat_route)),
            )
//
            .route("/hey", web::get().to(handlers::manual_hello))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}












//
//#[tokio::main]
//async fn main() {
//    println!("hello");
//
//    let listener: TcpListener = TcpListener::bind("localhost:5000").await.unwrap();
//
//
//    let (tx, rx) = broadcast::channel(10);
//
//    loop {
//
//        let (mut socket, addr) = listener.accept().await.unwrap();
//        println!("user connected {}", addr);
//
//
//        let tx = tx.clone();
//        let mut rx = tx.subscribe();
//
//
//        tokio::spawn(async move{
//
//            let (reader, mut writer) = socket.split();
//            let mut reader = BufReader::new(reader);
//            let mut line = String::new();
//    
//
//            loop {
//                tokio::select!{
//                    result = reader.read_line(&mut line) => {
//                        if result.unwrap() < 3 {
//                            break;
//                        }
//                        tx.send((line.clone(), addr)).unwrap();
//                        line.clear();
//                    }
//                    result = rx.recv() => {
//                        let (msg, address) = result.unwrap();
//                        //if addr != address{
//                            writer.write_all(&msg.as_bytes()).await.unwrap();
//                        //}
//                    }
//                }
//            }
//
//
//        });
//
//
//    }
//
//
//
//}
//