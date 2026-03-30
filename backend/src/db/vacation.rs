use chrono::Utc;
use entity::{prelude::*, sea_orm_active_enums::ApprovalState, vacation};
use sea_orm::{ActiveValue::Set, IntoActiveModel, prelude::*};
use serde::Serialize;

use crate::{
  db::group::GroupTable,
  permissions::{Permission, VacationManage},
};

#[derive(Serialize)]
pub struct Vacation {
  uuid: Uuid,
  start_date: chrono::DateTime<Utc>,
  end_date: chrono::DateTime<Utc>,
  approval: ApprovalState,
  user: String,
}

pub struct VacationTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> VacationTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn list_vacations(&self, user: Uuid) -> Result<Vec<Vacation>, DbErr> {
    let all = GroupTable::new(self.db)
      .user_hash_permissions(user, VacationManage::name())
      .await?;

    let models = if all {
      vacation::Entity::find()
        .find_with_related(User)
        .all(self.db)
        .await?
    } else {
      vacation::Entity::find()
        .find_with_related(User)
        .filter(vacation::Column::User.eq(user))
        .all(self.db)
        .await?
    };

    Ok(
      models
        .into_iter()
        .map(|(v, user)| {
          let user = user[0].name.clone();
          Vacation {
            uuid: v.id,
            start_date: chrono::DateTime::from_naive_utc_and_offset(v.start, Utc),
            end_date: chrono::DateTime::from_naive_utc_and_offset(v.end, Utc),
            approval: v.approval,
            user,
          }
        })
        .collect(),
    )
  }

  pub async fn create_vacation(
    &self,
    user: Uuid,
    start_date: chrono::DateTime<Utc>,
    end_date: chrono::DateTime<Utc>,
  ) -> Result<vacation::Model, DbErr> {
    let new_vacation = vacation::ActiveModel {
      id: Set(Uuid::new_v4()),
      user: Set(user),
      start: Set(start_date.naive_utc()),
      end: Set(end_date.naive_utc()),
      approval: Set(ApprovalState::Pending),
    };

    new_vacation.insert(self.db).await
  }

  pub async fn delete_vacation(&self, uuid: Uuid) -> Result<(), DbErr> {
    vacation::Entity::delete_by_id(uuid).exec(self.db).await?;
    Ok(())
  }

  pub async fn set_vacation_state(&self, uuid: Uuid, state: ApprovalState) -> Result<(), DbErr> {
    let mut vacation = vacation::Entity::find_by_id(uuid)
      .one(self.db)
      .await?
      .ok_or_else(|| DbErr::Custom(format!("Vacation with id {} not found", uuid)))?
      .into_active_model();

    vacation.approval = Set(state);
    vacation.update(self.db).await?;
    Ok(())
  }
}
