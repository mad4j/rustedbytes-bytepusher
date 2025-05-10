use rodio::{cpal::Sample, source::Source};
use std::{mem::transmute, time::Duration};

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
        let mut sample: i8 = 0;
        unsafe {
            sample = transmute::<u8, i8>(self.inner_sample_buffer[self.index]);
        }
        self.index += 1;
        Some(sample.to_sample::<Self::Item>())
    }
}
