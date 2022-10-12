use chrono;
extern crate paho_mqtt as mqtt;
use std::{process, thread, time};

const QOS: i32 = 1;
const HOST: &str = "tcp://mosquitto:1883";

fn main() {
    let sleep_duration = time::Duration::from_millis(5000);

    // Create a client & define connect options

    // matchen:
    let cli = match mqtt::AsyncClient::new(HOST) {
        Ok(client) => client,
        Err(error) => {
            println!("cant create client: {:?}", error);
            process::exit(1);
        }
    };

    let conn_opts = mqtt::ConnectOptions::new();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts).wait() {
        println!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    // Create a topic and publish to it
    println!("Publishing messages on the 'logger' topic");
    let topic = mqtt::Topic::new(&cli, "logger", QOS);

    loop {
        let time = chrono::offset::Local::now();
        let tok = topic.publish(format!("{:?}: hi!", time));

        match tok.wait() {
            Err(e) => println!("{:?} error sending message: {:?}", time, e),
            Ok(_) => println!("{:?} logger says hello!", time),
        }

        thread::sleep(sleep_duration);
    }

    // Disconnect from the broker
    // let tok = cli.disconnect(None);
    // tok.wait().unwrap();
}
