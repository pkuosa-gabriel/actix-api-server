# actix-api-server (code name: _lycanthropy_)

A RESTful API server based on Actix.rs

## Run

First, let's add Diesel to our dependencies. We're also going to use a tool called .env to manage our environment variables for us. We'll add it to our dependencies as well.

```toml
// in Cargo.toml

[dependencies]
diesel = { version = "1.0.0", features = ["postgres"] }
dotenv = "0.9.0"
```

Diesel provides a separate CLI tool to help manage your project. Since it's a standalone binary, and doesn't affect your project's code directly, we don't add it to Cargo.toml. Instead, we just install it on our system.

```sh
cargo install diesel_cli
```

We need to tell Diesel where to find our database. We do this by setting the DATABASE_URL environment variable. On our development machines, we'll likely have multiple projects going, and we don't want to pollute our environment. We can put the url in a .env file instead.

```sh
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

Now you are able to play with the existing code in the repository in your local environment.