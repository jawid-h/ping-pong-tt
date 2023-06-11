# `Ping <- ∞ -> Pong` Client Server Test Task

## Original Task Description

Create a client-server ping-pong application with the following specs:

### Essential Requirements

* Preferably the Client should be written in RUST, but feel free to use any other language you are comfortable with or believe is best for this operation.
* The Server should be written in Python
* Communication protocol should be in WebTransport
* Provide some unit test coverage for both sides to demonstrate your skill in unit testing

### Desirable Requirements

* Make the communication channel secure or suggest what security measures you would implement given more time.
* Provide a plan for Kubernetes deployment
* Provide a plan/design for an auto-recovery mechanism for both sides (in case of a temporary connection failure). Feel free to implement that if you have enough time.
* Provide integration tests
* Can you think of a way for the client to auto-discover the server without the need to point it to the exact server endpoint?

## Implementation and Commentary

### Client and Server Implementation

Both client and server were implement in Rust with the help of `wtransport` crate. The motivation of choosing this crate over in-house implementation is that by looking into amount of work there it's been decided that it would take me a lot of time.

### Unit and Integration Testing

Most of the parts of the `common` crate have been covered with unit tests. Altough there are some difficulties testing `stream` module due to `SendStream` and `RecvStream` having private access to `new` function and unability to create those. This applies to testing `client` and `server` handlers since they are accepting `Connection` structure.

This could potentially could be solved by implementing async `trait` for `Connection` and modify behavior of `#accept_*` and `#open_*` functions to our needs and use it across `server` and `client`. Then we could use `mockall` crate to be able to pass mocked version of `Connection` as well as our implementation of `SendStream` and `RecvStream`.

### Security of Communication

The implementation uses TLS certificates to ensure secure communication, altough for simplicity of the testing client ignores them since it would require installing those in the root system storage.

### Recovery Mechanisms

On the `client` side recovery implemented using simple retry mechanism in case connection is not possible to establish.

On the `server` side however currently nothing is implemented due to time constraints. However it is possible to use some kind of message box for server since all `Message`s have ids and if `client` would announce its unique id we can keep all the undelivered responses until this very client reconnects and sending those back. Then client could compare `response.request_id` and `request.id` to verify original request has been processed.

### Kubernetes Deployment Strategy

I should state that I've never worked with Kubernetes. However reading about strategies and giving it a tought I've came to conclusion that `Canary Deployment` or `A/B Deployment` strategies would be sufficient. Altough being slightly different both involves having several versions of the application being deployed and traffic split between them. This would help keeping service uptime high enough and at the same time allow testing of the new version for a subset of new users.

I've tried to prepare example deployment files which are located under `kubernetes-example` directory of this very project.

### Auto-Discovery

There a many ways of achiving that, however considering we were talking about Kebernetes, there is built in `DNS Discovery` mechanism already. Regarding other solutions we could use `Service Discovery Services` like `Apache Zookeper` for example which will help routing trafic to healthy nodes as well having ability to healthcheck those. Or as a simpliest solution we could use `API Gateway` acting as a single seed node and then route the trafic to other nodes. I suppose this is not the full list of possible solutions though.

## Usage and Requirements

### Requirements

To be able to work on or run the project you would need to install latest version of Rust. The easiest way to isntall it is to use [Rustup](https://rustup.rs/).

Then, you would be able to use `cargo` command to operate. For example:

```sh
$ cargo build # to build the project
$ cargo clippy # to lint the project
$ cargo test # to run the project's unit and integration tests
```

To run the `server` first of all you'd need to generate certificate and key files:

```sh
$ cargo run --bin cli gen-certs # that would generate cert.pem and key.pem in the current working directory
```

Then run the server:

```sh
$ cargo run --bin cli server # run server with default settings
$ cargo run --bin cli server --help # to explore available parameters
```

After that you could run `client` using:

```sh
$ cargo run --bin cli client # with default settings
$ cargo run --bin cli client --help # to explore available parameters
```

