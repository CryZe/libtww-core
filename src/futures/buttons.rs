use crate::{
    game::gamepad::{self, Buttons},
    system::tww,
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::stream::Stream;

pub struct ButtonPresses(Buttons, u32);

impl Stream for ButtonPresses {
    type Item = Buttons;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if tww::get_frame_count() != self.1 {
            self.1 = tww::get_frame_count();
            self.0.insert(gamepad::buttons_pressed());
        }

        for &button in &[
            Buttons::DPAD_LEFT,
            Buttons::DPAD_RIGHT,
            Buttons::DPAD_DOWN,
            Buttons::DPAD_UP,
            Buttons::Z,
            Buttons::R,
            Buttons::L,
            Buttons::A,
            Buttons::B,
            Buttons::X,
            Buttons::Y,
            Buttons::START,
        ] {
            if self.0.contains(button) {
                self.0.remove(button);
                return Poll::Ready(Some(button));
            }
        }

        return Poll::Pending;
    }
}

pub fn button_presses() -> ButtonPresses {
    ButtonPresses(Buttons::empty(), tww::get_frame_count())
}

pub struct ButtonsDown(Buttons);

impl Future for ButtonsDown {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if gamepad::is_down(self.0) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub const fn buttons_down(buttons: Buttons) -> ButtonsDown {
    ButtonsDown(buttons)
}

pub struct ButtonsPressed(Buttons);

impl Future for ButtonsPressed {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if gamepad::is_pressed(self.0) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub const fn buttons_pressed(buttons: Buttons) -> ButtonsPressed {
    ButtonsPressed(buttons)
}
