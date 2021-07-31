use std::convert::TryInto;

use anyhow::Result;
use miniaudio::{Decoder, DecoderConfig, Format, FramesMut};

#[derive(Default)]
pub struct Sample(Vec<f32>);

impl Sample {
    pub fn read_file(&mut self, i: usize) -> Result<()> {
        let path = std::env::current_dir()?
            .join("src")
            .join("samples")
            .join(format!("{:02}.wav", i));
        let config = DecoderConfig::new(Format::F32, 2, 44100);
        let mut decoder = Decoder::from_file(&path, Some(&config))?;
        self.0 = vec![0.0; (2 * decoder.length_in_pcm_frames()).try_into()?];
        decoder.read_pcm_frames(&mut FramesMut::wrap(&mut self.0[..], Format::F32, 2));
        Ok(())
    }

    pub fn at(&self, position: usize, channel: usize) -> f32 {
        self.0[2 * position + channel]
    }

    pub fn duration(&self) -> usize {
        self.0.len() / 2 as usize
    }
}
