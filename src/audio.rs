use crate::cpu::{AUDIO_BUFFER_SIZE, AUDIO_SAMPLES_PER_SECOND};
use crate::memory::Memory;

use rodio::{Sink, cpal::Sample, source::Source};
use std::cell::RefCell;
use std::rc::Rc;
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
            sample_rate: AUDIO_SAMPLES_PER_SECOND, // 256 * 60
            channels: 1,
        }
    }
}

pub struct AudioHandler {
    config: AudioConfig,
    memory_register_addr: usize,
    memory: Rc<RefCell<Memory>>,
    sink: Rc<RefCell<Sink>>,
}

impl AudioHandler {
    pub fn new(
        memory_register_addr: usize,
        memory: Rc<RefCell<Memory>>,
        sink: Rc<RefCell<Sink>>,
    ) -> Self {
        Self {
            config: AudioConfig::default(),
            memory,
            memory_register_addr,
            sink,
        }
    }

    /// Recupera il buffer dalla memoria e lo aggiunge al sink se contiene dati non-zero
    pub fn append_samples(&self) {
        let buffer = self.get_sample_buffer();
        if buffer.iter().any(|&sample| sample != 0) {
            let source = SampleBufferSource::new(buffer, self.config.clone());
            self.sink.borrow_mut().append(source);
        }
    }

    pub fn get_sample_buffer(&self) -> [u8; AUDIO_BUFFER_SIZE] {
        let mem = self.memory.borrow();
        let audio_addr = (mem.read_16_bits(self.memory_register_addr) as usize) << 8;
        let sample_buffer = &mem[audio_addr..audio_addr + AUDIO_BUFFER_SIZE];
        let mut arr = [0u8; AUDIO_BUFFER_SIZE];
        arr.copy_from_slice(sample_buffer);
        arr
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

    /// Converte un sample da u8 a i16 passando per i8
    fn convert_sample(sample: u8) -> i16 {
        (sample as i8).to_sample::<i16>()
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
        self.buffer.get(self.index).map(|&sample| {
            self.index += 1;
            Self::convert_sample(sample)
        })
    }
}
