#![allow(unused)]

use crate::{
    ch1_audio_io_in_rust::s4_audio_track_in_memory::AudioTrack,
    ch2_bridging_to_async_rust::{
        s2_recording_as_a_stream::AudioInputStream, s3_playing_as_a_sink::AudioOutputStream,
    },
};
use anyhow::Result;
use cpal::{traits::HostTrait, HostId, SampleRate, SupportedStreamConfig};
use futures::{SinkExt, StreamExt};
use rodio::DeviceTrait;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
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

    println!("Playing back audio...");
    let mut output_stream = AudioOutputStream::try_from_device_config(&device, config).unwrap();
    output_stream.send(track).await.unwrap();

    Ok(())
}
