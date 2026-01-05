
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{ Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

// 在Future和等待的线程间共享状态
struct SharedState {
    // 睡眠时间是否已过完？
    completed: bool,
    // timerFuture所运行于的任务的Waker
    // 设置completed为true后，thread可以用其通知：TimerFuture所在的任务可以被唤醒了，看到completed = true并前进
    waker: Option<Waker>,
    // 这个waker就是TimerFuture所需要运行的所在任务？
}

impl Future for TimerFuture {
    type Output = (); // 类型output是空的
    // 查看shared state，看下timer是否已经结束
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // todo!()
        // 用lock来取得共享的状态
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        }
        else {
            // 复制了1份waker
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
