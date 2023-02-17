use comrak::{markdown_to_html, ComrakOptions};
use warp::Filter;
use std::fs::read_to_string;

fn get_html_from_path(path: String) -> String {
    let markdown_contents = read_to_string(path).unwrap();
    markdown_to_html(
        &markdown_contents,
        &ComrakOptions::default()
        )
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // TODO: This currently is vulnerable to leak every file on the system
    let hello = warp::path!("hello" / String)
        .map(|article_path| warp::reply::html(get_html_from_path(article_path)));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
