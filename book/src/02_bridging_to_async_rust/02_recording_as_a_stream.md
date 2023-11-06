# Recording as Stream

Since the driver uses callbacks to handle events, it is possible to use `Mutex` and `Waker` to integrate it with the `futures` crate. However, the simplest way is to delegate the work to an unbounded channel. The channel will be used to send the recorded data to the main thread, where it will be processed.

```rust,noplayground
use anyhow::Result;
use cpal::{Device, SizedSample};

fn build_input_stream<T>(
    device: &Device,
    config: SupportedStreamConfig,
    sender: UnboundedSender<Vec<f32>>,
) -> Result<cpal::Stream>
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let stream = device.build_input_stream(
        &config.config(),
        move |data: &[T], _: &_| {
            let data = data
                .iter()
                .map(|&sample| f32::from_sample(sample))
                .collect::<Vec<f32>>();
            sender.send(data).unwrap();
        },
        |err| eprintln!("an error occurred on stream: {}", err),
        None,
    )?;
    Ok(stream)
}
```

Since the unbounded channel is asynchronous, it also handles the wake-up when any data is received, making the manual control of the `Waker` unnecessary.

```rust,noplayground
use futures::Stream;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub struct AudioInputStream {
    stream: cpal::Stream,
    receiver: UnboundedReceiver<Vec<f32>>,
}

impl AudioInputStream {
    pub fn try_from_device_config(device: &Device, config: SupportedStreamConfig) -> Result<Self> {
        let (sender, receiver) = mpsc::unbounded_channel();
        let stream = match config.sample_format() {
            SampleFormat::I8 => build_input_stream::<i8>(device, config, sender)?,
           ...
            _ => return Err(anyhow::anyhow!("unsupported sample format")),
        };
        Ok(Self { stream, receiver })
    }
}

impl Stream for AudioInputStream {
    type Item = Vec<f32>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> futures::task::Poll<Option<Self::Item>> {
        self.stream.play().unwrap();
        self.receiver.poll_recv(cx)
    }
}

```
