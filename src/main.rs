//! `staticfs HOST PORT`: serves the files of the current directory over HTTP.
//!
//! - `cli`: the command line
//! - `confine`: keeps what is served inside the current directory
//! - `listing`: the page of a directory

mod cli;
mod confine;
mod listing;

use cli::Command;
use confine::InsideRoot;
use ntex::web::{self, middleware::DefaultHeaders, HttpServer};
use ntex_files as fs;
use std::{env, path::Path, process::ExitCode};

#[ntex::main]
async fn main() -> ExitCode {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args: Vec<String> = env::args().skip(1).collect();
    match cli::parse(&args) {
        Ok(Command::Serve { host, port }) => match serve(&host, port).await {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                log::error!("{e}");
                ExitCode::FAILURE
            }
        },
        Ok(Command::Help) => {
            println!("{}", cli::USAGE);
            ExitCode::SUCCESS
        }
        Err(e) => {
            log::error!("{e}\n{}", cli::USAGE);
            ExitCode::from(2)
        }
    }
}

async fn serve(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let root = InsideRoot::new(Path::new("."))?;
    log::info!("serving {} on {host}:{port}", root.root().display());

    // ntex stops on Ctrl-C and SIGTERM by itself
    HttpServer::new(move || {
        let list_root = root.clone();
        web::App::new()
            .wrap(DefaultHeaders::new().header("x-content-type-options", "nosniff"))
            .service(
                fs::Files::new("/", root.root())
                    .use_guards(root.clone())
                    .show_files_listing()
                    .redirect_to_slash_directory()
                    .files_listing_renderer(move |dir, req| {
                        listing::directory_listing(&list_root, dir, req)
                    }),
            )
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
