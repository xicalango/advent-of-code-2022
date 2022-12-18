use std::mem::size_of;
use std::ops::{BitAnd, BitOr};
use std::marker::Copy;

#[derive(Clone)]
pub struct ConstBitVec<const S: usize, T>([T; S]);

impl<const S: usize, T: Default + Copy> Default for ConstBitVec<S, T> {
    fn default() -> Self {
        ConstBitVec([T::default(); S])
    }
}

impl<const S: usize, T> ConstBitVec<S, T> {
    fn index_to_word_idx(index: usize) -> usize {
        let word_size: usize = size_of::<T>();
        index / word_size
    }

    fn index_to_pos_in_word(index: usize) -> usize {
        let word_size: usize = size_of::<T>();
        index % word_size
    }

    fn get_word(&self, index: usize) -> &T {
        let ConstBitVec(bit_vec) = self;
        let word_idx = Self::index_to_word_idx(index);
        &bit_vec[word_idx]
    }

    fn get_word_mut(&mut self, index: usize) -> &mut T {
        let ConstBitVec(bit_vec) = self;
        let word_idx = Self::index_to_word_idx(index);
        &mut bit_vec[word_idx]
    }
}

impl<const S: usize, T> ConstBitVec<S, T>
    where T: From<usize> + PartialEq + Copy + Default + BitAnd<Output=T>
{
    pub fn is_set(&self, index: usize) -> bool {
        let word = self.get_word(index);
        let pos = Self::index_to_pos_in_word(index);
        *word & (1 << pos).into() != 0.into()
    }

    pub fn clear_at(&mut self, index: usize) {
        let word = self.get_word_mut(index);
        let pos = Self::index_to_pos_in_word(index);
        *word = *word & (!(1 << pos)).into();
    }
}

impl<const S: usize, T> ConstBitVec<S, T>
    where T: From<usize> + PartialEq + Copy + Default + BitOr<Output=T>
{
    pub fn set_at(&mut self, index: usize) {
        let word = self.get_word_mut(index);
        let pos = Self::index_to_pos_in_word(index);
        *word = *word | (1 << pos).into();
    }
}

impl<const S: usize, T> ConstBitVec<S, T>
    where T: From<usize> + PartialEq + Copy + Default + BitOr<Output=T> + BitAnd<Output=T> {

    pub fn set(&mut self, index: usize, value: bool) {
        if value {
            self.set_at(index);
        } else {
            self.clear_at(index);
        }
    }

}

pub struct BitVec(Vec<u64>);
