use std::ops::Range;
use std::str::FromStr;

use crate::utils::Error;

pub type Number = isize;

pub struct EncryptedFile {
    numbers: Vec<Number>,
    indices: Vec<usize>,
    indices_rev: Vec<usize>,
}

impl EncryptedFile {
    pub fn new(numbers: Vec<Number>) -> EncryptedFile {
        let indices: Vec<usize> = numbers.iter().enumerate().map(|(i, _)| i).collect();
        EncryptedFile {
            numbers,
            indices: indices.clone(),
            indices_rev: indices,
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        // println!("pre  swap: numbers: {:?} indices: {:?} swapping {}<->{}, resulting in {:?}", self.numbers, self.indices, a, b, self.original_content());

        self.numbers.swap(a, b);

        let pos_a = self.index_of_rev(a);
        let pos_b = self.index_of_rev(b);

        self.indices.swap(pos_a, pos_b);
        self.indices_rev.swap(a, b);

        // println!("post swap: numbers: {:?} indices: {:?} swapping {}<->{}, resulting in {:?}", self.numbers, self.indices, a, b, self.original_content());
        // println!();
    }

    pub fn index_of_rev(&self, index: usize) -> usize {
        self.indices_rev[index]
    }

    pub fn index_of(&self, index: usize) -> usize {
        self.indices[index]
    }

    pub fn calc_rel_pos(&self, cur: isize, rel: isize) -> usize {
        let double_len = (self.len() * 2) as isize;
        let dir = rel.signum();

        // println!("cur: {} rel: {}", cur, rel);

        let virt_cur = cur * 2;
        let virt_cur = virt_cur + dir;

        // println!("virt_cur: {}", virt_cur);

        let virt_dest = virt_cur + (rel * 2);
        let virt_dest = virt_dest % double_len;

        let mut virt_dest = virt_dest;

        while virt_dest < 0 {
            virt_dest += double_len;
        }

        // println!("virt dest: {}", virt_dest);

        let virt_dest = virt_dest;

        let dest = if virt_dest < virt_cur {
            (virt_dest + 1) / 2
        } else {
            virt_dest / 2
        };

        // println!("dest: {}", dest);

        assert!(dest >= 0);

        dest as usize
    }

    pub fn swap_towards(&mut self, idx: usize, final_idx: usize) {
        if final_idx >= idx {
            for i in 0..(final_idx - idx) {
                self.swap(idx + i, idx + i + 1);
            }
        } else {
            for i in 0..(idx - final_idx) {
                self.swap(idx - i, idx - (i + 1));
            }
        }
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    pub fn content(&self) -> &Vec<Number> {
        &self.numbers
    }

    pub fn original_content(&self) -> Vec<&Number> {
        self.indices.iter().map(|i| &self.numbers[*i]).collect()
    }

    pub fn decrypt(&mut self) -> Option<usize> {
        self.decrypt_range(0..self.len())
    }

    pub fn decrypt_range(&mut self, range: Range<usize>) -> Option<usize> {

        let mut zero_pos = None;

        for i in range {
            let displacement = *self.original_content()[i];

            if displacement == 0 {
                assert!(zero_pos.is_none());
                zero_pos = Some(i)
            }

            let index_of = self.index_of(i);
            let destination = self.calc_rel_pos(index_of as isize, displacement);

            // println!("round {}, displacement {}, index_of {} destination {}, before: {:?}", i, displacement, index_of, destination, self.content());

            self.swap_towards(index_of, destination);

            // println!("round {}, displacement {}, index_of {} destination {}, afterr: {:?}", i, displacement, index_of, destination, self.content());
        }

        zero_pos.map(|v| self.index_of(v))
    }

    pub fn access_at_wrapping(&self, index: usize) -> &Number {
        let index = index % self.len();
        &self.numbers[index]
    }
}

impl FromIterator<Number> for EncryptedFile {
    fn from_iter<T: IntoIterator<Item=Number>>(iter: T) -> Self {
        let numbers: Vec<Number> = iter.into_iter().collect();
        EncryptedFile::new(numbers)
    }
}

impl FromStr for EncryptedFile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ef: EncryptedFile = s.lines().map(str::trim_end).map(|l| l.parse::<Number>().unwrap()).collect();
        Ok(ef)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = include_str!("../res/day20-enc_example.txt");

    #[test]
    fn test_rel_pos() {
        let mut ef = EncryptedFile::new(vec![4, -2, 5, 6, 7, 8, 9]);

        let dest = ef.calc_rel_pos(1, -2);
        assert_eq!(dest, 5);

        ef.swap_towards(1, dest);
        assert_eq!(&vec![4, 5, 6, 7, 8, -2, 9], ef.content());

        assert_eq!(vec![&4, &-2, &5, &6, &7, &8, &9], ef.original_content())
    }

    #[test]
    fn test_move_around() {
        let mut ef: EncryptedFile = EXAMPLE.parse().unwrap();
        let zero_pos = ef.decrypt();
        assert_eq!(&vec![1, 2, -3, 4, 0, 3, -2], ef.content());

        assert_eq!(Some(4), zero_pos);

        let zero_pos = zero_pos.unwrap();

        let coordinates: Vec<&Number> = vec![1000, 2000, 3000].into_iter()
                .map(|v| v + zero_pos)
            .map(|v| ef.access_at_wrapping(v)).collect();

        println!("coords: {:?}", coordinates);

        let sum: Number = coordinates.iter().map(|v| *v).sum();
        assert_eq!(3, sum);
    }

    #[test]
    fn test_move_around_2() {
        let mut ef: EncryptedFile = EncryptedFile::new(vec![-3, -3, 5, 6, 7, 8, 9]);

        ef.decrypt();

        println!("{:?}", ef.content())
    }
}

