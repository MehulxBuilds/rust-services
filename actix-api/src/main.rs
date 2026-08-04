use actix_web::{
    HttpServer,
    Responder,
    App,
    get,
    web::Path,
};

// use rhai::Engine;

#[get("/multiply/{num1}/{num2}")]
async fn multiply(path: Path<(i32, i32)>) -> impl Responder {
    let (num1, num2) = path.into_inner();
    let result = num1 * num2;
    format!("The result of multiplying {} and {} is: {}", num1, num2, result)
}

#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i32, i32)>) -> impl Responder {
    let (num1, num2) = path.into_inner();
    let result = num1 + num2;
    format!("The result of adding {} and {} is: {}", num1, num2, result)
}

#[get("/health")]
async fn health() -> impl Responder {
    "status: ok"
}

#[get("/")]
async fn index() -> impl Responder {
    "Welcome to the Actix Web API! Use /multiply/{num1}/{num2} or /add/{num1}/{num2} to perform calculations."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(multiply)
            .service(add)
            .service(health)
            .service(index)
        })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}
