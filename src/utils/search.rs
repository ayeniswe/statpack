/// Performs a binary search on a sorted list of items, comparing the items' keys
/// (extracted using the provided `key_extractor` function) to the given `search_key`.
/// Depending on the value of the `right` parameter, it returns the index of the first
/// or last occurrence of the matching key.
/// ## Examples
///
/// ```
/// let items = vec!["apple", "banana", "cherry"];
/// let index = bisect_search_str_key(&items, "banana", false, |item, _| item.to_string());
/// assert_eq!(index, 1);
/// ```
/// ## Returns
/// Returns the `index` of the found option or `-1` if the option is not found.
pub fn bisect_search_str_key<'a, U, F>(
    list: &'a [U],
    search_key: &'a str,
    right: bool,
    key_extractor: F,
) -> i32
where
    F: Fn(&'a U, &str) -> String,
{
    let mut l = 0i32;
    let mut r = list.len() as i32 - 1i32;
    let mut index = -1;

    while l <= r {
        let mid: usize = ((l + r) / 2) as usize;
        let mid_i32 = mid as i32;
        let item_key = key_extractor(&list[mid], search_key);

        if item_key.starts_with(search_key) {
            index = mid_i32;
            if right {
                l = mid_i32 + 1;
            } else {
                r = mid_i32 - 1;
            }
        } else if item_key < search_key.to_string() {
            l = mid_i32 + 1;
        } else {
            r = mid_i32 - 1;
        }
    }

    index
}

#[cfg(test)]
mod bisect_search_str_key_tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        name: String,
    }

    fn key_extractor(item: &TestStruct, _search_key: &str) -> String {
        item.name.clone()
    }

    #[test]
    fn test_bisect_search_str_key_exact_match() {
        let list = vec![
            TestStruct {
                name: "apple".to_string(),
            },
            TestStruct {
                name: "banana".to_string(),
            },
            TestStruct {
                name: "cherry".to_string(),
            },
            TestStruct {
                name: "date".to_string(),
            },
            TestStruct {
                name: "fig".to_string(),
            },
        ];

        assert_eq!(
            bisect_search_str_key(&list, "banana", false, key_extractor),
            1
        );
        assert_eq!(
            bisect_search_str_key(&list, "cherry", true, key_extractor),
            2
        );
    }

    #[test]
    fn test_bisect_search_str_key_no_match() {
        let list = vec![
            TestStruct {
                name: "apple".to_string(),
            },
            TestStruct {
                name: "banana".to_string(),
            },
            TestStruct {
                name: "cherry".to_string(),
            },
            TestStruct {
                name: "date".to_string(),
            },
            TestStruct {
                name: "fig".to_string(),
            },
        ];

        assert_eq!(
            bisect_search_str_key(&list, "grape", false, key_extractor),
            -1
        );
        assert_eq!(
            bisect_search_str_key(&list, "apricot", true, key_extractor),
            -1
        );
    }

    #[test]
    fn test_bisect_search_str_key_partial_match() {
        let list = vec![
            TestStruct {
                name: "apple".to_string(),
            },
            TestStruct {
                name: "apricot".to_string(),
            },
            TestStruct {
                name: "banana".to_string(),
            },
            TestStruct {
                name: "cherry".to_string(),
            },
            TestStruct {
                name: "date".to_string(),
            },
            TestStruct {
                name: "fig".to_string(),
            },
        ];

        assert_eq!(bisect_search_str_key(&list, "ap", false, key_extractor), 0);
        assert_eq!(bisect_search_str_key(&list, "ap", true, key_extractor), 1);
    }

    #[test]
    fn test_bisect_search_str_key_empty_list() {
        let list: Vec<TestStruct> = vec![];

        assert_eq!(
            bisect_search_str_key(&list, "apple", false, key_extractor),
            -1
        );
    }

    #[test]
    fn test_bisect_search_str_key_single_element_match() {
        let list = vec![TestStruct {
            name: "apple".to_string(),
        }];

        assert_eq!(
            bisect_search_str_key(&list, "apple", false, key_extractor),
            0
        );
        assert_eq!(
            bisect_search_str_key(&list, "apple", true, key_extractor),
            0
        );
    }

    #[test]
    fn test_bisect_search_str_key_single_element_no_match() {
        let list = vec![TestStruct {
            name: "apple".to_string(),
        }];

        assert_eq!(
            bisect_search_str_key(&list, "banana", false, key_extractor),
            -1
        );
    }

    #[test]
    fn test_bisect_search_str_key_duplicates() {
        let list = vec![
            TestStruct {
                name: "apple".to_string(),
            },
            TestStruct {
                name: "banana".to_string(),
            },
            TestStruct {
                name: "banana".to_string(),
            },
            TestStruct {
                name: "banana".to_string(),
            },
            TestStruct {
                name: "cherry".to_string(),
            },
        ];

        assert_eq!(
            bisect_search_str_key(&list, "banana", false, key_extractor),
            1
        );
        assert_eq!(
            bisect_search_str_key(&list, "banana", true, key_extractor),
            3
        );
    }
}
