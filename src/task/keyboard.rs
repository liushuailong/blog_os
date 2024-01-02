use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println;
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

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