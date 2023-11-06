# Introduction

Welcome to Rust ASIO guide! If you're looking to use the Rust programming language for writing the athernet project, you've come to the right place. This guide will take you through from the basics of ASIO in Rust to creating your own asynchronous audio interface, and beyond.

## Preliminaries

This guide assumes you have adequate knowledge of Rust. If you're new to Rust, you should first read the holy [Rust Book](https://doc.rust-lang.org/book/). You should also be familiar with the [Rust Standard Library](https://doc.rust-lang.org/std/).

It is advised that you have a basic understanding of asynchronous programming, otherwise you may fall into the nightmares of OS threads based concurrency model. If you're new to asynchronous programming, you should first read the [Async Book](https://rust-lang.github.io/async-book/).

## Code Examples

All code in this book is written for and tested on the Windows operating system using Rust 1.72.0, which is released on August 24, 2023. Earlier versions may not include all the features used in this guide. Later versions, however, should work just fine.

For brevity, the code examples do not include `use` statements, except for the first time a new item from the standard library or other crate is used. As a convenience, the following prelude can be used to import everything necessary to compile any of the code examples in this guide:

```rust,noplayground
#![allow(unused_imports)]
// TODO: Add prelude
```

Supplemental material, including complete versions of all code examples, is available at [https://github.com/mousany/rust-asio](https://github.com/mousany/rust-asio).

You may use all example code offered with this guide for any purpose. Note that if you use them in your own projects, you may need to credit or mention the authors of this guide.
