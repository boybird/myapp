#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::{Path, Query, State};
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::_entities::posts::{ActiveModel, Column, Entity, Model};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct PostListItem {
    pub id: i32,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub published: Option<bool>,
    pub slug: Option<String>,
    pub user_id: Option<Uuid>,
    pub published_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<Model> for PostListItem {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            summary: model.summary,
            published: model.published,
            slug: model.slug,
            user_id: model.user_id,
            published_at: model.published_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page.max(1) - 1;
    let page_size = params.page_size.max(1);

    let paginator = Entity::find()
        .order_by_desc(Column::PublishedAt)
        .paginate(&ctx.db, page_size);

    let total = paginator.num_items().await?;
    let total_pages = (total + page_size - 1) / page_size;
    let items: Vec<PostListItem> = paginator
        .fetch_page(page)
        .await?
        .into_iter()
        .map(PostListItem::from)
        .collect();

    format::json(PaginatedResponse {
        items,
        total,
        page: page + 1,
        page_size,
        total_pages,
    })
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
pub async fn my_posts(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<PaginationParams>,
) -> Result<Response> {
    let page = params.page.max(1) - 1;
    let page_size = params.page_size.max(1);

    let paginator = Entity::find()
        .filter(Column::UserId.eq(Uuid::parse_str(&auth.claims.pid).unwrap()))
        .order_by_desc(Column::PublishedAt)
        .paginate(&ctx.db, page_size);

    let total = paginator.num_items().await?;
    let total_pages = (total + page_size - 1) / page_size;
    let items: Vec<PostListItem> = paginator
        .fetch_page(page)
        .await?
        .into_iter()
        .map(PostListItem::from)
        .collect();

    format::json(PaginatedResponse {
        items,
        total,
        page: page + 1,
        page_size,
        total_pages,
    })
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
