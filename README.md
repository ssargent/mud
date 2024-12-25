# mud
A MUD written in ~~RUST~~ **GO**.


## Tasks
1. Sanitize DB Structures
   1. Recreate tables and reorder columns to a more sane format
   2. Migrate **ALL** fields to i64 instead of UUID
2. Create api/worlds/<world_id>/items APIs
3. Implement Structs that are **NOT** database specific.  ie a Game version of Item instead of GameItem which mirrors db exactly.