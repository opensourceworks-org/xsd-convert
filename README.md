# **<xsd ⚡> xsd-convert**

running at [https://xsd-convert.com](https://xsd-convert.com)

## Introduction
This is an example application of the [yaxp](https://github.com/opensourceworks-org/yaxp) crate.

Takes in an xsd schema and tranforms to arrow, avro, duckdb, json, json representation of spark schema, jsonschema, polars.  WIP: pandas and protobuf.


> 📌 **Note:** Because it's a client side application, it runs completely locally. No data is sent to any server, nor is anything logged.


Written in [rust](https://www.rust-lang.org/) with [leptos](https://leptos.dev). Compiled to [webassembly](https://webassembly.org/) using [trunk](https://trunkrs.dev/). Served behind [openresty](https://openresty.org/). Running on [raspberry pi](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/).


## Running locally
You'll need a few things to get going.  And an additional step if you're on Apple Silicon.

1. [rust](https://www.rust-lang.org/tools/install)
   
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2. [trunk](https://trunkrs.dev/)
    
     ```bash
     cargo install --locked trunk
     ```
3. wasm toolchain 
    
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
4. Apple Silicon: llvm (not xcode)
    
    ```bash
    brew install llvm
    ```
   Then add the following to your shell profile:

   (verify the path with `brew info llvm`)
   
   ```bash
   export PATH=/opt/homebrew/opt/llvm/bin:$PATH
   ```
   
