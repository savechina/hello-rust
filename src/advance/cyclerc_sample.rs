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

use std::fmt::Debug; // 用于调试打印
use std::thread; // 虽然例子是单线程的，但 Arc 意味着它能用于多线程
use std::time::Duration;

// 定义一个简化的 B+Tree 节点
#[derive(Debug)] // 添加 Debug 以方便打印
struct BTreeNode {
    id: usize, // 节点标识符
    // 子节点列表：使用 Arc，表示父节点强拥有子节点的引用
    children: RefCell<Vec<sync::Arc<RefCell<BTreeNode>>>>,
    // 父节点：这是一个潜在的循环点。如果使用 Arc，Child -> Parent -> Child...
    // 我们将在这里展示导致泄漏的 Arc 版本 和 解决泄漏的 Weak 版本
    // parent: RefCell<Option<Arc<RefCell<BTreeNode>>>>, // <-- 导致泄漏的版本
    parent: RefCell<Option<sync::Weak<RefCell<BTreeNode>>>>, // <-- 解决泄漏的版本 (使用 Weak)
}

// 实现 Drop Trait，以便观察何时节点被销毁
impl Drop for BTreeNode {
    fn drop(&mut self) {
        println!("Dropping BTreeNode with id: {}", self.id);
    }
}

// --- 示例 1: 使用 Arc 创建父子循环 (会导致泄漏) ---
// 为了演示，这里我们先定义一个使用 Arc 做 parent 的 NodeProblematic
struct BTreeNodeProblematic {
    id: usize,
    children: RefCell<Vec<Arc<RefCell<BTreeNodeProblematic>>>>,
    // 这里使用 Arc 做 parent 链接
    parent: RefCell<Option<Arc<RefCell<BTreeNodeProblematic>>>>,
}
impl Drop for BTreeNodeProblematic {
    fn drop(&mut self) {
        println!("Dropping BTreeNodeProblematic with id: {}", self.id);
    }
}

fn demonstrate_leak_with_arc_cycle() {
    println!("--- 示例 1: 使用 Arc 创建父子循环 (会导致泄漏) ---");

    // 使用块作用域限制 Arc 变量的生命周期
    {
        // 创建根节点
        let root = Arc::new(RefCell::new(BTreeNodeProblematic {
            id: 0,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None), // 根节点没有父节点
        }));
        println!(
            "创建根节点 (ID: 0). Arc 强引用计数: {}",
            Arc::strong_count(&root)
        ); // 1

        // 创建子节点 1
        let child1 = Arc::new(RefCell::new(BTreeNodeProblematic {
            id: 1,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        }));
        println!(
            "创建子节点 1 (ID: 1). Arc 强引用计数: {}",
            Arc::strong_count(&child1)
        ); // 1

        // 创建子节点 2
        let child2 = Arc::new(RefCell::new(BTreeNodeProblematic {
            id: 2,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        }));
        println!(
            "创建子节点 2 (ID: 2). Arc 强引用计数: {}",
            Arc::strong_count(&child2)
        ); // 1

        println!("\n链接 Root -> Child1, Root -> Child2 (使用 Arc)...");
        // 将子节点添加到根节点的 children 列表中 (增加子节点的强引用计数)
        root.borrow_mut()
            .children
            .borrow_mut()
            .push(Arc::clone(&child1));
        root.borrow_mut()
            .children
            .borrow_mut()
            .push(Arc::clone(&child2));
        println!(
            "链接后 Arc 强引用计数 (root): {}, (child1): {}, (child2): {}",
            Arc::strong_count(&root),
            Arc::strong_count(&child1),
            Arc::strong_count(&child2)
        );
        // root: 1 (main)
        // child1: 2 (main + root.children 持有)
        // child2: 2 (main + root.children 持有)

        println!("\n链接 Child1 -> Root, Child2 -> Root (使用 Arc) - 这将创建循环...");
        // 将子节点的 parent 指向根节点 (使用 Arc 的克隆，增加根节点的强引用计数)
        child1.borrow_mut().parent.replace(Some(Arc::clone(&root)));
        child2.borrow_mut().parent.replace(Some(Arc::clone(&root)));
        println!(
            "链接后 Arc 强引用计数 (root): {}, (child1): {}, (child2): {}",
            Arc::strong_count(&root),
            Arc::strong_count(&child1),
            Arc::strong_count(&child2)
        );
        // root: 3 (main + child1.parent + child2.parent)
        // child1: 2 (main + root.children)
        // child2: 2 (main + root.children)

        // 此时，我们创建了多级循环：
        // root -> child1 -> root
        // root -> child2 -> root
        // 强引用计数永远不会归零。

        println!("\n块作用域结束，main 函数持有的 root, child1, child2 强引用即将丢弃...");
        // drop root, child1, child2 变量，它们持有的强引用计数减 1
    } // <- root, child1, child2 离开作用域

    // root 的强引用计数从 3 变为 0 -> 2 (child1.parent 和 child2.parent 仍然指着它)
    // child1 的强引用计数从 2 变为 0 -> 1 (root.children 仍然指着它)
    // child2 的强引用计数从 2 变为 0 -> 1 (root.children 仍然指着它)
    // 所有节点的强引用计数都未能降到 0。

    println!(
        "块作用域已结束。因为循环，你应该不会看到 'Dropping BTreeNodeProblematic...' 的输出。"
    );
    println!("节点因循环引用而泄漏。");
    println!("--- 示例 1 结束 ---");
}

// --- 示例 2: 使用 Arc 和 Weak 解决父子循环 ---
// 我们将使用最初定义的 BTreeNode 结构体，其中 parent 使用 Weak

fn demonstrate_fix_with_weak() {
    println!("\n--- 示例 2: 使用 Arc 和 Weak 解决父子循环 ---");

    // 使用块作用域限制 Arc 变量的生命周期
    {
        // 创建根节点
        let root = Arc::new(RefCell::new(BTreeNode {
            id: 100,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None), // 根节点没有父节点
        }));
        println!(
            "创建根节点 (ID: 100). Arc 强引用计数: {}, 弱引用计数: {}",
            Arc::strong_count(&root),
            Arc::weak_count(&root)
        ); // S=1, W=0

        // 创建子节点 1
        let child1 = Arc::new(RefCell::new(BTreeNode {
            id: 101,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        }));
        println!(
            "创建子节点 1 (ID: 101). Arc 强引用计数: {}, 弱引用计数: {}",
            Arc::strong_count(&child1),
            Arc::weak_count(&child1)
        ); // S=1, W=0

        // 创建子节点 2
        let child2 = Arc::new(RefCell::new(BTreeNode {
            id: 102,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        }));
        println!(
            "创建子节点 2 (ID: 102). Arc 强引用计数: {}, 弱引用计数: {}",
            Arc::strong_count(&child2),
            Arc::weak_count(&child2)
        ); // S=1, W=0

        println!("\n链接 Root -> Child1, Root -> Child2 (使用 Arc)...");
        // 将子节点添加到根节点的 children 列表中 (增加子节点的强引用计数)
        root.borrow_mut()
            .children
            .borrow_mut()
            .push(Arc::clone(&child1));
        root.borrow_mut()
            .children
            .borrow_mut()
            .push(Arc::clone(&child2));
        println!(
            "链接后 Arc 强引用计数 (root): {}, 弱引用计数 (root): {}",
            Arc::strong_count(&root),
            Arc::weak_count(&root)
        ); // S=1, W=0
        println!(
            "链接后 Arc 强引用计数 (child1): {}, 弱引用计数 (child1): {}",
            Arc::strong_count(&child1),
            Arc::weak_count(&child1)
        ); // S=2, W=0
        println!(
            "链接后 Arc 强引用计数 (child2): {}, 弱引用计数 (child2): {}",
            Arc::strong_count(&child2),
            Arc::weak_count(&child2)
        ); // S=2, W=0

        println!("\n链接 Child1 -> Root, Child2 -> Root (使用 Weak) - 这将打破循环...");
        // 将子节点的 parent 指向根节点 (使用 Arc::downgrade() 获取弱引用)
        child1
            .borrow_mut()
            .parent
            .replace(Some(Arc::downgrade(&root)));
        child2
            .borrow_mut()
            .parent
            .replace(Some(Arc::downgrade(&root)));
        println!(
            "链接后 Arc 强引用计数 (root): {}, 弱引用计数 (root): {}",
            Arc::strong_count(&root),
            Arc::weak_count(&root)
        ); // S=1, W=2 (child1.parent 和 child2.parent 持有弱引用)
        println!(
            "链接后 Arc 强引用计数 (child1): {}, 弱引用计数 (child1): {}",
            Arc::strong_count(&child1),
            Arc::weak_count(&child1)
        ); // S=2, W=0
        println!(
            "链接后 Arc 强引用计数 (child2): {}, 弱引用计数 (child2): {}",
            Arc::strong_count(&child2),
            Arc::weak_count(&child2)
        ); // S=2, W=0

        // 此时，我们创建了结构： root (强) -> child1 (强) -> root (弱), root (强) -> child2 (强) -> root (弱)
        // 父到子的链接是强的，子到父的链接是弱的。
        // a 的强引用计数是 1: main 函数持有 1个
        // b 的强引用计数是 2: main 函数持有 1个, root.children 持有 1个
        // a 的弱引用计数是 2: child1.parent 和 child2.parent 持有
        // b 的弱引用计数是 0

        // 在作用域内，我们仍然可以通过弱引用尝试访问父节点
        if let Some(parent_weak) = child1.borrow().parent.borrow().as_ref() {
            if let Some(parent_arc) = parent_weak.upgrade() {
                println!(
                    "\n成功从 child1 通过 Weak 引用升级并访问到父节点 (ID: {})",
                    parent_arc.borrow().id
                );
            } else {
                println!("\n无法从 child1 升级 Weak 引用，父节点可能已被丢弃。");
            }
        }

        println!("\n块作用域结束，main 函数持有的 root, child1, child2 强引用即将丢弃...");
        // drop root, child1, child2 变量，它们持有的强引用计数减 1
    } // <- root, child1, child2 离开作用域

    // 追踪 Drop 过程：
    // 1. root 的强引用计数从 1 变为 0。
    // 2. 因为 root 的强引用计数归零，root 指向的 NodeFixed 对象被 drop。
    // 3. 调用 Drop::drop for BTreeNode with value: 100。打印。
    // 4. 在 Drop(root) 中，其字段被丢弃：
    //    - children 字段 (Vec<Arc<NodeFixed>>) 被丢弃。Vec 中的 Arc 会被丢弃。
    //    - Dropping Arc<child1>: child1 的强引用计数从 2 变为 1。
    //    - Dropping Arc<child2>: child2 的强引用计数从 2 变为 1。
    //    - parent 字段是 None。
    // 5. child1 和 child2 的强引用计数现在是 1 (由 main 函数中原始的 child1, child2 变量持有)。它们尚未被 drop。
    // 6. child1 变量被丢弃。child1 的强引用计数从 1 变为 0。
    // 7. 因为 child1 强引用计数归零，child1 指向的 NodeFixed 对象被 drop。
    // 8. 调用 Drop::drop for BTreeNode with value: 101。打印。
    // 9. 在 Drop(child1) 中：
    //    - children 字段是空的 Vec，被丢弃。
    //    - parent 字段持有的 Weak<root> 被丢弃。root 的弱引用计数从 2 变为 1。
    // 10. child2 变量被丢弃。child2 的强引用计数从 1 变为 0。
    // 11. 因为 child2 强引用计数归零，child2 指向的 NodeFixed 对象被 drop。
    // 12. 调用 Drop::drop for BTreeNode with value: 102。打印。
    // 13. 在 Drop(child2) 中：
    //    - children 字段是空的 Vec，被丢弃。
    //    - parent 字段持有的 Weak<root> 被丢弃。root 的弱引用计数从 1 变为 0。
    // 14. 所有节点的强引用和弱引用计数都归零。所有节点都被正确 drop。

    println!("块作用域已结束。因为使用了 Weak，你应该看到了所有 'Dropping BTreeNode...' 的输出。");
    println!("节点被正确销毁。");
    println!("--- 示例 2 结束 ---");
}

fn cycle_btree_sample() {
    demonstrate_leak_with_arc_cycle();
    println!("\n======================================\n");
    demonstrate_fix_with_weak();
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

    #[test]
    fn test_cycle_btree_sample() {
        cycle_btree_sample();
    }
}
