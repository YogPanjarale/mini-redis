use mini_redis::{client,Result};

#[tokio::main]
async fn main()->Result<()> {
    //open a connection to the mini-server address
    let mut _client  = client::connect("localhost:8082").await?;

    //set key "hello" to value "world"
    _client.set("hello", "no".into()).await?;

    // get the value of key "hello"
    let result =  _client.get("hello").await?;

    // print the value
    println!("got value from server ; result = {:?}", result);
    Ok(())
}
