// src/migrator/m20220602_000001_create_Oauth2Iden_table.rs (create new file)

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "create_oauth_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Oauth2Iden table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Oauth2Record::Table)
                    .col(
                        ColumnDef::new(Oauth2Record::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Oauth2Record::Provider).string().not_null())
                    .col(ColumnDef::new(Oauth2Record::Name).string().not_null())
                    .col(ColumnDef::new(Oauth2Record::Email).string().not_null())
                    .col(ColumnDef::new(Oauth2Record::AccessToken).string().not_null())
                    .col(ColumnDef::new(Oauth2Record::RefreshToken).string().not_null())
                    .col(
                        ColumnDef::new(Oauth2Record::CreateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Oauth2Record::UpdateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Oauth2Record::ExpierTime).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Oauth2Iden table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Oauth2Record::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Oauth2Record {
    Table,
    Id,
    Name,
    Email,
    Provider,
    AccessToken,
    RefreshToken,
    CreateTime,
    UpdateTime,
    ExpierTime,
}
