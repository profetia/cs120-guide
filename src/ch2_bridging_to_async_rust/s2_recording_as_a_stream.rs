#![allow(unused)]

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Device, FromSample, Sample, SampleFormat, SizedSample, SupportedStreamConfig,
};
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
            SampleFormat::U8 => build_input_stream::<u8>(device, config, sender)?,
            SampleFormat::I16 => build_input_stream::<i16>(device, config, sender)?,
            SampleFormat::U16 => build_input_stream::<u16>(device, config, sender)?,
            SampleFormat::I32 => build_input_stream::<i32>(device, config, sender)?,
            SampleFormat::U32 => build_input_stream::<u32>(device, config, sender)?,
            SampleFormat::F32 => build_input_stream::<f32>(device, config, sender)?,
            SampleFormat::F64 => build_input_stream::<f64>(device, config, sender)?,
            SampleFormat::I64 => build_input_stream::<i64>(device, config, sender)?,
            SampleFormat::U64 => build_input_stream::<u64>(device, config, sender)?,
            _ => return Err(anyhow::anyhow!("unsupported sample format")),
        };
        Ok(Self { stream, receiver })
    }
}

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
