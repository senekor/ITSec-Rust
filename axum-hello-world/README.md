# Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started)

# Development

Run the server with `cargo run`.

# Usage

## SQL Injection

The route `/text/:param` accepts an HTTP POST request and writes the value of
`:param` into the in-memory sqlite database.

The route `/texts` accepts an HTTP GET request and returns everything that has
been written into the database as a JSON list.

A simple attack would be to try and inject something like
`test'); DROP TABLE texts; --` as shown in the following curl command.

```
curl -X POST http://localhost:3000/text/test%27%29%3B%20DROP%20TABLE%20texts%3B%20--
```

Afterwards the inserted texts can be checked for example with curl again as
shown here.

```
curl -v http://localhost:3000/texts | jq
```

With a successfull attack, only an HTTP error 500 is returned (because the table
no longer exists). In this case it will merely return the expected JSON list,
including `test'); DROP TABLE texts; --`.

# Nix

Not for the faint of heart. A working [nix](https://nixos.org/download)
installation with [flakes](https://nixos.wiki/wiki/Flakes) enabled is needed.

Only tested on Linux systems, though it should work on Windows (via wsl2) and
macOS as well.

## Dev Shells

Running `nix develop` drops you into a bash shell with everything setup for your
particular system. From there on the normal cargo commands work (running it for
the first time can take a while).

`nix develop` is equivalent to `nix develop .#default`, with the dot signifying
the current working directory and everything after the hash sign getting used as
the name of the dev shell defined in [flake.nix](flake.nix).

A second dev shell is hiding in there, by the name of `musl` (getting called by
`nix develop .#musl`), which sets up everything to statically compile the whole
project (leveraging [musl libc](https://musl.libc.org/)). This most probably
only works on Linux and only really makes sense there anyways.

## Building

Equivalently to the way the dev shells work, the nix flake defines how to build
the whole project.

Running `nix build` writes the resulting binary into the `result` folder.

Same as with the dev shells, `nix build .#musl` builds the project as a
statically linked binary.
