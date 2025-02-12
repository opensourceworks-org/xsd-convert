# **<xsd ⚡> xsd-convert**

running at [https://xsd-convert.com](https://xsd-convert.com)

## Introduction
This is an example application of the [yaxp](https://github.com/opensourceworks-org/yaxp) crate.

Takes in an xsd schema and tranforms to arrow, avro, duckdb, json, json representation of spark schema, jsonschema, polars.  WIP: pandas and protobuf.


> 📌 **Note:** Because it's a client side application, it runs completely locally. No data is sent to any server, nor is anything logged.


Written in [rust](https://www.rust-lang.org/) with [leptos](https://leptos.dev). Compiled to [webassembly](https://webassembly.org/) using [trunk](https://trunkrs.dev/). Served behind [openresty](https://openresty.org/). Running on [raspberry pi](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/).


