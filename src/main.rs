use actix_web::{get, web, App, HttpServer, HttpResponse, Responder}; 
use rand::Rng;
// use actix_files::Files;
    
struct Card {
    suit: String,
    value: String,
}

#[get("/randCard/{q}")]
async fn randCard(path: web::Path<i32>) -> impl Responder {
    let q: i32 = path.into_inner();
    let response: String = if q < 0 || q > 52 {
        "Invalid request number".to_string()
    } else {
        let random: i32 = rand::thread_rng().gen_range(0..4);
        let random2: i32 = rand::thread_rng().gen_range(0..13);
        let suit: String = match random {
            0 => "Hearts".to_string(),
            1 => "Diamonds".to_string(),
            2 => "Clubs".to_string(),
            3 => "Spades".to_string(),
            _ => "Invalid suit".to_string(),
        };
        let value: String = match random2 {
            0 => "Ace".to_string(),
            1 => "2".to_string(),
            2 => "3".to_string(),
            3 => "4".to_string(),
            4 => "5".to_string(),
            5 => "6".to_string(),
            6 => "7".to_string(),
            7 => "8".to_string(),
            8 => "9".to_string(),
            9 => "10".to_string(),
            10 => "Jack".to_string(),
            11 => "Queen".to_string(),
            12 => "King".to_string(),
            _ => "Invalid value".to_string(),
        };
        format!("{} of {}", value, suit)
    };
    HttpResponse::Ok().body(response)
}

/* ENDPOINTS END */

// -------------- Main --------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()  
            .service(randCard) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}