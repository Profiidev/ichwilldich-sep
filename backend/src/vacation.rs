use axum::{
  Json, Router,
  extract::FromRequest,
  routing::{delete, get, post, put},
};
use centaurus::{bail, db::init::Connection, error::Result};
use chrono::{DateTime, Utc};
use entity::sea_orm_active_enums::ApprovalState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  auth::jwt_auth::JwtAuth,
  db::{DBTrait, vacation::Vacation},
  permissions::VacationManage,
  ws::state::{UpdateMessage, Updater},
};

pub fn router() -> Router {
  Router::new()
    .route("/", get(list_vacations))
    .route("/", post(create_vacation))
    .route("/", delete(delete_vacation))
    .route("/", put(set_vacation_state))
}

async fn list_vacations(auth: JwtAuth, db: Connection) -> Result<Json<Vec<Vacation>>> {
  let models = db.vacation().list_vacations(auth.user_id).await?;
  Ok(Json(models))
}

#[derive(Deserialize, FromRequest)]
#[from_request(via(Json))]
struct CreateVacationRequest {
  start_date: DateTime<Utc>,
  end_date: DateTime<Utc>,
}

#[derive(Serialize)]
struct CreateVacationRes {
  uuid: Uuid,
}

async fn create_vacation(
  auth: JwtAuth,
  db: Connection,
  updater: Updater,
  req: CreateVacationRequest,
) -> Result<Json<CreateVacationRes>> {
  if req.end_date < req.start_date {
    bail!("End date must be after start date");
  }

  let model = db
    .vacation()
    .create_vacation(auth.user_id, req.start_date, req.end_date)
    .await?;
  updater
    .broadcast(UpdateMessage::Vacation { uuid: model.id })
    .await;
  Ok(Json(CreateVacationRes { uuid: model.id }))
}

#[derive(Deserialize, FromRequest)]
#[from_request(via(Json))]
struct DeleteVacationRequest {
  uuid: Uuid,
}

async fn delete_vacation(
  _auth: JwtAuth<VacationManage>,
  db: Connection,
  updater: Updater,
  req: DeleteVacationRequest,
) -> Result<()> {
  db.vacation().delete_vacation(req.uuid).await?;
  updater
    .broadcast(UpdateMessage::Vacation { uuid: req.uuid })
    .await;
  Ok(())
}

#[derive(Deserialize, FromRequest)]
#[from_request(via(Json))]
struct SetVacationStateRequest {
  uuid: Uuid,
  state: ApprovalState,
}

async fn set_vacation_state(
  _auth: JwtAuth<VacationManage>,
  db: Connection,
  updater: Updater,
  req: SetVacationStateRequest,
) -> Result<()> {
  db.vacation()
    .set_vacation_state(req.uuid, req.state)
    .await?;
  updater
    .broadcast(UpdateMessage::Vacation { uuid: req.uuid })
    .await;
  Ok(())
}
