fn hoare_partition(mut arr: Vec<i32>) -> (Vec<i32>, i32, Vec<i32>) {
  if arr.is_empty() {
      return (vec![], 0, vec![]);
  }

  let pivot = arr[0]; // Choose the first element as the pivot
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

fn quicksort(mut arr: Vec<i32>) -> Vec<i32> {
  if arr.len() <= 1 {
      return arr;
  }

  let mut stack = vec![arr]; // Stack for iterative processing
  let mut sorted = vec![];

  while let Some(curr_arr) = stack.pop() {
      if curr_arr.len() <= 1 {
          sorted.extend(curr_arr);
          continue;
      }

      let (left, pivot, right) = hoare_partition(curr_arr);
      sorted.push(pivot); // Add pivot to sorted list

      if !right.is_empty() {
          stack.push(right);
      }
      if !left.is_empty() {
          stack.push(left);
      }
  }

  sorted
}

fn main() {
  let arr = vec![5, 3, 8, 4, 2, 7, 1, 10];
  let sorted_arr = quicksort(arr);

  println!("{:?}", sorted_arr);
}
