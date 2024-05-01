use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
}

#[get("/{path:.*}")]
async fn index(path: web::Path<String>) -> impl Responder {
    let path = path.into_inner();
    format!("Hello {:?}!", path)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("Starting server at http://{}:{}/", args.host, args.port);

    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind((args.host, args.port))?
    .run()
    .await
}