use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha512, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use md5;
use sha1::Sha1;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct HashRequest {
    password: String,
    salt: String,
    algorithm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct HashResponse {
    hashed_password: String,
    algorithm: String,
}

struct AppState {
    last_hash: Mutex<Option<HashResponse>>,
}

fn hash_password(password: &str, salt: &str, algorithm: &str) -> String {
    match algorithm.to_lowercase().as_str() {
        "md5" => {
            let mut hasher = md5::Context::new();
            hasher.consume(password.as_bytes());
            hasher.consume(salt.as_bytes());
            format!("{{MD5}}{}", BASE64.encode(hasher.compute().0))
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(password.as_bytes());
            hasher.update(salt.as_bytes());
            format!("{{SHA}}{}", BASE64.encode(hasher.finalize()))
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            hasher.update(salt.as_bytes());
            format!("{{SHA256}}{}", BASE64.encode(hasher.finalize()))
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(password.as_bytes());
            hasher.update(salt.as_bytes());
            format!("{{SHA512}}{}", BASE64.encode(hasher.finalize()))
        }
        _ => "Unsupported algorithm".to_string(),
    }
}

async fn hash_password_endpoint(
    data: web::Json<HashRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let hashed = hash_password(&data.password, &data.salt, &data.algorithm);
    let response = HashResponse {
        hashed_password: hashed,
        algorithm: data.algorithm.clone(),
    };
    
    // Store the last hash for comparison
    *app_state.last_hash.lock().unwrap() = Some(response.clone());
    
    HttpResponse::Ok().json(response)
}

async fn compare_hash_endpoint(
    data: web::Json<HashRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let new_hash = hash_password(&data.password, &data.salt, &data.algorithm);
    let last_hash = app_state.last_hash.lock().unwrap();
    
    let comparison = if let Some(last) = last_hash.as_ref() {
        last.hashed_password == new_hash
    } else {
        false
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "matches": comparison,
        "new_hash": new_hash,
        "last_hash": last_hash.as_ref().map(|h| h.hashed_password.clone())
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let app_state = web::Data::new(AppState {
        last_hash: Mutex::new(None),
    });

    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/api/hash").route(web::post().to(hash_password_endpoint)))
            .service(web::resource("/api/compare").route(web::post().to(compare_hash_endpoint)))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
