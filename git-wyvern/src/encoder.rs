use crate::canvas::Canvas;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Encode a sequence of Canvas frames into an animated GIF file with transparency and looping.
pub fn save_animated_gif<P: AsRef<Path>>(
    frames: &[Canvas],
    delay_centisecs: u16,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    if frames.is_empty() {
        return Ok(());
    }

    let width = frames[0].width as u16;
    let height = frames[0].height as u16;

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let mut encoder = gif::Encoder::new(&mut writer, width, height, &[])?;
    encoder.set_repeat(gif::Repeat::Infinite)?;

    for canvas in frames {
        let mut frame_data = canvas.to_rgba_bytes();
        // Speed 2 provides high-fidelity NeuQuant color palette sampling across all frames,
        // eliminating quantization banding and color stepping artifacts.
        let mut gif_frame = gif::Frame::from_rgba_speed(width, height, &mut frame_data, 2);
        gif_frame.delay = delay_centisecs;
        gif_frame.dispose = gif::DisposalMethod::Background;
        encoder.write_frame(&gif_frame)?;
    }

    Ok(())
}

