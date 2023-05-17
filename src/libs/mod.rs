pub mod http;
pub mod issues;
pub mod image;

// use std::sync::Mutex;



// lazy_static! {
//     pub static ref GITHUB_CLIENT: Mutex<http::GithubHttpClient> = {
//         let client = http::GithubHttpClient::new();
//         Mutex::new(client)
//     };
// }