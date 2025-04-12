use super::game::{Enemy, NewEnemy};
use crate::db::game_schema::game::enemies::dsl::*;
use diesel::prelude::*;

pub struct EnemyRepository;

impl EnemyRepository {
    pub fn find_by_id(conn: &mut PgConnection, enemy_id: i64) -> QueryResult<Enemy> {
        enemies
            .filter(id.eq(enemy_id))
            .select(Enemy::as_select())
            .first(conn)
    }

    pub fn find_by_code(
        conn: &mut PgConnection,
        enemy_world_id: i64,
        enemy_code: &str,
    ) -> QueryResult<Enemy> {
        enemies
            .filter(world_id.eq(enemy_world_id))
            .filter(code.eq(enemy_code))
            .select(Enemy::as_select())
            .first(conn)
    }

    pub fn create(conn: &mut PgConnection, new_enemy: &NewEnemy) -> QueryResult<Enemy> {
        diesel::insert_into(enemies)
            .values(new_enemy)
            .returning(Enemy::as_select())
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, enemy: &Enemy) -> QueryResult<Enemy> {
        diesel::update(enemies)
            .filter(id.eq(&enemy.id))
            .set((
                world_id.eq(&enemy.world_id),
                code.eq(&enemy.code),
                name.eq(&enemy.name),
                description.eq(&enemy.description),
                level.eq(&enemy.level),
                hit_points.eq(&enemy.hit_points),
                stamina.eq(&enemy.stamina),
                strength.eq(&enemy.strength),
                dexterity.eq(&enemy.dexterity),
                constitution.eq(&enemy.constitution),
                intelligence.eq(&enemy.intelligence),
                wisdom.eq(&enemy.wisdom),
                weapons.eq(&enemy.weapons),
                armor.eq(&enemy.armor),
                updated_at.eq(&enemy.updated_at),
            ))
            .returning(Enemy::as_select())
            .get_result(conn)
    }

    pub fn create_or_update(conn: &mut PgConnection, enemy: &Enemy) -> QueryResult<Enemy> {
        if enemy.id == 0 {
            EnemyRepository::create(conn, &enemy.as_new_enemy())
        } else {
            EnemyRepository::update(conn, enemy)
        }
    }
}
