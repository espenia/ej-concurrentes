mod models;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use models::APIResponse;

#[tokio::main]
async fn main() {
    let token = "BQAvsm1_hJHmaUcXOlfEpXrd9kUNcwuqNJlFvH9DamaoheoeyShll4ShvfZXuSTeC8LC8x3bcS7GPr8JfEzKwVuiEaBT-d7nSz_Uf34jfHqzvhMCd94";
    let client = reqwest::Client::new();
    let response = client
    .get(format!("https://api.spotify.com/v1/search?q={query}&type=track,artist", query="Daft Punk"))
    .header(AUTHORIZATION, format!("Bearer {token}", token=token))
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .send()
    .await
    .unwrap();

    match response.status() {
        reqwest::StatusCode::OK=> {
            let formatted_response = response.json::<APIResponse>().await.unwrap();
            print!("{:?}", formatted_response.tracks.items);
        },
        _ => panic!("Error: {:?}", response),
    } 
    
}

