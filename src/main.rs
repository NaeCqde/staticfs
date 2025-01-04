use ntex::web::{self, HttpServer};
use ntex_files as fs;
use std::{env, error::Error};

#[ntex::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args: Vec<String> = env::args().collect();
    if 3 == args.len() {
        let host = args[1].as_str();

        match args[2].parse::<u16>() {
            Ok(port) => {
                return Ok(HttpServer::new(|| {
                    web::App::new().service(
                        fs::Files::new("/", ".")
                            .show_files_listing()
                            .redirect_to_slash_directory(),
                    )
                })
                .bind((host, port))?
                .run()
                .await?)
            }
            Err(e) => log::error!("{}", e),
        }
    }

    log::error!("staticfs HOST PORT");
    Ok(())
}
