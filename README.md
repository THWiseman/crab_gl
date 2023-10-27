# crab_gl
My obligatory rust sideproject.
Particle simulator that compiles to WebAssembly and renders using WebGL.

### dependencies
- Rust & `cargo`. www.rust-lang.org
- `wasm-pack` - for compiling to web assembly and JavaScript interop. `cargo install wasm-pack`

#### build
run `wasm-pack build --target web`
Open `web/index.html` with your favorite web server. I use the Live Server VSCode extension for local development. 