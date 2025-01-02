use crate::db::game_schema::game::items::dsl::*;
use crate::db::models::game::{Item, NewItem};
use diesel::prelude::*;

pub struct ItemRepository;

impl ItemRepository {
    pub fn find_by_id(conn: &mut PgConnection, item_id: i64) -> QueryResult<Item> {
        items
            .filter(id.eq(item_id))
            .select(Item::as_select())
            .first(conn)
    }

    pub fn find_item_by_code(
        conn: &mut PgConnection,
        item_world_id: i64,
        item_code: &str,
    ) -> QueryResult<Item> {
        items
            .filter(world_id.eq(item_world_id))
            .filter(code.eq(item_code))
            .select(Item::as_select())
            .first(conn)
    }

    pub fn create(conn: &mut PgConnection, new_item: &NewItem) -> QueryResult<Item> {
        diesel::insert_into(items)
            .values(new_item)
            .returning(Item::as_select())
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, item: &Item) -> QueryResult<Item> {
        diesel::update(items)
            .filter(id.eq(&item.id))
            .set((
                world_id.eq(&item.world_id),
                code.eq(&item.code),
                category_id.eq(&item.category_id),
                name.eq(&item.name),
                description.eq(&item.description),
                base_price.eq(&item.base_price),
                item_properties.eq(&item.item_properties),
                item_type.eq(&item.item_type),
                updated_at.eq(&item.updated_at),
            ))
            .returning(Item::as_select())
            .get_result(conn)
    }

    pub fn create_or_update(conn: &mut PgConnection, item: &Item) -> QueryResult<Item> {
        if item.id == 0 {
            ItemRepository::create(conn, &item.as_new_item())
        } else {
            ItemRepository::update(conn, item)
        }
    }
}
