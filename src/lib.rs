#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn integers() {
        let result = merge_sort(vec![9,8,7,6,5,4,3,2,1], |a, b| a - b);
        assert_eq!(vec![1,2,3,4,5,6,7,8,9], result);
    }
}

pub fn merge_sort<T, F>(input: Vec<T>, _compare: F) -> Vec<T>
    where F: Fn(T, T) -> i64
{
    input
}
