use crate::routes::{health_check, subscribe};
use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// run is not a async, depends on caller to run it, either tokio::spawn for asynchronocity or .await to wait for it
pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    println!("Starting web server");
    // HttpServer::new creates a HTTP server,the arguement is a (application factory) function that creates Actix Web Application (App)

    // wrap the connection into a ARC Atomic Reference Counter (ARC) smart pointer
    // cause we can clone, why we need to clone ?
    // Because HttpServer::new() takes a closure arguement, then actix-web spawns a worker for each core on the systesm by running the copy of
    // application built by actix-web, thats why we need clone
    let db_conn = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet)) // web::get() is a helper function that returns Route instance with guard for get
            .wrap(TracingLogger::default()) // wrap is use to use middleware for the App
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_conn.clone()) // app_data adds db_conn as state of that app
    })
    .listen(listener)?
    .run();

    Ok(server)
}
