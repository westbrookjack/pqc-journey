// sorts a reference to a vector with entries of type T that are both clonable and ordered with the merge sort algorithm
pub fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {

    if arr.len()<=1 {
        return arr.to_vec();
    }
        let mid = arr.len()/2;
        let left = &arr[..mid];
        let right = &arr[mid..];
        let sorted_left = merge_sort(left);
        let sorted_right = merge_sort(right);
        merge(&sorted_left, &sorted_right)

}
//takes two sorted vectors and merges them into one larger sorted vector
fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut out: Vec<T> = Vec::with_capacity(left.len() + right.len());
    let mut i:usize = 0;
    let mut j:usize = 0;
    let llen = left.len();
    let rlen = right.len();
    while i<llen && j<rlen {
        if left[i]< right[j] {
            out.push(left[i].clone());
            i+=1;
        }
        else {
            out.push(right[j].clone());
            j+=1;
        }
    }
    // Append any remaining elements from left or right
    out.extend_from_slice(&left[i..]);
    out.extend_from_slice(&right[j..]);

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_integers() {
        let input = vec![5, 2, 9, 1];
        let result = merge_sort(&input);
        assert_eq!(result, vec![1, 2, 5, 9]);
    }
    
    #[test]
    fn test_merge_sort_odd_num_ints() {
        let input = vec![5,-1,3,0,11];
        let result = merge_sort(&input);
        assert_eq!(result, vec![-1,0,3,5,11]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let input: Vec<i32> = vec![];
        let result = merge_sort(&input);
        assert_eq!(result, vec![]);
    }
}
