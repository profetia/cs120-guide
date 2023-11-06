use std::{
    iter::ExactSizeIterator,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use cpal::{
    traits::{HostTrait, StreamTrait},
    FromSample, HostId, Sample, SampleFormat, SampleRate, SupportedStreamConfig,
};
use rodio::{DeviceTrait, OutputStream, Sink, Source};

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

fn main() {
    let host = cpal::host_from_id(HostId::Asio).expect("failed to initialise ASIO host");
    let device = host
        .default_input_device()
        .expect("failed to find input device");

    let default_config = device.default_input_config().unwrap();
    let config = SupportedStreamConfig::new(
        1,                 // mono
        SampleRate(48000), // sample rate
        default_config.buffer_size().clone(),
        default_config.sample_format(),
    );

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let writer = Arc::new(Mutex::new(vec![]));

    let reader = writer.clone();
    let stream_config = config.clone().into();

    let stream = match config.sample_format() {
        SampleFormat::I8 => device.build_input_stream(
            &stream_config,
            move |data: &[i8], _: &_| {
                write_input_data::<i8, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::I16 => device.build_input_stream(
            &stream_config,
            move |data: &[i16], _: &_| {
                write_input_data::<i16, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::I32 => device.build_input_stream(
            &stream_config,
            move |data: &[i32], _: &_| {
                write_input_data::<i32, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::I64 => device.build_input_stream(
            &stream_config,
            move |data: &[i64], _: &_| {
                write_input_data::<i64, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::U8 => device.build_input_stream(
            &stream_config,
            move |data: &[u8], _: &_| {
                write_input_data::<u8, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::U16 => device.build_input_stream(
            &stream_config,
            move |data: &[u16], _: &_| {
                write_input_data::<u16, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::U32 => device.build_input_stream(
            &stream_config,
            move |data: &[u32], _: &_| {
                write_input_data::<u32, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::U64 => device.build_input_stream(
            &stream_config,
            move |data: &[u64], _: &_| {
                write_input_data::<u64, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::F32 => device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &_| {
                write_input_data::<f32, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::F64 => device.build_input_stream(
            &stream_config,
            move |data: &[f64], _: &_| {
                write_input_data::<f64, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        _ => panic!("unsupported sample format"),
    }
    .unwrap();

    stream.play().expect("failed to play stream");

    thread::sleep(Duration::from_secs(5));

    stream.pause().expect("failed to pause stream");

    let (_stream, handle) = OutputStream::try_from_device_config(&device, config.clone()).unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let reader = reader.lock().unwrap();
    let track = AudioTrack::new(reader.clone().into_iter(), config);
    sink.append(track);
    sink.sleep_until_end();

    sink.pause();
    sink.play();
}

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
