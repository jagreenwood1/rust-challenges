fn return_sorted_list<T: Ord>(input: &mut [T]) -> &mut [T] {
    quick_sort(input);

    input
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot);
    store_index
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use test_case::test_case;

    use crate::other::sandbox::return_sorted_list;

    #[test_case(
      &mut [4, 65, 2, -31, 0, 99, 2, 83, 782, 1],
      &mut [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]
    )]
    #[test_case(
      &mut ["beach", "hotel", "airplane", "car", "house", "art"],
      &mut ["airplane", "art", "beach", "car", "hotel", "house"]
    )]
    fn should_sort_correctly<T>(unsorted: &mut [T], expected_result: &mut [T])
    where
        T: Ord,
        T: Debug,
    {
        let result = return_sorted_list(unsorted);
        assert_eq!(result, expected_result)
    }
}
