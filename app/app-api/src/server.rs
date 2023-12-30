use crate::{models::suggestions::Suggestions, AppState};
use actix_web::{error::Error, web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct QueryParams {
    prefix: String,
}

async fn get_auto_completion(
    query_params: web::Query<QueryParams>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    log::info!("Handling request for suggestions {:?}", query_params);

    let prefix = &query_params.prefix;

    // Fetch database from app_data
    // Handle errors during database access or suggestion generation
    let results = app_data
        .trie_db
        .get_tries()
        .into_iter()
        .flat_map(|trie| trie.get_suggestions(prefix.clone(), 1))
        .collect::<Vec<_>>(); // Collect results into a Vec

    // Log the number of suggestions
    log::info!("Generated {} suggestions", results.len());

    // Return the results as JSON
    Ok(HttpResponse::Ok().json(Suggestions {
        suggestions: results,
    }))
}

// #[post("/todos")]
// pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
//     let todo = db.create_todo(new_todo.into_inner());
//     match todo {
//         Ok(todo) => HttpResponse::Ok().json(todo),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/suggestions").route(web::get().to(get_auto_completion))),
    );
}
