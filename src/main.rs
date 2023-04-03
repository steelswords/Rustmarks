use comrak::{markdown_to_html, ComrakOptions};
use warp::Filter;
use std::fs::read_to_string;

extern crate markdown;

const HTML_BEGINNING_PATH: &str = "_config/beginning.html";
const HTML_ENDING_PATH: &str =    "_config/ending.html";

fn get_html_from_path(path: String) -> String {
    //let markdown_contents = read_to_string(path).unwrap();
    let markdown_contents = read_to_string(path).unwrap();
    let html_beginning = read_to_string(HTML_BEGINNING_PATH).unwrap();
    let html_end = read_to_string(HTML_ENDING_PATH).unwrap();

    let mut html: String = html_beginning;
    html.push_str(
        markdown::to_html(markdown_contents.as_str()).as_str()
    );
    html.push_str(html_end.as_str());
    html

    /*
    html_beginning + 
    markdown_to_html(
        &markdown_contents.as_str(),
        &ComrakOptions::default()
        ) +
    html_end
    */
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
