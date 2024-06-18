use tg_astro_bot::tg::telegram::start;

#[tokio::main]
async fn main() {
    println!("Bot started!");
    tokio::spawn(start()).await.unwrap();
}
