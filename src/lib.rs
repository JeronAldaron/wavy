//! Cross-platform real-time audio recording &amp; playback.
//!
//! The sound waves are _so_ wavy!
//!
//! # Getting Started
//! This example records audio and plays it back in real time as it's being recorded.  (Make sure to
//! wear headphones to avoid feedback).
//!
//! ```rust,no_run
//! use wavy::*;
//!
//! use std::collections::VecDeque;
//!
//! fn main() -> Result<(), AudioError> {
//!     // Connect to the speakers and microphones.
//!     let mut mic = MicrophoneList::new(SampleRate::Normal)?;
//!     let mut speaker = SpeakerList::new(SampleRate::Normal)?;
//!
//!     let mut buffer = VecDeque::new();
//!
//!     loop {
//!         // Record some sound.
//!         mic.record(&mut |_whichmic, l, r| {
//!             buffer.push_back((l, r));
//!         });
//!
//!         // Play that sound.
//!         speaker.play(&mut || {
//!             if let Some((lsample, rsample)) = buffer.pop_front() {
//!                 AudioSample::stereo(lsample, rsample)
//!             } else {
//!                 // Play silence if not enough has been recorded yet.
//!                 AudioSample::stereo(0, 0)
//!             }
//!         });
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![doc(
    html_logo_url = "http://free.plopgrizzly.com/plop/icon.svg",
    html_favicon_url = "http://free.plopgrizzly.com/plop/icon.svg"
)]

mod error;
mod sample_rate;
// mod system;
mod stereo;
// mod resampler;
mod player;
mod recorder;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(any(target_os = "linux", target_os = "android"))]
use linux as ffi;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm as ffi;

#[cfg(target_os = "macos")]
mod apple;
#[cfg(target_os = "macos")]
use apple as ffi;

pub use error::AudioError;
pub use sample_rate::SampleRate;
pub use stereo::StereoS16Frame;
pub use recorder::Recorder;
pub use player::Player;
