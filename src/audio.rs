use rodio::{cpal::Sample, source::Source};
use std::{mem::transmute, time::Duration};

pub struct AudioHandler {
    pub sample_buffer: [u8; 256],
}

impl AudioHandler {
    pub fn new() -> Self {
        Self {
            sample_buffer: [0; 256],
        }
    }

    pub fn update_sample_buffer(&mut self, memory: &[u8]) {
        let audio_addr = memory[6] as usize * 65536 + memory[7] as usize * 256;
        self.sample_buffer.copy_from_slice(&memory[audio_addr..audio_addr + 256]);
    }

    pub fn append_to_sink(&self, sink: &rodio::Sink) {
        if self.sample_buffer.iter().any(|&sample| sample != 0) {
            sink.append(SampleBufferSource::from(self.sample_buffer));
        }
    }
}

pub struct SampleBufferSource {
    inner_sample_buffer: [u8; 256],
    index: usize,
}

impl From<[u8; 256]> for SampleBufferSource {
    fn from(sample_buffer: [u8; 256]) -> Self {
        Self {
            index: 0,
            inner_sample_buffer: sample_buffer,
        }
    }
}

impl Source for SampleBufferSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.inner_sample_buffer.len() - self.index - 1)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        256 * 60
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::new(0, 16666666))
    }
}

impl Iterator for SampleBufferSource {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 256 {
            return None;
        }
        let sample: i8;
        unsafe {
            sample = transmute::<u8, i8>(self.inner_sample_buffer[self.index]);
        }
        self.index += 1;
        Some(sample.to_sample::<Self::Item>())
    }
}
