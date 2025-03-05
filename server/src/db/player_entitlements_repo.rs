use crate::db::models::player::{
    Entitlement, EntitlementMapping, NewEntitlement, NewEntitlementMapping,
};
use crate::db::player_schema::player::entitlement_mappings::dsl::*;
use crate::db::player_schema::player::entitlements::dsl::*;
use diesel::prelude::*;

pub struct PlayerEntitlementsRepository;

impl PlayerEntitlementsRepository {
    pub fn create_entitlement(
        conn: &mut PgConnection,
        new_entitlement: &NewEntitlement,
    ) -> QueryResult<Entitlement> {
        diesel::insert_into(entitlements)
            .values(new_entitlement)
            .returning(Entitlement::as_select())
            .get_result(conn)
    }

    pub fn create_entitlement_mapping(
        conn: &mut PgConnection,
        new_entitlement_mapping: &NewEntitlementMapping,
    ) -> QueryResult<EntitlementMapping> {
        diesel::insert_into(entitlement_mappings)
            .values(new_entitlement_mapping)
            .returning(EntitlementMapping::as_select())
            .get_result(conn)
    }

    pub fn get_active_entitlements_by_user_id(
        conn: &mut PgConnection,
        user_id_val: i64,
    ) -> QueryResult<Vec<Entitlement>> {
        entitlement_mappings
            .inner_join(entitlements)
            .filter(user_id.eq(user_id_val))
            .filter(start_date.le(diesel::dsl::now))
            .filter(end_date.is_null().or(end_date.ge(diesel::dsl::now)))
            .select(Entitlement::as_select())
            .load::<Entitlement>(conn)
    }

    pub fn get_all_entitlements_by_user_id(
        conn: &mut PgConnection,
        user_id_val: i64,
    ) -> QueryResult<Vec<Entitlement>> {
        entitlement_mappings
            .inner_join(entitlements)
            .filter(user_id.eq(user_id_val))
            .select(Entitlement::as_select())
            .load::<Entitlement>(conn)
    }

    pub fn get_entitlements_by_user_id(
        conn: &mut PgConnection,
        user_id_val: i64,
        active: bool,
    ) -> QueryResult<Vec<Entitlement>> {
        if active {
            Self::get_active_entitlements_by_user_id(conn, user_id_val)
        } else {
            Self::get_all_entitlements_by_user_id(conn, user_id_val)
        }
    }
}
