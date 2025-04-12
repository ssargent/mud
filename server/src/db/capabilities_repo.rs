use crate::db::game_schema::game::capabilities::dsl::*;
use diesel::prelude::*;

use super::game::{Capability, NewCapability};

pub struct CapabilitiesRepository;

impl CapabilitiesRepository {
    pub fn find_by_id(conn: &mut PgConnection, capability_id: i64) -> QueryResult<Capability> {
        capabilities
            .filter(id.eq(capability_id))
            .select(Capability::as_select())
            .first(conn)
    }

    pub fn find_by_code(
        conn: &mut PgConnection,
        world_id_value: i64,
        capability_code: &str,
    ) -> QueryResult<Capability> {
        capabilities
            .filter(world_id.eq(world_id_value))
            .filter(code.eq(capability_code))
            .select(Capability::as_select())
            .first(conn)
    }

    pub fn find_by_type(
        conn: &mut PgConnection,
        world_id_val: i64,
        capability_type_val: &str,
    ) -> QueryResult<Vec<Capability>> {
        capabilities
            .filter(capability_type.eq(capability_type_val))
            .filter(world_id.eq(world_id_val))
            .select(Capability::as_select())
            .load::<Capability>(conn)
    }

    pub fn create(
        conn: &mut PgConnection,
        new_capability: &NewCapability,
    ) -> QueryResult<Capability> {
        diesel::insert_into(capabilities)
            .values(new_capability)
            .returning(Capability::as_select())
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, capability: &Capability) -> QueryResult<Capability> {
        diesel::update(capabilities)
            .filter(id.eq(&capability.id))
            .set((
                code.eq(&capability.code),
                name.eq(&capability.name),
                description.eq(&capability.description),
                requirements.eq(&capability.requirements),
                access_requirements.eq(&capability.access_requirements),
                actions.eq(&capability.actions),
                gameplay_definition.eq(&capability.gameplay_definition),
                tags.eq(&capability.tags),
                updated_at.eq(diesel::dsl::now),
            ))
            .returning(Capability::as_select())
            .get_result(conn)
    }

    pub fn create_or_update(
        conn: &mut PgConnection,
        capability: &Capability,
    ) -> QueryResult<Capability> {
        if capability.id == 0 {
            CapabilitiesRepository::create(conn, &capability.as_new_capability())
        } else {
            CapabilitiesRepository::update(conn, capability)
        }
    }
}
