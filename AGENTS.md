# AGENTS — How to work with RustDiscordBot (Rustey)

This guide captures the minimal, actionable knowledge an AI coding agent needs to be productive in this repository.

1) Big picture
- Component-driven Discord bot using the poise framework (serenity-based) + SurrealDB local store.
- Main wiring: `src/main.rs` — loads environment, builds list of `Component`s, gathers commands via `src/init.rs`, attaches `GlobalData` (contains `components` + Surreal `database`) to the poise framework, and starts the serenity client.
- Event dispatch: `src/core/events.rs::MainEventHandler` forwards every Serenity `FullEvent` to each component's `event_handler` but only if that component is enabled (checked in DB).

2) Core concepts & patterns (must-know to modify behavior)
- Component abstraction: `src/component.rs::Component` (fields: `id`, `commands: Vec<fn() -> Command<..>>`, `event_handler: Arc<dyn EventHandler>`, `initializer: Option<Initializer>`). New components must follow this shape.
- Commands are functions that return a `poise::Command` (stored as `fn() -> Command<GlobalData, Error>`). The repository sets `custom_data` on each command to a small `CommandData { component_id }` so commands can be associated to their component (see `src/init.rs::add_custom_data`).
- Component enable/disable: `src/core/mod.rs` — core commands and `command_check` use the SurrealDB-backed component record to decide if a command should run. Core component cannot be toggled.
- Initializers: components may provide an asynchronous initializer (`Initializer`) that runs at bot startup; core migrations (`core::database::migrate`) run before component initializers (see `src/main.rs` lines 42–57).

3) How to add a component (step-by-step)
- Create `src/components/your_component/` with `mod.rs` returning `Box<Component>` via a `pub fn component() -> Box<Component>`.
- Provide `id`, `commands` vector (functions returning `Command<GlobalData, Error>`), event handler (implement `serenity::EventHandler`) and optional `initializer` (if you need DB migrations or setup).
- Register it in `src/components/mod.rs::get_components()` (append to the vec). Examples: `src/components/todo/mod.rs` and `src/components/moderation/mod.rs`.
- If commands have subcommands, the `init::add_custom_data` recursion will attach `CommandData` to subcommands automatically.

4) Database & migrations
- DB is SurrealDB in local mode: created with `Surreal::new::<SurrealKv>(env.database_path)` then `use_ns`/`use_db` (see `src/main.rs`).
- Core and component migrations are functions invoked at startup. Look for `migrate` under `src/core/database.rs` and component `initializer`s (e.g. `src/components/todo/database.rs`).
- Component enabled/disabled flags are persisted in DB (`core::database::ComponentData` model). `GlobalData::is_component_enabled` queries DB on every command — there is a TODO to cache this.

5) Security / permissions conventions
- Owner-only admin commands use `#[poise::command(owners_only, ...)]` (e.g. `register_commands`) and core forbids toggling the `core` component.
- Per-command Discord permission checks use poise attributes like `required_permissions = "BAN_MEMBERS"` (see `src/components/moderation/commands.rs`).

6) Developer workflows (how to build / run / debug)
- Run locally (development): set `DISCORD_TOKEN` and optional vars then `cargo run` (see `README.md` and `src/environment.rs`). Example:
  export DISCORD_TOKEN="your_token"
  cargo run
- Build release: `cargo build --release` — binary at `./target/release/Rustey` (README shows `Rustey` binary name).
- Register global slash commands from an owner account with built-in command: call the `register_commands` command (prefix or slash) — implementation: `crate::core::commands::register_commands` calls `poise::builtins::register_globally`.
- Container: repository has Docker/compose metadata; README recommends `electric8steve/rustey` image. Use environment variables documented in README (`DISCORD_TOKEN`, `DB_PATH`, `DB_NAMESPACE`, `DB_DATABASE`, `PREFIX`).

7) Project-specific pitfalls & constraints
- Commands are stored as zero-argument functions returning `Command` (not closures with captures). This means you cannot compare command identities or store state on creation-time closures — follow the pattern used in `src/components/*`.
- `custom_data` is relied upon to determine the component for `command_check`. If custom_data is missing or cannot be downcast, `command_check` currently returns Ok(true) (see `src/core/mod.rs`).
- Component enable/disable check is synchronous per-command via DB query — expect latency and avoid relying on repeated checks in hot code paths.

8) Key files / reference locations
- Top-level: `README.md`, `Cargo.toml`, `compose.yaml`, `Dockerfile`
- Wiring & runtime: `src/main.rs`, `src/environment.rs`
- Component abstraction & command wiring: `src/component.rs`, `src/init.rs`, `src/types.rs`
- Core behavior: `src/core/mod.rs`, `src/core/commands.rs`, `src/core/events.rs`, `src/core/database.rs`
- Example components: `src/components/todo/`, `src/components/moderation/` (look at `mod.rs`, `commands.rs`, `database.rs`)

9) Quick tasks an AI agent can do immediately
- Add a new component scaffold (follow `todo` pattern). Update `get_components()`.
- Add a command: write a `fn() -> Command<GlobalData, Error>` using poise macros, push into component `commands` vec.
- Add a DB migration: implement `migrate` in the component's `database.rs` and expose it through `initializer`.

If you need more detail about any one file or want a template component created, tell me which name and I will scaffold it following the repository conventions.
