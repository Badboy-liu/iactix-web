use std::env;
use futures_util::FutureExt;
use actix_web::dev::Service;
use std::rc::Rc;
use std::sync::Arc;
use actix_web::{dev, rt, test, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{from_fn, ErrorHandlerResponse, ErrorHandlers, Logger, Next};
use crate::controller::TS;
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use sea_orm::Database;
use tracing_subscriber::FmtSubscriber;

// use imacro::register_modules as other_register_modules;
mod controller;
mod iimacro;
mod err;
mod midd;
mod entity;

async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    println!("req : {:?}", req);
    next.call(req).await
    // post-processing
}

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>,Error> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    unsafe {
        // std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();
    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let database_url = "sqlite:app.db";

    let conn = Database::connect(&db_url).await.unwrap();
    // 初始化 tracing 日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG) // 输出 debug 及以上日志
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");


    HttpServer::new(move || {
        let logger = Logger::default();
        let mut  app = App::new()
            .wrap(logger)
            .app_data(web::Data::new(conn.clone()))
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR,add_error_header))
            .wrap(from_fn(my_middleware))
            .route("/echo",web::get().to(echo))
            // .wrap(Logger::new("%a %t %r %{User-Agent}i"))
            .wrap_fn(|req,srv|{
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .configure(TS::routes::config);
        // app = register_modules!(app, [ts::config]);
        app
    }).bind(("127.0.0.1",8080))
        .unwrap().run()
        .await
}
