use rosu_v2::prelude::*;
use dotenv::dotenv;
use std::env;


#[tokio::main]
async fn main() {
    println!("[Test] osu!v2 api client");
    // Initialize the client
    dotenv().ok();
    let client_id: u64 = 41154;
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let _client_id = env::var("CLIENT_ID")
        .expect("CLIENT_ID not set")
        .parse::<u64>()
        .expect("CLIENT_ID must be a valid u64");
    let _osu = Osu::new(client_id, client_secret).await.unwrap();

    // Get user's top 10-15 scores
    let scores: Vec<Score> = _osu.user_scores("Pascal")
        .mode(GameMode::Osu)
        .best()
        .offset(10)
        .limit(5)
        .await
        .unwrap();
    println!("Scores: {:?}", scores);

}
