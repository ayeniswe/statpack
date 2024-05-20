/// Performs a binary search on a list of elements to find an element whose key (extracted by the key_extractor function) starts with the given search_key.
/// If right is true, it finds the rightmost such element; otherwise, it finds the leftmost one.
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
