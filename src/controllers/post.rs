#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::{Order, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::_entities::posts::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub published: Option<bool>,
    pub slug: Option<String>,
    pub user_id: Option<Uuid>,
    pub published_at: Option<DateTimeWithTimeZone>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.content = Set(self.content.clone());
        item.summary = Set(self.summary.clone());
        item.published = Set(self.published.clone());
        item.slug = Set(self.slug.clone());
        item.user_id = Set(self.user_id);
        
        // Set published_at based on published value
        if let Some(published) = self.published {
            if published {
                item.published_at = Set(Some(chrono::Utc::now().into()));
            } else {
                item.published_at = Set(None);
            }
        } else {
            item.published_at = Set(self.published_at.clone());
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublishParams {
    pub published: bool,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    item.user_id = Set(Some(Uuid::parse_str(&auth.claims.pid).unwrap()));

    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut active_item = item.clone().into_active_model();
    
    // Create new params with the existing user_id if not provided
    let mut updated_params = params;
    if updated_params.user_id.is_none() {
        updated_params.user_id = item.user_id;
    }
    
    updated_params.update(&mut active_item);
    let item = active_item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn publish(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<PublishParams>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut active_item = item.into_active_model();
    
    active_item.published = Set(Some(params.published));
    // Update published_at timestamp when publishing
    if params.published {
        active_item.published_at = Set(Some(chrono::Utc::now().into()));
    } else {
        active_item.published_at = Set(None);
    }
    
    let item = active_item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

#[debug_handler]
pub async fn my_posts(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(
        Entity::find()
            // .filter(Entity::posts::Column::UserId.eq(Some(auth.claims.pid)))
            .filter(crate::models::_entities::posts::Column::UserId.eq(Uuid::parse_str(&auth.claims.pid).unwrap()))
            .order_by(crate::models::_entities::posts::Column::PublishedAt, Order::Desc)
            .all(&ctx.db)
            .await?,
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/posts/")
        .add("/", get(list))
        .add("/", post(add))
        .add(":id", get(get_one))
        .add(":id", delete(remove))
        .add(":id", put(update))
        .add(":id", patch(update))
        .add(":id/publish", patch(publish))
        .add("my", get(my_posts))
}
