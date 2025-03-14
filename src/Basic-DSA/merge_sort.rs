fn merge_sort(arr: &mut [i32], buffer: &mut [i32]) {
  let n = arr.len();
  
  // Start with subarrays of size 1 and merge into increasingly larger subarrays
  let mut width = 1;
  while width < n {
      // Process the array in blocks of size 2*width
      let mut left = 0;
      while left < n {
          // Calculate middle and right boundaries for the current merge
          let mid = (left + width).min(n);
          let right = (left + 2 * width).min(n);
          
          // Only merge if there are elements in both subarrays
          if mid < right {
              // Copy the portion we're about to merge into buffer
              buffer[left..right].copy_from_slice(&arr[left..right]);
              
              // Merge from buffer back into arr
              let mut i = left;      // Index in left subarray (in buffer)
              let mut j = mid;       // Index in right subarray (in buffer)
              let mut k = left;      // Index in destination array
              
              while i < mid && j < right {
                  if buffer[i] <= buffer[j] {
                      arr[k] = buffer[i];
                      i += 1;
                  } else {
                      arr[k] = buffer[j];
                      j += 1;
                  }
                  k += 1;
              }
              
              // Copy remaining elements from either subarray
              while i < mid {
                  arr[k] = buffer[i];
                  i += 1;
                  k += 1;
              }
              
              while j < right {
                  arr[k] = buffer[j];
                  j += 1;
                  k += 1;
              }
          }
          
          // Move to next pair of subarrays
          left += 2 * width;
      }
      
      // Double the width for the next iteration
      width *= 2;
  }
}

fn main() {
  let mut arr = [9, 4, 1, 6, 7, 3, 8, 2, 5];
  let mut buffer = vec![0; arr.len()]; // Pre-allocate buffer of same size
  
  merge_sort(&mut arr, &mut buffer.as_mut_slice());
  println!("{:?}", arr);
}