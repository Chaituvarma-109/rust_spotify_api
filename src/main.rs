use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ExternalUrls {
    spotify: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Artist{
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Album{
    name: String,
    release_date: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    album: Album,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
    href: String,
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Items {
    items: Vec<Track>
}

#[derive(Debug, Serialize, Deserialize)]
struct APIResponse {
    tracks: Items
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let auth_token = std::env::var("AUTH_TOKEN").expect("check your auth token");
    let query = "yo mama";

    let url = format!("https://api.spotify.com/v1/search?q={}&type=track", query);

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    
    match response.status() {
        StatusCode::OK => {
            let api_response = response.json::<APIResponse>().await.unwrap();
            let tracks = api_response.tracks.items.iter().collect::<Vec<&Track>>();

            for track in tracks {
                println!("Track Name: {}", track.name);
                println!("Track Album Name: {}", track.album.name);
                println!("Album Release Date: {}", track.album.release_date);
                println!("Artists: {}", track.artists.iter().map(|artist| format!("{}, ", artist.name)).collect::<String>());
                println!("Track URL: {}", track.external_urls.spotify);
                println!("Track Details URL: {}", track.href);
                println!("-----------------------------------------------------------------------");
            }
        },
        StatusCode::UNAUTHORIZED => println!("You are unauthorized."),
        StatusCode::FORBIDDEN => println!("This request is forbidden, please check your request details"),
        StatusCode::TOO_MANY_REQUESTS => println!("You exceed the api rate limiting, try again"),
        other => panic!("Uh oh! Something unexpected happened: {:?}", other),
    }
}
