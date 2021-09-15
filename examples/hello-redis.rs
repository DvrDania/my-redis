use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
