use clap::{value_parser, App, Arg, ArgAction, ArgMatches, Command};
use crate::libs::http::GithubHttpClient;
use crate::libs::image::upload_sarif_report;


pub fn new_sub_command<'help>() -> App<'help> {
    Command::new("issues")
        .about("issues command")
        .subcommands(vec![
            Command::new("new")
                .arg(
                    Arg::new("file")
                        .action(ArgAction::Append)
                        .value_parser(value_parser!(String))
                        .default_values(&["./result.sarif", "./results.sarif"])
                        .short('f')
                        .help("文件路径"),
                )
                .arg(
                    Arg::new("token")
                        .value_parser(value_parser!(String))
                        .long("token")
                        .takes_value(true)
                        .help("Token"),
                )
                .arg(
                    Arg::new("owner")
                        .value_parser(value_parser!(String))
                        .long("owner")
                        .takes_value(true)
                        .help("Repo Owner"),
                )
                .arg(
                    Arg::new("repo")
                        .value_parser(value_parser!(String))
                        .long("repo")
                        .takes_value(true)
                        .help("Repo Name"),
                )
                .override_usage("gctl issues new -f ./result.sarif --token <token> --owner <owner> --repo <repo>\n  ")
        ])
        .override_usage("gctl issues <command>\n  ")
}

pub async fn handler(matches: &ArgMatches) {
    let files: Vec<&String> = matches
        .get_many::<String>("file")
        .expect("input file with [-f <file1> -f <file2>]")
        .collect::<Vec<&String>>();
    let token = matches.get_one::<String>("token").expect("input token with [--token=<token>]");
    let owner = matches.get_one::<String>("owner").expect("input owner with [--owner=<owner>]");
    let repo = matches.get_one::<String>("repo").expect("input repo with [--repo=<repo>]");
    let files = files.iter().map(|x| x.clone().as_str()).collect::<Vec<&str>>();
    upload_sarif_report(GithubHttpClient::new(), files, token, owner, repo).await.unwrap();
}
