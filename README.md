# rust-restful-example

This project provides an MVC-like restful server example. Like what you may have seen in any other Java's Spring Boot project, it is organized in structure with some classic elements like controller, service, and dao. If you want a quick start of a rust-base web application example, this one may be what you are looking for. Futher more, you can simple modify a few codes to make it your own application for production.

This projcet includes following useful features:

- Good project structrue thanks to Rust and MVC idea.
- Understandable codes written in plain Rust.
- Simple but useful command line options.
- A complete dockerfile to make container image.

This project acts as a AK/SK data operator. You may adapt it to any logic of your own, if want you to use it as your porject.

## Get Stated
### Complie & Run

As a Rustacean, you can compile or run the project easily by using `cargo build` or `cargo run`.

This repository use [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) as the build system. If you want to get binary from source, simply use `cargo build`. You may refer to the `Cargo`'s documantations for more build options like target platform and optimization level.

To run this project by `cargo`, use command like:

```shell
cargo run -- -c res/config.yaml
```

This project use [Clap](https://github.com/clap-rs/clap) as the command line argument parser. Thanks to Clap, we can easily access the help infomation like:

```
cargo run -- --help

USAGE:
    rust-restful-example.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    [default: config.yaml]
```

### Use Config File

The configuartion file used but this project is organized like below (provided in `res/config.yaml`):

```yaml
log_level: info # the actix's log level
mongo_uri: mongodb://mongo1:27017,mongo2:27017,mongo3:27017 # to connnect to your mongoDB
port: 8084  # where the actix will be running on
```

## Code Structure Detail

In this section, we will talk about the code structure used by this easy example project.

If your application has only a few functional modules, you may use the flat code structure style like below:

```
src
  └─app
      ├─cli # command line related codes
      ├─config
      ├─constants
      ├─controller
      ├─dao
      ├─entity
      └─service
```

If you are going to add a lot of other functional modules, the following code structure is recommended:

```
src
  └─app
      ├─cli
      ├─config
      ├─constants   # global constants
      └─module
          |-moduleA
          |-moduleB
          |-...
          └─moduleN
              ├─constants   # module related constants
              ├─controller
              ├─dao
              ├─entity
              └─service
```

Anyway, take the one that fits you and your team best. You can also use the style from any other MVC-like project.

### Controller

The `Controller` is described in `src/app/module/[ModuleName]/controller`.

[actix-web](https://github.com/actix/actix-web) helps to build up the project's API router. In the api-auth controller, `api_auth_config` helps to setup the actix's service config, and the  restful APIs are described in it like below:

```rust
pub fn api_auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api-auth")
            .route(web::get().to(query))
            .route(web::post().to(create))
            .route(web::put().to(update_by_id))
            .route(web::delete().to(delete_by_id)),
    );
}
```
This controller use functions defined in the same source code file to call the `Service` level codes.

For more details, see the codes in [`src/app/module/api_auth/controller/mod.rs`](src/app/module/api_auth/controller/mod.rs).

If you want to add your own controller, just add a *.rs code file to `src/app/module/[ModuleName]/controller`.

### Service

After the api-auth controller, it's time to describe the bussiness related codes in `src/app/module/[ModuleName]/service`.

In the api-auth's `Service` level, a struct `APIAuthService` is defined. The `APIAuthService` is a struct with empty data members but also a collection of bussiness related functions. Here the DTO passed in by upper controller is examinted, converted, or extracted for the next following database related operations.

The api-auth related service is defined like below:

```rust
pub struct APIAuthService {}

impl APIAuthService {
    pub async fn get_by_id(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        // ...
    }
    pub async fn get_by_cond(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        // ...
    }
    pub async fn create(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        // ...
    }
    pub async fn update_by_id(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        // ...
    }
    pub async fn delete_by_id(&self, mg_cli: web::Data<Client>, dto: APIAuthInfo) -> HttpResponse {
        // ...
    }
}
```

For more details, see the codes in [`src/app/module/api_auth/service/mod.rs`](src/app/module/api_auth/service/mod.rs).

If you want to add your own service, just add a *.rs code file to `src/app/module/[ModuleName]/service`.

### DAO

The `DAO` describes the operations to the data source. You may have already noticed that there this project doesn't used any ORM framwork like MyBatis or JAP in Java. This project directly operated the database client because the data backend we used here is MongoDB, a nosql database. And, thanks to `actix-web`, the only one mongo client can be initialized in the very beginning of the application and passed to anywhere in our code by using `actix_web::web::Data<T>`. To see how to share a object through all the actix's handle steps, see the `State` section [here](https://actix.rs/docs/application).

The api-auth related dao is defined like below:

```rust
pub struct APIAuthDAO {
    pub client: web::Data<Client>,
}

impl APIAuthDAO {
    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<APIAuthInfo>, Error> {
        // ...
    }
    pub async fn find_all(&self, cond: &APIAuthInfo) -> Result<Vec<APIAuthInfo>, Error> {
        // ...
    }
    pub async fn insert_one(&self, entity: &APIAuthInfo) -> Result<APIAuthInfo, Error> {
        // ...
    }
    pub async fn insert_one(&self, entity: &APIAuthInfo) -> Result<APIAuthInfo, Error> {
        // ...
    }
    pub async fn update_by_id(&self, entity: &APIAuthInfo) -> Result<APIAuthInfo, Error> {
        // ...
    }
    pub async fn delete_one(&self, id: &ObjectId) -> Result<(), Error> {
        // ...
    }
}
```

For more details, see the codes in [`src/app/module/api_auth/dao/mod.rs`](src/app/module/api_auth/dao/mod.rs). Some of the codes in it are commented in it to helps the newcomers of Rust to understand the syntax sugar.

If you want to add your own dao, just add a *.rs code file to `src/app/module/[ModuleName]/dao`.

### Entity

The `Entity` describes the structure of the records in MongoDB. The [serde](https://github.com/serde-rs/serde) together with the procedural macro feature in rust are awesome helpers for developers to create serializable structures.

An api-auth entity is quickly defined like below:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIAuthInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "AK", skip_serializing_if = "Option::is_none")]
    pub ak: Option<String>,
    #[serde(rename = "APP", skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "API", skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(rename = "EXPIRE", skip_serializing_if = "Option::is_none")]
    pub expire: Option<DateTime>,
    #[serde(rename = "SK", skip_serializing_if = "Option::is_none")]
    pub sk: Option<String>,
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
```

By adding the macros provided by `serde`, we can easily control the behaviours in serializing/deserializing.

For more details, see the codes in [`src/app/module/api_auth/entity/mod.rs`](src/app/module/api_auth/entity/mod.rs). 

If you want to add your own entity, just add a *.rs code file to `src/app/module/[ModuleName]/entity`.

## Dockerfike

Here comes the content of how to make a container images, actually not a part of the Rust code itself. Since the restful application itself is stateless and we want the application to be prepared for production environment, a dockerfile example would be very useful.

The dockerfile used in this project is simple:

```dockerfile
# adapt the next line it to your own rust version
FROM rust:1.54 as builder   
WORKDIR /usr/src/rust-restful-example
COPY . .
# if you want to change the cargo's config (for any possible package pulling issue, etc), do it like the next line with your cargo config
# COPY ./res/config.toml /usr/local/cargo
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/rust-restful-example/target/release/rust-restful-example /usr/local/bin/rust-restful-example
COPY --from=builder /usr/src/rust-restful-example/res/config.yaml /usr/local/conf/config.yaml
ENTRYPOINT ["rust-restful-example", "-c", "/usr/local/conf/config.yaml"]
```

To build the container image, use the following command:

```shell
docker build -t rust-restful-example:0.1 .
```

## Others

Feel free to make any issue or pull request.

Hope this project helps you build your own application, enjoy!