use bumpalo::Bump;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    // next 是强引用，拥有所有权
    next: Option<Rc<Node>>,
    // prev 是弱引用，不拥有所有权，用于防止循环
    prev: Option<Weak<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node with value: {}", self.value);
    }
}

fn cycle_weak_sample() {
    // 创建节点 A (强引用)
    let a = Rc::new(Node {
        value: 1,
        next: None,
        prev: None,
    });

    // 创建节点 B (强引用)
    let b = Rc::new(Node {
        value: 2,
        next: None,
        prev: None,
    });

    println!(
        "Initial strong counts: a={}, b={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b)
    );

    // 修改 A 的 next 指向 B (强引用)
    // 需要使用 RefCell 或类似方法来修改 Rc 内部的值，这里为了简化省略 RefCell，
    // 假设可以通过其他方式设置字段或在构建时设置
    // 实际中更像这样:
    let a_ref = Rc::clone(&a); // 克隆强引用给 B 持有
    let b_ref = Rc::clone(&b); // 克隆强引用给 A 持有

    // Node { value: 1, next: Some(b_ref), prev: ??? }
    // Node { value: 2, next: ???, prev: Some(a_ref) }

    // 为了演示方便，我们先创建，再手动设置（实际代码需要 RefCell 或 builder模式）
    // 我们不能直接修改 Rc 内部字段，通常结构会更像:
    struct NodeWithRefCell {
        value: i32,
        next: RefCell<Option<Rc<NodeWithRefCell>>>,
        prev: RefCell<Option<Weak<NodeWithRefCell>>>,
    }

    let a_rc = Rc::new(NodeWithRefCell {
        value: 1,
        next: RefCell::new(None),
        prev: RefCell::new(None),
    });
    let b_rc = Rc::new(NodeWithRefCell {
        value: 2,
        next: RefCell::new(None),
        prev: RefCell::new(None),
    });

    // A -> B (next 是强引用)
    a_rc.next.borrow_mut().replace(Rc::clone(&b_rc));
    println!(
        "Set A.next to B. Strong counts: a={}, b={}",
        Rc::strong_count(&a_rc),
        Rc::strong_count(&b_rc)
    ); // a_rc_初始(1) + a_rc_给b(1) = 2; b_rc_初始(1) + b_rc_被a_next指(1) = 2

    // B -> A (prev 是弱引用)
    b_rc.prev.borrow_mut().replace(Rc::downgrade(&a_rc)); // 使用 Weak
    println!(
        "Set B.prev to Weak(A). Strong counts: a={}, b={}",
        Rc::strong_count(&a_rc),
        Rc::strong_count(&b_rc)
    ); // Weak 不增加强引用计数，所以计数不变

    // 此时，A 和 B 互相引用，但 B 指向 A 的是弱引用。
    // 外部对 a_rc 和 b_rc 还有强引用 (main 函数持有的)。

    println!("\nDropping a_rc and b_rc strong references...");
    // main 函数持有的 a_rc 和 b_rc 强引用即将离开作用域被丢弃
} // <- a_rc, b_rc 离开作用域被丢弃

// a_rc 强引用计数减 1 -> 1
// b_rc 强引用计数减 1 -> 1 (因为 a_rc.next 仍然持有 b_rc 的强引用)

// 当 a_rc 所在的内存（假设它先被处理）的强引用计数降到 1 时，
// 检查发现它没有其他外部强引用了，只有 b_rc.prev 指向它的弱引用。
// 对象 A 被 drop。调用 Drop::drop for NodeWithRefCell value: 1.
// Drop(A) 会尝试 drop 其字段 next 和 prev。
// next 字段持有的 Rc<B> 的强引用计数减 1 -> 0。
// 对象 B 的强引用计数降到 0。调用 Drop::drop for NodeWithRefCell value: 2.
// Drop(B) 会尝试 drop 其字段 next 和 prev。
// next 字段是 None。prev 字段持有的 Weak<A> 被丢弃，不影响 A 的计数。

// 结论：通过使用 Weak 引用，循环被打破，所有对象都能被正确 drop 和回收内存。

use std::sync::{self, Arc}; // 导入 Arc 和 Weak // 用于 Arc/Rc 内部的可变性
                            // 修改节点结构，prev 链接使用 Weak 引用
struct NodeFixed {
    value: i32,
    next: RefCell<Option<Arc<NodeFixed>>>,
    // prev 使用 Weak 引用，不会增加强引用计数
    prev: RefCell<Option<sync::Weak<NodeFixed>>>,
}

// 仍然实现 Drop 来观察销毁时机
impl Drop for NodeFixed {
    fn drop(&mut self) {
        println!("Dropping NodeFixed with value: {}", self.value);
    }
}
fn cycle_arc_sample() {
    println!("\n--- 使用 Arc 和 Weak 解决循环引用 ---");

    // 使用块作用域来明确控制 Arc 变量的生命周期，方便观察 Drop
    {
        // 创建节点 A (强引用)
        let a = Arc::new(NodeFixed {
            value: 1,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        });
        println!(
            "创建节点 A. Arc 强引用计数 (a): {}, 弱引用计数 (a): {}",
            Arc::strong_count(&a),
            Arc::weak_count(&a)
        ); // S=1, W=0

        // 创建节点 B (强引用)
        let b = Arc::new(NodeFixed {
            value: 2,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        });
        println!(
            "创建节点 B. Arc 强引用计数 (b): {}, 弱引用计数 (b): {}",
            Arc::strong_count(&b),
            Arc::weak_count(&b)
        ); // S=1, W=0

        println!("\n链接 A -> B (使用 Arc)...");
        // 将 A 的 next 指向 B (使用 Arc 的克隆，增加 B 的强引用计数)
        a.next.borrow_mut().replace(Arc::clone(&b));
        println!(
            "链接后 Arc 强引用计数 (a): {}, 弱引用计数 (a): {}",
            Arc::strong_count(&a),
            Arc::weak_count(&a)
        ); // S=1, W=0
        println!(
            "链接后 Arc 强引用计数 (b): {}, 弱引用计数 (b): {}",
            Arc::strong_count(&b),
            Arc::weak_count(&b)
        ); // S=2 (原始 b + a.next 持有), W=0

        println!("链接 B -> A (使用 Weak)...");
        // 将 B 的 prev 指向 A (使用 Arc::downgrade() 获取弱引用)
        b.prev.borrow_mut().replace(Arc::downgrade(&a));
        println!(
            "链接后 Arc 强引用计数 (a): {}, 弱引用计数 (a): {}",
            Arc::strong_count(&a),
            Arc::weak_count(&a)
        ); // S=1, W=1 (原始 a + b.prev 持有弱引用)
        println!(
            "链接后 Arc 强引用计数 (b): {}, 弱引用计数 (b): {}",
            Arc::strong_count(&b),
            Arc::weak_count(&b)
        ); // S=2, W=0

        // 此时，我们创建了一个循环： a (强) -> b (强) -> a (弱)
        // a 的强引用计数是 1: main 函数持有 1个
        // b 的强引用计数是 2: main 函数持有 1个, a.next 持有 1个
        // a 的弱引用计数是 1: b.prev 持有 1个

        println!("\n块作用域结束，main 函数持有的 a 和 b 强引用即将丢弃...");
        // 当块作用域结束时，a 和 b 变量（它们是 Arc 强引用）被丢弃。
    } // <- a 和 b 在这里被丢弃

    // 追踪 Drop 过程：
    // 1. 假设 a 先被丢弃。a 的强引用计数从 1 变为 0。
    // 2. 因为 a 的强引用计数归零，a 指向的 NodeFixed 对象被 drop。
    // 3. 调用 Drop::drop for NodeFixed value: 1。打印 "Dropping NodeFixed with value: 1"。
    // 4. 在 Drop(a) 中，其字段被丢弃：
    //    - next 字段持有的 Arc<NodeFixed> (指向 b) 被丢弃。b 的强引用计数从 2 变为 1。
    //    - prev 字段是 None。
    // 5. 现在 b 的强引用计数是 1 (由 main 函数中原始的 b 变量持有)。对象 B 尚未被 drop。
    // 6. 接着 b 被丢弃。b 的强引用计数从 1 变为 0。
    // 7. 因为 b 的强引用计数归零，b 指向的 NodeFixed 对象被 drop。
    // 8. 调用 Drop::drop for NodeFixed value: 2。打印 "Dropping NodeFixed with value: 2"。
    // 9. 在 Drop(b) 中，其字段被丢弃：
    //    - next 字段是 None。
    //    - prev 字段持有的 Weak<NodeFixed> (指向 a) 被丢弃。a 的弱引用计数从 1 变为 0。
    // 10. 所有强引用和弱引用计数都归零，所有对象都已 drop，内存被回收。

    println!("\n块作用域已结束。如果节点被正确销毁，你应该已经看到了它们的 Drop 输出。");
    println!("--- 程序结束 (无泄漏) ---");
}
struct NodeWithBump<'a> {
    value: i32,
    next: RefCell<Option<&'a NodeWithBump<'a>>>, // 使用 RefCell 允许内部可变性
}

fn create_cycle<'a>(bump: &'a Bump) {
    let node1 = bump.alloc(NodeWithBump {
        value: 1,
        next: RefCell::new(None),
    });
    let node2 = bump.alloc(NodeWithBump {
        value: 2,
        next: RefCell::new(None),
    });

    // 创建循环引用 (不可变引用，通过 RefCell 修改内部 Option)
    node1.next.borrow_mut().replace(node2);
    node2.next.borrow_mut().replace(node1);

    println!("Created cycle in bump arena.");
    // 循环在这里存在，但因为是引用，不涉及所有权或引用计数。
    // 生命周期由 'a (bump) 决定。
} // <- node1, node2 引用超出作用域，但它们指向的内存在 bump 中

fn cycle_bump_sample() {
    // 创建一个 Bump 分配器
    let bump = Bump::new();

    // 创建节点并形成循环
    create_cycle(&bump);

    // bump 在这里被丢弃，内存被整体回收。
    // 由于使用了 Bump 分配器，所有分配的内存会在 bump 被 drop 时自动释放。
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_weak_cycle_sample() {
        cycle_weak_sample();
    }

    #[test]
    fn test_cycle_arc_sample() {
        cycle_arc_sample();
    }

    #[test]
    fn test_cycle_bump_sample() {
        cycle_bump_sample();
    }
}
