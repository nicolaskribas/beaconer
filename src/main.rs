use clap::Parser;
use paho_mqtt as mqtt;
use std::{process, thread, time::Duration};

const BEACON_TOPIC_LEVEL: &str = "federator/beacon";

/// Periodically publishes a beacon to a broker to simulate the presence of a subscriber to a
/// federated topic
#[derive(Parser)]
struct Args {
    /// Broker's hostname
    #[clap(short = 'h', default_value = "localhost")]
    host: String,

    /// Port number where the broker is listening on
    #[clap(short = 'p', default_value = "1883")]
    port: u32,

    /// Federated topic
    #[clap(short = 't')]
    topic: String,

    /// Interval in seconds between each beacon
    #[clap(short = 'i', default_value_t = 5)]
    beacon_interval: u64,
}

fn main() {
    let Args {
        topic,
        host,
        port,
        beacon_interval,
    } = Args::parse();

    let address = format!("tcp://{host}:{port}");

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(&address)
        .finalize();

    let client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {err}");
        process::exit(1);
    });

    let connect_opts = mqtt::ConnectOptionsBuilder::new()
        .clean_session(true)
        .finalize();

    client.connect(connect_opts).wait().unwrap_or_else(|err| {
        println!("Error connecting the client: {err}");
        process::exit(1);
    });

    let beacon_topic = format!("{BEACON_TOPIC_LEVEL}/{topic}");

    let beacon = mqtt::MessageBuilder::new()
        .topic(&beacon_topic)
        .qos(mqtt::QOS_1)
        .payload([])
        .finalize();

    println!("Broker address: {address}");
    println!("Beacon topic: {beacon_topic}");
    println!("Started sending beacons...");

    loop {
        if let Err(err) = client.publish(beacon.clone()).wait() {
            println!("Error publishing beacon to the broker: {err}")
        }
        thread::sleep(Duration::from_secs(beacon_interval));
    }
}
