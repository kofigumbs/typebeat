use anyhow::Result;
use miniaudio::{Decoder, DecoderConfig, Format, FramesMut};

use crate::SAMPLE_RATE;

pub fn read_stereo_file(i: i32) -> Result<Vec<f32>> {
    let path = std::env::current_dir()?
        .join("src")
        .join("samples")
        .join(format!("{:02}.wav", i));
    let config = DecoderConfig::new(Format::F32, 2, SAMPLE_RATE as u32);
    let mut decoder = Decoder::from_file(&path, Some(&config))?;
    let frame_count = decoder.length_in_pcm_frames() as usize;
    let mut samples = vec![0.0; 2 * frame_count];
    decoder.read_pcm_frames(&mut FramesMut::wrap(&mut samples[..], Format::F32, 2));
    Ok(samples)
}
