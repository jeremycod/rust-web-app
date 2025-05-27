#![allow(unused)] // For early development.

// region:    --- Modules

mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};

use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static};
use axum::{middleware, Router};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use std::sync::LazyLock;
// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
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
		.fallback_service(routes_static::serve_dir(&STATIC_PATH_STRING));

	// region:    --- Start Server
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
