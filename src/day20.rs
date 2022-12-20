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

        let original_index_a = self.indices.iter().position(|i| i == &a).unwrap();
        let original_index_b = self.indices.iter().position(|i| i == &b).unwrap();

        self.indices.swap(original_index_a, original_index_b);

        // println!("post swap: numbers: {:?} indices: {:?} swapping {}<->{}, resulting in {:?}", self.numbers, self.indices, a, b, self.original_content());
        // println!();
    }

    pub fn calc_rel_pos(&self, cur: isize, rel: isize) -> usize {
        let double_len = (self.len() * 2) as isize;
        let dir = rel.signum();

        let virt_cur = cur * 2;
        let virt_cur = virt_cur + dir;

        // println!("virt_cur: {}", virt_cur);

        let virt_dest = virt_cur + (rel * 2);
        let virt_dest = virt_dest % double_len;

        // println!("virt dest: {}", virt_dest);

        let virt_dest = if virt_dest < 0 {
            virt_dest + double_len
        } else {
            virt_dest
        };

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

    #[test]
    fn test_rel_pos() {
        let mut ef = EncryptedFile::new(vec![4, -2, 5, 6, 7, 8, 9]);

        let dest = ef.calc_rel_pos(1, -2);
        assert_eq!(dest, 5);

        ef.swap_towards(1, dest);
        assert_eq!(&vec![4, 5, 6, 7, 8, -2, 9], ef.content());

        assert_eq!(vec![&4, &-2, &5, &6, &7, &8, &9], ef.original_content())
    }
}

