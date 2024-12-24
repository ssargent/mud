use crate::db::game_schema::game::item_categories::dsl::*;
use crate::db::models::game::ItemCategory;
use diesel::prelude::*;

pub struct ItemCategoryRepository;

impl ItemCategoryRepository {
    pub fn find_by_id(conn: &mut PgConnection, item_category_id: i64) -> QueryResult<ItemCategory> {
        item_categories
            .filter(id.eq(item_category_id))
            .select(ItemCategory::as_select())
            .first(conn)
    }

    pub fn create_or_update(
        conn: &mut PgConnection,
        item_category: &ItemCategory,
    ) -> QueryResult<ItemCategory> {
        diesel::insert_into(item_categories)
            .values(item_category)
            .on_conflict(id)
            .do_update()
            .set((
                parent_id.eq(&item_category.parent_id),
                name.eq(&item_category.name),
                description.eq(&item_category.description),
                updated_at.eq(&item_category.updated_at),
            ))
            .returning(ItemCategory::as_select())
            .get_result(conn)
    }
}
