use std::{thread::sleep, time::Duration};

use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::Parser;
use rand::Rng;
use tracing::{info, Level};

#[derive(Parser, PartialEq, Eq, Clone, Debug)]
#[command(version, about)]
struct Args {
    #[clap(short = 'p', long = "port")]
    port: u16,
    // Actix webserver workers
    #[clap(short = 'w', long = "worker", default_value_t = 4)]
    worker: u16,
}

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    info!("Received a request, waiting…");
    for i in 0..10 {
        match i {
            0 => info!("{} second", i),
            _ => info!("{} seconds", i),
        }
        sleep(Duration::from_secs(1));
    }
    info!("Responded!");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/latency")]
async fn latency() -> impl Responder {
    info!("Receive a GET request on the /latency route");
    let mut rng = rand::thread_rng();
    let mut latency: f64 = rng.gen();
    latency = latency * 100.0;
    sleep(Duration::from_millis(latency as u64));

    HttpResponse::Ok().body(format!("Hi, I come with {} milliseconds latency", latency))
}

#[get("/api")]
async fn api() -> impl Responder {
    info!("Receive a GET request on the /api route");

    HttpResponse::Ok().body("Hey There!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let local_ip = "0.0.0.0";

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!(
        "Launching a simple lagging server, listening on {}:{}",
        local_ip, args.port
    );

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(latency)
            .service(api)
    })
    .workers(args.worker.into())
    .bind((local_ip, args.port))?
    .run()
    .await
}
