// #![allow(unused)]
fn main() {
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
    }
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // methods that produce other iterators

    assert_eq!(v2, vec![2, 3, 4]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // iter - iterator over immutable references
        // into_iter - iterator that takes ownership of v1 and return owned values
        // iter_mut - iterate over mutable references
        // let mut v1_iter = v1.iter();
        let v1_iter = v1.iter();

        // assert_eq!(v1_iter.next(), Some(&1));
        // assert_eq!(v1_iter.next(), Some(&2));
        // assert_eq!(v1_iter.next(), Some(&3));
        // assert_eq!(v1_iter.next(), None);

        let total: i32 = v1_iter.sum(); // methods that consume the iterator i.e. sum

        assert_eq!(total, 6);
    }
}