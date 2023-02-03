use std::fmt::Debug;

fn return_sorted_list<T: Ord + Debug>(input: &mut [T]) -> &mut [T] {
    quick_sort(input);
    input
}

pub fn quick_sort<T: Ord + Debug>(unsorted: &mut [T]) {
    let len = unsorted.len();
    call_qs_recursively(unsorted, 0, (len - 1) as isize, 1);
}

fn call_qs_recursively<T: Ord + Debug>(
    unsorted: &mut [T],
    lowest_index: isize,
    highest_index: isize,
    depth: usize,
) {
    // println!(
    //     "depth:{0:1} / unsorted: {1:?} / lowest_index: {2:?} / highest_index: {3:?}",
    //     depth, unsorted, lowest_index, highest_index
    // );

    if lowest_index < highest_index {
        let partition_index = get_partition_index(unsorted, lowest_index, highest_index);
        call_qs_recursively(unsorted, lowest_index, partition_index - 1, depth + 1);
        call_qs_recursively(unsorted, partition_index + 1, highest_index, depth + 1);
    }
    // the array is now sorted
}

fn get_partition_index<T: Ord>(unsorted: &mut [T], left: isize, highest_index: isize) -> isize {
    let mut partition_index = left - 1;
    let mut right = highest_index;

    let pivot = highest_index as usize;

    loop {
        partition_index += 1;
        while unsorted[partition_index as usize] < unsorted[pivot] {
            partition_index += 1;
        }
        right -= 1;
        while right >= 0 && unsorted[right as usize] > unsorted[pivot] {
            right -= 1;
        }
        if partition_index >= right {
            break;
        } else {
            unsorted.swap(partition_index as usize, right as usize);
        }
    }
    unsorted.swap(partition_index as usize, pivot);
    partition_index
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use test_case::test_case;

    use crate::other::sandbox::return_sorted_list;

    // #[test_case(
    //   &mut [4, 65, 2, -31, 0, 99, 2, 83, 782, 1, 432, 432, 654, 43, 32,5, -10, 39],
    //   &mut [-31, -10, 0, 1, 2, 2, 4, 5, 32, 39, 43, 65, 83, 99, 432, 432, 654, 782]
    // )]
    #[test_case(
      &mut [
"4fff21a7-972a-42f6-b919-2fc1690cbfd5",
"0cdde707-b83f-4b4d-8f8b-c0a95df4aa82",
"e37f4164-bc86-4173-9588-ddda7138f6bf",
"073675f5-c320-4401-8b2c-eb5afbd25ea5",
"13ced697-00c7-4f09-9f36-fcff17894480",
"7568b7bc-123c-4b19-a2d9-7490b671fa5d",
"951408cf-529e-4e06-a0a4-71c01ce46414",
"1f7cf409-c0ee-466f-9c01-e06505abf2d0",
"19643896-7dbe-4fe8-a587-b7158f4d7915",
"4a3f3f87-2f36-4b68-a0de-c59731033ffa",
"fe4889f7-748b-4928-937b-6fa7e38ebe1c",
"6cf08343-4c8f-4f55-bc76-471b4af2953a",
"1f5bacdb-c34a-40ec-9c6b-dfd8d95611c0",
"aaacbf07-fc9d-4c4a-a1b5-612e8d7c97a7",
"2afb8b6a-cc86-440c-ba09-47b1b31f5b52",
"c8d35e94-c459-4ec7-83c6-5736f1133e97",
"6ae66f7a-78eb-4441-aa84-cee6a10e3c38",
"89259eda-fac1-41fc-8180-65e1a4d49afd",
"92c81484-f50d-45ca-959f-48970106b9c1",
"b167869b-199e-4198-81fc-9fd51a3e2692",
"be7c8baf-7db1-4223-aadf-0ed9e2ce739a",
"6710d421-2e43-4b3e-b231-c65feecc0a3f",
"84b1d34e-d7e5-4700-a7cd-52c84312c98e",
"b6ea8b27-23cb-4e0c-9f38-e047bfc4459d",
"50606b73-68c2-408e-afee-e7915196cd62",
"c4195e69-ca24-4235-81c4-a4bda6c5d992",
"14205d24-7e48-47c4-b9c8-5c0faab254a9",
"3008801b-f1ab-4bb7-853e-8d7444f39537",
"d7ae2870-15c9-4039-99c4-240bdc794281",
"11172382-d203-4923-b34f-0d3e40941c24",
"11b3c5ce-1175-40ea-bcc3-78373eea91dc",
"28c707ca-2bc3-4b89-ba7d-9c513ad453be",
"721a88bb-d500-4a15-8365-cf7ad8ef388b",
"72e6dd6e-a6fd-4774-a333-efa8a0816e3b",
"5c5f6e65-9177-4775-ac04-ba2727753def",
"c5a03ecc-80ae-4be2-8019-21989b5ea05e",
"5210e56c-1d1a-4fec-8ff2-f475c21d65ce",
"a2695290-9af2-47d2-a792-2f803aa837ef",
"045b9a1c-e734-465f-ae96-2eb748309b84",
"410e7f9d-ae53-4819-bd2f-3f952bf017e6",
"474c8514-7308-4981-955b-0065570b3f16",
"46f4b99d-1eb0-4df1-9e25-cd582318be92",
"9d938413-b87c-4d93-a601-2e0fa52e8d02",
"4370f261-0e78-43f4-9242-ce1184b629e7",
"415bc566-4f23-437f-88c3-964bd9babe16",
"abfbdfdd-4dce-4a3e-a601-2fe632c02837",
"4cad7304-6b38-420c-9352-3db832bbfd10",
"7fece5e9-a00b-4e78-a71e-468821080d7a",
"e72930a9-27c1-4087-bdca-9408f40bddb8",
"663328fb-d65d-4839-a32c-956173715adf",
"eade9f6b-2222-40a4-bb25-3a6b6aefa70a"
        ],
      &mut ["airplane", "art", "beach", "car", "hotel", "house"]
    )]
    fn should_sort_correctly<T: Ord + Debug>(unsorted: &mut [T], expected_result: &mut [T]) {
        let result = return_sorted_list(unsorted);
        assert_eq!(result, expected_result)
    }
}
