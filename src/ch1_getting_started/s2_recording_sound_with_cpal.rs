use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, HostId, Sample, SampleFormat, SampleRate, SupportedStreamConfig,
};
use hound::{WavSpec, WavWriter};
use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

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
        SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data: &[i16], _: &_| {
                write_input_data::<i16, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data: &[i32], _: &_| {
                write_input_data::<i32, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::I64 => device.build_input_stream(
            &config.into(),
            move |data: &[i64], _: &_| {
                write_input_data::<i64, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::U8 => device.build_input_stream(
            &config.into(),
            move |data: &[u8], _: &_| {
                write_input_data::<u8, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::U16 => device.build_input_stream(
            &config.into(),
            move |data: &[u16], _: &_| {
                write_input_data::<u16, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::U32 => device.build_input_stream(
            &config.into(),
            move |data: &[u32], _: &_| {
                write_input_data::<u32, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::U64 => device.build_input_stream(
            &config.into(),
            move |data: &[u64], _: &_| {
                write_input_data::<u64, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                write_input_data::<f32, f32>(data, &writer);
            },
            err_fn,
            None,
        ),
        SampleFormat::F64 => device.build_input_stream(
            &config.into(),
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

    println!("Recording stopped. Output written to output.wav");
}

fn wav_spec_from_config(config: &SupportedStreamConfig) -> WavSpec {
    WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: if config.sample_format().is_float() {
            hound::SampleFormat::Float
        } else {
            hound::SampleFormat::Int
        },
    }
}

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
