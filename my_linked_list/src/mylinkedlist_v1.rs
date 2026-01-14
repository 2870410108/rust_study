/*
手动完成单项链表的实现
1.实现链表结构定义
2.实现链表节点的添加
3.实现链表节点的删除
4.实现链表的遍历

*/
#![allow(unused)]

//首先链表结构
//List a = Empty | Elem a (List a)
pub struct MyLinkedList {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    element: i32,
    next: Link,
}

//rust规范（如果一个没有参数的构造函数只负责构建自身，建议实现rust Default特征）
impl Default for MyLinkedList {
    fn default() -> Self {
        Self::new()
    }
}

impl MyLinkedList {
    //新建
    pub fn new() -> Self {
        MyLinkedList { head: Link::Empty }
    }
    //添加元素
    pub fn push(&mut self, element: i32) {
        let new_node = Box::new(Node {
            element,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
    //删除元素
    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }
    }
}
/*
自动销毁由于BOX实现的drop特性在销毁后还需要回收堆内存，
所以在递归销毁时，链表有多少元素就会开辟多少栈空间，会导致栈溢出，
所以需要我们手动拆解链表元素实现元素的销毁
*/
impl Drop for MyLinkedList {
    fn drop(&mut self) {
        //第一步使用std::mem::replace(&mut self.head, Link::Empty)接管当前head,再将已完成接管的head节点设置为空
        let mut current_head = std::mem::replace(&mut self.head, Link::Empty);
        //第二步使用循环匹配利用已接管current_head拿到下一个head指向的非空节点
        while let Link::More(mut next_head) = current_head {
            //第三步利用这个匹配拿到的next_head再次获取它的head指向的非空节点，
            //将这个获取到的next_next_head赋值给current_node,
            //现在这个next_head在离开这个循环匹配的时候会被销毁，达到目的，依次循环，直到最后一个空节点
            current_head = std::mem::replace(&mut next_head.next, Link::Empty);
        }
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        let mut list = super::MyLinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn long_list() {
        let mut list = super::MyLinkedList::new();

        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }
}
