use actix_web::{
    delete, get, patch, post,
    web::{self},
    HttpResponse, Responder,
};
use chrono::Utc;
use models::listing::{Collection, CollectionSchema, UpdateCollectionSchema};
use sqlx::{query_as, Pool, Postgres};

pub mod models;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_all_collections)
        .service(post_a_collection)
        .service(get_a_collection)
        .service(edit_a_collection)
        .service(delete_a_collection)
        .service(health_checker_handler);

    conf.service(scope);
}

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and MongoDB";
    HttpResponse::Ok().json(serde_json::json!({"status": "success","message": MESSAGE}))
}

/// TODO: Add query parametres like pages and limits
#[get("/collections")]
async fn get_all_collections(data: web::Data<AppState>) -> impl Responder {
    let query_result = query_as::<Postgres, Collection>("SELECT * FROM collections")
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(collections) => {
            let json_response = serde_json::json!(
                {
                    "status": "success",
                    "results": collections.len(),
                    "data": collections
                }
            );
            HttpResponse::Ok().json(json_response)
        }
        Err(err) => {
            let json_response = serde_json::json!({
                   "status": "fail",
                   "message": format!("Error: {}", err)
            });
            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

/// TODO: See if i can get rid of the "clones";
#[post("/collections")]
async fn post_a_collection(
    data: web::Data<AppState>,
    body: web::Json<CollectionSchema>,
) -> impl Responder {
    let query_result = query_as::<Postgres, Collection>(
        "INSERT INTO collections (title, description, display_image_url, location, contact_information, listings,amenities, rules) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
    )
    .bind(body.title.clone())
    .bind(body.description.clone())
    .bind(body.display_image_url.clone())
    .bind(body.location.clone())
    .bind(body.contact_information.clone())
    .bind(&body.listings)
    .bind(&body.amenities)
    .bind(body.rules.clone())
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(collection) => {
            let json_response = serde_json::json!(
                {
                    "status": "success",
                    "results": collection,
                }
            );
            HttpResponse::Ok().json(json_response)
        }
        Err(err) => {
            let json_response = serde_json::json!({
                   "status": "fail",
                   "message": format!("Error: {}", err)
            });
            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

#[patch("/collections/{id}")]
async fn edit_a_collection(
    data: web::Data<AppState>,
    body: web::Json<UpdateCollectionSchema>,
    id: web::Path<i32>,
) -> impl Responder {
    // 1. Confirm that the collection exists
    let id = id.into_inner();
    let query_result = query_as::<Postgres, Collection>("SELECT * FROM collections WHERE id = $1")
        .bind(&id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let json_response = serde_json::json!({
            "status": "fail",
            "message": "Collection not found"
        });
        return HttpResponse::NotFound().json(json_response);
    }

    // 2. Update the collection

    let now = Utc::now();
    let collection = query_result.unwrap();

    let new_query = query_as::<Postgres, Collection>(
        "UPDATE collections SET title = $1, description = $2, location = $3, contact_information =$4, listings = $5, rules = $6, created_at = $7 WHERE id = $8 RETURNING *",
    )
    .bind(body.title.to_owned().unwrap_or(collection.title.clone()))
    .bind(body.description.to_owned().unwrap_or(collection.description.unwrap().clone()))
    .bind(body.location.to_owned().unwrap_or(collection.location.clone()))
    .bind(body.contact_information.to_owned().unwrap_or(collection.contact_information.clone()))
    .bind(body.listings.to_owned().unwrap_or(collection.listings.clone()))
    .bind(body.rules.to_owned().unwrap_or(collection.rules.clone()))
    .bind(now)
    .bind(&id)
    .fetch_one(&data.db)
    .await;

    match new_query {
        Ok(collection) => {
            let json_response = serde_json::json!({
                "status": "success",
                "message": "Collection updated",
                "data": collection
            });
            HttpResponse::Ok().json(json_response)
        }
        Err(error) => {
            let json_response = serde_json::json!({
                "status": "fail",
                "message": "Collection not updated",
                "error": error.to_string()
            });
            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

#[delete("/collections/{id}")]
async fn delete_a_collection(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();

    // 1. Check if the collection exists
    let query_result = query_as::<Postgres, Collection>("SELECT * FROM collections WHERE id = $1")
        .bind(&id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let json_response = serde_json::json!({
            "status": "fail",
            "message": "Collection not found"
        });
        return HttpResponse::NotFound().json(json_response);
    }

    // 2. Delete the collection

    let new_query = query_as::<Postgres, Collection>("DELETE FROM collections WHERE id = $1")
        .bind(&id)
        .fetch_optional(&data.db)
        .await;

    match new_query {
        Ok(_collection) => {
            let json_response = serde_json::json!({
                "status": "success",
                "message": "Collection deleted",
            });
            HttpResponse::Ok().json(json_response)
        }
        Err(error) => {
            let json_response = serde_json::json!({
                "status": "fail",
                "message": "Collection not deleted",
                "error": error.to_string()
            });
            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

#[get("/collections/{id}")]
async fn get_a_collection(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let query_result = query_as::<Postgres, Collection>("SELECT * FROM collections WHERE id = $1")
        .bind(id.into_inner())
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(collection) => {
            let json_response = serde_json::json!(
                {
                    "status": "success",
                    "results": collection,
                }
            );
            HttpResponse::Ok().json(json_response)
        }
        Err(err) => {
            let json_response = serde_json::json!({
                   "status": "fail",
                   "message": format!("Error: {}", err)
            });
            HttpResponse::InternalServerError().json(json_response)
        }
    }
}
