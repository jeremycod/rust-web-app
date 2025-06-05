use crate::ctx::Ctx;
use crate::model::task::Task;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlb::HasFields;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

pub trait DbBmc {
	const TABLE: &'static str;
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();

	let fields = data.not_none_fields();
	let (id,) = sqlb::insert()
		.table(MC::TABLE)
		.data(fields)
		.returning(&["id"])
		.fetch_one::<_, (i64,)>(db)
		.await?;
	Ok(id)
}
// MC for model controller, E for entity
pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let db = mm.db();

	//let sql = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);
	let entity: E = sqlb::select()
		.table(MC::TABLE)
		.columns(E::field_names())
		.and_where("id", "=", id)
		.fetch_optional(db)
		.await?
		.ok_or(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})?;
	/*    let entity: E = sqlx::query_as(&sql)
	.bind(id)
	.fetch_optional(db)
	.await?
	.ok_or(Error::EntityNotFound {
		entity: MC::TABLE,
		id
	})?;*/
	Ok(entity)
}
pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let db = mm.db();
	let sql = format!("SELECT * FROM {} ORDER BY id", MC::TABLE);
	let entities: Vec<E> = sqlx::query_as(&sql).fetch_all(db).await?;
	Ok(entities)
}

pub async fn delete<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<bool>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let db = mm.db();
	let sql = format!("DELETE FROM {} WHERE id = $1", MC::TABLE);
	let count = sqlx::query(&sql)
		.bind(id)
		.execute(db)
		.await?
		.rows_affected();

	if count == 0 {
		return Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		});
	}

	Ok(true)
}
pub async fn update<MC, E>(
	_ctx: &Ctx,
	mm: &ModelManager,
	id: i64,
	data: E,
) -> Result<()>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();

	let fields = data.not_none_fields();
	let count = sqlb::update()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.data(fields)
		.exec(db)
		.await?;
	if count == 0 { 
		Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})
	} else {
		Ok(())
	}
	
}
