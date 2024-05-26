use crate::{bot::run, words::Words};

mod bot;
mod words;

#[tokio::main]
async fn main() {
    let words = Words::load("words.ron");
    let token = include_str!("../TOKEN");
    run(token, words).await;
}
