use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest;
use dotenv::dotenv;
use std::env;
use reqwest::Client;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
    data: Option<SubscriberCount>,
}

#[derive(Deserialize, Serialize)]
struct SubscriberCount {
    channel_id: String,
    subscriber_count: u64,
}

#[derive(Deserialize, Debug)]
struct YouTubeResponse {
    items: Vec<YouTubeItem>,
}

#[derive(Deserialize, Debug)]
struct YouTubeItem {
    statistics: YouTubeStatistics,
}

#[derive(Deserialize, Debug)]
struct YouTubeStatistics {
    subscriberCount: String,
}

#[get("/subscribers/{channel_id}")]
async fn get_subscribers(channel_id: web::Path<String>) -> impl Responder {
    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set");
    let url = format!(
        "https://www.googleapis.com/youtube/v3/channels?part=statistics&id={}&key={}",
        channel_id, api_key
    );

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                // 取得に成功したらレスポンスをパースする
                match response.json::<YouTubeResponse>().await {
                    Ok(youtube_data) => {
                        println!("YouTube API Response: {:?}", youtube_data);
                        if let Some(item) = youtube_data.items.first() {
                            let subscriber_count = item.statistics.subscriberCount.parse().unwrap_or(0);
                            let data = SubscriberCount {
                                channel_id: channel_id.to_string(),
                                subscriber_count,
                            };
                            web::Json(ApiResponse {
                                message: "Subscriber count retrieved successfully".to_string(),
                                status: "success".to_string(),
                                data: Some(data),
                            })
                        } else {
                            web::Json(ApiResponse {
                                message: "Channel not found".to_string(),
                                status: "error".to_string(),
                                data: None,
                            })
                        }
                    }
                    Err(_) => web::Json(ApiResponse {
                        message: "Failed to parse YouTube API response".to_string(),
                        status: "error".to_string(),
                        data: None,
                    }),
                }
            } else {
                // エラーレスポンスを返す
                web::Json(ApiResponse {
                    message: "YouTube API request failed".to_string(),
                    status: "error".to_string(),
                    data: None,
                })
            }
        }
        Err(_) => web::Json(ApiResponse {
            message: "Failed to connect to YouTube API".to_string(),
            status: "error".to_string(),
            data: None,
        }),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client = Client::new();

    println!("Server running at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(get_subscribers)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}