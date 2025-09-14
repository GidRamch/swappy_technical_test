use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

#[derive(Serialize)]
struct Joke {
    joke: &'static str,
}


async fn joke() -> impl Responder {

  let jokes = vec![
    "What did one snowman say to the other snowman? It smells like carrots over here!",
    "Why did Beethoven get rid of his chickens? All they ever said was, “Bach, Bach, Bach!”",
    "What did 20 do when it was hungry? Twenty-eight.",
    "Why is grass so dangerous? Because it's full of blades!",
    "Why are mountains so funny? They're hill areas.",
    "Why wasn't the cactus invited to hang out with the mushrooms? He wasn't a fungi.",
    "Why shouldn't you fundraise for marathons? They just take the money and run.",
  ];

  let mut rng = thread_rng();
  let random_joke = jokes.choose(&mut rng).unwrap();
  HttpResponse::Ok().json(Joke { joke: random_joke })

}


async fn jooke() -> impl Responder {
  HttpResponse::UnprocessableEntity().json(Joke { joke: "Who mispelled the api route ? You" })
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/joke", web::get().to(joke))
            .route("/jooke", web::get().to(jooke))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}