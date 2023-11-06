# Playing Sound with Rodio

Playing sound can be done similarly to recording sound with CPAL. Fortunately, developers of CPAL have also created a library called Rodio, which is a simple audio playback library. It is built on top of CPAL and provides a simpler API for playing sound. Let's see how we can use it to play the sound we recorded in the previous section.

## Creating a Sink

The Rodio library also provides a type named `Sink` which represents an audio track. It can be created from the `Device` and `SupportedStreamConfig` types from CPAL.

```rust,noplayground
use rodio::{OutputStream, Sink};

let (_stream, handle) = OutputStream::try_from_device_config(&device, config).unwrap();
let sink = Sink::try_new(&handle).unwrap();
```

## Playing Sound from a WAV file

The `Sink` type provides a method named `append` which can be used to append a `Source` to the audio track. The `Source` is a trait which is implemented by many types, including `rodio::Decoder`, which allows us to play audio from a file.

Playing a sound in `Sink` will not block the thread. Instead, it is done in the background by a dedicated thread. However, you can use the `Sink::sleep_until_end` method to block the thread until the sound has finished playing.

```rust,noplayground
use rodio::Decoder;
use std::fs::File;

let source = Decoder::new(File::open("output.wav").unwrap()).unwrap();
sink.append(source);
sink.sleep_until_end();
```

## Pausing and Resuming a Sink

The `Sink` type also provides methods for pausing and resuming the audio track. Let's see how we can use them.

```rust,noplayground
sink.pause();
sink.play();
```

It is also possible to clear the audio track by using the `Sink::clear` method.

```rust,noplayground
sink.clear();
```
