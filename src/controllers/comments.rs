#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use axum::{debug_handler, extract::Query};
use uuid::Uuid;

use crate::models::_entities::comments::{ActiveModel, Column, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub content: Option<String>,
    pub post_id: Option<i32>,
    #[serde(skip_deserializing)]
    pub user_id: Option<Uuid>,
    pub parent_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryPostParams {
    pub post_id: u64,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.content = Set(self.content.clone());
        item.post_id = Set(self.post_id);
        if let Some(user_id) = self.user_id {
            item.user_id = Set(user_id);
        }
        item.parent_id = Set(self.parent_id);
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    Query(params): Query<QueryPostParams>,
    State(ctx): State<AppContext>
) -> Result<Response> {
    let post_id = params.post_id;
    format::json(Entity::find().filter(Column::PostId.eq(post_id)).all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>, 
    Json(mut params): Json<Params>
) -> Result<Response> {
    // Set the user_id from the auth token
    params.user_id = Some(Uuid::parse_str(&auth.claims.pid).unwrap());

    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    
    // Check if the user is the owner of the comment
    if item.user_id != Uuid::parse_str(&auth.claims.pid).unwrap() {
        return Err(Error::Unauthorized("..".to_owned()));
    }

    let mut active_item = item.into_active_model();
    params.update(&mut active_item);
    let item = active_item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(
    auth: auth::JWT,
    Path(id): Path<i32>, 
    State(ctx): State<AppContext>
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    
    // Check if the user is the owner of the comment
    if item.user_id != Uuid::parse_str(&auth.claims.pid).unwrap() {
        return Err(Error::Unauthorized("..".to_owned()));
    }

    item.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/comments/")
        .add("/", get(list))
        .add("/", post(add))
        .add(":id", get(get_one))
        .add(":id", delete(remove))
        .add(":id", put(update))
        .add(":id", patch(update))
}
