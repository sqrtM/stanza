use std::{env, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::{Client, NoTls};
//mod secrets;

#[derive(Clone)]
struct AppState {
    db_client: Arc<Client>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").expect("Failed to load .env.dev");

    let (client, connection) =
        tokio_postgres::connect(&env::var("PG_CONNECTION_STRING").unwrap(), NoTls)
            .await
            .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let app_state = web::Data::new(AppState {
        db_client: Arc::new(client),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Cors::permissive())
            .service(hello)
            .service(echo)
            .service(post_block)
            .service(get_all_blocks)
    })
    .bind((
        env::var("SERVER_HOST").unwrap(),
        env::var("SERVER_PORT").unwrap().parse().unwrap(),
    ))?
    .run()
    .await
}

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("echo !!!");
    HttpResponse::Ok().body(req_body)
}

#[derive(Debug, Serialize, Deserialize)]
enum ActivityPubObjectType {
    Note,
    Article,
    Tombstone,
    Image,
    Video,
    Audio,
    Page,
    Event,
    Place,
    Collection,
    OrderedCollection,
    Create,
    Update,
    Delete,
    Announce,
    Like,
    Follow,
    Invite,
    Reject,
    Accept,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    #[serde(rename = "@context")]
    context: Vec<serde_json::Value>,
    #[serde(rename = "type")]
    activity_type: ActivityPubObjectType,
    actor: String,
    object: ActivityPubObject,
    to: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActivityPubObject {
    #[serde(rename = "type")]
    object_type: ActivityPubObjectType,
    content: String,
    published: String,
    #[serde(rename = "attributedTo")]
    attributed_to: String,
    #[serde(rename = "inReplyTo")]
    in_reply_to: Option<String>,
}

#[actix_web::get("/api/v1/block")]
async fn get_all_blocks(state: web::Data<AppState>) -> impl Responder {
    let client = &state.db_client;

    match client
        .query("SELECT * FROM activity_pub.box LIMIT 10;", &[])
        .await
    {
        Ok(rows) => {
            if !rows.is_empty() {
                let data: Vec<Value> = rows.iter().map(|row| row.get("data")).collect();
                HttpResponse::Ok().json(data)
            } else {
                HttpResponse::Ok().json("emptie")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[actix_web::post("/api/v1/block")]
async fn post_block(state: web::Data<AppState>, req_body: web::Json<Block>) -> impl Responder {
    let client = &state.db_client;

    let block_json: Value =
        serde_json::to_value(req_body).expect("Failed to serialize Block into JSON");

    match client
        .execute(
            "INSERT INTO activity_pub.box (data) VALUES ($1);",
            &[&block_json],
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Block inserted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[actix_web::post("/users")]
async fn create_user(
    state: web::Data<AppState>,
    req_body: web::Json<CreateUserRequest>,
) -> impl Responder {
    let client = &state.db_client;

    match client
        .execute(
            "INSERT INTO activity_pub.users (username, password, email) VALUES ($1, $2, $3);",
            &[
                &req_body.0.username,
                &req_body.0.password,
                &req_body.0.email,
            ],
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[actix_web::post("/.well-known/webfinger")]
async fn webfinger(
    resource: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    if let Some(account) = resource.get("resource") {
        if let Some(username) = account.split(':').nth(1) {
            // Here's where we would use the username to generate the finger.
            let username = username.split('@').next().unwrap().to_string();
            return HttpResponse::Ok()
                .content_type("application/json")
                .body(username);
        }
    }
    HttpResponse::NotFound().finish()
}
