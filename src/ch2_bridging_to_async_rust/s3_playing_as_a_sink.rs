use crate::ch1_audio_io_in_rust::s4_audio_track_in_memory::AudioTrack;
use anyhow::{Error, Result};
use cpal::{Device, FromSample, SupportedStreamConfig};
use futures::{FutureExt, Sink};
use rodio::OutputStream;
use tokio::{
    sync::{
        mpsc::{self, UnboundedSender},
        oneshot::{self, Receiver, Sender},
    },
    task,
};

pub struct AudioOutputStream<I>
where
    I: ExactSizeIterator + Send + 'static,
    I::Item: rodio::Sample + Send,
    f32: FromSample<I::Item>,
{
    _stream: OutputStream,
    sender: UnboundedSender<(AudioTrack<I>, Sender<()>)>,
    task: Option<Receiver<()>>,
}

impl<I> AudioOutputStream<I>
where
    I: ExactSizeIterator + Send + 'static,
    I::Item: rodio::Sample + Send,
    f32: FromSample<I::Item>,
{
    pub fn try_from_device_config(device: &Device, config: SupportedStreamConfig) -> Result<Self> {
        let (_stream, handle) = OutputStream::try_from_device_config(device, config)?;
        let sink = rodio::Sink::try_new(&handle)?;

        let (sender, mut receiver) = mpsc::unbounded_channel::<(AudioTrack<I>, Sender<()>)>();
        task::spawn_blocking(move || {
            while let Some((track, sender)) = receiver.blocking_recv() {
                sink.append(track);
                sink.sleep_until_end();
                let _ = sender.send(());
            }
        });

        Ok(Self {
            _stream,
            sender,
            task: None,
        })
    }
}

impl<I> AudioOutputStream<I>
where
    I: ExactSizeIterator + Send + 'static,
    I::Item: rodio::Sample + Send,
    f32: FromSample<I::Item>,
{
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Error>> {
        match self.as_mut().task {
            Some(ref mut inner) => {
                if inner.poll_unpin(cx).is_ready() {
                    self.as_mut().task = None;
                    std::task::Poll::Ready(Ok(()))
                } else {
                    std::task::Poll::Pending
                }
            }
            None => std::task::Poll::Ready(Ok(())),
        }
    }
}

impl<I> Sink<AudioTrack<I>> for AudioOutputStream<I>
where
    I: ExactSizeIterator + Send + 'static,
    I::Item: rodio::Sample + Send,
    f32: FromSample<I::Item>,
{
    type Error = Error;

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.poll(cx)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.poll(cx)
    }

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.poll(cx)
    }

    fn start_send(
        mut self: std::pin::Pin<&mut Self>,
        item: AudioTrack<I>,
    ) -> std::result::Result<(), Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send((item, sender))
            .map_err(|_| Error::msg("failed to send audio track"))?;
        self.as_mut().task = Some(receiver);
        Ok(())
    }
}
