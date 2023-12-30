use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
// use std::sync::{Arc, Mutex};

pub mod database;
pub mod models;
pub mod server;

use crate::database::Database;
use crate::models::error::Error;
type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Serialize)]
struct AppConfig {
    app: AppSettings,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppSettings {
    initialized: bool,
}

// This struct represents state
struct AppState {
    // app_name: String,
    trie_db: Database,
}

// For healthcheck
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> HttpResponse {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    HttpResponse::NotFound().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    // Load configuration from file
    let config_str = fs::read_to_string("./Config.toml").expect("Failed to read config file");
    let config: AppConfig = toml::from_str(&config_str).expect("Failed to parse config file");

    let mut database = Database::new();

    // Check if the app has been run before
    if !config.app.initialized {
        database.learn_from_resources();

        // Update the configuration
        let new_config = AppConfig {
            app: AppSettings { initialized: true },
        };
        let new_config_str =
            toml::to_string_pretty(&new_config).expect("Failed to serialize config");
        fs::write("./Config.toml", new_config_str).expect("Failed to write config file");
    }

    // Remove when connect database works!  
    database.learn_from_resources();

    // Create the application state and wrap it in an Arc
    let app_data = web::Data::new(AppState {
        // app_name: String::from("Grammarlin"),
        // trie_db: connect_database().unwrap_or_else(|_e| {
        //     eprintln!("Failed to get database!");
        //     Database::new()
        // }),
        trie_db: database,
    });

    println!("{:?}", app_data.trie_db);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"]);

        let app_data = app_data.clone();

        App::new()
            .app_data(app_data.clone())
            .configure(server::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// fn connect_database() -> Result<Database> {
//     let file_path = "./database.json";
//     if Path::new(file_path).exists() {
//         let database_str = fs::read_to_string(file_path)?;
//         println!("{}", database_str);
//         let database = serde_json::from_str(&database_str)?;
//         println!("{:?}", database);
//         Ok(database)
//     } else {
//         // If the file doesn't exist, return the Database
//         eprintln!("Could not find database.json");
//         Ok(Database::new())
//     }
// }
