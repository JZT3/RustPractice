fn hoare_partition(arr: &[i32]) -> (Vec<i32>, i32, Vec<i32>) {
  if arr.is_empty() {
      return (vec![], 0, vec![]);
  }
  let pivot = arr[0];
  let mut left = vec![];
  let mut right = vec![];
  
  for &val in &arr[1..] {
      if val < pivot {
          left.push(val);
      } else {
          right.push(val);
      }
  }
  
  (left, pivot, right)
}

fn quicksort(arr: Vec<i32>) -> Vec<i32> {
  if arr.len() <= 1 {
      return arr;
  }
  
  let mut result = vec![];
  let mut stack = vec![arr];
  
  while let Some(curr_arr) = stack.pop() {
      if curr_arr.len() <= 1 {
          result.extend(curr_arr);
          continue;
      }
      
      let (left, pivot, right) = hoare_partition(&curr_arr);
      
      stack.push(right);
      stack.push(vec![pivot]);
      stack.push(left);
  }
  
  result
}

fn main() {
  let arr = vec![5, 3, 8, 4, 2, 7, 1, 10];
  let sorted_arr = quicksort(arr);
  println!("{:?}", sorted_arr);
}