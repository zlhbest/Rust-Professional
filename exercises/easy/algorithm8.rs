/*
    queue
    This question requires you to use queues to implement the functionality of the stack
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}
/// 栈是先进后出
/// 队列是先进先出
/// 只需要将n-1的元素移到空的那边，剩下的那个就应该出栈
pub struct myStack<T> {
    size: usize,
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            size: 0usize,
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        if self.q1.is_empty() && self.q2.is_empty() {
            // 如果俩都空着，那就往q1走
            self.q1.enqueue(elem);
        } else {
            let q = if self.q1.is_empty() {
                &mut self.q2
            } else {
                &mut self.q1
            };
            q.enqueue(elem);
        }
        self.size += 1;
    }
    /// 实现的思路就是将有数据的队列除了最后一个，全都放到另一个队列里面
    /// 这样就实现了栈
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.size == 0 {
            Err("Stack is empty")
        } else {
            self.size -= 1;
            if !self.q1.is_empty() {
                while !self.q1.is_empty() {
                    if self.q1.size() == 1 {
                        break;
                    }
                    self.q2.enqueue(self.q1.dequeue().unwrap());
                }
                Ok(self.q1.dequeue().unwrap())
            } else {
                while !self.q2.is_empty() {
                    if self.q2.size() == 1 {
                        break;
                    }
                    self.q1.enqueue(self.q2.dequeue().unwrap());
                }
                Ok(self.q2.dequeue().unwrap())
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
