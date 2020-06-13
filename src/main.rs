/*
 * osccli
 * Simple command line tool for sending osc messages
 * usage: osccli send /path value 127.0.0.1 1234
 *
 */
use clap::{App, Arg, SubCommand};
use nannou_osc as osc;
use std::env;

fn main() {
    // Retrieve arguments
    let args: Vec<String> = env::args().collect();

    let default_path = "/path";
    let path = match args.get(1) {
        None => default_path,
        Some(argument) => argument,
    };

    let default_value = "0.5";
    let value = match args.get(2) {
        None => default_value,
        Some(argument) => argument,
    };

    let default_addr = "127.0.0.1";
    let addr = match args.get(3) {
        None => default_addr,
        Some(argument) => argument,
    };

    let default_port = "1234";
    let port = match args.get(4) {
        None => default_port,
        Some(argument) => argument,
    };

    let target = format!("{}:{}", addr, port);
    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target)
        .expect("Could not connect to socket at address");

    let packet = (path, vec![osc::Type::Float(value.parse::<f32>().unwrap())]);

    sender.send(packet).ok();

    println!("{:?}", args);
}
