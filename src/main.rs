use dotenv::dotenv;
use oauth2::{ClientId, ClientSecret};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Create an OAuth2 client
    let google_client_id =
        ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID"));
    let gogle_client_se3cret =
        ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET"));
    let google_redirect_uri = env::var("GOOGLE_REDIRECT_URI").expect("Missing GOOELC_REDIRECT_URI");

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        AuthUrl::new("https://accounts.googlecom/o/oauth2/auth".to_string())
            .expect("Invalid autheorization endpoint URL"),
        Some(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .expect("Invalid token endpoint URL"),
        ),
    )
    .set_redirect_uri(RedirectUrl::new(google_redirect_uri).expect("Invalid redirect URL"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/", web::get().to(index))
            .route("/auth/google", web::get().to(auth_google))
            .route("/auth/callback", web::get().to(auth_callback))
    })
    .bind("127.0.0.1:3500")?
    .run()
    .await
}

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use serde::{Deserialize, Serialize};

// // Define a struct for handling JSON requests
// #[derive(Serialize, Deserialize)]
// struct MyData {
//     name: String,
// }

// // Define a simple handler for the root path "/"
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello, world!")
// }

// // Define a POST request handler for "/submit" path
// #[post("/submit")]
// async fn submit_data(data: web::Json<MyData>) -> impl Responder {
//     let response_message = format!("Hello, {}!", data.name);
//     HttpResponse::Ok().json(response_message)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello) // register the GET handler
//             .service(submit_data) // register the POST handler
//     })
//     .bind("127.0.0.1:3400")? // bind to the specified address and port
//     .run()
//     .await
// }
