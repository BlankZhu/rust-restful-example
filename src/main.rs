use clap::Clap;

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // parse command line arguments
    let opts = app::cli::Opts::parse();

    // read config
    let conf: app::config::AppConfig;
    let conf_res = app::config::AppConfig::load_from_yaml_file(&opts.config);
    match conf_res {
        Ok(c) => conf = c,
        Err(err) => {
            eprintln!("failed to read config from {}, detail: {}", opts.config, err);
            return Ok(());
        }
    }

    // make up app
    let mut app = app::APIAuthAPP { config: conf };
    app.run().await
}
