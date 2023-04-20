/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::JsResult;
use gloo::net::http::{Request, RequestRedirect, Response};
use pulldown_cmark::{Parser, Options, html};

/// Function for quickly converting markdown into html
pub fn markdown_to_html(input: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    
    let parser = Parser::new_ext(input.as_str(), options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

///
pub async fn fetch(url: &str) -> JsResult<Response> {
    let res = request(url).send().await?;
    Ok(res)
}
/// Function wrapper for quickly initializing new [Request] given a valid url
pub fn request(url: &str) -> Request {
    Request::new(url)
}
///
pub async fn redirect(url: &str) -> JsResult {
    request(url)
        .redirect(RequestRedirect::Follow)
        .send()
        .await?;
    Ok(())
}
