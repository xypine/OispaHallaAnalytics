use poem::listener::{Listener, RustlsCertificate, RustlsConfig};
use poem::{http::Method, listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

pub mod db;
use db::connect_db;

pub mod api;
use api::Api;

mod keys;
use keys::{get_cert, get_key};

/// Käynnistää Poem-palvelimen ja tekee tarvitut määritykset
pub async fn start_server(https: bool) -> Result<(), std::io::Error> {
    pub mod built_info {
        // The file has been placed there by the build script.
        include!(concat!(env!("OUT_DIR"), "/built.rs"));
    }
    let version = built_info::PKG_VERSION;

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let db = connect_db().await;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_credentials(false);

    let api_service = OpenApiService::new(Api::default(), "AnalyticsServer", version)
        .server("http://localhost/api")
        .server("https://localhost/api")
        .server("https://analytics.oispahalla.com/api");
    let ui = api_service.swagger_ui();
    let spec = api_service.spec_endpoint();

    let key = get_key()?;
    let cert = get_cert()?;

    if https {
        let https_bind = std::env::var("HTTPS_BIND").unwrap_or_else(|_| "0.0.0.0:443".to_string());
        println!("Binding https to {}...", https_bind);
        let listener = TcpListener::bind(https_bind)
            .rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(key).cert(cert)));

        Server::new(listener)
            .run(
                Route::new()
                    .nest("/", ui)
                    .nest("/openapi.json", spec)
                    .nest("/api", api_service)
                    .with_if(true, cors)
                    .data(db),
            )
            .await
    } else {
        let http_bind = std::env::var("HTTP_BIND").unwrap_or_else(|_| "0.0.0.0:80".to_string());
        println!("Binding http to {}...", http_bind);
        let listener = TcpListener::bind(http_bind);

        Server::new(listener)
            .run(
                Route::new()
                    .nest("/", ui)
                    .nest("/openapi.json", spec)
                    .nest("/api", api_service)
                    .with_if(true, cors)
                    .data(db),
            )
            .await
    }
}
