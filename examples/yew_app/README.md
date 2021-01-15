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

