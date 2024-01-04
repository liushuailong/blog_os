use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println;
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

// 这个函数不应该在main.rs中调用，使用pub(crate)限制该函数只能在lib.rs库中使用;
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScanCodeSteam {
    _private: (),
}

impl ScanCodeSteam {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100)).expect("ScanCodeStream::new should try be called once");
        ScanCodeSteam {
            _private: ()
        }
    }
}
use futures_util::stream::Stream;
use core::{
    pin::Pin,
    task::{
        Poll,
        Context,
    },
};
impl Stream for ScanCodeSteam {
    type Item = u8;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
        if let Ok(scan_code) = queue.pop() {
            return Poll::Ready(Some(scan_code));
        }
        // 为什么不是将waker注册后，直接返回Poll::Pending?
        // 为什么要先注册后，再进行一次pop操作？
        // 这是因为再注册后，可能立马就会唤醒，这是就会发生资源竞争，返回Poll::Ready;如果立马返回Poll::Rending，就会发生冲突；
        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scan_code) => {
                WAKER.take();
                Poll::Ready(Some(scan_code))
            },
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

use futures_util::task::AtomicWaker;
// Waker 的注册、通知、删除，需要进一步理解，该过程不太清楚；
// AtomicWaker 结构体实例化一个warker，向worker队列中注册任务的waker方法，使用waker的wake方法，唤醒waker中对应的任务，运行任务的；
static WAKER: AtomicWaker = AtomicWaker::new();

use futures_util::stream::StreamExt;
use pc_keyboard::{
    layouts,
    DecodedKey,
    HandleControl,
    Keyboard,
    ScancodeSet1,
};
use crate::print;

pub async fn print_keypresses() {
    let mut scancodes = ScanCodeSteam::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    while let Some(scan_code) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                    DecodedKey::Unicode(character) => print!("{}", character),
                }
            }
        }
    }
}