# Minesweeper

This is yet another Minesweeper clone written in Rust compiled to WebAssembly, embedded in JavaScript and HTML running on a Python webserver.

## Dependencies
- `python 3`
- `cargo`
- `wasm-pack`
- `GNU make`
- `GNU tar` (for packaging only)
- `zip`	    (for packaging only)

## Compilation

You can compile this program via make
```console
$ make build
```

Running the webserver:
```console
$ make run
```

## Packaging

You can create precompiled packages by running:
```console
$ make package
```

If compiled successful, you can now access the game by going to http://localhost:3001.
