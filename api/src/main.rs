use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpServer, Responder, get, post, HttpResponse};
use actix_web_actors::ws;

mod server;
mod handlers;


struct WsChatSession {
    id: usize,
    room: String,
    name: String,
}



impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
}



//                            _             _                                                                     _                           _   _               
//       ___    ___     ___  | | __   ___  | |_     _ __ ___     ___   ___   ___    __ _    __ _    ___   ___    | |__     __ _   _ __     __| | | |   ___   _ __ 
//      / __|  / _ \   / __| | |/ /  / _ \ | __|   | '_ ` _ \   / _ \ / __| / __|  / _` |  / _` |  / _ \ / __|   | '_ \   / _` | | '_ \   / _` | | |  / _ \ | '__|
//      \__ \ | (_) | | (__  |   <  |  __/ | |_    | | | | | | |  __/ \__ \ \__ \ | (_| | | (_| | |  __/ \__ \   | | | | | (_| | | | | | | (_| | | | |  __/ | |   
//      |___/  \___/   \___| |_|\_\  \___|  \__|   |_| |_| |_|  \___| |___/ |___/  \__,_|  \__, |  \___| |___/   |_| |_|  \__,_| |_| |_|  \__,_| |_|  \___| |_|   
//                                                                                          |___/

//                                     888               888                                                                                         888                             888 888                  
//                                     888               888                                                                                         888                             888 888                  
//                                     888               888                                                                                         888                             888 888                  
//          .d8888b   .d88b.   .d8888b 888  888  .d88b.  888888     88888b.d88b.   .d88b.  .d8888b  .d8888b   8888b.   .d88b.   .d88b.  .d8888b      88888b.   8888b.  88888b.   .d88888 888  .d88b.  888d888 
//          88K      d88""88b d88P"    888 .88P d8P  Y8b 888        888 "888 "88b d8P  Y8b 88K      88K          "88b d88P"88b d8P  Y8b 88K          888 "88b     "88b 888 "88b d88" 888 888 d8P  Y8b 888P"   
//          "Y8888b. 888  888 888      888888K  88888888 888        888  888  888 88888888 "Y8888b. "Y8888b. .d888888 888  888 88888888 "Y8888b.     888  888 .d888888 888  888 888  888 888 88888888 888     
//               X88 Y88..88P Y88b.    888 "88b Y8b.     Y88b.      888  888  888 Y8b.          X88      X88 888  888 Y88b 888 Y8b.          X88     888  888 888  888 888  888 Y88b 888 888 Y8b.     888     
//           88888P'  "Y88P"   "Y8888P 888  888  "Y8888   "Y888     888  888  888  "Y8888   88888P'  88888P' "Y888888  "Y88888  "Y8888   88888P'     888  888 "Y888888 888  888  "Y88888 888  "Y8888  888     
//                                                                                                                         888                                                                                
//                                                                                                                    Y8b d88P                                                                                
//                                                                                                                     "Y88P"                                                                                 
//                                              
//        ..%%%%....%%%%....%%%%...%%..%%..%%%%%%..%%%%%%..........%%...%%..%%%%%%...%%%%....%%%%....%%%%....%%%%...%%%%%%...%%%%...........%%..%%...%%%%...%%..%%..%%%%%...%%......%%%%%%..%%%%%..
//        .%%......%%..%%..%%..%%..%%.%%...%%........%%............%%%.%%%..%%......%%......%%......%%..%%..%%......%%......%%..............%%..%%..%%..%%..%%%.%%..%%..%%..%%......%%......%%..%%.
//        ..%%%%...%%..%%..%%......%%%%....%%%%......%%............%%.%.%%..%%%%.....%%%%....%%%%...%%%%%%..%%.%%%..%%%%.....%%%%...........%%%%%%..%%%%%%..%%.%%%..%%..%%..%%......%%%%....%%%%%..
//        .....%%..%%..%%..%%..%%..%%.%%...%%........%%............%%...%%..%%..........%%......%%..%%..%%..%%..%%..%%..........%%..........%%..%%..%%..%%..%%..%%..%%..%%..%%......%%......%%..%%.
//        ..%%%%....%%%%....%%%%...%%..%%..%%%%%%....%%............%%...%%..%%%%%%...%%%%....%%%%...%%..%%...%%%%...%%%%%%...%%%%...........%%..%%..%%..%%..%%..%%..%%%%%...%%%%%%..%%%%%%..%%..%%.
//        
//                            _             _                                                                     _                           _   _               
//       ___    ___     ___  | | __   ___  | |_     _ __ ___     ___   ___   ___    __ _    __ _    ___   ___    | |__     __ _   _ __     __| | | |   ___   _ __ 
//      / __|  / _ \   / __| | |/ /  / _ \ | __|   | '_ ` _ \   / _ \ / __| / __|  / _` |  / _` |  / _ \ / __|   | '_ \   / _` | | '_ \   / _` | | |  / _ \ | '__|
//      \__ \ | (_) | | (__  |   <  |  __/ | |_    | | | | | | |  __/ \__ \ \__ \ | (_| | | (_| | |  __/ \__ \   | | | | | (_| | | | | | | (_| | | | |  __/ | |   
//      |___/  \___/   \___| |_|\_\  \___|  \__|   |_| |_| |_|  \___| |___/ |___/  \__,_|  \__, |  \___| |___/   |_| |_|  \__,_| |_| |_|  \__,_| |_|  \___| |_|   
//                                                                     __/ |                                                                 
//                                                                    |___/                                                                  
//                                                                                                                                                              





impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("hhelloooo1");
                ctx.pong(&msg);
            },
            Ok(ws::Message::Pong(msg)) => {
                println!("hhelloooo1 from pong");
                ctx.pong(&msg);
            },
            //                        _                                  _   _                  _                                                           
            //   _ __    ___    ___  (_)   ___  __   __   ___      ___  | | (_)   ___   _ __   | |_     _ __ ___     ___   ___   ___    __ _    __ _    ___ 
            //  | '__|  / _ \  / __| | |  / _ \ \ \ / /  / _ \    / __| | | | |  / _ \ | '_ \  | __|   | '_ ` _ \   / _ \ / __| / __|  / _` |  / _` |  / _ \
            //  | |    |  __/ | (__  | | |  __/  \ V /  |  __/   | (__  | | | | |  __/ | | | | | |_    | | | | | | |  __/ \__ \ \__ \ | (_| | | (_| | |  __/
            //  |_|     \___|  \___| |_|  \___|   \_/    \___|    \___| |_| |_|  \___| |_| |_|  \__|   |_| |_| |_|  \___| |___/ |___/  \__,_|  \__, |  \___|
            //                                                                                                                                 |___/        
            Ok(ws::Message::Text(text)) => {
                println!("{}", text);
                //println!("{:?}", ctx);
                ctx.text(text);
                //println!("{:?}", ThreadRng);
                println!("{:?}", rand::thread_rng());
                //let m = text.trim();
                //let msg = if let Some(ref name) = self.name {
                //    format!("{name}: {m}")
                //} else {
                //    m.to_owned()
                //};
                //// send message to chat server
                //self.addr.do_send(server::Message {
                //    id: self.id,
                //    msg,
                //    room: self.room.clone(),
                //})
            },
            Ok(ws::Message::Binary(bin)) => {
                println!("hhelloooo3");
                ctx.binary(bin);
            },
            _ => (),
        }
    }
}



//       _              _   _             _     _                          
//      | |__     ___  | | | |   ___     | |_  | |__     ___   _ __    ___ 
//      | '_ \   / _ \ | | | |  / _ \    | __| | '_ \   / _ \ | '__|  / _ \
//      | | | | |  __/ | | | | | (_) |   | |_  | | | | |  __/ | |    |  __/
//      |_| |_|  \___| |_| |_|  \___/     \__| |_| |_|  \___| |_|     \___|
//                                                                         
async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", req.peer_addr());
        
    let resp = ws::start(WsChatSession { id: 1, room: String::from("start room"), name: String::from("hello there statr room bame") }, &req, stream);
    println!("{:?}", resp);
    resp
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{:?}", "hello there 1111111111111111111");

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
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