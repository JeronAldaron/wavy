use crate::ffi;
use crate::frame::Frame;

/// Audio Player (Speaker output).
pub struct Player<F: Frame>(pub(crate) ffi::Player<F>);

impl<F: Frame> Player<F> {
    /// Connect to the speaker system at a specific sample rate.
    pub fn new(sr: u32) -> Option<Player<F>> {
        Some(Player(ffi::Player::new(sr)?))
    }

    /// Play audio samples from an iterator.  Get a future that returns the
    /// number of audio samples played.
    pub async fn play_last(&mut self, iter: &[F]) -> usize {
        self.0.play_last(iter).await
    }
}
