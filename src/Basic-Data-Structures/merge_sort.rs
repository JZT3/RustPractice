fn merge_sort(arr: &mut [i32], buffer: &mut [i32]) {
  let len = arr.len();
  if len <= 1 {
      return;
  }

  let mid = len / 2;
  
  // Sort both halves using buffer as temporary space
  merge_sort(&mut arr[..mid], &mut buffer[..mid]);
  merge_sort(&mut arr[mid..], &mut buffer[mid..]);

  // Merge both halves into buffer
  buffer.copy_from_slice(arr);
  
  let (left, right) = buffer.split_at(mid);
  let (mut i, mut j, mut k) = (0, 0, 0);

  while i < left.len() && j < right.len() {
      if left[i] <= right[j] {
          arr[k] = left[i];
          i += 1;
      } else {
          arr[k] = right[j];
          j += 1;
      }
      k += 1;
  }

  // Copy any remaining elements
  if i < left.len() {
      arr[k..].copy_from_slice(&left[i..]);
  } else if j < right.len() {
      arr[k..].copy_from_slice(&right[j..]);
  }
}

fn main() {
  let mut arr = [4, 1, 3, 9, 7];
  let mut buffer = arr.clone(); // Temporary buffer to avoid excessive allocations
  
  merge_sort(&mut arr, &mut buffer);
  println!("{:?}", arr);
}
