use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
    collections::HashMap,
};
use serde::{Deserialize, Serialize};
use actix::*;
use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get
};
use actix_web_actors::ws;

mod server;
mod session;




#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct RoomId{
    room_id: usize,
}


#[get("/ws/{room_id}")]
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
    room_id: web::Path<usize>,
) -> Result<HttpResponse, Error> {
    println!("hello there");
    ws::start(
        session::WsChatSession {
            id: 0, //this users_id
            hb: Instant::now(),
            room_id: room_id.into_inner(),
            name: None,
            addr: srv.get_ref().clone(),
            rng: rand::thread_rng(),
        },
        &req,
        stream,
    )
}

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    //let rooms_state = Arc::new(AtomicUsize::new(0));
    //let users_state = Arc::new(AtomicUsize::new(0));

    log::info!("starting HTTP server at http://localhost:8080");

    let server = server::ChatServer::new(0).start();



    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            //.app_data(web::Data::from(rooms_state.clone()))
            //.app_data(web::Data::from(users_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .service(chat_route)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
