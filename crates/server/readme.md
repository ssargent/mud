# Mud Server

## APIs

### Rest API 

The Rest API handles administrative and content responsibilites.  All data is loaded through the API for the game world, character creation, etc..  There is also will be a set of commerce APIs that let players buy and sell items to NPCs via REST APIs.  These APIs will be web site accessible, and will not be directly callable by third parties.

#### Game World APIs

1. PUT `/api/worlds` *Create a new game world*
2. PUT `/api/worlds/<WORLD_ID>/races` *Create or update a playable race for the game world*
1. PUT `/api/worlds/<WORLD_ID>/skills` *Create a Skill for the game such as Crafting Armor, Computer Hacking, etc..*
2. PUT `/api/worlds/<WORLD_ID>/feats` *Create or update a feat such as Long Arms, Sniper Weapons etc...*
3. PUT `/api/worlds/<WORLD_ID>/equipment` *Create or update a piece of equipment, weapon, armor, tools, etc..*
4. PUT `/api/worlds/<WORLD_ID>/enemies` *Create or update an enemy for the game world*
5. PUT `/api/worlds/<WORLD_ID>/nodes` *Create or update a place in the world.  Could be a room of a dungeon, on the surface etc..*