# Putting It All Together

Now that we have all the pieces, let's put them together. Here is the record-then-playback example from last section:

```rust,noplayground
use futures::{SinkExt, StreamExt};
use tokio::time;

... // Create device and config

let mut input_stream =
    AudioInputStream::try_from_device_config(&device, config.clone()).unwrap();
let mut inputs = vec![];
time::timeout(Duration::from_secs(5), async {
    while let Some(sample) = input_stream.next().await {
        inputs.extend(sample);
    }
})
.await
.ok();

let track = AudioTrack::new(inputs.into_iter(), config.clone());
let mut output_stream = AudioOutputStream::try_from_device_config(&device, config).unwrap();
output_stream.send(track).await.unwrap();
```

One obvious changes is that we are using `async` and `await` instead of `sleep` and `sleep_until_end`. Under the hood, `sleep` and `sleep_until_end` may still happen, but they are scheduled by the `tokio` runtime instead of manually by us, like we did in the previous section.

You may find this extremely helpful when writing the athernet project, as explicit synchronization is rarely needed, let alone the confusing synchronization bugs.

In addition, this approach also allows us to make use of existing asynchronous infrastructure, such as `tokio_utils` and `futures_util`, where framing, buffering, and other utilities are provided.
