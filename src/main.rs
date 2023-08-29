#[cfg(feature = "accelerate")]
extern crate accelerate_src;

#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

const SAMPLE_RATE: usize = 16000;
const N_FFT: usize = 400;
const N_MELS: usize = 80;
const HOP_LENGTH: usize = 160;
const CHUNK_LENGTH: usize = 30;
const N_SAMPLES: usize = CHUNK_LENGTH * SAMPLE_RATE; // 480000 samples in a 30-second chunk
const N_FRAMES: usize = N_SAMPLES / HOP_LENGTH; // 3000 frames in a mel spectrogram input

const NO_SPEECH_THRESHOLD: f64 = 0.6;
const LOGPROB_THRESHOLD: f64 = -1.0;
const TEMPERATURES: [f64; 6] = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
const COMPRESSION_RATIO_THRESHOLD: f64 = 2.4;

// Tokenizer dependent bits.
const SOT_TOKEN: &str = "<|startoftranscript|>";
const TRANSCRIBE_TOKEN: &str = "<|transcribe|>";
const TRANSLATE_TOKEN: &str = "<|translate|>";
const NO_TIMESTAMPS_TOKEN: &str = "<|notimestamps|>";
const EOT_TOKEN: &str = "<|endoftext|>";
const NO_SPEECH_TOKEN: &str = "<|nocaptions|>";

mod model;
mod audio;
mod multilingual;
use model::Whisper;
use audio::pcm_to_mel;
use multilingual::detect_language;

extern crate candle_core as candle;


fn main() {
    println!("Hello, world!");
}
