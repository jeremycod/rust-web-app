use crate::{Error, Result};
use std::env;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env().unwrap_or_else(|err| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {err:?}")
		})
	})
}
#[allow(non_snake_case)]
pub struct Config {
	// -- Db
	pub DB_URL: String,
	// -- Web
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Config> {
		Ok(Config {
			// -- Db
			DB_URL: get_env("SERVICE_DB_URL")?,
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
