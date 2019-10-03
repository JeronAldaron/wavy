//! Audio System (SpeakerSystem & MicrophoneSystem).

/// An AudioSample (with surround sound 5.1 support).
pub struct AudioSample {
    front_left: i16,
    front_right: i16,
    #[allow(unused)] // TODO
    center: i16,
    #[allow(unused)] // TODO
    lfe: i16,
    surround_left: i16,
    surround_right: i16,
}

impl AudioSample {
    /// Create stereo audio sample.
    pub fn stereo(left: i16, right: i16) -> AudioSample {
        AudioSample {
            front_left: left,
            front_right: right,
            center: 0,
            lfe: 0,
            surround_left: left,
            surround_right: right,
        }
    }

    /// Create surround sound 5.1 audio sample.
    /// * Center: 0°.
    /// * Front-Left: -30°
    /// * Front-Right: 30°
    /// * Surround-Left: -110°
    /// * Surround-Right: 110°
    ///
    /// _source:_ [https://en.wikipedia.org/wiki/5.1_surround_sound#Music](https://en.wikipedia.org/wiki/5.1_surround_sound#Music)
    pub fn surround(
        front_left: i16,
        front_right: i16,
        front_center: i16,
        lfe: i16,
        surround_left: i16,
        surround_right: i16,
    ) -> AudioSample {
        AudioSample {
            front_left,
            front_right,
            center: front_center,
            lfe,
            surround_left,
            surround_right,
        }
    }
}

/// Audio (Speaker) output.  This type represents a speaker system.
pub struct SpeakerSystem(crate::ffi::Speaker);

impl SpeakerSystem {
    /// Connect to the speaker system at a specific sample rate.
    pub fn new(
        sr: crate::SampleRate,
    ) -> Result<SpeakerSystem, crate::AudioError> {
        Ok(SpeakerSystem(crate::ffi::Speaker::new(sr)?))
    }

    /// Generate audio samples as they are needed.  In your closure return S16_LE audio samples.
    pub fn play(&mut self, generator: &mut dyn FnMut() -> AudioSample) {
        // TODO: Right now we're just combining into a stereo track for playback whether or not we
        // have 5.1 support.
        self.0.play(&mut || {
            let sample = generator();

            let l = (i32::from(sample.front_left)
                + i32::from(sample.surround_left))
                / 2;
            let r = (i32::from(sample.front_right)
                + i32::from(sample.surround_right))
                / 2;
            (l as i16, r as i16)
        })
    }
}

/// Audio (Microphone) input.
pub struct MicrophoneSystem(crate::ffi::Microphone);

impl MicrophoneSystem {
    /// Connect to the microphone system at a specific sample rate.
    pub fn new(
        sr: crate::SampleRate,
    ) -> Result<MicrophoneSystem, crate::AudioError> {
        Ok(MicrophoneSystem(crate::ffi::Microphone::new(sr)?))
    }

    /// Record audio from the microphone system.  The closures first parameter is the microphone id.
    /// The 2nd and 3rd are left and right sample.
    pub fn record(&mut self, generator: &mut dyn FnMut(usize, i16, i16)) {
        self.0.record(generator);
    }
}
