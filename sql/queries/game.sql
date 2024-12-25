-- Items
-- name: GetItemByID :one
select * from game.items where id = $1;

-- name: GetItemByCode :one
select * from game.items where code = $1 and world_id = $2;

-- name: CreateOrUpdateItem :one
insert into game.items (id, category_id,world_id, code, name, description, item_properties, base_price)
values ( $1, $2, $3, $4, $5, $6, $7, $8)
on conflict (code, world_id) do update set category_id = $2, world_id = $3,code = $4, name = $5, description = $6, item_properties = $7, base_price = $8
returning *;
