# vinseers mono-repo

This mono-repo contains 3 rust projects:
- [vinseers lib](https://github.com/Lukas-Les/vinseers/tree/main/vinseers)
- [vinseers-cli](https://github.com/Lukas-Les/vinseers/tree/main/vinseers-cli)
- [vinsers-gui](https://github.com/Lukas-Les/vinseers/tree/main/vinseers-gui)

## Table of Contents
- [Usage](#usage)
    - [Installation](#installation)
    - [Build](#build)
    - [Use Cli Tool](#use-cli-tool)
    - [Use Gui Tool](#use-gui-tool)
    - [Documentation](#documentation)

## Usage

### Installation
To install vinseers-gui to your system, run:
```sh
curl https://raw.githubusercontent.com/Lukas-Les/vinseers/main/vinseers-gui/scripts/install-unix-like.sh | sh
```

To check installation scrpit before installation, run:
```sh
curl https://raw.githubusercontent.com/Lukas-Les/vinseers/main/vinseers-gui/scripts/install-unix-like.sh | less
```


### Build
To build both binaries run 
```sh
cargo build -r
```

To build a specific binary run
```sh
cargo build --bin <name of the project> -r
```

for example, to build only GUI version of the app, run
```shell
cargo build --bin vinseers-gui -r
```

### Use cli tool
Cli tool takes cl arguments as flags:
```
-f or --file: target file;
-d or --dir: target directory;
-o or --output: [optional] directs where to put results;
-m or --max: [optional] max results from a single file;
--vid: [optional] provided target VID (Vehicle Identifier). Must be one of:
    vin (default)
    lpn-fin
    lpn-fra
    lpn-hun
    lpn-ita
    lpn-ltu
```
You must provide either -f or -d, but not both.
If -o is not provided, the result will be printed in console.

### Use gui tool
Just build and run :)

### Documentation
To build the documentation run
```shell
cargo doc --no-deps --open
```


# [Roadmap](https://docs.google.com/spreadsheets/d/12Dp7DlwmRPYVgzjRKY51OGcWUiygf9YCNgGr3l3T20Y/edit?usp=sharing)
