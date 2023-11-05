#![allow(unused)]

use cpal::{
    traits::{DeviceTrait, HostTrait},
    HostId, SampleRate, SupportedStreamConfig,
};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;

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

    let (_stream, handle) = OutputStream::try_from_device_config(&device, config).unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let source = Decoder::new(File::open("output.wav").unwrap()).unwrap();
    sink.append(source);
    sink.sleep_until_end();

    sink.pause();
    sink.play();

    sink.clear();
}
