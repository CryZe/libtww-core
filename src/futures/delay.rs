use crate::system::tww::get_frame_count;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct DelayUntil(u32);

impl Future for DelayUntil {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if get_frame_count() >= self.0 {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub fn delay_until(frame: u32) -> DelayUntil {
    DelayUntil(frame)
}

pub fn delay_for(frames: u32) -> DelayUntil {
    DelayUntil(get_frame_count() + frames)
}
