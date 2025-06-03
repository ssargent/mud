# Aether Core MUD

A modern Multi-User Dungeon (MUD) game engine written in Rust, featuring a microservices architecture with real-time gameplay and REST API-based content management.

## Overview

Aether Core is a text-based multiplayer online role-playing game platform that combines the classic MUD experience with modern technology. The project is built using Rust for performance and safety, with a clean separation between game logic, server infrastructure, and client interfaces.

## Architecture

The project follows a modular crate-based architecture:

- **`server/`** - Main game server handling REST APIs and game state
- **`client/`** - Game client implementation
- **`logic/`** - Core game logic and mechanics
- **`protocol/`** - Communication protocol definitions
- **`tools/`** - Administrative tools for content management

## Features

### Game Features
- Character creation with classes and races
- Skill and feat systems
- Equipment and inventory management
- Enemy encounters and combat
- World exploration through interconnected nodes
- Real-time multiplayer interactions

### Technical Features
- RESTful API for content management
- Real-time game communication
- Database-backed persistence with Diesel ORM
- Migration-based schema management
- Modular architecture for extensibility

## Getting Started

### Prerequisites
- Rust (latest stable version)
- PostgreSQL database
- Diesel CLI for database management

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd aether-core
```

2. Install Diesel CLI:
```bash
cargo install diesel_cli --no-default-features --features postgres
```

3. Set up your environment variables:
```bash
cp .env.example .env
# Edit .env with your database configuration
```

4. Run database migrations:
```bash
diesel migration run
```

5. Build the project:
```bash
cargo build --release
```

### Running the Server

```bash
cargo run --bin server
```

### Content Management

Use the administrative tools to populate your game world:

```bash
cargo run --bin tools -- load-world path/to/world/data
```

## API Documentation

### Game World Management

The server provides REST APIs for managing game content:

- `PUT /api/worlds` - Create a new game world
- `PUT /api/worlds/<WORLD_ID>/races` - Create or update playable races
- `PUT /api/worlds/<WORLD_ID>/skills` - Create skills (crafting, hacking, etc.)
- `PUT /api/worlds/<WORLD_ID>/feats` - Create feats (Long Arms, Sniper Weapons, etc.)
- `PUT /api/worlds/<WORLD_ID>/equipment` - Create equipment, weapons, armor
- `PUT /api/worlds/<WORLD_ID>/enemies` - Create or update enemies
- `PUT /api/worlds/<WORLD_ID>/nodes` - Create world locations and rooms

For detailed API documentation, see [`server/readme.md`](server/readme.md).

## Development

### Project Structure

```
aether-core/
├── crates/           # Rust workspace crates
│   ├── client/       # Game client
│   ├── logic/        # Game logic
│   ├── protocol/     # Communication protocols
│   ├── server/       # Main server
│   └── tools/        # Administrative tools
├── data/             # Game world data
├── migrations/       # Database schema migrations
└── target/           # Build artifacts
```

### Database Schema

The project uses Diesel for database management with migrations located in the `migrations/` directory. The schema includes:

- Character classes and races
- Items and equipment
- Skills and feats
- World nodes and geography
- Enemy definitions

### Adding New Content

1. Create your content data files in the `data/` directory
2. Use the tools crate to load content:
   ```bash
   cargo run --bin tools -- load-world data/your-world.json
   ```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests where appropriate
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Enhanced combat system
- [ ] Player vs Player (PvP) mechanics
- [ ] Guild and party systems
- [ ] Crafting and economy
- [ ] Quest system
- [ ] Web-based administration interface
- [ ] Mobile client support

## Support

For questions, issues, or contributions, please open an issue on the project repository.

