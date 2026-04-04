// CycleRC - 引用计数循环检测示例
// 完整示例：https://github.com/savechina/hello-rust/blob/main/src/advance/async/cyclerc_sample.rs

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// 树节点结构 - 使用 Weak 打破循环引用
#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,    // 子节点：强引用
    parent: RefCell<Option<Weak<TreeNode>>>, // 父节点：弱引用（打破循环）
}

impl TreeNode {
    /// 创建新节点
    fn new(value: i32) -> Rc<TreeNode> {
        Rc::new(TreeNode {
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        })
    }

    /// 添加子节点
    fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
        // 设置子节点的父节点（弱引用）
        *child.parent.borrow_mut() = Some(Rc::downgrade(parent));
        // 添加子节点到父节点（强引用）
        parent.children.borrow_mut().push(child);
    }

    /// 获取父节点（可能不存在）
    fn get_parent(&self) -> Option<Rc<TreeNode>> {
        self.parent
            .borrow()
            .as_ref()
            .and_then(|weak| weak.upgrade())
    }
}

/// 演示循环引用问题
fn demonstrate_cycle_prevention() {
    println!("=== 循环引用预防演示 ===\n");

    // 创建根节点
    let root = TreeNode::new(1);
    println!("创建根节点：{}", root.value);

    // 创建子节点
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);
    println!("创建子节点：{}, {}", child1.value, child2.value);

    // 添加子节点（使用 Weak 打破循环）
    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());
    println!("添加子节点到根节点\n");

    // 验证引用计数
    println!("根节点强引用计数：{}", Rc::strong_count(&root));
    println!("根节点弱引用计数：{}", Rc::weak_count(&root));
    println!("子节点 1 强引用计数：{}", Rc::strong_count(&child1));
    println!("子节点 1 弱引用计数：{}", Rc::weak_count(&child1));

    // 验证可以访问父节点
    if let Some(parent) = child1.get_parent() {
        println!("\n子节点 1 的父节点：{}", parent.value);
    }

    // 验证离开作用域后内存会被释放
    drop(child1);
    drop(child2);
    drop(root);
    println!("\n所有节点已释放，无内存泄漏 ✓");
}

/// 演示如果不使用 Weak 会发生什么
fn demonstrate_cycle_problem() {
    println!("\n=== 循环引用问题演示（反面教材）===\n");

    #[derive(Debug)]
    struct BadNode {
        value: i32,
        next: RefCell<Option<Rc<BadNode>>>,
    }

    let node1 = Rc::new(BadNode {
        value: 1,
        next: RefCell::new(None),
    });

    let node2 = Rc::new(BadNode {
        value: 2,
        next: RefCell::new(None),
    });

    // ❌ 创建循环引用
    *node1.next.borrow_mut() = Some(Rc::clone(&node2));
    *node2.next.borrow_mut() = Some(Rc::clone(&node1));

    println!("创建循环引用：node1 -> node2 -> node1");
    println!(
        "node1 强引用计数：{} (应该是 1，实际是 2)",
        Rc::strong_count(&node1)
    );
    println!(
        "node2 强引用计数：{} (应该是 1，实际是 2)",
        Rc::strong_count(&node2)
    );
    println!("\n即使离开作用域，内存也不会释放（内存泄漏）⚠️");

    // 注意：这里故意不 drop，展示问题
    // 在实际代码中应该使用 Weak 来避免这个问题
}

/// 观察者模式示例 - 使用 Weak 管理观察者
#[derive(Debug)]
struct Observer {
    id: usize,
}

trait EventHandler: Send + Sync {
    fn handle(&self, event: &str);
}

impl EventHandler for Observer {
    fn handle(&self, event: &str) {
        println!("观察者 {} 收到事件：{}", self.id, event);
    }
}

struct Subject {
    name: String,
    observers: RefCell<Vec<Weak<dyn EventHandler>>>,
}

impl Subject {
    fn new(name: &str) -> Self {
        Subject {
            name: name.to_string(),
            observers: RefCell::new(vec![]),
        }
    }

    fn add_observer(&self, observer: Rc<dyn EventHandler>) {
        self.observers.borrow_mut().push(Rc::downgrade(&observer));
        println!("添加观察者到 {}", self.name);
    }

    fn notify(&self, event: &str) {
        println!("\n通知事件：{}", event);

        // 清理失效的弱引用并通知有效的观察者
        self.observers.borrow_mut().retain(|weak| {
            if let Some(observer) = weak.upgrade() {
                observer.handle(event);
                true // 保留有效的观察者
            } else {
                false // 移除失效的观察者
            }
        });
    }

    fn observer_count(&self) -> usize {
        self.observers.borrow().len()
    }
}

fn demonstrate_observer_pattern() {
    println!("\n=== 观察者模式演示（使用 Weak）===\n");

    let subject = Rc::new(Subject::new("新闻推送"));

    // 创建观察者
    let observer1: Rc<dyn EventHandler> = Rc::new(Observer { id: 1 });
    let observer2: Rc<dyn EventHandler> = Rc::new(Observer { id: 2 });
    let observer3: Rc<dyn EventHandler> = Rc::new(Observer { id: 3 });

    // 添加观察者
    subject.add_observer(observer1.clone());
    subject.add_observer(observer2.clone());
    subject.add_observer(observer3.clone());

    println!("当前观察者数量：{}", subject.observer_count());

    // 通知所有观察者
    subject.notify("新版本发布！");

    // 移除一个观察者
    drop(observer2);
    println!("\n观察者 2 已销毁");
    println!("当前观察者数量（未清理）：{}", subject.observer_count());

    // 再次通知（自动清理失效的观察者）
    subject.notify("紧急更新！");
    println!("当前观察者数量（已清理）：{}", subject.observer_count());
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   CycleRC - 引用计数循环检测示例       ║");
    println!("╚════════════════════════════════════════╝\n");

    // 演示 1：树结构（正确使用 Weak）
    demonstrate_cycle_prevention();

    // 演示 2：循环引用问题（反面教材）
    demonstrate_cycle_problem();

    // 演示 3：观察者模式
    demonstrate_observer_pattern();

    println!("\n╔════════════════════════════════════════╗");
    println!("║   所有演示完成 ✓                       ║");
    println!("╚════════════════════════════════════════╝");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_parent_child() {
        let root = TreeNode::new(1);
        let child = TreeNode::new(2);

        TreeNode::add_child(&root, child.clone());

        assert!(child.get_parent().is_some());
        assert_eq!(child.get_parent().unwrap().value, 1);
    }

    #[test]
    fn test_no_cycle_with_weak() {
        let root = TreeNode::new(1);
        let child = TreeNode::new(2);

        TreeNode::add_child(&root, child.clone());

        // 根节点应该只有 1 个强引用（root 变量）
        assert_eq!(Rc::strong_count(&root), 1);
        // 应该有 1 个弱引用（来自 child 的 parent 字段）
        assert_eq!(Rc::weak_count(&root), 1);
    }

    #[test]
    fn test_observer_cleanup() {
        let subject = Subject::new("test");
        let observer: Rc<dyn EventHandler> = Rc::new(Observer { id: 1 });

        subject.add_observer(observer.clone());
        assert_eq!(subject.observer_count(), 1);

        drop(observer);
        subject.notify("test event");

        // 通知后应该自动清理失效的观察者
        assert_eq!(subject.observer_count(), 0);
    }
}
