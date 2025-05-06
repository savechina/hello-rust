use std::cell::RefCell;
use std::fmt::Debug;
use std::sync::{Arc, Weak};

//类型别名：NodeCell 是 RefCell<TreeNode> 的简写，
//NodeArcPtr 是 Arc<NodeCell> 的简写，
//NodeWeakPtr 是 Weak<NodeCell> 的简写。
// 这样可以简化代码，避免重复书写长类型名。
type NodeCell = RefCell<TreeNode>;
type NodeArcPtr = Arc<NodeCell>;
type NodeWeakPtr = Weak<NodeCell>;

// 使用 Arc 和 RefCell 创建一个双向链表节点
#[derive(Debug)]
struct TreeNode {
    value: i32,
    next: RefCell<Option<NodeArcPtr>>,
    prev: RefCell<Option<NodeWeakPtr>>,
}

impl TreeNode {
    // 构造函数，简化创建过程
    fn new(value: i32) -> NodeArcPtr {
        Arc::new(RefCell::new(TreeNode {
            value,
            next: RefCell::new(None),
            prev: RefCell::new(None),
        }))
    }

    // 设置 next 节点，隐藏 RefCell 和 Option 细节
    fn set_next(&self, next_node: Option<NodeArcPtr>) {
        // self 是 &RefCell<NodeMethods>，直接在其上调用 borrow_mut()
        self.next.borrow_mut().replace(next_node.unwrap());
    }

    // 获取 next 节点，返回一个克隆的 Arc (强引用)
    fn get_next(&self) -> Option<NodeArcPtr> {
        // self 是 &RefCell<NodeMethods>
        self.next.borrow().clone() // clone() Arc 会增加引用计数
    }

    // 设置 prev 节点，从 Arc 创建 Weak
    fn set_prev(&self, prev_node: Option<&NodeArcPtr>) {
        // 接收 Option<&NodePtr> 更灵活
        let weak_ptr = prev_node.map(|arc_ptr| Arc::downgrade(arc_ptr));
        self.prev.borrow_mut().replace(weak_ptr.unwrap());
    }

    // 获取 prev 节点，尝试升级 Weak 引用为 Arc
    fn get_prev(&self) -> Option<NodeArcPtr> {
        // self 是 &RefCell<NodeMethods>
        // borrow().as_ref() 获取 Option<&Weak<...>>
        // 然后 map() 转换 Option 内部的值
        self.prev
            .borrow()
            .as_ref()
            .and_then(|weak_ptr| weak_ptr.upgrade())
    }

    // 获取节点的值
    fn value(&self) -> i32 {
        self.value
    }
}

impl Drop for TreeNode {
    fn drop(&mut self) {
        println!("Dropping NodeMethods with value: {}", self.value);
    }
}

fn typealias_sample() {
    println!("--- 使用辅助方法优化 Node 的使用 ---");

    {
        // 使用块作用域观察 Drop
        let a = TreeNode::new(1); // 使用构造函数
        let b = TreeNode::new(2);

        println!(
            "创建节点 A(S={},W={}), B(S={},W={})",
            Arc::strong_count(&a),
            Arc::weak_count(&a),
            Arc::strong_count(&b),
            Arc::weak_count(&b)
        );

        // 使用辅助方法链接节点
        a.borrow().set_next(Some(Arc::clone(&b))); // a 是 Arc<RefCell<...>>，所以需要 borrow() 来调用其方法
        b.borrow().set_prev(Some(&a)); // set_prev 接收 &NodePtr (&Arc<RefCell<...>>)

        println!(
            "链接后 A(S={},W={}), B(S={},W={})",
            Arc::strong_count(&a),
            Arc::weak_count(&a),
            Arc::strong_count(&b),
            Arc::weak_count(&b)
        );

        // 使用辅助方法访问节点
        if let Some(next_node) = a.borrow().get_next() {
            println!(
                "从 A 访问到 next 节点，值为: {}",
                next_node.borrow().value()
            );
        }
        if let Some(prev_node) = b.borrow().get_prev() {
            println!(
                "从 B 访问到 prev 节点，值为: {}",
                prev_node.borrow().value()
            );
        }

        println!("\n块作用域结束，Arc 强引用即将丢弃...");
    } // <- a 和 b 离开作用域

    println!("块作用域已结束。节点应该已被销毁。");
    println!("--- 优化 Node 的使用结束 ---");
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::cell::RefCell;
    use std::sync::Weak;
    #[test]
    fn test_node_type_alias_sample() {
        println!("--- 测试 typealias_sample ---");
        typealias_sample();
        println!("--- 测试结束 ---");
    }
}
