use sea_orm_migration::{
  prelude::{extension::postgres::Type, *},
  schema::*,
  sea_orm::DatabaseBackend,
};

use crate::m20260123_144752_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let backend = match manager.get_connection() {
      sea_orm_migration::SchemaManagerConnection::Connection(conn) => conn.get_database_backend(),
      sea_orm_migration::SchemaManagerConnection::Transaction(trans) => {
        trans.get_database_backend()
      }
    };

    if backend == DatabaseBackend::Postgres {
      manager
        .create_type(
          Type::create()
            .as_enum(ApprovalState::Enum)
            .values([
              ApprovalState::Pending,
              ApprovalState::Approved,
              ApprovalState::Rejected,
            ])
            .to_owned(),
        )
        .await?;
    }

    manager
      .create_table(
        Table::create()
          .table(Vacation::Table)
          .if_not_exists()
          .col(pk_uuid(Vacation::Id))
          .col(date_time(Vacation::Start))
          .col(date_time(Vacation::End))
          .col(uuid(Vacation::User))
          .col(custom(Vacation::Approval, ApprovalState::Enum))
          .foreign_key(
            ForeignKey::create()
              .from(Vacation::Table, Vacation::User)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Vacation::Table).to_owned())
      .await?;

    let backend = match manager.get_connection() {
      sea_orm_migration::SchemaManagerConnection::Connection(conn) => conn.get_database_backend(),
      sea_orm_migration::SchemaManagerConnection::Transaction(trans) => {
        trans.get_database_backend()
      }
    };

    if backend == DatabaseBackend::Postgres {
      manager
        .drop_type(Type::drop().name(ApprovalState::Enum).to_owned())
        .await?;
    }

    Ok(())
  }
}

#[derive(DeriveIden)]
enum ApprovalState {
  #[sea_orm(iden = "approval_state")]
  Enum,
  Pending,
  Approved,
  Rejected,
}

#[derive(DeriveIden)]
enum Vacation {
  Table,
  Id,
  Start,
  End,
  Approval,
  User,
}
