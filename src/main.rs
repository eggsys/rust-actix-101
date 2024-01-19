use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::Client;

#[get("/")]
async fn hello() -> impl Responder {
    println!("someone say Hello");
    HttpResponse::Ok().body("Hello Melbourne")
}

#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {name}"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
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
    let _db = client.database("user_service");

    list_databases(&client).await?;
    
    HttpServer::new(|| {    
        App::new()
        .service(hello)
        .service(hello_name)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn list_databases(client: &Client) -> Result<(), std::io::Error> {
    let db_names = client.list_database_names(None, None).await.map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("MongoDB error: {}", e))
    })?;

    for db_name in db_names {
        println!("{}", db_name);
    }

    Ok(())
}