use actix_web::middleware::Logger;
use actix_web::{http::header::ContentType, web, HttpResponse};
use env_logger::Env;
use serde::Serialize;
use std::io::Result;

#[derive(Serialize)]
struct Resp {
    status: String,
    version: String,
}

async fn get_version() -> HttpResponse {
    let resp = Resp {
        status: "ok".to_string(),
        version: "v0.0.1".to_string(),
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}
#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    use actix_web::{App, HttpServer};
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(get_version))
    })
    .bind("127.0.0.1:8080")
    .expect("error on server")
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::to_bytes, http, test};

    #[test]
    async fn test_get_version_ok() {
        let resp: HttpResponse = get_version().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body_bytes, r##"{"status":"ok","version":"v0.0.1"}"##);
    }
}
