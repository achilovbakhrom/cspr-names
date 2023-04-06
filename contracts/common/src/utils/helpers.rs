pub fn is_array_contain<T: PartialEq>(arr: &[T], item: &T) -> bool {
    let found = arr.iter().find(|i| i == &item);
    found.is_some()
}
