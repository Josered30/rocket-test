use chrono::NaiveDateTime;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};

use crate::cores::errors::ApiError;
use crate::domain::models::item;
use crate::domain::models::item::Entity as Item;

pub struct ItemInfo {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl ItemInfo {
    pub fn new(
        id: i32,
        name: String,
        created_at: NaiveDateTime,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            created_at,
            updated_at,
        }
    }
}

pub struct ItemService;

impl ItemService {
    pub async fn create_item(db: &DbConn, name: String, user_id: i32) -> Result<i32, ApiError> {
        let result = item::ActiveModel {
            name: Set(name),
            ..Default::default()
        }
        .save(db)
        .await?;
        return Ok(result.id.unwrap());
    }

    pub async fn get_items(db: &DbConn, user_id: i32) -> Result<Vec<ItemInfo>, ApiError> {
        let result = Item::find()
            .filter(item::Column::UserId.eq(user_id))
            .all(db)
            .await?;

        let items = result
            .into_iter()
            .map(|item| ItemInfo::new(item.id, item.name, item.created_at, item.updated_at))
            .collect::<Vec<ItemInfo>>();
        return Ok(items);
    }
}
