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
                    for cmd in args.get_subcommands_mut() {
                        if cmd.get_name() == "issues" {
                            match cmd.print_help() {
                                Ok(ret) => ret,
                                Err(err) => {
                                    println!("{:#?}", err);
                                }
                            };
                            break;
                        }
                    }
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
