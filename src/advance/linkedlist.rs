//!
//! LinkedList 链表
//! 实现单向链表和双向链表
//!

/**
 * List 结构
 *  结构布局
 *  ' [ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)
 */
#[derive(Debug, Clone)]
pub struct List {
    head: Link,
}

///
/// 类型别名，type alias
type Link = Option<Box<Node>>;

/**
 *  List Node
 */
#[derive(Debug, Clone)]
struct Node {
    elem: i32,
    next: Link,
}

/**
 * List 实现方法
 */
impl List {
    /**
     *  创建List
     * List::new()
     */
    pub fn new() -> Self {
        List { head: None }
    }

    /**
     * List push Elem
     */
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /**
     *  Pop elem
     */
    pub fn pop(&mut self) -> Option<i32> {
        let result;

        match self.head.take() {
            None => result = None,

            Some(node) => {
                self.head = node.next;
                result = Some(node.elem);
            }
        };
        return result;

        //或者以下使用map 闭包
        /*
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            })
        */
    }

    pub fn peek(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.elem)

        // let result;

        // match self.head.as_ref() {
        //     Some(node) => {
        //         result = Some(node.elem);
        //     }
        //     None => result = None,
        // };

        // return result;
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }
}

/**
 * 实现Drop 特征，自动清除List
 */
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter(List);

impl Iterator for IntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
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
    fn test_list_new() {
        let mut list = List::new();

        println!("fist list :{:?}", list);

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_list_push() {
        let mut list = List::new();

        println!("list befor push :{:?}", list);

        list.push(1);

        println!("list after push 1 :{:?}", list);

        list.push(2);

        println!("list after push 2 :{:?}", list);
    }

    #[test]
    fn test_list_pop() {
        let mut list = List::new();

        list.push(1);

        list.push(2);

        println!("list before pop :{:?}", list);

        let elem = list.pop();

        println!("list after pop :{:?}, elem:{:?}", list, elem);
    }

    #[test]
    fn test_long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }

    #[test]
    fn test_list_peek() {
        let mut list = List::new();

        list.push(1);

        list.push(2);

        println!("list before peek :{:?}", list);

        let elem: Option<&i32> = list.peek();

        println!("list after peek :{:?}, elem:{:?}", list, elem);
    }

    #[test]
    fn test_list_into_iter() {
        let mut list = List::new();

        list.push(1);

        list.push(2);

        let mut iter = list.into_iter();
        let elem1: Option<i32> = iter.next();
        let elem2: Option<i32> = iter.next();
        let elem3: Option<i32> = iter.next();


        println!("list iter :{:?}, elem:{:?},elm3:{:?}", elem1, elem2,elem3);
    }
}
