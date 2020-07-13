use std::path::PathBuf;
use serde_derive::Deserialize;

mod root;

use root::{Model as Root};
use yew::prelude::*;
use yew::virtual_dom::smr::HtmlString;

use std::convert::TryFrom;

#[rustfmt::skip]
use warp::{
    filters::BoxedFilter,
    fs::File,
    path::Peek,
    path,
    Filter, Reply,
    http::header::{HeaderMap, HeaderValue}
};

#[tokio::main]
async fn main() {
    let localhost = [0, 0, 0, 0];
    let port = 8000;
    let addr = (localhost, port);

    let routes = api();

    warp::serve(routes).run(addr).await;
}

const ROOT_STRING: &str = "/";

#[derive(Deserialize)]
struct Params {
    tag: String,
}

pub fn api() -> BoxedFilter<(impl Reply,)> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html; charset=utf-8"));

    warp::filters::method::get()
        .and(warp::query::<Params>())
        .map(|params: Params| {
            let app_root = html! {
                <div>
                    <style>
                        {".tag {
                            border: 1px solid gray;
                            padding: 3px;
                        }"}
                    </style>
                    <div>
                        {"This is what a "}{params.tag.clone()}{" looks like: "}
                        <@{params.tag.clone()} class=("tag")>{params.tag.clone()}</@>
                    </div>
                    <div style="margin-top: 10px;">
                        <a href="/?tag=SPAN">{"see SPAN"}</a> {" \u{00B7} "}
                        <a href="/?tag=DIV">{"see DIV attack"}</a> {" \u{00B7} "}
                        <a href="/?tag=%3E%3Cscript%3Ealert%28%22malicious%20code%21%21%22%29%3C/script%3E">{"see XSS attack"}</a>
                    </div>
                </div>
            };
            match HtmlString::try_from(app_root) {
                Ok(body) => {
                    body.to_string()
                },
                Err(err) => {
                    format!("could not render :(")
                }
            }
        })
        .with(warp::reply::with::headers(headers))
        .boxed()
}