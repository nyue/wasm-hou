# wasm-hou
Calling compiled WASM function from Houdini python

## Build

```
cargo build --target wasm32-unknown-unknown
```

## Test

Need to host the WASM file via http, no other choice at the moment

Change directory to the location of the generated WASM file
```
python3 -m http.server
```
