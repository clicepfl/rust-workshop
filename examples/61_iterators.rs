#![allow(dead_code, unused_variables, unreachable_code, clippy::all)]

fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // Iterators return Option with reference to the value
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn consume_iterator() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter();

  let total: i32 = v1_iter.sum();

  assert_eq!(total, 6);
}

fn iterator_adapters() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}


struct Shoe {
    size: u32,
    style: String,
}

// Take ownership with into_iter
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
