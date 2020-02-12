use crate::game::gamepad;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct ButtonsDown(u16);

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

pub fn buttons_down(buttons: u16) -> ButtonsDown {
    ButtonsDown(buttons)
}

pub struct ButtonsPressed(u16);

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

pub fn buttons_pressed(buttons: u16) -> ButtonsPressed {
    ButtonsPressed(buttons)
}
