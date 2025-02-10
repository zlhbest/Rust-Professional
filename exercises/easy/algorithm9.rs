/*
    heap
    This question requires you to implement a binary heap function
    堆的性质：
    * 堆的本质就是一个二叉树
    * 堆中的某个节点的值总是不大于或不小于其父节点的值
    * 堆总还是一个完全二叉树 ：完全二叉树只允许最后一行不为满，且最后一行必须从左往右排序
    堆的存储其实是一个一维数组，是按照完全二叉树的层序遍历进行的，所以数组的index具有如下性质：left = 2*root +1  right = 2*root+2
    根据该视频实现: https://www.bilibili.com/video/BV1AF411G7cA/?spm_id_from=333.337.search-card.all.click&vd_source=1d8eef615091c7606b5741f74273e762

*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// 将新元素加到最后面，然后从后面往上走  如果自己的父节点比自己的大或者小，就移动 指到移不动为止
    pub fn add(&mut self, value: T) {
        // 使用上滤，将数据加入到最底下，然后往上走
        self.items.push(value);
        self.count += 1;
        let mut index = self.count;
        let mut parent_point = self.parent_idx(index);
        while parent_point >= 1 {
            if (self.comparator)(&self.items[index], &self.items[parent_point]) {
                self.items.swap(index, parent_point);
            }
            index = parent_point;
            parent_point = self.parent_idx(index);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;
    /// 取数据的时候就下滤， 最顶上的拿走以后，将最后面的移到最上面，然后开始向下走，子节点最小的与root交换
    fn next(&mut self) -> Option<T> {
        let result;
        // 弹出第一个
        if self.count == 0 {
            result = None;
        } else {
            result = self.items.get(1).map(|item| item.clone());
            // 然后将最底下的移动到上面来
            self.items.swap(1, self.count);
            self.items.remove(self.count);
            self.count -= 1;
            // 做完这些操作以后，再下推
            let mut root = 1;
            while root < self.count {
                let left_child = self.left_child_idx(root);
                let right_child = self.right_child_idx(root);
                let swap_child = if left_child < self.count && right_child < self.count {
                    if (self.comparator)(&self.items[left_child], &self.items[right_child]) {
                        left_child
                    } else {
                        right_child
                    }
                } else {
                    if left_child < self.count {
                        left_child
                    } else {
                        right_child
                    }
                };
                if swap_child < self.count {
                    self.items.swap(root, swap_child);
                    //交换完了以后root下移
                    root = swap_child;
                } else {
                    break;
                }
            }
        }
        result
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
