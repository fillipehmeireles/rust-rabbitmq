extern crate dotenv;
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use dotenv::dotenv;
use std::env;

struct Consumer {
    connection_string: String,
}

impl Consumer {
    fn new() -> Self {
        Self {
            connection_string: env::var("RMQ_URL").unwrap(),
        }
    }
}

fn main() -> Result<()> {
    dotenv().ok();
    let consumer = Consumer::new();
    println!("{}", consumer.connection_string);
    let mut conn = Connection::insecure_open(&consumer.connection_string)?;

    let channel = conn.open_channel(None)?;

    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    conn.close()
}
