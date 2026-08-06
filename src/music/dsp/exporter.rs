use hound::{SampleFormat, WavSpec, WavWriter};
use std::io::Cursor;

pub struct AudioExporter;

impl AudioExporter {
    /// Creates a standard WavSpec for our audio buffers (32-bit float, mono).
    fn create_spec(sample_rate: u32) -> WavSpec {
        WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        }
    }

    /// Saves an audio buffer directly to a .wav file on the local filesystem.
    pub fn save_to_wav_file(
        filename: &str,
        buffer: &[f32],
        sample_rate: u32,
    ) -> Result<(), hound::Error> {
        let spec = Self::create_spec(sample_rate);
        let mut writer = WavWriter::create(filename, spec)?;
        for &sample in buffer {
            writer.write_sample(sample)?;
        }
        writer.finalize()
    }

    /// Encodes an audio buffer into an in-memory .wav byte array.
    /// Useful for sending audio over HTTP or to a WebAssembly frontend.
    pub fn encode_to_wav_bytes(buffer: &[f32], sample_rate: u32) -> Result<Vec<u8>, hound::Error> {
        let spec = Self::create_spec(sample_rate);
        let mut cursor = Cursor::new(Vec::new());

        {
            let mut writer = WavWriter::new(&mut cursor, spec)?;
            for &sample in buffer {
                writer.write_sample(sample)?;
            }
            writer.finalize()?;
        }

        Ok(cursor.into_inner())
    }
}
