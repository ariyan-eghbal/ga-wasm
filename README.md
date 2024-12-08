# ga-wasm

<p align="center">
  <img src=".assets/shot.jpg" alt="Screenshot"><br/>
  <strong>Some Generative Arts in WebAssembly</strong>
</p>

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

