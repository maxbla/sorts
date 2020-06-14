#![feature(const_int_pow)]

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod quicksort {
    /// Finds the median of the first, last and middle elements of an array
    ///
    /// returns the index of this element
    /// panics if arr is empty
    fn median_pivot<T: Ord>(arr: &[T]) -> usize {
        let last = arr.len() - 1;
        let mid = arr.len() / 2;
        //assert!(0 != mid && mid != last);
        let mut tmp = [(&arr[0], 0), (&arr[mid], mid), (&arr[last], last)];
        tmp.sort();
        let (_val, idx) = tmp[1];
        idx
    }

    // panics if arr is small (empty or )
    pub(crate) fn partition<T: Ord>(arr: &mut [T]) -> (&mut [T], &mut [T]) {
        let pivot_idx = median_pivot(arr);
        //move pivot to end of array
        arr.swap(pivot_idx, arr.len() - 1);
        let mut partition_idx = 0;
        for idx in 0..arr.len() - 1 {
            if arr[idx] < arr[arr.len() - 1] {
                arr.swap(partition_idx, idx);
                partition_idx += 1;
            }
        }
        arr.swap(partition_idx, arr.len() - 1);

        let (lower, upper) = arr.split_at_mut(partition_idx);
        let (_pivot, upper) = upper.split_at_mut(1);
        (lower, upper)
    }

    pub fn quicksort<T: Ord>(arr: &mut [T]) {
        // base case
        if arr.len() <= 1 {
            return;
        }
        let (lower, upper) = partition(arr);
        quicksort(lower);
        quicksort(upper);
    }
}

pub mod merge_sort {
    use std::mem::swap;

    // none of the arguments can overlap with any other
    // if they overlap, it will cause dest, left and right to contain disordered
    // values from dest, left and right
    unsafe fn merge_nonoverlapping<T: Ord + Default>(
        dest: &mut [T],
        left: &mut [T],
        right: &mut [T],
    ) {
        assert!(dest.len() >= left.len() + right.len());
        let mut left_idx = 0;
        let mut right_idx = 0;
        while let Some(dest_val) = dest.get_mut(left_idx + right_idx) {
            match (left.get_mut(left_idx), right.get_mut(right_idx)) {
                (Some(left_val), Some(right_val)) => {
                    if left_val <= right_val {
                        swap(dest_val, left_val);
                        left_idx += 1;
                    } else {
                        swap(dest_val, right_val);
                        right_idx += 1;
                    }
                }
                (Some(left_val), None) => {
                    swap(dest_val, left_val);
                    left_idx += 1;
                }
                (None, Some(right_val)) => {
                    swap(dest_val, right_val);
                    right_idx += 1;
                }
                (None, None) => break,
            }
        }
    }

    // TODO: remove Default requirement
    pub fn recursive_merge_sort<T: Ord + Default>(arr: &mut [T]) {
        //setup spare array. Use default to avoid UB
        let mut spare = Vec::with_capacity(arr.len());
        (0..arr.len()).for_each(|_| spare.push(T::default()));
        recursive_merge_sort_helper(arr, &mut spare);
    }

    pub fn recursive_merge_sort_helper<'a, T: Ord + Default>(arr: &'a mut [T], spare: &'a mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let mid = arr.len() / 2;
        let (left, right) = arr.split_at_mut(mid);
        let (spare_left, spare_right) = spare.split_at_mut(mid);

        recursive_merge_sort_helper(left, spare_left);
        recursive_merge_sort_helper(right, spare_right);

        unsafe {
            merge_nonoverlapping(spare, left, right);
        }
        spare.swap_with_slice(arr);
    }

    // // iterative merge sort
    // pub fn merge_sort<T: Ord + Default>(arr: &mut [T]) {
    //     //setup spare array. Use default to avoid UB
    //     let spare = Vec::with_capacity(arr.len());
    //     for _ in 0..arr.len() {
    //         spare.push(T::default());
    //     }

    //     for size in (0..).map(|exp| 2_usize.pow(exp)).take_while(|&size| size < arr.len()) {
    //         println!("new size: {}", size);
    //         for idx in (0..arr.len()).step_by(2*size) {
    //             let (left, right) = (&arr[idx]).split_at_mut(idx+size);
    //             let dest = spare.split_at_mut(mid)

    //             let range1 = idx..(idx+size);
    //             let range2 = (idx+size)..(idx+size+size);
    //             println!("ranges: {:?} {:?}", range1, range2);
    //         }
    //     }
    // }
}

pub mod heap_sort {
    // try to move an element down a level in a heap
    // returns the index it was moved to or the input index if it was not moved
    fn move_down<T: Ord>(arr: &mut [T], idx: usize) -> usize {
        let (left_idx, right_idx) = (2 * idx + 1, 2 * idx + 2);
        let mut largest_idx = idx;
        if arr.get(left_idx) > arr.get(largest_idx) {
            largest_idx = left_idx;
        }
        if arr.get(right_idx) > arr.get(largest_idx) {
            largest_idx = right_idx;
        }

        if largest_idx != idx {
            arr.swap(idx, largest_idx);
        }
        largest_idx
    }

    // repeadedly moves element at idx down until both children are smaller, or
    // there are no children
    fn sift_down<T: Ord>(arr: &mut [T], idx: usize) {
        let mut last_idx = idx;
        let mut next_idx;
        while {
            next_idx = move_down(arr, last_idx);
            next_idx != last_idx
        } {
            last_idx = next_idx;
        }
    }

    // creates an array that satisfies a max on top heap property
    // index 0 is the root, index 1 is the left node, index 2 the right, etc.
    fn heapify_slice<T: Ord>(arr: &mut [T]) {
        for idx in (0..(arr.len() / 2)).rev() {
            sift_down(arr, idx);
        }
    }

    pub fn heap_sort<T: Ord>(arr: &mut [T]) {
        heapify_slice(arr);
        for idx in (0..arr.len()).rev() {
            arr.swap(0, idx);
            sift_down(&mut arr[0..idx], 0);
        }
    }
}

pub mod selection_sort {
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        for idx in 0..arr.len() {
            //find smallest element
            let mut min_idx: Option<usize> = None;
            for jdx in idx..arr.len() {
                min_idx = match min_idx {
                    None => Some(jdx),
                    Some(min_idx) if arr[jdx] < arr[min_idx] => Some(jdx),
                    _ => min_idx,
                }
            }
            //insert (swap) lowest element
            match min_idx {
                Some(min_idx) => arr.swap(min_idx, idx),
                None => panic!("Pretty sure this shouldn't happen"),
            }
        }
    }
}

pub mod insertion_sort {
    use std::mem::MaybeUninit;

    //moves an emelemt elsewhere, sliding other elements down to make room
    pub(crate) fn slide_swap<T>(arr: &mut [T], start_idx: usize, end_idx: usize) {
        assert!(start_idx <= end_idx);
        for idx in (start_idx..end_idx).rev() {
            arr.swap(idx, idx + 1);
        }
    }

    // moves end_idx to start_idx, sliding all other elements down
    pub(crate) unsafe fn unsafe_slide_swap<T>(arr: &mut [T], start_idx: usize, end_idx: usize) {
        assert!(start_idx <= end_idx);
        assert!(end_idx < arr.len());
        if start_idx == end_idx {
            return;
        }
        let mut tmp: MaybeUninit<T> = MaybeUninit::uninit();
        std::ptr::copy(&mut arr[end_idx] as *mut T, tmp.as_mut_ptr(), 1);
        let tmp = tmp.assume_init();
        std::ptr::copy(
            &arr[start_idx],
            &mut arr[start_idx + 1],
            end_idx - start_idx,
        );
        arr[start_idx] = tmp;
    }

    pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
        for idx in 1..arr.len() {
            let dest = &arr[0..idx].binary_search(&arr[idx]);
            let dest_idx = match dest {
                Ok(idx) => *idx + 1,
                Err(idx) => *idx,
            };
            unsafe { unsafe_slide_swap(arr, dest_idx, idx) };
        }
    }
}

pub mod shell_sort {

    // panics if idx < 1 or idx > 31
    const fn shell_gap_sequence_unchecked(idx: u32) -> usize {
        // debug_assert!(1..=31.constins(idx)); // <- can't assert in const fn
        // 4^idx + 3 * 2^idx + 1, OEIS A036562
        4_usize.pow(idx) + 3 * 2_usize.pow(idx - 1) + 1
    }

    macro_rules! gen_shell_gaps_helper {
        ($($idx:literal), *) => {{
            [1,
            $(
                shell_gap_sequence_unchecked($idx),
            )*
            ]
        }};
    }

    macro_rules! gen_shell_gaps {
        () => {
            gen_shell_gaps_helper!(
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31
            )
        };
    }

    static SHELL_GAPS: [usize; 32] = gen_shell_gaps!();

    pub fn shell_sort<T: Ord>(arr: &mut [T]) {
        let start_gap_idx = match SHELL_GAPS.binary_search(&arr.len()) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        for gap in SHELL_GAPS[0..=start_gap_idx].iter().rev() {
            for start_idx in 0..*gap {
                gap_insertion_sort(arr, start_idx, *gap);
            }
        }
    }

    // performs an insertion sort on elements separated by a constant gap
    pub(crate) fn gap_insertion_sort<T: Ord>(arr: &mut [T], start_idx: usize, gap: usize) {
        for idx in (start_idx..arr.len()).step_by(gap).skip(1) {
            let mut jdx = idx;
            while jdx >= gap && arr[jdx] < arr[jdx - gap] {
                arr.swap(jdx, jdx - gap);
                jdx -= gap;
            }
        }
    }
}

pub mod bubble_sort {
    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for idx in 1..arr.len() {
                if arr[idx - 1] > arr[idx] {
                    arr.swap(idx - 1, idx);
                    swapped = true;
                }
            }
        }
    }
}

pub mod hybrid_sorts {
    fn log_2(mut num: usize) -> usize {
        let mut count = 0;
        while num > 0 {
            num /= 2;
            count += 1;
        }
        count
    }

    pub fn intro_sort<T: Ord>(arr: &mut[T]) {
        intro_sort_helper(arr, log_2(arr.len()))
    }

    fn intro_sort_helper<T: Ord>(arr: &mut [T], max_depth: usize) {
        if arr.len() < 16 {
            crate::insertion_sort::insertion_sort(arr);
        } else if max_depth == 0 {
            crate::heap_sort::heap_sort(arr);
        } else {
            let (lower, upper) = crate::quicksort::partition(arr);
            intro_sort_helper(lower, max_depth - 1);
            intro_sort_helper(upper, max_depth - 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::bubble_sort::bubble_sort;
    use super::heap_sort::heap_sort;
    use super::insertion_sort::{insertion_sort, slide_swap, unsafe_slide_swap};
    use super::merge_sort::recursive_merge_sort;
    use super::quicksort::quicksort;
    use super::selection_sort::selection_sort;
    use super::shell_sort::{gap_insertion_sort, shell_sort};
    use super::hybrid_sorts::intro_sort;
    use std::iter::once;

    extern crate quickcheck;
    use quickcheck::TestResult;

    macro_rules! quickcheck_sort_fn {
        ($fn_name:tt, $sort_fn:ident, $ty:ty) => {
            #[quickcheck]
            fn $fn_name(mut vec: Vec<$ty>) -> bool {
                let mut stdlib_sorted = vec.clone();
                stdlib_sorted.sort();
                $sort_fn(&mut vec);
                vec == stdlib_sorted
            }
        };
    }

    quickcheck_sort_fn!(qc_quicksort, quicksort, isize);
    quickcheck_sort_fn!(qc_recursive_merge_sort, recursive_merge_sort, isize);
    quickcheck_sort_fn!(qc_heap_sort, heap_sort, isize);
    quickcheck_sort_fn!(qc_shell_sort, shell_sort, isize);
    quickcheck_sort_fn!(qc_bubble_sort, bubble_sort, isize);
    quickcheck_sort_fn!(qc_insertion_sort, insertion_sort, isize);
    quickcheck_sort_fn!(qc_selection_sort, selection_sort, isize);
    quickcheck_sort_fn!(qc_intro_sort, intro_sort, isize);


    #[quickcheck]
    fn qc_gap_insertion_sort(mut vec: Vec<isize>, mut gaps: Vec<usize>) -> bool {
        let mut gap_insertion_sorted = vec.clone();
        gaps.sort();
        for gap in gaps {
            for start_idx in 0..gap {
                gap_insertion_sort(&mut gap_insertion_sorted, start_idx, gap);
            }
        }
        gap_insertion_sort(&mut gap_insertion_sorted, 0, 1);
        vec.sort();
        gap_insertion_sorted == vec
    }

    #[quickcheck]
    fn qc_slide_swap(mut vec: Vec<isize>, indicies: (usize, usize)) -> TestResult {
        let mut indicies = [indicies.0, indicies.1];
        indicies.sort();
        let (start_idx, end_idx) = (indicies[0], indicies[1]);
        if vec.get(end_idx).is_none() {
            return TestResult::discard();
        }

        let mut unsafe_slide = vec.clone();
        slide_swap(&mut vec, start_idx, end_idx);
        unsafe { unsafe_slide_swap(&mut unsafe_slide, start_idx, end_idx) }
        TestResult::from_bool(unsafe_slide == vec)
    }

    #[test]
    fn test_slide_swap() {
        let vec: Vec<_> = (0..).take(10).collect();
        {
            let mut vec = vec.clone();
            unsafe { unsafe_slide_swap(&mut vec, 0, 9) };
            let expected: Vec<_> = once(9).chain((0..).take(9)).collect();
            assert_eq!(vec, expected);
        }
        {
            let mut vec = vec.clone();
            vec.truncate(2);
            unsafe { unsafe_slide_swap(&mut vec, 0, 1) };
            let expected: Vec<_> = vec![1, 0];
            assert_eq!(vec, expected);
        }
    }
}
