use crate::bot::run;

mod bot;

#[tokio::main]
async fn main() {
    let token = include_str!("../TOKEN");
    run(token).await;
}
