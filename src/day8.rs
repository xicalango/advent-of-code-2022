
use std::str::FromStr;
use std::fmt::{Display, Formatter};

use crate::Error;

#[derive(Debug)]
pub struct Field<const N: usize> {
  trees: [usize; N],
}

impl<const N: usize> Field<N> {

  pub fn side_length() -> usize {
    (N as f32).sqrt() as usize
  }

  pub fn xy_to_idx(&self, x: usize, y: usize) -> usize {
    let side_length = Self::side_length();
    assert!(x < side_length);
    assert!(y < side_length);

    y * side_length + x
  }

  pub fn get(&self, x: usize, y: usize) -> &usize {
    let index = self.xy_to_idx(x, y);
    &self.trees[index]
  }

  pub fn set(&mut self, x: usize, y: usize, value: usize) {
    let index = self.xy_to_idx(x, y);
    self.trees[index] = value;
  }

  pub fn is_visible(&self, x: usize, y: usize) -> bool {
    let side_length = Self::side_length();

    let cur_tree_height = self.get(x, y);

    if x == 0 || x == side_length - 1 || y == 0 || y == side_length -1 {
      return true;
    } else {
      let mut visibility_check = true;

      // left
      for check_x in 0..x {
        let other_tree = self.get(check_x, y);
        if other_tree >= cur_tree_height {
          visibility_check = false;
          break;
        }
      }

      if visibility_check {
        return true;
      }

      visibility_check = true;

      // right
      for check_x in x+1..side_length {
        let other_tree = self.get(check_x, y);
        if other_tree >= cur_tree_height {
          visibility_check = false;
          break;
        }
      }

      if visibility_check {
        return true;
      }

      visibility_check = true;

      // up
      for check_y in 0..y {
        let other_tree = self.get(x, check_y);
        if other_tree >= cur_tree_height {
          visibility_check = false;
          break;
        }
      }

      if visibility_check {
        return true;
      }

      visibility_check = true;

      // down
      for check_y in y+1..side_length {
        let other_tree = self.get(x, check_y);
        if other_tree >= cur_tree_height {
          visibility_check = false;
          break;
        }
      }

      return visibility_check;
    }
  }

/*

  01234
0 abcde
1 fghij
2 klmno
3 pqrst
4 uvwxy

*/

  pub fn get_visibility_score(&self, x: usize, y: usize) -> usize {
    let side_length = Self::side_length();

    if x == 0 || x == side_length - 1 || y == 0 || y == side_length -1 {
      return 0;
    }

    let cur_tree_height = self.get(x, y);

    let mut left_score: usize = 0;
    let mut right_score: usize = 0;
    let mut up_score: usize = 0;
    let mut down_score: usize = 0;

    // left
    for d_x in 1..=x as isize {
      let check_tree = self.get((x as isize - d_x) as usize, y);
      left_score += 1;
      if check_tree >= cur_tree_height {
        break;
      }
    }

    // right
    for d_x in 1..(side_length - x) as isize {
      let check_tree = self.get((x as isize + d_x) as usize, y);
      right_score += 1;
      if check_tree >= cur_tree_height {
        break;
      }
    }

    // up
    for d_y in 1..=y as isize {
      let check_tree = self.get(x, (y as isize - d_y) as usize);
      up_score += 1;
      if check_tree >= cur_tree_height {
        break;
      }
    }

    // down
    for d_y in 1..(side_length - y) as isize {
      let check_tree = self.get(x, (y as isize + d_y) as usize);
      down_score += 1;
      if check_tree >= cur_tree_height {
        break;
      }
    }

    left_score * right_score * down_score * up_score
  }

  pub fn to_visibility_field(&self) -> Field<N> {
    let side_length = Self::side_length();
    let mut trees: [usize; N] = [0; N];
    let mut i: usize = 0;

    for y in 0..side_length {
      for x in 0..side_length {
        trees[i] = self.is_visible(x, y) as usize;
        i += 1;
      }
    }

    Field { trees }
  }

  pub fn to_score_field(&self) -> Field<N> {
    let side_length = Self::side_length();
    let mut trees: [usize; N] = [0; N];
    let mut i: usize = 0;

    for y in 0..side_length {
      for x in 0..side_length {
        trees[i] = self.get_visibility_score(x, y);
        i += 1;
      }
    }

    Field { trees }
  }

  pub fn count_non_zero(&self) -> usize {
    self.trees.iter().filter(|t| **t != 0).count()
  }

  pub fn max(&self) -> Option<&usize> {
    self.trees.iter().max()
  }
}

impl<const N: usize> FromStr for Field<N> {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> { 
    let mut trees: [usize; N] = [0; N];
    let mut i: usize = 0;

    for line in s.lines() {
      for c in line.trim_end().chars() {
        let value = (c as u8) - ('0' as u8);
        if value >= 10 {
          return Err(Error(format!("invalid tree: {}", c)));
        }
        trees[i] = value as usize;
        i += 1;
      }
    }

    Ok(Field { trees })
  }

}

impl<const N: usize> Display for Field<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    let side_length = Self::side_length();
    for y in 0..side_length {
      for x in 0..side_length {
        write!(f, "{}", self.get(x, y))?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  
  use super::*;

  static INPUT: &'static str = include_str!("../res/day8-trees_example.txt");

  #[test]
  fn test_parse_field() {

    let field: Field<{5*5}> = INPUT.parse().unwrap();

    println!("{:#?}", field);
    println!("{}", field);

  }

  #[test]
  fn test_to_viz_map() {

    let field: Field<{5*5}> = INPUT.parse().unwrap();
    let viz_field = field.to_visibility_field();

    println!("{}", field);
    println!("{}", viz_field);

    println!("visible: {}", viz_field.count_non_zero());

  }

  #[test]
  fn test_to_score_map() {

    let field: Field<{5*5}> = INPUT.parse().unwrap();
    let score_field = field.to_score_field();

    println!("{}", field);
    println!("{:#?}", score_field);
    println!("{:?}", score_field.max());
  }

}

