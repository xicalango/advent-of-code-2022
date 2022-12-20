use std::fmt::{Debug, Formatter};
use std::mem::replace;
use crate::utils::Error;

pub struct RingBuf<T> {
    buffer: Vec<T>,
    size: usize,
    cur: usize,
}

impl<T: Debug> Debug for RingBuf<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RingBuf{{size = {}, cur = {}, buffer = {:?}}}", self.size, self.cur, self.buffer)
    }
}

impl<T> RingBuf<T> {

    pub fn new(size: usize) -> RingBuf<T> {
        RingBuf {
            buffer: Vec::new(),
            size,
            cur: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Option<T> {

        if self.buffer.len() < self.size {
            self.buffer.push(value);
            return None;
        }

        let evicted = replace(&mut self.buffer[self.cur], value);

        self.cur += 1;
        self.cur %= self.size;

        Some(evicted)
    }

    pub fn iter(&self) -> RingBufIterator<'_, T> {
        RingBufIterator {
            ring_buf: self,
            cur: 0,
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.adjust_index(idx)
            .ok()
            .map(|i| &self.buffer[i])
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.adjust_index(idx)
            .ok()
            .map(|i| &mut self.buffer[i])
    }

    pub fn set(&mut self, idx: usize, value: T) -> Option<T> {
        match self.adjust_index(idx) {
            Ok(i) => {
                let previous = replace(&mut self.buffer[i], value);
                Some(previous)
            },
            Err(_) => None,
        }
    }

    pub fn swap(&mut self, idx1: usize, idx2: usize) {
        let ai1 = self.adjust_index(idx1).unwrap();
        let ai2 = self.adjust_index(idx2).unwrap();
        self.buffer.swap(ai1, ai2);
    }

    pub fn check_index(&self, idx: usize) -> Result<(), Error> {
        if idx > self.size {
            return Err(Error(format!("index out of bounds: {}/{}", idx, self.size)));
        }

        if idx >= self.buffer.len() {
            return Err(Error(format!("index out of bounds: {}/{}", idx, self.buffer.len())));
        }

        Ok(())
    }

    fn adjust_index(&self, idx: usize) -> Result<usize, Error> {
        self.check_index(idx).map(|_| (self.cur + idx) % self.size)
    }

    pub fn len(&self) -> usize {
        std::cmp::min(self.size, self.buffer.len())
    }

}

impl<E> FromIterator<E> for RingBuf<E> {
    fn from_iter<T: IntoIterator<Item=E>>(iter: T) -> Self {
        let buffer: Vec<E> = iter.into_iter().collect();
        let size = buffer.len();
        RingBuf {
            buffer,
            size,
            cur: 0,
        }
    }
}

pub struct RingBufIterator<'a, T: 'a> {
    ring_buf: &'a RingBuf<T>,
    cur: usize,
}

impl<'a, T: 'a> Iterator for RingBufIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.ring_buf.adjust_index(self.cur) {
            Ok(ai) => Some(&self.ring_buf.buffer[ai]),
            Err(_) => None,
        };
        self.cur += 1;
        item
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_ring_buf_ops() {
        let mut ring_buf = RingBuf::new(5);

        assert_eq!(0, ring_buf.len());
        assert_eq!(ring_buf.iter().next(), None);

        ring_buf.push(1);
        ring_buf.push(2);
        ring_buf.push(3);
        ring_buf.push(4);

        assert_eq!(4, ring_buf.len());

        let elements: Vec<&i32> = ring_buf.iter().collect();
        assert_eq!(vec![&1, &2, &3, &4], elements);

        ring_buf.push(5);

        assert_eq!(5, ring_buf.len());

        let elements: Vec<&i32> = ring_buf.iter().collect();
        assert_eq!(vec![&1, &2, &3, &4, &5], elements);

        ring_buf.push(6);

        assert_eq!(5, ring_buf.len());

        let elements: Vec<&i32> = ring_buf.iter().collect();
        assert_eq!(vec![&2, &3, &4, &5, &6], elements);

        ring_buf.swap(0, 1);

        let elements: Vec<&i32> = ring_buf.iter().collect();
        assert_eq!(vec![&3, &2, &4, &5, &6], elements);
    }

}
