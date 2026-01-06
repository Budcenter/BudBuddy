# BudBuddy
BudBuddy is a discord bot written entirely in Rust.

Built by stoners, for stoners.

## Features
- Searching strains by name, flavor, effects and more
- Over 12,000+ unique strains

## Tech Stack
- [Rust](https://www.rust-lang.org/) - for its type safety and performance
- [Serenity](https://github.com/serenity-rs/serenity) - Discord Client
- [Poise](https://github.com/serenity-rs/poise) - Discord Bot Framework
- [Sqlx](https://github.com/launchbadge/sqlx) - SQL query validator and pooler
- [Postgres](https://www.postgresql.org/) - Database
- [Tracing](https://github.com/tokio-rs/tracing) - Logging

## Setup
This project uses nix and devenv to manage the developer environment. This includes setting up the postgres database. You can probably build this and get it to work without using these, however unless someone does the work and makes a PR, nix is going to be the only officially supported setup.

### Prerequisites
- Nix package manager (see [official installation guide](https://nixos.org/download))
- devenv tool (see [official installation guide](https://devenv.sh/getting-started/#installation))

### Quick Start
1. Clone the repository
2. Navigate to project directory
3. Enter development environment:
    ```bash
    devenv shell
    ```
4. Verify installation:
    ```bash
    cargo -V && sqlx -V
    ```
5. Rename `.env.example` to `.env` and set your own values.
6. Start PostgreSQL:
    ```bash
    devenv up
    ```
7. Create DB and run migrations:
    ```bash
    sqlx db setup
    ```


### Detailed Steps

#### Project Setup
After cloning, navigate to project directory:
```bash
cd budbuddy
```

Then enter development environment:
```bash
devenv shell
```

### Troubleshooting
If you encounter issues:
1. Verify nix installation
    ```bash
    nix --version
    ```
2. Verify devenv installation:
    ```bash
    devenv --version
    ```
3. If sqlx returns the following error:
    ```bash
    error: error returned from database: database "budbuddy" is being accessed by other users
    ```
    Then most likely rust-analyzer is using the DB to validate inline SQL. If you set `SQLX_OFFLINE` to true, it will use the prepared queries in `.sqlx/` instead of the actual DB for compilation, allowing you to run whatever sqlx command that threw the error in the first place.

    If these queries are not present, or are outdated, please run the following:
    ```bash
    cargo sqlx prepare
    ```

### Leaving Environment
Exit development shell:
```bash
exit
```