use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub fn block_on(f: Pin<&mut dyn Future<Output = ()>>) {
    let cx = &mut Context::from_waker(Waker::noop());
    match f.poll(cx) {
        Poll::Pending => unreachable!(),
        Poll::Ready(()) => (),
    }
}
