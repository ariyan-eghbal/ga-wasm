# ga-wasm
Some Generative Arts in WebAssembly
![Scheenshot](.assets/shot.jpg)

# Compile 
## install WASM target
```bash
rustup target add wasm32-unknown-unknown
```

## install wasm-pack
```bash
cargo install wasm-pack
```

## compile 
```bash
wasm-pack build --target web
```

# Run

Use a web server to serve contents of `www` directory
```bash
basic-http-server .
```

