use poem::listener::{RustlsConfig, RustlsCertificate, Listener};
use poem::{http::Method, middleware::Cors, listener::TcpListener, Route, Server, EndpointExt};
use poem_openapi::{
    OpenApiService,
};

pub mod db;
use db::connect_db;

pub mod api;
use api::Api;

mod keys;
use keys::{ get_cert, get_key };

/// Käynnistää Poem-palvelimen ja tekee tarvitut määritykset
pub async fn start_server() -> Result<(), std::io::Error>{
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

    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_credentials(false);

    let api_service =
        OpenApiService::new(Api::default(), "AnalyticsServer", version)
            .server("https://localhost:8002/overwatch/api")
            .server("http://localhost:8002/overwatch/api")
            .server("https://hac.oispahalla.com:8002/overwatch/api");
    let ui = api_service.swagger_ui();
    let spec = api_service.spec_endpoint();

    let key = get_key()?;
    let cert = get_cert()?;
    let listener = TcpListener::bind("0.0.0.0:8002")
        .rustls(
            RustlsConfig::new()
            .fallback(
                RustlsCertificate::new()
                .key( key )
                .cert( cert )
            )
        );

    Server::new(listener)
        .run(Route::new()
        .nest("/overwatch", ui)
        .nest("/overwatch/openapi.json", spec)
        .nest("/overwatch/api", api_service)
            .with_if(true, cors)
            .data(db)
        )
        .await
}
