# Example of how leptos receives two on:click events when two custom elements are registered

## Repro
Build the two components:

```
cd first && wasm-pack build --target web && cd ../second && wasm-pack build --target web && cd ..
```

Then start a dev server and open it. I did this with
```
npm i -g http-server && http-server
```

Then open the console. Try pressing either of the buttons. All click events are fired twice
