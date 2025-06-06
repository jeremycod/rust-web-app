#![allow(unused)] // For early development.

// region:    --- Modules
pub mod _dev_utils;
mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};
pub use config::config;

use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static};
use axum::{middleware, Router};
use std::net::SocketAddr;
use std::sync::LazyLock;
use tokio::net::TcpListener;
use tokio::signal;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time() // for early local development
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY
	_dev_utils::init_dev().await;

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;
	// -- Define Routes
	// let routes_rpc = rpc::routes(mm.clone())
	//   .route_layer(middleware::from_fn(mw_ctx_require));
	static STATIC_PATH: &str = "./";
	static STATIC_PATH_STRING: LazyLock<String> = LazyLock::new(|| "./".to_string());

	let routes_all = Router::new()
		.merge(routes_login::routes())
		// .nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server
	let listener = TcpListener::bind("127.0.0.1:8003").await.unwrap();
	info!("->> {:<12} - {:?}\n", "LISTENING", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();

	// endregion: --- Start Server

	Ok(())
}
