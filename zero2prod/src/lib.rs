use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("wordl");
    format!("Hello {}", &name)
}

// Respponder is a trait, which requires certain functions to be completed so that a struct can be changed to HTTP response
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// an extractor in actix_web is something that can extract things like data and url from the
// request, the data extracter can be passed in as an arguement
// Extractor must implement FromRequest
// So before calling this function, in the back, "from_request" method is invoked in this case
// especiically Form::from_request
// from_request deserializes the data into FormData struct we defined, according to the rules of
// URL encoding and leveragin serde_urlencoded and Deserialize implementation of FormData which was
// auto geneated with #[derive(serde::Deserialize)]
// Any error is returned to caller with 400 BAD request
async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    println!("name {} email {}", form.name, form.email);
    HttpResponse::Ok().finish()
}
// run is not a async, depends on caller to run it, either tokio::spawn for asynchronocity or .await to wait for it
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("Starting web server");
    // HttpServer::new creates a HTTP server,the arguement is a (application factory) function that creates Actix Web Application (App)
    let server = HttpServer::new(|| {
        App::new()
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet)) // web::get() is a helper function that returns Route instance with guard for get
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
