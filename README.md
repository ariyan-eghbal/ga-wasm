# ga-wasm

<p align="center">
  <img src=".assets/shot.jpg" alt="Screenshot"><br/>
  <strong>Some Generative Arts in WebAssembly</strong><br/>
  Based on <a href="https://x.com/yuruyurau/status/1865420201086636376" target="_blank">this Tweet</a>
</p>

<strong>You can see the compiled version at <a href="https://ariyan-eghbal.github.io/ga-wasm/" target="_blank">https://ariyan-eghbal.github.io/ga-wasm/</a></strong>

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

