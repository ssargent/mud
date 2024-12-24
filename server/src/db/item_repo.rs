use crate::db::game_schema::game::items::dsl::*;
use crate::db::models::game::Item;
use diesel::prelude::*;

pub struct ItemRepository;

impl ItemRepository {
    pub fn find_by_id(conn: &mut PgConnection, item_id: i64) -> QueryResult<Item> {
        items
            .filter(id.eq(item_id))
            .select(Item::as_select())
            .first(conn)
    }

    pub fn create_or_update(conn: &mut PgConnection, item: &Item) -> QueryResult<Item> {
        diesel::insert_into(items)
            .values(item)
            .on_conflict(id)
            .do_update()
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
}
