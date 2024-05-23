use http::{files, index};
use ntex::web::{get, middleware, App, HttpServer};

mod http;

/*
 * Main function, the base of the entire website as a whole
 * Manages all site structure and initializes state on startup.
 */

#[ntex::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::default()
                .header("Content-Security-Policy", 
                    "default-src 'self'; style-src 'self'; img-src 'self' https://http.cat/ data: blob:; font-src 'self' https://fonts.bunny.net/")
            )
            .service(index)
            .route("/{filename}*", get().to(files))
            .wrap(ntex::web::middleware::Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
