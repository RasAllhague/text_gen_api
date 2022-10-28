mod api;

use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};
use api::{text_generation::gen_text, version::get_version};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .wrap(logger)
            .service(gen_text)
            .service(get_version)
            .app_data(json_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
