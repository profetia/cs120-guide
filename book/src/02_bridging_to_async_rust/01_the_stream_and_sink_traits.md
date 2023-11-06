# The Stream and Sink Traits

The `Stream` and `Sink` traits are part of the core of the `futures` crate. They are used to represent asynchronous streams and sinks, respectively. A stream is a source of values that are produced asynchronously, and a sink is a destination for values that are consumed asynchronously. The `Stream` and `Sink` traits are the asynchronous equivalents of the `Iterator` and `Write` traits, respectively.

The `Stream` trait is defined as follows:

```rust,noplayground
pub trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}
```

The `poll_next` method is similar to the `next` method of the `Iterator` trait, except that it returns a `Poll<Option<Self::Item>>` instead of an `Option<Self::Item>`. The `Poll` type is the same type that is returned by the `poll` method of the `Future` trait, which has been discussed in the [Async Book](https://rust-lang.github.io/async-book/02_execution/02_future.html).

The `Sink` trait is defined as follows:

```rust,noplayground
pub trait Sink<Item> {
    type Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error>;
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}
```

The `poll_ready` method is used to check if the sink is ready to receive a value. The `start_send` method is used to send a value to the sink. The `poll_flush` method is used to flush the sink, and the `poll_close` method is used to close the sink.

The `Stream` and `Sink` traits are implemented for many types in the `futures` crate. For example, the `TcpStream` type implements both the `Stream` and `Sink` traits. Bridging to the asynchronous world is often as simple as using the `Stream` and `Sink` traits, while they can also precisely describe the layers of the athernet.
