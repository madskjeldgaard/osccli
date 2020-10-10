# osccli

`osccli` is a simple and fast tool for sending OSC messages from the command line. It is written in Rust and among other things it lets you send custom messages and choose a type for them, the program will then try and parse the message you have passed to it as the type you have defined.

The program exits as soon as the message is sent.

By default, the base command `osccli` will send an osc message with the path `/ping` and argument `ping!` of type String to `127.0.0.1:1234`.

Supply the tool with command line flags (see Usage below) to change any of these defaults.

Example:

`osccli -a /yo -m whatsup` will send the osc message `["/yo", "whatsup"]` to `127.0.0.1:1234`

## Installing
For users of Arch based distros `osccli` is available on the [aur](https://aur.archlinux.org) and may be installed via yay:

```
yay -S osccli
```

## Building
Building is quite easy if you have rust and cargo installed, dependencies are handled by the same:

`cargo build --release`

## Usage
Usage:
run with `-h` flag to see all options: `osccli -h`:

```sh
Mads Kjeldgaard <mail@madskjeldgaard.dk>
Fast and simple cli tool for Open Sound Control communication, written in Rust

USAGE:
    osccli [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>      The osc address path part of the message, eg /ping
    -m, --argument <argument>    The contents of the message, eg. 1 or 'hello'
    -i, --ip <ip>                The ip address of the receiver, default: 127.0.0.1
    -p, --port <port>            The port of the receiver, default: 1234
    -t, --type <type>            The type of the message. This is used to parse the command line argument to a specific
                                 OSC type, options: float|int|double|string, default: string
```
