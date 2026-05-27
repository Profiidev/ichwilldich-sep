pub use sea_orm_migration::prelude::*;

mod m20260330_161154_vacation;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(centaurus::db::migrations::m0_key::Migration),
      Box::new(centaurus::db::migrations::m1_invalid_jwt::Migration),
      Box::new(centaurus::db::migrations::m2_settings::Migration),
      Box::new(centaurus::db::migrations::m3_user::Migration),
      Box::new(centaurus::db::migrations::m4_groups::Migration),
      Box::new(centaurus::db::migrations::m5_setup::Migration),
      Box::new(m20260330_161154_vacation::Migration),
    ]
  }
}
