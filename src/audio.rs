use rodio::{Sink, cpal::Sample, source::Source};
use std::time::Duration;

/// Configurazione per l'audio
#[derive(Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 15360, // 256 * 60
            channels: 1,
        }
    }
}

/// Gestore audio semplificato
pub struct AudioHandler {
    config: AudioConfig,
}

impl AudioHandler {
    pub fn new() -> Self {
        Self::with_config(AudioConfig::default())
    }

    pub fn with_config(config: AudioConfig) -> Self {
        Self { config }
    }

    /// Aggiunge un buffer audio al sink se contiene dati non-zero
    pub fn append_buffer_to_sink<const N: usize>(&self, sink: &Sink, buffer: &[u8; N]) {
        if buffer.iter().any(|&sample| sample != 0) {
            let source = SampleBufferSource::new(*buffer, self.config.clone());
            sink.append(source);
        }
    }
}

impl Default for AudioHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Sorgente audio per buffer a dimensione fissa
pub struct SampleBufferSource<const N: usize> {
    buffer: [u8; N],
    index: usize,
    config: AudioConfig,
}

impl<const N: usize> SampleBufferSource<N> {
    pub fn new(buffer: [u8; N], config: AudioConfig) -> Self {
        Self {
            buffer,
            index: 0,
            config,
        }
    }
}

impl<const N: usize> Source for SampleBufferSource<N> {
    fn current_frame_len(&self) -> Option<usize> {
        let remaining = self.buffer.len().saturating_sub(self.index);
        if remaining == 0 {
            None
        } else {
            Some(remaining)
        }
    }

    fn channels(&self) -> u16 {
        self.config.channels
    }

    fn sample_rate(&self) -> u32 {
        self.config.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        let samples = self.buffer.len() as u64;
        let duration_nanos = (samples * 1_000_000_000) / (self.config.sample_rate as u64);
        Some(Duration::from_nanos(duration_nanos))
    }
}

impl<const N: usize> Iterator for SampleBufferSource<N> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buffer.len() {
            return None;
        }

        // Conversione sicura da u8 a i8, poi a i16
        let sample_u8 = self.buffer[self.index];
        let sample_i8 = sample_u8 as i8; // Conversione sicura
        self.index += 1;

        Some(sample_i8.to_sample::<i16>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_handler_creation() {
        let handler = AudioHandler::new();
        assert_eq!(handler.config.sample_rate, 15360);
        assert_eq!(handler.config.channels, 1);
    }

    #[test]
    fn test_sample_source_duration() {
        let buffer = [128u8; 256];
        let config = AudioConfig::default();
        let source = SampleBufferSource::new(buffer, config);

        let expected_duration = Duration::from_nanos((256 * 1_000_000_000) / 15360);
        assert_eq!(source.total_duration(), Some(expected_duration));
    }
}
