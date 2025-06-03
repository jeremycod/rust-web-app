// region:    --- Modules

mod error;
mod store;
mod task;

use crate::model::store::{new_db_pool, Db};
pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	 db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		Ok(ModelManager {db})
	}
	// Returns the sqlx db pool only for the model layer
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
