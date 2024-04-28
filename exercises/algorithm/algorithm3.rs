/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::mem::swap;

fn sort<T: std::cmp::PartialOrd + Clone + std::fmt::Debug>(array: &mut [T]) {
    quick_sort(array);
    println!("{:?}", array);
}

fn quick_sort<T: std::cmp::PartialOrd + Clone + std::fmt::Debug>(arr: &mut [T]) {
    println!("{:?}", arr);

    if arr.len() <= 1 {
        return;
    }

    let pivotpos = partition(arr);
    println!(
        "POS: {}", pivotpos
    );
    quick_sort(&mut arr[..pivotpos]);
    quick_sort(&mut arr[pivotpos + 1..]);
}

fn partition<T: std::cmp::PartialOrd + Clone>(array: &mut [T]) -> usize {
    let pivot = array[0].clone();

    let (mut l, mut r) = (0_usize, array.len() - 1);

    loop {
        while array[r] > pivot {
            r -= 1;
        }
        while array[l] < pivot {
            l += 1;
        }

        array.swap(l, r);

        if l >= r {
            return r
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}