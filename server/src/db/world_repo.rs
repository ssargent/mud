use crate::db::game_schema::game::worlds::dsl::*;
use crate::db::models::game::{NewWorld, World};
use diesel::prelude::*;

pub struct WorldRepository;

impl WorldRepository {
    pub fn find_by_code(conn: &mut PgConnection, world_code: &str) -> QueryResult<World> {
        worlds
            .filter(code.eq(world_code))
            .select(World::as_select())
            .first(conn)
    }

    pub fn create(conn: &mut PgConnection, new_world: &NewWorld) -> QueryResult<World> {
        diesel::insert_into(worlds)
            .values(new_world)
            .returning(World::as_select())
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, world: &World) -> QueryResult<World> {
        diesel::update(worlds)
            .filter(id.eq(&world.id))
            .set((
                name.eq(&world.name),
                description.eq(&world.description),
                updated_at.eq(&world.updated_at),
            ))
            .returning(World::as_select())
            .get_result(conn)
    }

    pub fn create_or_update(conn: &mut PgConnection, world: &World) -> QueryResult<World> {
        if world.id == 0 {
            WorldRepository::create(conn, &world.as_new_world())
        } else {
            WorldRepository::update(conn, world)
        }
    }
}
