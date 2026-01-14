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
pub struct MyLinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

//rust规范（如果一个没有参数的构造函数只负责构建自身，建议实现rust Default特征）
impl<T> Default for MyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MyLinkedList<T> {
    //新建
    pub fn new() -> Self {
        MyLinkedList { head: Link::None }
    }
    //添加元素
    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element,
            next: self.head.take(),
        });

        self.head = Link::Some(new_node);
    }
    //删除元素
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }
    //返回头部元素不可变引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
    //返回头部元素可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

pub struct IntoIter<T>(MyLinkedList<T>);
//消耗所有权迭代器
impl<T> MyLinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

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
//不可变借用迭代器
impl<T> MyLinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // self.next 是 Option<&'a Node<T>>
        self.next.map(|node| {
            // 关键：将指针移向下一个节点
            // node.next 是 Option<Box<Node<T>>>，同样需要 as_deref()
            self.next = node.next.as_deref();
            &node.element // 返回当前节点元素的引用
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
//可变借用迭代器
impl<T> MyLinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    /*self.next 拥有对节点的可变借用。如果你直接通过 self.next.map 操作，
    由于闭包会试图长时间占用这个借用，你就无法在闭包内部执行
    self.next = ... 的重新赋值操作（这违反了唯一性）。
    通过 .take()，你暂时把 Option 里的借用拿出来，让 self.next 暂时变成 None。
    这样你就可以自由地操作拿出来的那个借用，并在最后把新的借用重新放回 self.next 中
    不可变引用 &T可以Copy直接 .map() 即可，原值不动。
    可变引用&mut T只能Move必须 .take().map()，先取走，处理完再放回新的。*/
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

/*
自动销毁由于BOX实现的drop特性在销毁后还需要回收堆内存，
所以在递归销毁时，链表有多少元素就会开辟多少栈空间，会导致栈溢出，
所以需要我们手动拆解链表元素实现元素的销毁
*/
impl<T> Drop for MyLinkedList<T> {
    fn drop(&mut self) {
        //第一步接管当前head,再将已完成接管的head节点设置为空
        let mut current_head = self.head.take();
        //第二步使用循环匹配利用已接管current_head拿到下一个head指向的非空节点
        while let Some(mut next_head) = current_head {
            //第三步利用这个匹配拿到的next_head再次获取它的head指向的非空节点，
            //将这个获取到的next_next_head赋值给current_node,
            //现在这个next_head在离开这个循环匹配的时候会被销毁，达到目的，依次循环，直到最后一个空节点
            current_head = next_head.next.take();
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
    #[test]
    fn 泛型检查() {
        let mut list = super::MyLinkedList::new();
        list.push('s');
        list.push('r');
        assert_eq!(list.pop(), Some('r'));
        assert_eq!(list.pop(), Some('s'));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn peek() {
        let mut list = super::MyLinkedList::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        if let Some(value) = list.peek_mut() {
            *value = 42;
        }
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn into_iter() {
        let mut list = super::MyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = super::MyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn iter_mut() {
        let mut list = super::MyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
