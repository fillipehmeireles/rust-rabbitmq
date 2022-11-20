extern crate dotenv;
use amiquip::{Connection, Exchange, Publish, Result};
use dotenv::dotenv;
use std::env;

struct Producer {
    connection_string: String,
}

impl Producer {
    fn new() -> Self {
        Self {
            connection_string: env::var("RMQ_URL").unwrap(),
        }
    }
}

fn main() -> Result<()> {
    dotenv().ok();

    let producer = Producer::new();

    let mut conn = Connection::open(&producer.connection_string)?;

    let channel = conn.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    exchange.publish(Publish::new("hello there".as_bytes(), "hello"))?;
    conn.close()
}
