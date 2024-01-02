use actix_web::{get, web, App, HttpServer, HttpResponse, Responder}; 
use rand::Rng;
use serde::Serialize;
use std::collections::HashSet;
// use actix_files::Files;
    

#[derive(Serialize)]
struct Card {
    suit: String,
    value: String,
}

#[get("/randCard/{q}")]
async fn rand_card(path: web::Path<i32>) -> impl Responder {
    let q: i32 = path.into_inner();
    let response: Vec<Card> = if q < 0 || q > 52 {
        let mut cards: Vec<Card> = Vec::new();
        let card: Card = Card {
            suit: "Invalid".to_string(),
            value: "Invalid".to_string(),
        };
        cards.push(card);
        cards
    } else {
        let mut cards: Vec<Card> = Vec::new();
        let mut unique_cards: HashSet<String> = HashSet::new();
        while cards.len() < q as usize {
            let random_suit: i32 = rand::thread_rng().gen_range(0..4);
            let random_value: i32 = rand::thread_rng().gen_range(0..13);
        
            let suit: &str = match random_suit {
                0 => "Hearts",
                1 => "Diamonds",
                2 => "Clubs",
                3 => "Spades",
                _ => "Invalid suit",
            };

            let value: &str = match random_value {
                0 => "Ace",
                1 => "2",
                2 => "3",
                3 => "4",
                4 => "5",
                5 => "6",
                6 => "7",
                7 => "8",
                8 => "9",
                9 => "10",
                10 => "Jack",
                11 => "Queen",
                12 => "King",
                _ => "Invalid value",
            };

            let card_string = format!("{}{}", suit, value);

            if unique_cards.insert(card_string.clone()) {
                let card: Card = Card {
                    suit: suit.to_string(),
                    value: value.to_string(),
                };
            cards.push(card);
            }
    }
    cards
    };
    HttpResponse::Ok().json(response)
}

/* ENDPOINTS END */

// -------------- Main --------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()  
            .service(rand_card) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
