use auth::{login, Keys};
use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, HeaderValue, Method, Request, StatusCode},
    response::Response,
    routing::{get, post},
    Router,
};
use config::Config;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use dummy::protected;
use frontend::json;
use std::{env, net::SocketAddr, time::Duration};
use tower_http::trace::TraceLayer;
use tower_http::{classify::ServerErrorsFailureClass, cors::CorsLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use user::{create_user, list_users};

mod auth;
mod config;
mod dummy;
mod frontend;
mod user;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../common/migrations/");

// gotta give credit where credit is due and stuff
lazy_static::lazy_static! {
    static ref CONFIG: Config = {
         dotenv().ok();
         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
         let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
         let fmp_api_key = env::var("FMP_API_KEY").expect("FMP_API_KEY must be set");
         let fed_api_key = env::var("FED_API_KEY").expect("FED_API_KEY must be set");

         let secret = &CONFIG.jwt_secret.clone();
         let keys = Keys::new(secret.as_bytes());

         return Config{
             database_url,
             jwt_secret,
             keys,
             fmp_api_key,
             fed_api_key,
         };
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("bla");
    tracing::info!("bla");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("bla2");
    tracing::info!("bla2");

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(
        &CONFIG.database_url,
        deadpool_diesel::Runtime::Tokio1,
    );

    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("should work");

    println!("bla3");
    tracing::info!("bla3");

    // run the migrations on server startup
    {
        println!("running migrations");
        tracing::info!("running migrations");

        let conn = pool.get().await.unwrap();
        let _ = conn
            .interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .expect("first")
            .expect("second");

        println!("migrations done");
        tracing::info!("migrations done");
    }
    tracing::info!("bla4");

    // let fe = async {
    //     let app = Router::new().route("/", get(html));
    //     let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    //     tracing::debug!("listening on {addr}");
    //     println!("fe listening on {addr}");
    //     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    //     axum::serve(listener, app)
    //         .await;
    // };

    println!("XXXXXXXXXXXXXXXXXXXX");
    tracing::info!("bla5");
    // build our application with some routes
    let app = Router::new()
        .route("/user/login", post(login))
        .route("/user/list", get(list_users))
        .route("/user/create", post(create_user))
        .route("/json", get(json))
        .route("/protected", get(protected))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        )
        .with_state(pool);

    tracing::info!("bla6");

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    println!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("should listen on port 3000");

    //tokio::join!(fe, be);
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
