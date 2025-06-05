use crate::ctx::Ctx;
use crate::model::base::DbBmc;
use crate::model::Result;
use crate::model::{base, Error, ModelManager};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
// region: -- Task Types

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
	pub id: i64,
	pub title: String,
	#[field(name = "description")] // this is rename for Fields (sqlb)
	#[sqlx(rename = "description")] // this is for FromRow (sqlx)
	pub desc: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
	pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
	pub title: Option<String>,
}

// endregion: --- Task Types
pub struct TaskBmc;
impl DbBmc for TaskBmc {
	const TABLE: &'static str = "task";
}
impl TaskBmc {
	pub async fn create(
		_ctx: &Ctx,
		mm: &ModelManager,
		task_c: TaskForCreate,
	) -> Result<i64> {
		let db = mm.db();
		let (id,) = sqlx::query_as::<_, (i64,)>(
			"INSERT INTO task (title) VALUES ($1) returning id",
		)
		.bind(task_c.title)
		.fetch_one(db)
		.await?;
		Ok(id)
	}
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
		base::list::<Self, _>(_ctx, mm).await
	}

	pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<bool> {
		base::delete::<Self, Task>(_ctx, mm, id).await
	}
}

// region: --- Tests
#[cfg(test)]
mod tests {
	#![allow(unused)]
	use super::*;
	use crate::_dev_utils;
	use anyhow::Result;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok title";

		let task_c = TaskForCreate {
			title: fx_title.to_string(),
		};
		let id = TaskBmc::create(&ctx, &mm, task_c).await?;

		// - Check
		let task = TaskBmc::get(&ctx, &mm, id).await?;
		assert_eq!(task.title, fx_title);

		// - clean
		TaskBmc::delete(&ctx, &mm, id).await?;

		Ok(())
	}

	#[tokio::test]
	async fn test_create_err_not_found() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		let res = TaskBmc::get(&ctx, &mm, fx_id).await;
		assert!(matches!(
			res,
			Err(Error::EntityNotFound {
				entity: "task",
				id: 100
			})
		),);
		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_ok() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_titles = &["test_list_ok 01", "test_list_ok 02"];

		_dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

		let tasks = TaskBmc::list(&ctx, &mm).await?;

		let tasks: Vec<Task> = tasks
			.into_iter()
			.filter(|t| t.title.starts_with("test_list_ok"))
			.collect();
		assert_eq!(tasks.len(), 2, "number of seeded tasks");

		// Clean
		for task in tasks {
			TaskBmc::delete(&ctx, &mm, task.id).await?;
		}
		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_delete_err_not_found() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

		assert!(matches!(
			res,
			Err(Error::EntityNotFound {
				entity: "task",
				id: 100
			})
		));

		Ok(())
	}
}
