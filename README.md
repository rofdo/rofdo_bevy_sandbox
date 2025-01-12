# my_bevy_sandbox
A repository for me to try out bevy and nix related stuff

# Building
Nix toolchain needs to be installed and configured with flakes enabled.

## Client

```nix build```

```nix run```

## Server

```nix build .#server```

```nix run .#server```

## Building from the dev shell

To enter the dev shell:

```nix develop```

Build client from inside of the dev shell using:

```cargo build```

```cargo run```

Build server from inside of the dev shell using:

```cargo build --bin client```

```cargo run --bin client```
