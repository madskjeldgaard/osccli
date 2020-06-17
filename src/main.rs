/*
 * osccli by Mads Kjeldgaard, 2020
 *
 */
use clap::{App, Arg};
use nannou_osc as osc;
mod parsers;

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "1234";

struct Model {
    ip: String,
    port: String,
    address: String,
    argument: String,
    parsed: osc::Type,
}

fn model() -> Model {
    let ip = DEFAULT_IP.to_string();
    let port = DEFAULT_PORT.to_string();
    let address = "/ping".to_string();
    let argument = "ping!".to_string();
    let parsed = osc::Type::Nil;

    let model = Model {
        ip,
        port,
        address,
        argument,
        parsed,
    };

    model
}

fn add_arguments_to_model(model: &Model, arguments: Vec<&str>) {}

fn main() {
    let matches = App::new("osccli")
        .version("0.01")
        .author("Mads Kjeldgaard <mail@madskjeldgaard.dk>")
        .about("Fast and simple cli tool for Open Sound Control communication, written in Rust")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .help("The osc address path part of the message, eg /ping")
                .required(false)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("argument")
                .short("m")
                .long("argument")
                .help("The contents of the message, eg. 1 or 'hello'")
                .required(false)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("ip")
                .short("i")
                .long("ip")
                .help("The ip address of the receiver, default: 127.0.0.1")
                .required(false)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("The port of the receiver, default: 1234")
                .required(false)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .help("The type of the message. This is used to parse the command line argument to a specific OSC type, options: float|int|double|string, default: string")
                .required(false)
                .takes_value(true)
        )
        .get_matches();

    let mut model = model();

    if let Some(string) = matches.value_of("argument") {
        model.argument = string.to_string()
    }

    if let Some(string) = matches.value_of("address") {
        model.address = string.to_string()
    }

    if let Some(string) = matches.value_of("ip") {
        model.ip = string.to_string()
    }

    if let Some(string) = matches.value_of("port") {
        model.port = string.to_string()
    }

    model.parsed = parsers::parse_message(model.argument, matches.value_of("type"));
    let full_address = format!("{}:{}", model.ip, model.port);

    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(full_address)
        .expect("Could not connect to socket at address");

    let packet = (model.address, vec![model.parsed]);

    sender.send(packet).ok();
}
