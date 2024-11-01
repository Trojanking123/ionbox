mod m20241031_000001_create_oauth_table;

use sea_orm::{prelude::*, Database};
use sea_orm_migration::prelude::*;

use crate::IonResult;

struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241031_000001_create_oauth_table::Migration)]
    }
}

pub async fn refresh_db(db_file: &str) -> IonResult<()> {
    let db = Database::connect(db_file).await.unwrap();
    let db = &db;
    Migrator::refresh(db).await.unwrap();
    Ok(())
}
