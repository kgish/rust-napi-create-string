# Rust NAPI Create String

A simple example showing how to create a rust string and pass it back to the node application.

## Build
```
$ cargo build --release && cp ./target/release/librust_napi_create_string.so index.node
```
## Run
```
$ node ./index.js
```

## References

Here are a few relevant links that you might find interesting.

* [Node.js API](https://nodejs.org/api/n-api.html#n_api_n_api)
* [nodejs-sys](https://crates.io/crates/nodejs-sys)
* [Rust and Node.js: A match made in heaven](https://blog.logrocket.com/rust-and-node-js-a-match-made-in-heaven/)
* [High Performance Apps with JavaScript and Rust, It's Easier Than You Think](https://www.youtube.com/watch?v=Pfbw4YPrwf4)


