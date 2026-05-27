use aide::axum::{
  ApiRouter,
  routing::{delete_with, get_with, post_with, put_with},
};
use axum::Json;
use centaurus::{bail, db::init::Connection, error::Result};
use chrono::{DateTime, Utc};
use entity::sea_orm_active_enums::ApprovalState;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  auth::jwt_auth::JwtAuth,
  db::{DBTrait, vacation::Vacation},
  utils::{UpdateMessage, Updater, VacationManage},
};

pub fn router() -> ApiRouter {
  ApiRouter::new()
    .api_route("/", get_with(list_vacations, |op| op.id("listVacations")))
    .api_route(
      "/",
      post_with(create_vacation, |op| op.id("createVacation")),
    )
    .api_route(
      "/",
      delete_with(delete_vacation, |op| op.id("deleteVacation")),
    )
    .api_route(
      "/",
      put_with(set_vacation_state, |op| op.id("setVacationState")),
    )
}

async fn list_vacations(auth: JwtAuth, db: Connection) -> Result<Json<Vec<Vacation>>> {
  let models = db.vacation().list_vacations(auth.user_id).await?;
  Ok(Json(models))
}

#[derive(Deserialize, JsonSchema)]
struct CreateVacationRequest {
  start_date: DateTime<Utc>,
  end_date: DateTime<Utc>,
}

#[derive(Serialize, JsonSchema)]
struct CreateVacationRes {
  uuid: Uuid,
}

async fn create_vacation(
  auth: JwtAuth,
  db: Connection,
  updater: Updater,
  Json(req): Json<CreateVacationRequest>,
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

#[derive(Deserialize, JsonSchema)]
struct DeleteVacationRequest {
  uuid: Uuid,
}

async fn delete_vacation(
  _auth: JwtAuth<VacationManage>,
  db: Connection,
  updater: Updater,
  Json(req): Json<DeleteVacationRequest>,
) -> Result<()> {
  db.vacation().delete_vacation(req.uuid).await?;
  updater
    .broadcast(UpdateMessage::Vacation { uuid: req.uuid })
    .await;
  Ok(())
}

#[derive(Deserialize, JsonSchema)]
struct SetVacationStateRequest {
  uuid: Uuid,
  state: ApprovalState,
}

async fn set_vacation_state(
  _auth: JwtAuth<VacationManage>,
  db: Connection,
  updater: Updater,
  Json(req): Json<SetVacationStateRequest>,
) -> Result<()> {
  db.vacation()
    .set_vacation_state(req.uuid, req.state)
    .await?;
  updater
    .broadcast(UpdateMessage::Vacation { uuid: req.uuid })
    .await;
  Ok(())
}
