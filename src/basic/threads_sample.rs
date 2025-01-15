//!
//! Basic Thread Sample
//!
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Barrier, Condvar, Mutex, Once, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

/**
 * 一个简单创建线程样例
 *  
 *  `
 *  thread::spawn(|| {});
 *  `
 */
pub(crate) fn create_thread_sample() {
    println!("thread sample ,create on thread .....start");

    //创建线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //等待线程结束
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3, 4, 5, 6];

    //使用 move 来将v 所有权从一个线程转移到另外一个线程。
    //可以尝试删除 move ,将报错
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    println!("thread sample ,create on thread .....end\n");
}

/**
 * 创建多个线程，并发进行生成数据，返回给主线程打印结果。
 */
pub(crate) fn thread_callable_sample() {
    println!("threads callable sample .....start ");

    let mut handles = Vec::new();

    //线程数量
    let num_threads = 16;

    //per thread size
    let adds_per_thread = 10;

    //创建线程
    for i in 0..num_threads {
        //创建线程并返回结果HasMap
        let handle: thread::JoinHandle<HashMap<String, String>> = thread::spawn(move || {
            let mut ht: HashMap<String, String> = HashMap::new();

            for j in 0..adds_per_thread {
                //key
                let key = format!("key-{}-{}", i, j);

                //value
                let value = format!("value-{}", j * 100);

                println!("inner thread insert is  kye: {} ,vlaue: {} ", key, value);

                ht.insert(key, value);
            }

            //返回线程执行结果
            return ht;
        });

        handles.push(handle);
    }

    //全部线程执行结果
    let mut results: HashMap<String, String> = HashMap::new();

    for handle in handles {
        //等待线程结束
        let result = handle.join().unwrap();

        println!("results: {:?}", result);

        //将线程结果 插入到 Results
        for (k, v) in result {
            results.insert(k, v);
        }

        //此功能，等同于以上遍历插入全部Map值
        //    results.extend(r);
    }

    //打印结果集长度
    println!("ht result length is {}", results.len());

    for (key, value) in results {
        println!(
            "hi thread callable result ,is  kye: {} ,vlaue: {} ",
            key, value
        );
    }

    println!("threads callable sample .....end\n");
}

/**
 * 线程屏障(Barrier)
 */
pub(crate) fn thread_barrier_sample() {
    let mut handles = Vec::with_capacity(6);

    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/**
 *  thread local sample
 */
pub(crate) fn thread_local_sample() {
    //定义thread local 变量 FOO
    //FOO 使用 static 声明为生命周期为 'static 的静态变量
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    //初始化 FOO 值为2
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到线程局部变量的FOO的初始值
    let t = thread::spawn(move || {
        //线程内设置 FOO 的值
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    // 等待线程完成
    t.join().unwrap();

    // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

/**
 * 用条件控制线程的挂起和执行
 * 条件变量(Condition Variables)经常和 Mutex 一起使用，可以让线程挂起，直到某个条件发生后再继续执行
 */
pub(crate) fn thread_lock_sample() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;

        let mut started = lock.lock().unwrap();
        println!("thread lock status : {}", started);

        println!("changing started");

        *started = true;

        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;

    let mut started = lock.lock().unwrap();

    println!("main thread lock status : {}", started);

    while !*started {
        started = cvar.wait(started).unwrap();

        println!("thread changed after lock status : {}", started);
    }

    println!("started changed");

    //读写锁
    let lock = RwLock::new(5);

    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被drop

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
    } // 写锁在此处被drop
}

/**
 * 线程只运行1次
 */
pub(crate) fn thread_call_once_sample() {
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    unsafe {
        println!("call once val is {}", VAL);
    }

    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            thread::sleep(Duration::from_millis(10));
            VAL = 1;
            println!("call once val is {}", VAL);
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
            println!("call once val is {}", VAL);
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}

/**
 *  thread message channel
 *  mpsc是multiple producer, single consumer的缩写
 */
pub(crate) fn thread_mpsc_channel_sample() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        println!("send 1 once is 1");
        tx.send(1).unwrap();

        println!("send 2 once is 2");
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());

    println!("receive {}", rx.recv().unwrap());

    //使用 for 进行循环接收
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        //多次发送
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //使用 for 进行循环接收
    for received in rx {
        println!("Got: {}", received);
    }

    //多个线程发送者，一个接收者
    //所有发送者被drop或者所有接收者被drop后，通道会自动关闭。
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

//线程安全
//atomic
//原子类型是一种特殊的类型，它们可以在多线程环境中安全地共享和修改。
const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;
//原子类型
static R: AtomicU64 = AtomicU64::new(0);
//线程安全 无锁累计
fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

/**
 *  Atomic 原子类型，AtomicU64 线程安全无锁累计
 */
pub(crate) fn thread_atomic_sample() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("{:?}", Instant::now().sub(s));
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_simple_thread_sample() {
        create_thread_sample();
    }

    #[test]
    fn test_multi_thread_val_sample() {
        thread_callable_sample();
    }

    #[test]
    fn test_thread_barrier_sample() {
        thread_barrier_sample();
    }

    #[test]
    fn test_thread_local_sample() {
        thread_local_sample();
    }

    #[test]
    fn test_thread_lock_sample() {
        thread_lock_sample();
    }

    #[test]
    fn test_thread_call_once_sample() {
        thread_call_once_sample();
    }

    #[test]
    fn test_thread_mpsc_channel_sample() {
        thread_mpsc_channel_sample();
    }

    #[test]
    fn test_thread_atomic_sample() {
        thread_atomic_sample();
    }
}
