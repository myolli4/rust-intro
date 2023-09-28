mod models;

use std::io;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use config::{Config, Environment};
use crate::models::Status;
use dotenv::dotenv;

async fn status() -> impl Responder
{
    HttpResponse::Ok()
        .json(Status { status: "Ok".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()>
{
    println!("\n");
    println!("Load environmental variable...\n");

    dotenv().ok();

    let address : String;

    let builder = Config::builder()
        .add_source(Config::default())
        .add_source(Environment::default());

    let config : Config;

    match builder.build() {
        Ok(cfg) => {
            config = cfg
            // println!("{:?}", address);
        }
        Err(err) => {
            panic!("{}", err);
        }
    }

    address = format!(
        "{}:{}",
        config.get_string("server_host").unwrap(),
        config.get_string("server_port").unwrap()
    );

    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(status))
    });

    println!("Starting webserver at `http://{}`", address);

    server
        .bind(address)?
        .run()
        .await
}