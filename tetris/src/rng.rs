use alloc::collections::vec_deque::Iter;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// 标准伪随机数算法
pub fn prng(state: &mut u32) -> u32 {
    let new_state = state.wrapping_mul(1103515245).wrapping_add(12345);
    *state = new_state;
    new_state & 0x7fffffff
}

/// 标准洗牌算法
pub fn shuffle<T>(list: &mut [T], state: &mut u32) {
    let len = list.len();
    for i in 0..len {
        let r = prng(state) as usize;
        list.swap(i, i + r % (len - i));
    }
}

/// 7-Bag 生成器
pub struct SevenBagGenerator {
    state: u32,
    type_count: usize,
    queue: Vec<usize>,
}

impl SevenBagGenerator {
    pub fn new(seed: u32, type_count: usize) -> Self {
        Self {
            state: seed,
            type_count,
            queue: Vec::with_capacity(type_count),
        }
    }
}

impl Iterator for SevenBagGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() == 0 {
            self.queue.extend(0..self.type_count);
            shuffle(self.queue.as_mut_slice(), &mut self.state);
        }
        self.queue.pop()
    }
}

/// 带预览功能的生成器
pub struct PreviewGenerator<T, G> {
    rng: G,
    preview_count: usize,
    preview_list: VecDeque<T>,
}

impl<T, G> PreviewGenerator<T, G> {
    pub fn new(rng: G, preview_count: usize) -> Self {
        Self {
            rng,
            preview_count,
            preview_list: VecDeque::with_capacity(preview_count),
        }
    }
}

impl<T, G> PreviewGenerator<T, G>
    where
        G: Iterator<Item=T>
{
    pub fn preview(&mut self) -> Iter<'_, T> {
        while self.preview_list.len() < self.preview_count {
            if let Some(v) = self.rng.next() {
                self.preview_list.push_back(v);
            } else {
                break;
            }
        }
        self.preview_list.iter()
    }
}

impl<T, G> Iterator for PreviewGenerator<T, G>
    where
        G: Iterator<Item=T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.preview_list.pop_front() {
            Some(v)
        } else {
            self.rng.next()
        }
    }
}
