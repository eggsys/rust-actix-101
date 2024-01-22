use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{Client};
use mongodb::bson::{doc, Document};
use std::io::Error;

#[get("/")]
async fn hello() -> impl Responder {
    println!("someone say Hello");
    HttpResponse::Ok().body("Hello Melbourne")
}

#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    println!("someone called {name}");
    HttpResponse::Ok().body(format!("Hello {name}"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/book")]
async fn add_book(data: web::Data<mongodb::Database>, req_body: String) -> impl Responder {
    println!("someone called add book");
    let db = data.get_ref();
    let _ = add_book_mongo(db).await;
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey Bourne!")
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{

    let uri = std::env::var("MOGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("Failed to connect to MongoDB");

    // Get a handle to a database.
    list_databases(&client).await?;
    let db: mongodb::Database = client.database("product_service_db");
    let db_pool = web::Data::new(db);

    HttpServer::new(move || {    
        App::new()
        .app_data(db_pool.clone())
        .service(hello)
        .service(hello_name)
        .service(echo)
        .service(add_book)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn list_databases(client: &Client) -> Result<(), Error> {
    let db_names = client.list_database_names(None, None).await.map_err(|e| {
        Error::new(std::io::ErrorKind::Other, format!("MongoDB error: {}", e))
    })?;
    println!("Available databases:");
    for db_name in db_names {
        println!("{}", db_name);
    }

    Ok(())
}

async fn add_book_mongo(db: &mongodb::Database) -> Result<(), Error>{
    let collection = db.collection::<Document>("books");

    let doc = doc! {
        "title": "The Great Gatsby",
        "author": "F. Scott Fitzgerald",
        "year": 1925,
    };

    collection.insert_one(doc, None).await.expect("Failed to insert document");
    println!("Document inserted successfully");

    Ok(())
}