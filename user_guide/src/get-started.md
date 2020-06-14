# Get Started

## Installation

`mover` is a binary you will be able to run from the command line. Below are
different options to obtain the binary.

*Currently this is not published anywhere so build from source is your best option*.

### Build from source

Clone the repository.

Build the binary.

```
$ cargo build --release
$ ls target/release
```

## Usage

Firstly create the [`db.json` configuration file][configuration] containing the
resources that will be mocked.

Once the configuration file has been created run the binary. It will look for
the `db.json` file and begin the server.

```
$ ls
db.json
$ mover
Listening on 127.0.0.1:5212
```

[configuration]: configuration.md
