-- System
-- name: GetSettingByName :one
select * from system.settings where name = $1;

-- name: CreateOrUpdateSetting :one
insert into system.settings (id, name, data_type, value)
values ( $1, $2, $3, $4)
on conflict (name) do update set value = $4, data_type = $3
returning *;

-- name: GetSettings :many
select * from system.settings;

-- name: DeleteSettingByName :exec
delete from system.settings where name = $1;

-- name: GetSettingByID :one
select * from system.settings where id = $1;

-- Users
-- name: CreateUser :one
insert into system.users (username, password, email, full_name)
values ($1, $2, $3, $4)
returning *;

-- name: GetUserByUsername :one
select * from system.users where username = $1;

-- name: GetUserByID :one
select * from system.users where id = $1;
