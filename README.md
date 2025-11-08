# Axum + SeaORM Starter

Minimal REST API using Axum, SeaORM, and tracing.

## Quickstart

```bash
docker compose up -d
cargo run
```

## Cargo Commands

```bash
cargo fmt            # Format the codebase
cargo clippy         # Run Clippy static analysis
cargo test           # Execute the test suite
cargo build          # Compile the project without running it
cargo run            # Build and run the binary
cargo check          # Type-check without producing binaries
cargo clean          # Remove build artifacts in target/
cargo update         # Update dependency lockfile to latest compatible versions
```

## cargo-edit

Installation:

```bash
cargo install cargo-edit           # Install cargo-edit subcommands
```

Commands:

```bash
cargo add <package_name>           # Add a dependency to Cargo.toml
cargo rm <package_name>            # Remove a dependency from Cargo.toml
cargo upgrade <package_name>       # Upgrade a dependency (or all if omitted)
cargo set-version 1.2.3            # Update the package version in Cargo.toml
cargo edit                         # Show cargo-edit help
```

## Endpoints

```bash
# Create user
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com"}'

# List users
curl http://localhost:3000/api/users

# Get user by ID
curl http://localhost:3000/api/users/1

# Update user email
curl -X PUT http://localhost:3000/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@newmail.com"}'

# Delete user
curl -X DELETE http://localhost:3000/api/users/1

# Health check
curl http://localhost:3000/api/healthz
```
