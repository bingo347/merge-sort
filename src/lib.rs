use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn integers() {
        let result = merge_sort(vec![9,8,7,6,5,4,3,2,1], |a, b| (a - b) as f64);
        assert_eq!(vec![1,2,3,4,5,6,7,8,9], result);
        let result = merge_sort(vec![8,7,6,5,4,3,2,1], |a, b| (a - b) as f64);
        assert_eq!(vec![1,2,3,4,5,6,7,8], result);
    }

    #[test]
    fn floats() {
        let result = merge_sort(vec![9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0], |a, b| a - b);
        assert_eq!(vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0], result);
        let result = merge_sort(vec![8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0], |a, b| a - b);
        assert_eq!(vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0], result);
    }

    #[test]
    fn single_element() {
        assert_eq!(vec![0], merge_sort(vec![0], |_, _| 0.0))
    }

    #[test]
    fn structs() {
        #[derive(Debug, Clone, PartialEq)]
        struct TestStruct {
            f: f64
        }
        assert_eq!(
            vec![TestStruct{f: -0.5}, TestStruct{f: 0.5}],
            merge_sort(vec![TestStruct{f: 0.5}, TestStruct{f: -0.5}], |a, b| a.f - b.f)
        );
    }
}

pub fn merge_sort<T, F>(input: Vec<T>, compare: F) -> Vec<T>
    where F: Fn(T, T) -> f64,
          T: Clone
{
    let compare = Rc::new(Box::new(compare));
    merge_sort_internal(input, compare)
}

fn merge_sort_internal<T, F>(input: Vec<T>, compare: Rc<Box<F>>) -> Vec<T>
    where F: Fn(T, T) -> f64,
          T: Clone
{
    let len = input.len();
    if len == 1 {
        return input;
    }
    let half_len = len / 2;
    let mut left = Vec::with_capacity(half_len);
    left.extend_from_slice(&input[..half_len]);
    let mut right = Vec::with_capacity(len - half_len);
    right.extend_from_slice(&input[half_len..]);
    let left = merge_sort_internal(left, compare.clone());
    let right = merge_sort_internal(right, compare.clone());
    let mut result = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = 0;
    while i < half_len && j < len - half_len {
        match compare(left[i].clone(), right[j].clone()) {
            k if k == 0.0 => {
                result.push(left[i].clone());
                result.push(right[j].clone());
                i += 1;
                j += 1;
            },
            k if k > 0.0 => {
                result.push(right[j].clone());
                j += 1;
            },
            k if k < 0.0 => {
                result.push(left[i].clone());
                i += 1;
            },
            _ => ()
        }
        if i >= half_len {
            for k in j..(len - half_len) {
                result.push(right[k].clone());
            }
            break;
        }
        if j >= (len - half_len) {
            for k in i..half_len {
                result.push(left[k].clone());
            }
            break;
        }
    }
    result
}
