# baam

[![Rustfmt](https://github.com/DCNick3/baam/actions/workflows/rust-check.yml/badge.svg)](https://github.com/DCNick3/baam/actions/workflows/rust-check.yml)

(🔥 blazing 🚀 fast 💿 memory 😷 safe 🦀) Best automated attendance monitoring

## Running

### Prerequisites

- Rust toolchain (stable) and `cargo` installed.
  - [rustup](https://rustup.rs/) is recommended.
- native libraries for `openssl-sys` and `pq-sys` installed.
  - `libssl-dev` and `libpq-dev` on Debian/Ubuntu
  - `openssl-devel` and `postgresql-devel` on Fedora
  - `openssl` and `postgresql` on macOS (via [brew](https://brew.sh/))
- npm and nodejs installed.
- docker installed for running the database.

### Building

```bash
cargo build
```

### Running

Start the database:

```bash
docker run --name baam-postgres -e POSTGRES_HOST_AUTH_METHOD=trust --rm -it -p 5432:5432 postgres
```

Run the server in root directory of the repo (it's needed to find the config files):

```bash
ENVIRONMENT=dev DATABASE_URL=postgres://postgres@localhost/postgres \
  cargo run
```

### Frontend development

First install the dependencies:

```bash
npm install
```

To start the frontend development server, run:

```bash
cd frontend
npm run dev 
```

Now you can access the frontend at http://localhost:5173/.

You can also run the backend in `front-dev` mode and access the frontend at http://localhost:8080/ (the backend will proxy the frontend requests):

```bash
ENVIRONMENT=front-dev DATABASE_URL=postgres://postgres@localhost/postgres \
  cargo run
```