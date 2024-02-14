//!
//! LinkedList 链表
//! 实现单向链表和双向链表
//!

///
/// 第一版本 List 链表List 实现，仅支持i32 类型
///
mod first {

    /**
     * List 结构
     *  结构布局
     *  ' [ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)
     */
    #[derive(Debug, Clone)]
    pub struct List {
        head: Link,
        len: usize,
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
            List { head: None, len: 0 }
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
            self.len += 1;
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

            self.len -= 1;

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

        pub fn len(&self) -> usize {
            return self.len;
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
            assert_eq!(list.len(), 0);
        }

        #[test]
        fn test_list_push() {
            let mut list = List::new();

            println!("list befor push :{:?}", list);

            list.push(1);

            println!("list after push 1 :{:?}", list);

            list.push(2);

            println!("list after push 2 :{:?}", list);

            assert_eq!(list.len(), 2);
        }

        #[test]
        fn test_list_pop() {
            let mut list = List::new();

            list.push(1);

            list.push(2);

            println!("list before pop :{:?}", list);

            let elem = list.pop();

            println!("list after pop :{:?}, elem:{:?}", list, elem);

            assert_eq!(list.len(), 1);
        }

        #[test]
        fn test_long_list() {
            let mut list = List::new();
            for i in 0..100000 {
                list.push(i);
            }

            assert_eq!(list.len(), 1);

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

            println!("list iter :{:?}, elem:{:?},elm3:{:?}", elem1, elem2, elem3);
        }
    }
}

////////////////////////////////////////////////////////////
//第二版本链表，支持泛型
//
mod second {
    use std::ops::Deref;

    /**
     * List 结构
     *  结构布局
     *  ' [ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)
     */
    #[derive(Debug, Clone)]
    pub struct List<T> {
        head: Link<T>,
        len: usize,
    }

    ///
    /// 类型别名，type alias
    type Link<T> = Option<Box<Node<T>>>;

    /**
     *  List Node
     */
    #[derive(Debug, Clone)]
    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    /**
     * List 实现方法
     */
    impl<T> List<T> {
        /**
         *  创建List
         * List::new()
         */
        pub fn new() -> Self {
            List { head: None, len: 0 }
        }

        /**
         * List push Elem
         */
        pub fn push(&mut self, elem: T) {
            let new_node = Box::new(Node {
                elem: elem,
                next: self.head.take(),
            });

            self.head = Some(new_node);
            self.len += 1;
        }

        /**
         *  Pop elem
         */
        pub fn pop(&mut self) -> Option<T> {
            let result;

            match self.head.take() {
                None => result = None,

                Some(node) => {
                    self.head = node.next;
                    result = Some(node.elem);
                }
            };

            self.len -= 1;

            return result;

            //或者以下使用map 闭包
            /*
                self.head.take().map(|node| {
                    self.head = node.next;
                    node.elem
                })
            */
        }

        pub fn peek(&self) -> Option<&T> {
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

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }

        pub fn len(&self) -> usize {
            return self.len;
        }
    }

    /**
     * 实现Drop 特征，自动清除List
     */
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur_link = self.head.take();

            while let Some(mut boxed_node) = cur_link {
                cur_link = boxed_node.next.take();
            }
        }
    }

    pub struct IntoIter<T>(List<T>);

    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            // access fields of a tuple struct numerically
            self.0.pop()
        }
    }

    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }

    impl<T> List<T> {
        pub fn iter<'a>(&'a self) -> Iter<'a, T> {
            Iter {
                next: self.head.as_ref().map(|node| node.deref()),
            }
        }
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                // self.next = node.next.as_ref().map(|node| &**node);
                //等于下面

                self.next = node.next.as_deref();
                &node.elem
            })
        }
    }

    pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
    }

    impl<T> List<T> {
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut {
                next: self.head.as_deref_mut(),
            }
        }
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_deref_mut();
                &mut node.elem
            })
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
            //初始化泛型 List
            let list: List<&str> = List::new();

            println!("list :{:?}", list);

            //初始化泛型 List
            let mut list = List::<&str>::new();

            println!("fist list :{:?}", list);

            assert_eq!(list.pop(), None);
        }

        #[test]
        fn test_list_push() {
            let mut list: List<&str> = List::new();

            println!("list befor push :{:?}", list);

            list.push("1");

            println!("list after push 1 :{:?}", list);

            list.push("2");

            list.push("3");

            println!("list after push 2 :{:?}", list);
        }

        #[test]
        fn test_list_pop() {
            let mut list = List::new();

            list.push("1");

            list.push("2");

            println!("list before pop :{:?}", list);

            let elem = list.pop();

            println!("list after pop :{:?}, elem:{:?}", list, elem);
        }

        #[test]
        fn test_long_list() {
            let mut list = List::new();
            for i in 0..100000 {
                list.push(i.to_string());
            }
            drop(list);
        }

        #[test]
        fn test_list_peek() {
            let mut list = List::new();

            list.push("1");

            list.push("2");

            println!("list before peek :{:?}", list);

            let elem: Option<&&str> = list.peek();

            println!("list after peek :{:?}, elem:{:?}", list, elem);
        }

        #[test]
        fn test_list_into_iter() {
            let mut list: List<&str> = List::new();

            list.push("1");

            list.push("2");

            let mut iter = list.into_iter();
            let elem1: Option<&str> = iter.next();
            let elem2: Option<&str> = iter.next();
            let elem3: Option<&str> = iter.next();

            println!("list iter :{:?}, elem:{:?},elm3:{:?}", elem1, elem2, elem3);
        }

        #[test]
        fn test_list_iter() {
            let mut list: List<&str> = List::new();

            list.push("1");
            list.push("2");
            list.push("3");
            list.push("4");

            for x in list.iter() {
                println!("list iter :{:?}", x);
            }
        }

        #[test]
        fn test_list_iter_mut() {
            let mut list: List<&str> = List::new();

            list.push("1");
            list.push("2");
            list.push("3");
            list.push("4");

            let mut iter = list.iter_mut();
            let elem1: Option<&mut &str> = iter.next();
            let elem2: Option<&mut &str> = iter.next();
            let elem3: Option<&mut &str> = iter.next();
            let elem4: Option<&mut &str> = iter.next();

            assert_eq!(elem1, Some(&mut "4"));
            assert_eq!(elem2, Some(&mut "3"));
            assert_eq!(elem3, Some(&mut "2"));
            assert_eq!(elem4, Some(&mut "1"));
        }
    }
}
