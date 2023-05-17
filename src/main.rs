pub mod command;
pub mod libs;

extern crate clap;
extern crate lazy_static;

use clap::App;
use tokio;


#[tokio::main]
async fn main() {
    let mut args = App::new("github-helper")
        .version("v1.0")
        .author("smartcat")
        .subcommands(vec![
            command::issues::new_sub_command(),
        ])
        .override_usage("gctl <command>\n  ");
    let matches = args.clone().get_matches();
    match matches.subcommand() {
        Some(("issues", matches)) => {
            match matches.subcommand() {
                Some(("new", matches)) => {
                    command::issues::handler(matches).await;
                }
                _ => {
                    match args.print_help() {
                        Ok(ret) => ret,
                        Err(err) => {
                            println!("{:#?}", err);
                        }
                    };
                }
            }
        }
        _ => {
            match args.print_help() {
                Ok(ret) => ret,
                Err(err) => {
                    println!("{:#?}", err);
                }
            };
        }
    };
}
