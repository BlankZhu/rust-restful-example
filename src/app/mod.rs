use crate::app::module::api_auth::controller::api_auth_config;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use log::debug;
use mongodb::{options::ClientOptions, Client};

pub mod cli;
pub mod config;
pub mod constants;
pub mod module;

pub struct APIAuthAPP {
    pub config: config::AppConfig,
}

impl APIAuthAPP {
    pub async fn run(&mut self) -> std::io::Result<()> {
        std::env::set_var("RUST_BACKTRACE", "1");

        // handle config values

        // set log level
        match &self.config.log_level {
            Some(l) => env_logger::init_from_env(Env::default().default_filter_or(l)),
            None => env_logger::init_from_env(Env::default().default_filter_or("trace")),
        }

        // set bind address
        let bind_addr: String;
        match self.config.port {
            Some(p) if p > 0 => bind_addr = format!("0.0.0.0:{}", p),
            Some(_) => {
                bind_addr = String::from("0.0.0.0:8084");
                self.config.port = Some(8084);
            },
            None => {
                bind_addr = String::from("0.0.0.0:8084");
                self.config.port = Some(8084);
            },
        }

        // setup mongodb connection
        let mg_cli_opt: ClientOptions;
        let mg_cli_opt_res = ClientOptions::parse(self.config.mongo_uri.as_str()).await;
        match mg_cli_opt_res {
            Ok(res) => mg_cli_opt = res,
            Err(err) => {
                eprintln!(
                    "failed to setup connection option with Mongo URI {}, detail: {}",
                    self.config.mongo_uri, err
                );
                return Ok(());
            }
        }
        let mg_cli: Client;
        let mg_cli_res = Client::with_options(mg_cli_opt);
        match mg_cli_res {
            Ok(c) => mg_cli = c,
            Err(err) => {
                eprintln!("failed to connect to mongodb, detail: {}", err);
                return Ok(());
            }
        }
        let mg_cli = web::Data::new(mg_cli);

        debug!("api-auth server launched with params: {}", serde_json::to_string(&self.config).unwrap());

        // setup HttpServer and run it
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(mg_cli.clone())
                .service(web::scope("/api/v1").configure(api_auth_config))
        })
        .bind(bind_addr)?
        .run()
        .await
    }
}
