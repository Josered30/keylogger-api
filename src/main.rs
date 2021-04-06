#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

use std::path::Path;
use std::fs;

mod errors;
mod models;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    if !Path::new("./logs").exists() {
        fs::create_dir_all("./logs")?;
    }

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // we implement middleares with the warp method
            .wrap(
                Cors::new()
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::CONTENT_TYPE,
                        header::ACCEPT,
                    ])
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .finish(),
            )
            .configure(routes::init_info_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    return server.run().await;
}
