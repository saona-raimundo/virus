# Yew app

## Usage

### Building

```
wasm-pack build --target web --out-name wasm --out-dir ./static
```

### Serving

Use a web server!

#### miniserve

```
miniserve static/
```

Install with `cargo install miniserve`.

#### Python 3

```
python -m http.server
```

Then visit [http://localhost:8000/static/](http://localhost:8000/static/)

## Browser compatibility

### Edge (old version)

Add in `<head>` 

```
<!-- Edge TextEncoder TextDecoder: https://rustwasm.github.io/docs/wasm-bindgen/reference/browser-support.html -->
        <script src="https://unpkg.com/text-encoding@0.6.4/lib/encoding-indexes.js"></script>
        <script src="https://unpkg.com/text-encoding@0.6.4/lib/encoding.js"></script>
```

Replace in `wasm.js`

```
async function init(input) {
    if (typeof input === 'undefined') {
        // input = import.meta.url.replace(/\.js$/, '_bg.wasm');
        input = 'wasm_bg.wasm';
    }
```

