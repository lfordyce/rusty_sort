use std::cmp::Ord;
use std::clone::Clone;

//pub fn selection_sort(list: &mut [i64]) {
//    for i in 0..list.len() {
//
//        let mut small = i;
//
//        for j in (i + 1)..list.len() {
//            if list[j] < list[small]  {
//                small = j;
//            }
//        }
//        // swap smallest onto end of sorted portion
//        // list[small], list[i] = list[i], list[small]
//        list.swap(small, i);
//
//    }
//}


pub fn selection_sort_start<T: Ord>(list: &mut [T]) {

//    list.iter():             [4, 3, 1, 2]
//    list.iter().enumerate(): [(0,4), (1,3), (2,1), (3,2)]

    for i in 0..list.len() {

        if let Some((j, _)) = list.iter()
            .enumerate()
            .skip(i)
            .min_by_key(|x| x.1) {

            // swap smallest onto end of sorted portion
            // list[small], list[i] = list[i], list[small]
            list.swap(i, j)
        }
    }
}

pub trait SelectSortable {
    fn selection_sort(&mut self);
}

impl <T: Ord + Clone + PartialEq> SelectSortable for Vec<T> {
    fn selection_sort(&mut self) {

        for i in 0..self.len() {

            if let Some((j, _)) = self.iter()
                .enumerate()
                .skip(i)
                .min_by_key(|x| x.1) {

                self.swap(i , j)
            }
        }
    }
}

pub trait InsertionSortable {
    fn insertion_sort(&mut self);
}

impl <T: Ord + Clone + PartialEq> InsertionSortable for Vec<T> {
    fn insertion_sort(&mut self) {

        for i in 1..self.len() {
            for j in ( 1..i+ 1).rev() {
                if self[j - 1] <= self[j] { break; }
                self.swap(j - 1, j);
            }
        }
    }
}

pub trait QuickSortable {
    fn quick_sort(&mut self, start: Option<isize>, end: Option<isize>);
    fn _partition(&mut self, start: isize, end: isize) -> isize;
}

impl<T: Ord + Clone + PartialEq> QuickSortable for Vec<T> {

    fn quick_sort(&mut self, start: Option<isize>, end: Option<isize>) {

        let start_val = start.unwrap_or(0);
        let end_val = end.unwrap_or((self.len() - 1) as isize);

        if start_val >= end_val {
            return;
        }

        let pivot = self._partition(start_val, end_val);
        self.quick_sort(Some(start_val), Some(pivot - 1));
        self.quick_sort(Some(pivot + 1), Some(end_val));
    }

    fn _partition(&mut self, start: isize, end: isize) -> isize {

        let mut pivot = start;

        for idx in (start + 1)..(end + 1) {
            if self[idx as usize] <= self[start as usize] {
                pivot += 1;
                let tmp = self[idx as usize].clone();
                self[idx as usize] = self[pivot as usize].clone();
                self[pivot as usize] = tmp;
            }
        }

        let tmp = self[start as usize].clone();
        self[start as usize] = self[pivot as usize].clone();
        self[pivot as usize] = tmp;
        return pivot;
    }
}

pub trait MergeSortable {
    fn merge_sort(&mut self);
}

impl <T: Ord + Clone + PartialEq> MergeSortable for Vec<T> {
    fn merge_sort(&mut self) {

        let mid = self.len() / 2;
        let _low = self[mid..].to_vec();
        let _high = self[..mid].to_vec();
    }
}

/// `euler_solution` function solves the first Project Euler problem
pub fn euler_solution(max: u64) -> u64 {

    // let mut result = 0;

    // USING LOOP
    // let mut i = 0;
    // loop {
    //     if i >= max {
    //         break;
    //     }
    //     if i % 3 ==0 || i % 5 == 0 {
    //         result += i;
    //     }
    //     i += 1;
    // }


    // USING for loop
    // for i in 0..max {
    //     if i % 3 == 0 || i % 5 == 0 {
    //         result += i;
    //     }
    // }

    // USING For loop with closure: Anonymous function that can be passed into another function.
    // `|n|` is the argument/s, and the rest is the function body.
    // can also be written as: |n| { return n % 3 == 0 || n % 5 == 0; }
    //
    // for i in (0..max).filter(|n| n % 3 == 0 || n % 5 == 0) {
    //     result += i
    // }
    // // Implicit return below:
    // result

    (0..max).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}


pub struct Fibonacci {
    pub a: Option<i64>,
    pub b: Option<i64>,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            a: Some(0),
            b: Some(1),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {

        match self.a.take() {
            None => None,
            Some(result) => {

                let next = self.b.and_then(|b| result.checked_add(b) );

                self.a = self.b;

                self.b = next;

                Some(result)
            }
        }
    }
}


#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod test {

    use super::{SelectSortable, InsertionSortable, euler_solution, Fibonacci, QuickSortable};
    use rand::{self, Rng};
    use rand::distributions::{Standard};

    #[test]
    fn test_selection_sort() {

        let mut rng = rand::thread_rng();

        let mut v: Vec<i32> = rng.sample_iter(&Standard).take(10000).collect();

        v.selection_sort();

        for i in 0..v.len() - 1 {
            assert!(v[i] <= v[i + 1])
        }
    }

    #[test]
    fn test_insertion_sort() {

        let mut rng = rand::thread_rng();
        let mut v: Vec<i32> = rng.sample_iter(&Standard).take(10000).collect();

        v.insertion_sort();

        for i in 0..v.len() - 1 {
            assert!(v[i] <= v[i + 1]);
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut rng = rand::thread_rng();
        let mut v: Vec<i32> = rng.sample_iter(&Standard).take(10000).collect();

        v.quick_sort(None, None);
        for i in 0..v.len() - 1 {
            assert!(v[i] <= v[i + 1]);
        }
    }

    #[test]
    fn test_euler_solution() {
        let solution = euler_solution(10);
        assert_eq!(23, solution);
    }

    #[test]
    fn test_fibonacci() {
        let expected = &[0, 1, 1, 2, 3, 5, 8, 13];
        let actual: Vec<_> = Fibonacci::new().take(8).collect();

        assert_eq!(expected, &*actual);
    }
}