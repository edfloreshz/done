use anyhow::{anyhow, bail, Result};
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use diesel_migrations::{
	embed_migrations, EmbeddedMigrations, MigrationHarness,
};
use libset::{Config, FileType};

pub mod models;

pub type Pool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Debug, Clone)]
pub struct Database {
	application_id: String,
	pool: Option<Pool>,
}

impl Database {
	pub fn new(application_id: String) -> Result<Self> {
		let database = Self {
			application_id,
			pool: None,
		};

		database
			.ensure_database_exists()
			.expect("Failed to ensure database exists.");

		Ok(database)
	}

	pub fn database_url(&self) -> Result<String> {
		let app_id = self.application_id.clone();
		let url = Config::new(&app_id, 1, Some("database"))?
			.path(&format!("{app_id}.db"), libset::FileType::Plain)?
			.display()
			.to_string();
		Ok(url)
	}

	pub fn establish_connection(
		&mut self,
	) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
		if self.pool.is_none() {
			let manager =
				ConnectionManager::<SqliteConnection>::new(self.database_url()?);
			let pool = Pool::builder()
				.build(manager)
				.expect("Failed to create pool");
			self.pool = Some(pool);
		}

		let Some(pool) = &self.pool else {
			bail!("Failed to get pool");
		};

		self
			.ensure_migrations_up_to_date()
			.expect("Failed to ensure migrations are up to date");

		pool.get().map_err(|e| anyhow!(e))
	}

	pub fn ensure_migrations_up_to_date(&self) -> Result<()> {
		let Some(pool) = &self.pool else {
			bail!("Failed to get pool");
		};

		let mut connection = pool.get()?;
		match connection.run_pending_migrations(MIGRATIONS) {
			Ok(_) => Ok(()),
			Err(err) => {
				tracing::error!("{err}");
				Err(anyhow!(err))
			},
		}
	}

	pub fn ensure_database_exists(&self) -> Result<()> {
		let app_id = self.application_id.clone();
		let database_config = Config::new(&app_id, 1, Some("database"))?;

		let database_path =
			database_config.path(&format!("{app_id}.db"), FileType::Plain)?;

		if !database_path.exists() {
			database_config.set_plain(&format!("{app_id}.db"), String::new())?;
		}
		Ok(())
	}
}
