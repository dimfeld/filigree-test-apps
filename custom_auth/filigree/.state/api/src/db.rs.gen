use error_stack::{Report, ResultExt};
use sqlx::PgPool;

use crate::Error;

/// Run the database migrations, if needed
pub async fn run_migrations(db: &PgPool) -> Result<(), Report<Error>> {
    sqlx::migrate!().run(db).await.change_context(Error::Db)
}
