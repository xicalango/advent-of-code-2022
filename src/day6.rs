
#[derive(Debug)]
pub struct CodeDetector<const N: usize> {
  ring_buf: [u8; N],
  counters: [usize; 26],
  cur: usize,
  len: usize,
}

impl<const N: usize> Default for CodeDetector<N> {
  
  fn default() -> Self {
    CodeDetector {
      ring_buf: [0; N],
      counters: [0; 26],
      cur: 0,
      len: 0,
    }
  }

}

impl<const N: usize> CodeDetector<N> {

  pub fn feed(&mut self, c: char) {
    let c = (c as u8) - ('a' as u8);

    if self.len < N {
      self.ring_buf[self.len] = c;
      self.counters[c as usize] += 1;
      self.len += 1;
      return;
    }

    let evicted = self.ring_buf[self.cur];
    self.counters[evicted as usize] -= 1;

    self.ring_buf[self.cur] = c;
    self.counters[c as usize] += 1;

    self.cur += 1;
    self.cur %= N;
  }

  pub fn all_unique(&self) -> Option<bool> {
    if self.len < N {
      None
    } else {
      Some(!self.ring_buf.iter().map(|e| self.counters[*e as usize]).any(|e| e > 1))
    }
  }

  pub fn feed_and_check(&mut self, c: char) -> Option<bool> {
    self.feed(c);
    self.all_unique()
  }

}

pub fn find_sync_start<const N: usize>(s: &str) -> Option<usize> {
    let mut detector: CodeDetector<N> = CodeDetector::default();
    for (i, c) in s.chars().enumerate() {
        if let Some(true) = detector.feed_and_check(c) {
            return Some(i+1);
        }
    }
    return None;
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_feed() {

    let mut d: CodeDetector<4> = CodeDetector::default();

    println!("{:#?}", d);
    println!("{:?}", d.all_unique());

    d.feed('a');
    d.feed('b');
    d.feed('c');
    d.feed('a');

    println!("{:#?}", d);
    println!("{:?}", d.all_unique());

    d.feed('b');
    d.feed('c');
    d.feed('d');

    println!("{:#?}", d);
    println!("{:?}", d.all_unique());

    d.feed('e');

    println!("{:#?}", d);
    println!("{:?}", d.all_unique());
  }

  #[test]
  fn test_find_sync_start() {
      let test_0 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
      let test_1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
      let test_2 = "nppdvjthqldpwncqszvftbrmjlhg";
      let test_3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
      let test_4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

      assert_eq!(Some(7), find_sync_start::<4>(test_0));
      assert_eq!(Some(5), find_sync_start::<4>(test_1));
      assert_eq!(Some(6), find_sync_start::<4>(test_2));
      assert_eq!(Some(10), find_sync_start::<4>(test_3));
      assert_eq!(Some(11), find_sync_start::<4>(test_4));

      assert_eq!(Some(19), find_sync_start::<14>(test_0));
      assert_eq!(Some(23), find_sync_start::<14>(test_1));
      assert_eq!(Some(23), find_sync_start::<14>(test_2));
      assert_eq!(Some(29), find_sync_start::<14>(test_3));
      assert_eq!(Some(26), find_sync_start::<14>(test_4));
  }


}

