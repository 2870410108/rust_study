use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// 线程池创建错误
#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize,
    TooLarge(usize),
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoolCreationError::ZeroSize => write!(f, "线程池大小不能为0"),
            PoolCreationError::TooLarge(size) => {
                write!(f, "线程池大小 {} 超过最大限制 1000", size)
            }
        }
    }
}

impl std::error::Error for PoolCreationError {}

/// 任务类型
type Job = Box<dyn FnOnce() + Send + 'static>;

/// 工作线程接收的消息
enum Message {
    /// 新任务
    NewJob(Job),
    /// 终止信号
    Terminate,
}

/// 线程池
pub struct ThreadPool {
    /// 工作线程
    workers: Vec<Worker>,

    /// 任务发送端
    sender: Option<mpsc::Sender<Message>>,

    /// 活跃任务计数
    active_count: Arc<AtomicUsize>,

    /// 完成任务计数
    completed_count: Arc<AtomicUsize>,

    /// 总提交任务计数
    submitted_count: Arc<AtomicUsize>,
}

impl ThreadPool {
    /// 创建线程池（panic 版本，用于简单场景）
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool::build(size).expect("创建线程池失败")
    }

    /// 创建线程池（Result 版本，用于需要错误处理的场景）
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroSize);
        }

        if size > 1000 {
            return Err(PoolCreationError::TooLarge(size));
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let active_count = Arc::new(AtomicUsize::new(0));
        let completed_count = Arc::new(AtomicUsize::new(0));
        let submitted_count = Arc::new(AtomicUsize::new(0));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&active_count),
                Arc::clone(&completed_count),
            ));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
            active_count,
            completed_count,
            submitted_count,
        })
    }

    /// 执行任务
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.submitted_count.fetch_add(1, Ordering::SeqCst);

        self.sender
            .as_ref()
            .expect("线程池已关闭")
            .send(Message::NewJob(job))
            .expect("发送任务失败");
    }

    /// 执行任务并返回结果
    pub fn execute_with_result<F, T>(&self, f: F) -> mpsc::Receiver<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();

        self.execute(move || {
            let result = f();
            let _ = tx.send(result);
        });

        rx
    }

    /// 获取工作线程数
    pub fn size(&self) -> usize {
        self.workers.len()
    }

    /// 获取活跃任务数
    pub fn active_count(&self) -> usize {
        self.active_count.load(Ordering::SeqCst)
    }

    /// 获取已完成任务数
    pub fn completed_count(&self) -> usize {
        self.completed_count.load(Ordering::SeqCst)
    }

    /// 获取已提交任务数
    pub fn submitted_count(&self) -> usize {
        self.submitted_count.load(Ordering::SeqCst)
    }

    /// 获取等待中的任务数
    pub fn queued_count(&self) -> usize {
        let submitted = self.submitted_count.load(Ordering::SeqCst);
        let completed = self.completed_count.load(Ordering::SeqCst);
        let active = self.active_count.load(Ordering::SeqCst);
        submitted.saturating_sub(completed + active)
    }

    /// 优雅关闭线程池
    /// 等待所有任务完成后再关闭
    pub fn shutdown(&mut self) {
        println!("正在关闭线程池...");

        // 关闭发送端，不再接受新任务
        drop(self.sender.take());

        // 等待所有线程完成
        for worker in &mut self.workers {
            println!("关闭 worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("线程池已关闭");
    }

    /// 强制关闭线程池
    /// 立即停止所有线程，不等待任务完成
    pub fn shutdown_now(&mut self) {
        println!("强制关闭线程池...");

        // 向所有 worker 发送终止信号
        for _ in &self.workers {
            self.sender
                .as_ref()
                .unwrap()
                .send(Message::Terminate)
                .unwrap();
        }

        // 关闭发送端
        drop(self.sender.take());

        // 等待所有线程退出
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("线程池已强制关闭");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if self.sender.is_some() {
            self.shutdown();
        }
    }
}

/// 工作线程
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        active_count: Arc<AtomicUsize>,
        completed_count: Arc<AtomicUsize>,
    ) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // 接收消息
                let message = receiver.lock().expect("获取锁失败").recv();

                match message {
                    Ok(Message::NewJob(job)) => {
                        println!("Worker {} 开始执行任务", id);

                        // 增加活跃计数
                        active_count.fetch_add(1, Ordering::SeqCst);

                        // 执行任务
                        job();

                        // 减少活跃计数，增加完成计数
                        active_count.fetch_sub(1, Ordering::SeqCst);
                        completed_count.fetch_add(1, Ordering::SeqCst);

                        println!("Worker {} 完成任务", id);
                    }
                    Ok(Message::Terminate) => {
                        println!("Worker {} 收到终止信号", id);
                        break;
                    }
                    Err(_) => {
                        println!("Worker {} 发送端已关闭，退出", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

use std::time::Duration;

fn main() {
    // 示例1：基本使用
    println!("=== 示例1：基本使用 ===");
    {
        let pool = ThreadPool::new(4);

        for i in 0..8 {
            pool.execute(move || {
                println!("任务 {} 执行中", i);
                thread::sleep(Duration::from_millis(500));
            });
        }

        // pool 在作用域结束时自动调用 drop，优雅关闭
    }

    println!("\n=== 示例2：带返回值的任务 ===");
    {
        let pool = ThreadPool::new(2);

        let receivers: Vec<_> = (0..5)
            .map(|i| {
                pool.execute_with_result(move || {
                    thread::sleep(Duration::from_millis(100));
                    i * i
                })
            })
            .collect();

        for (i, rx) in receivers.into_iter().enumerate() {
            let result = rx.recv().unwrap();
            println!("任务 {} 的结果：{}", i, result);
        }
    }

    println!("\n=== 示例3：监控线程池状态 ===");
    {
        let pool = ThreadPool::new(3);

        println!("线程池大小：{}", pool.size());

        for i in 0..10 {
            pool.execute(move || {
                println!("  任务 {} 开始", i);
                thread::sleep(Duration::from_millis(200));
                println!("  任务 {} 结束", i);
            });
        }

        for _ in 0..8 {
            thread::sleep(Duration::from_millis(100));
            println!(
                "状态 - 已提交:{}, 活跃:{}, 队列:{}, 已完成:{}",
                pool.submitted_count(),
                pool.active_count(),
                pool.queued_count(),
                pool.completed_count()
            );
        }
    }

    println!("\n=== 示例4：错误处理 ===");
    {
        match ThreadPool::build(0) {
            Ok(_) => println!("不应该成功"),
            Err(e) => println!("预期的错误：{}", e),
        }

        match ThreadPool::build(2000) {
            Ok(_) => println!("不应该成功"),
            Err(e) => println!("预期的错误：{}", e),
        }
    }

    println!("\n=== 示例5：强制关闭 ===");
    {
        let mut pool = ThreadPool::new(2);

        for i in 0..10 {
            pool.execute(move || {
                thread::sleep(Duration::from_secs(5));
                println!("任务 {} 完成（不会打印）", i);
            });
        }

        thread::sleep(Duration::from_millis(100));
        pool.shutdown_now(); // 立即关闭，不等待任务完成
    }
}
