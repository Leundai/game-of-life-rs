<div align="center">

  <h1><code>Game Of Life Rust WASM</code></h1>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

![Sample Game Of Life](media/sample.gif)

## About

### 🛠️ Build with `wasm-pack build`

```
cd rust
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
cd rust
wasm-pack test --headless --chrome
```

### 🏠 Run website locally

```
cd web
npm install
npm run start
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- `LICENSE-MIT`

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
