use std::fs;
use std::path::Path;
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use tokio::task;
use warp::Filter;

#[derive(Deserialize)]
struct PlayRequest {
    sound: String,
}

#[derive(Serialize)]
struct PlayResponse {
    message: String,
    success: bool,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    success: bool,
}

async fn play_sound(sound_name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let sounds_dir = "sounds";
    let sound_path = format!("{}/{}", sounds_dir, sound_name);
    
    if !Path::new(&sound_path).exists() {
        let error_response = ErrorResponse {
            error: format!("Sound file '{}' not found", sound_name),
            success: false,
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&error_response),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    let sound_path_clone = sound_path.clone();
    match task::spawn_blocking(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        let file = std::fs::File::open(&sound_path_clone).unwrap();
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        
        sink.sleep_until_end();
    }).await {
        Ok(_) => {
            let response = PlayResponse {
                message: format!("Successfully played sound: {}", sound_name),
                success: true,
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                warp::http::StatusCode::OK,
            ))
        }
        Err(_) => {
            let error_response = ErrorResponse {
                error: format!("Failed to play sound: {}", sound_name),
                success: false,
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&error_response),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

async fn list_sounds() -> Result<impl warp::Reply, warp::Rejection> {
    let sounds_dir = "sounds";
    
    match fs::read_dir(sounds_dir) {
        Ok(entries) => {
            let sounds: Vec<String> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_file() {
                            path.file_name()
                                .and_then(|name| name.to_str())
                                .map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                })
                .collect();
            
            Ok(warp::reply::json(&sounds))
        }
        Err(_) => {
            let error_response = ErrorResponse {
                error: "Unable to read sounds directory".to_string(),
                success: false,
            };
            Ok(warp::reply::json(&error_response))
        }
    }
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST"]);

    let play_route = warp::path("play")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: PlayRequest| play_sound(req.sound));

    let list_route = warp::path("sounds")
        .and(warp::get())
        .and_then(list_sounds);

    let routes = play_route
        .or(list_route)
        .with(cors);

    println!("Audio server starting on http://localhost:3030");
    println!("Endpoints:");
    println!("  POST /play - Play a sound file (JSON body: {{\"sound\": \"filename.wav\"}})");
    println!("  GET /sounds - List available sound files");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}