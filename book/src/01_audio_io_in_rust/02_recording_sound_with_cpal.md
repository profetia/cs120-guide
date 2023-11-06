# Recording Sound with CPAL

Here are some concepts cpal exposes:

- A `Host` provides access to the available audio devices on the system.
  Some platforms have more than one host available, but every platform supported by CPAL has at least one `default_host` that is guaranteed to be available.
- A `Device` is an audio device that may have any number of input and
  output streams.
- A `Stream` is an open flow of audio data. Input streams allow you to
  receive audio data, output streams allow you to play audio data. You must choose which
  `Device` will run your stream before you can create one. Often, a default device can be retrieved via the `Host`.

## Creating a Stream

To create a stream, you must first create a `Host` and a `Device`:

```rust,noplayground
use cpal::{
  traits::{DeviceTrait, HostTrait},
  HostId
};

let host = cpal::host_from_id(HostId::Asio).expect("failed to initialise ASIO host");
let device = host.default_input_device().expect("failed to find input device");
```

Since we only need one channel of audio, you need to replace the device's default config with one that only has one channel:

```rust,noplayground
use cpal::{SampleRate, SupportedStreamConfig};

let default_config = device.default_input_config().unwrap();
let config = SupportedStreamConfig::new(
    1,                 // mono
    SampleRate(48000), // sample rate
    default_config.buffer_size().clone(),
    default_config.sample_format(),
);
```

Now you can create a stream from the device and the config:

```rust,noplayground
use cpal::SampleFormat;

let stream = match config.sample_format() {
    SampleFormat::I8 => device.build_input_stream(
        &config.into(),
        move |data: &[i8], _: &_| {
            // react to stream events and read or write stream data here.
        },
        move |err| {
            // react to errors here.
        },
        None,
    ),

    ...
}
.unwrap();
```

While the stream is running, the selected audio device will periodically call the data callback that was passed to the function.

Creating and running a stream will not block the thread. On modern platforms, the given callback is called by a dedicated, high-priority thread responsible for delivering audio data to the system’s audio device in a timely manner.

## Starting and Stopping a Stream

Not all platforms automatically start a stream when it is created. To start a stream, call `play()` on it:

```rust,noplayground
use cpal::traits::StreamTrait;

stream.play().expect("failed to play stream");
```

Some devices support pausing the audio stream. This can be done by calling `pause()` on the stream:

```rust,noplayground
stream.pause().expect("failed to pause stream");
```

## Writing a WAV File

This example shows how to write a WAV file from a stream. It uses the `hound` crate to write the WAV file.

```rust,noplayground
use cpal::{Sample, FromSample};
use hound::{WavSpec, WavWriter};
use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn write_input_data<T, U>(data: &[T], writer: &Arc<Mutex<WavWriter<BufWriter<File>>>>)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    let mut writer = writer.lock().unwrap();
    for &sample in data {
        writer.write_sample(sample.to_sample::<U>()).ok();
    }
}

let spec = wav_spec_from_config(&config);
let writer = Arc::new(Mutex::new(WavWriter::create("output.wav", spec).unwrap()));
let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

let stream = match config.sample_format() {
    SampleFormat::I8 => device.build_input_stream(
        &config.into(),
        move |data: &[i8], _: &_| {
            write_input_data::<i8, f32>(data, &writer);
        },
        err_fn,
        None,
    ),
    ...
}
.unwrap();

stream.play().expect("failed to play stream");
thread::sleep(Duration::from_secs(5));
stream.pause().expect("failed to pause stream");
```
