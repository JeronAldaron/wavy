// Copyright Jeron Aldaron Lau 2019 - 2020.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use std::{
    fmt::{Display, Error, Formatter},
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use fon::{chan::Ch32, Frame, Stream};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    MediaStream, MediaStreamAudioSourceNode, MediaStreamAudioSourceOptions,
    MediaStreamConstraints,
};

use super::SoundDevice;

pub(crate) struct Microphone();

impl Display for Microphone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("Default")
    }
}

impl SoundDevice for Microphone {
    const INPUT: bool = true;
}

impl Default for Microphone {
    fn default() -> Self {
        let state = super::state();

        // Lazily Initialize audio context & processor node.
        state.lazy_init();

        // Prompt User To Connect Microphone.
        let md = web_sys::window()
            .unwrap()
            .navigator()
            .media_devices()
            .ok()
            .unwrap();
        let promise = md
            .get_user_media_with_constraints(
                MediaStreamConstraints::new().audio(&JsValue::TRUE),
            )
            .unwrap();
        #[allow(trivial_casts)] // Actually needed here.
        let cb = Closure::wrap(Box::new(|media_stream| {
            let state = super::state();
            // Create audio source from media stream.
            let audio_src = MediaStreamAudioSourceNode::new(
                state.context.as_ref().unwrap(),
                &MediaStreamAudioSourceOptions::new(
                    &MediaStream::unchecked_from_js(media_stream),
                ),
            )
            .unwrap();

            // Connect microphones to processor node.
            audio_src
                .connect_with_audio_node(state.proc.as_ref().unwrap())
                .unwrap();

            // Add to connected microphones (refresh browser to remove).
            state.microphone.push(audio_src);
        }) as Box<dyn FnMut(_)>);
        let _ = promise.then(&cb);
        cb.forget();

        Self()
    }
}

impl Microphone {
    pub(crate) fn record<F: Frame<Chan = Ch32>>(
        &mut self,
    ) -> MicrophoneStream<'_, F> {
        MicrophoneStream { index: 0, _phantom: PhantomData }
    }

    pub(crate) fn channels(&self) -> u8 {
        0b0000_0001
    }
}

impl Future for Microphone {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let state = super::state();
        if state.recorded {
            state.recorded = false;
            Poll::Ready(())
        } else {
            state.mics_waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub(crate) struct MicrophoneStream<'a, F: Frame<Chan = Ch32>> {
    // Index into buffer
    index: usize,
    //
    _phantom: PhantomData<&'a F>,
}

impl<F: Frame<Chan = Ch32>> Iterator for MicrophoneStream<'_, F> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        // Grab global state.
        let state = super::state();

        if self.index == state.i_buffer.len() {
            return None;
        }
        let frame = F::from_channels(&[Ch32::new(state.i_buffer[self.index])]);
        self.index += 1;
        Some(frame)
    }
}

impl<F: Frame<Chan = Ch32>> Stream<F> for MicrophoneStream<'_, F> {
    fn sample_rate(&self) -> Option<f64> {
        Some(super::SAMPLE_RATE.into())
    }

    fn len(&self) -> Option<usize> {
        Some(crate::consts::PERIOD.into())
    }
}
