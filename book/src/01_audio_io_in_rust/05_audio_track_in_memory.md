# Audio Track in Memory

It is unnecessary to write the audio data to a file. We can keep the audio data in memory and play it directly. To do this, we need to create our own `AudioTrack` struct implementing the `Source` trait.

## The `Source` Trait

The `Source` trait is defined in the `rodio` crate as follows:

```rust,noplayground
pub trait Source: Iterator
where
    Self::Item: rodio::Sample, 
{
    fn current_frame_len(&self) -> Option<usize>;
    fn channels(&self) -> u16;
    fn sample_rate(&self) -> u32;
    fn total_duration(&self) -> Option<Duration>;
}
```

To put it simply, the `Source` trait is an iterator that iterates over the audio samples, with additional information about the audio data. The `current_frame_len` method returns the number of samples in the current frame. The `channels` method returns the number of channels. The `sample_rate` method returns the sample rate. The `total_duration` method returns the total duration of the audio data.

## Wrapping an Iterator

Hence, we can create our own `AudioTrack` struct by adding some extra fields to existing `Iterator` structs.

```rust,noplayground
use std::iter::ExactSizeIterator;

pub struct AudioTrack<I: ExactSizeIterator>
where
    I::Item: rodio::Sample,
{
    inner: I,
    config: SupportedStreamConfig,
}

impl<I: ExactSizeIterator> AudioTrack<I>
where
    I::Item: rodio::Sample,
{
    pub fn new(iter: I, config: SupportedStreamConfig) -> Self {
        Self {
            inner: iter,
            config,
        }
    }
}

impl<I: ExactSizeIterator> Iterator for AudioTrack<I>
where
    I::Item: rodio::Sample,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
```

The `AudioTrack` struct has two fields. The `inner` field is an iterator that iterates over the audio samples. The `config` field is the configuration of the audio stream. The `Iterator` trait is implemented for the `AudioTrack` struct as required by the `Source` trait.

Next, we need to implement the `Source` trait for the `AudioTrack` struct.

```rust,noplayground
impl<I: ExactSizeIterator> Source for AudioTrack<I>
where
    I::Item: rodio::Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.inner.len())
    }

    fn channels(&self) -> u16 {
        self.config.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.config.sample_rate().0
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
```

## Recording and Playing

Finally, we can use the `AudioTrack` struct to record and play audio data without writing to a file.

```rust,noplayground

fn write_input_data<T, U>(data: &[T], writer: &Arc<Mutex<Vec<U>>>)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    writer
        .lock()
        .unwrap()
        .extend(data.iter().map(|sample| U::from_sample(*sample)));
}

let writer = Arc::new(Mutex::new(vec![]));
let reader = writer.clone();

... // Create the input stream and record the audio data

let reader = reader.lock().unwrap();
let track = AudioTrack::new(reader.clone().into_iter(), config);

... // Create the output sink

sink.append(track);
sink.sleep_until_end();
```
