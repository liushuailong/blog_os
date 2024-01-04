pub mod sample_executor;
pub mod keyboard;
pub mod executor;
use core::{
    future::Future,
    pin::Pin,
};
use alloc::boxed::Box;
pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }
}

use core::task::{
    Context,
    Poll,
};

impl Task {
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

use core::sync::atomic::{
    AtomicU64,
    Ordering,
};

impl TaskId {
    fn new() -> Self {
        // 怎么理解这个static变量？
        // 通常理解taskid是递增的，每创建一个task，赋予一个taskid；
        // 但是下面语句我理解为每创建一个taskid，这个静态变量NEXT_ID新生成初始化一次，这显然不符合实际；那推测自己在这个地方对这个静态变量的理解是错误的。
        // 个人理解：在编译器 静态变量已经已经初始化，在程序运行的时候之后执行下面第二行的语句内容，第一行在编译期已经完成，打包在了二进制文件里面；因此这个初始化在函数内和函数外效果是一样的；
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}