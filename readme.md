# osccli
## Send an osc command from the command line

Usage:
run with `-h` flag to see all options: `osccli -h`:

```sh
USAGE:
    osccli [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>      The osc address path part of the mes
sage, eg /ping
    -m, --argument <argument>    The contents of the message, eg. 1 o
r 'hello'
    -i, --ip <ip>                The ip address of the receiver, defa
ult: 127.0.0.1
    -p, --port <port>            The port of the receiver, default: 1
234
    -t, --type <type>            The type of the message. This is use
d to parse the command line argument to a specific
                                 OSC type, default: float

```
