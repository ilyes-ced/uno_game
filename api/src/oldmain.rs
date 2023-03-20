//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//
//mod handlers;
//
//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
//    HttpServer::new(|| {
//        App::new()
//            .service(handlers::hello)
//            .service(handlers::echo)
//            .service(
//                // prefixes all resources and routes attached to it...
//                web::scope("/app")
//                    // ...so this handles requests for `GET /app/index.html`
//                    .route("/", web::get().to(handlers::index)),
//            )
//
//            .route("/hey", web::get().to(handlers::manual_hello))
//    })
//    .bind(("127.0.0.1", 5000))?
//    .run()
//    .await
//}



use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}