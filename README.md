## Setup

### Clone this Repository

```shell
$ git clone https://github.com/skill-mind/FortiChain-Server
$ cd FortiChain-Server
```

### Installing Rust and Cargo

Install Rust as described in [The Rust Programming Language, chapter 1](https://doc.rust-lang.org/book/ch01-01-installation.html).

This is the official Rust language manual and is freely available on doc.rust-lang.org.

The latest stable version is fine.


### Installing `sqlx-cli`

SQLx provides a command-line tool for creating and managing databases as well as migrations. It is published
on the Cargo crates registry as `sqlx-cli` and can be installed like so:

```shell
$ cargo install sqlx-cli --features postgres,rustls --no-default-features --locked
```

### Running Postgres

By far the easiest way to run Postgres these days is using a container with [a pre-built image][docker-postgres].

The following command will start version 17 of Postgres (the latest at time of writing) using [Docker]
(this command should also work with [Podman], a daemonless FOSS alternative).

```shell
$ docker run -d --name postgres-17 -p 5432:5432 -e POSTGRES_PASSWORD={password} postgres:17
```

Set `{password}` to a password of your choosing.

Ensure the Postgres server is running:
```shell
$ docker ps
```
```shell
CONTAINER ID   IMAGE                COMMAND                  CREATED          STATUS          PORTS                                       NAMES
621eb8962016   postgres:17          "docker-entrypoint.s…"   30 seconds ago   Up 30 seconds   0.0.0.0:5432->5432/tcp, :::5432->5432/tcp   postgres-17
```

[docker-postgres]: https://hub.docker.com/_/postgres
[Docker]: https://www.docker.com/
[Podman]: https://podman.io/

### Configuring the Application

Configuring the backend application is done, preferentially, via environment variables. This is the easiest way
to pass sensitive configuration data like database credentials and HMAC keys in a deployment environment such as
[Kubernetes secrets].

To make working with environment variables easier during development, we can use [.env files] to avoid having
to define the variables every time.

As a starting point, you can simply `cp .env.sample .env` in this repo and modify the `.env` file as described by
the comments there.

[Kubernetes secrets]: https://kubernetes.io/docs/concepts/configuration/secret/
[.env files]: https://github.com/dotenv-rs/dotenv

### Setting Up the Application Database

With `sqlx-cli` installed and your `.env` file set up, you only need to run the following command to get the
Postgres database ready for use:

```
$ sqlx db setup
```

### Starting the Application

With everything else set up, all you should have to do at this point is:

```
$ cargo run
```

If successful, the FortiChain Server is now listening at port 8080.
