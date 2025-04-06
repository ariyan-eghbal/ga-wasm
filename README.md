# ga-wasm

<p align="center">
  <img src=".assets/shot_jellyfish.jpg" alt="Screenshot" height="350">
  <img src=".assets/shot_nudibranch.jpg" alt="Screenshot" height="350">
  <img src=".assets/shot_heartbeat.jpg" alt="Screenshot" height="350">
  <img src=".assets/shot_planetarytimer.jpg" alt="Screenshot" height="350"><br/>
  <strong>Some Generative Arts in WebAssembly</strong><br/>
</p>

<strong>You can see the compiled version at <a href="https://ariyan-eghbal.github.io/ga-wasm/" target="_blank">https://ariyan-eghbal.github.io/ga-wasm/</a></strong>

# Math notation 

See [HERE](MathNotations.md)


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

