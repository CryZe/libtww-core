use core::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

mod buttons;
mod delay;

pub use buttons::*;
pub use delay::*;

pub fn run<F: Future + ?Sized>(f: Pin<&mut F>) -> Option<F::Output> {
    const VTABLE: &'static RawWakerVTable = &RawWakerVTable::new(clone_waker, dummy, dummy, dummy);
    const RAW_WAKER: RawWaker = RawWaker::new(ptr::null(), VTABLE);

    unsafe fn clone_waker(_: *const ()) -> RawWaker {
        RAW_WAKER
    }

    unsafe fn dummy(_: *const ()) {}

    match f.poll(&mut Context::from_waker(unsafe {
        &Waker::from_raw(RAW_WAKER)
    })) {
        Poll::Ready(val) => Some(val),
        _ => None,
    }
}
