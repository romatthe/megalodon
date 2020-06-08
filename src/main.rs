use actix_web::{App, HttpServer, Responder, web};

async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Actix requires spawning its own single-threaded runtime. Here we use tokio to explicitly spawn
    // an Actix system inside of it. This way we can still use tokio::spawn when we need it for other libraries
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("system", &local);
    let server = HttpServer::new(|| App::new().route("/", web::get().to(hello_world)))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    sys.await?;

    Ok(server)
}
