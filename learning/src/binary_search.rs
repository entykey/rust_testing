// https://dev-notes.eu/2020/03/Binary-Search-in-Rust/
// Binary search, iterative.
// Arguments: Array slice, array length, target integer to look up.

pub fn iterative(a: &[i32], len: usize, target_value: &i32) -> Option<usize> {
    let mut low: i8 = 0;
    let mut high: i8 = len as i8 - 1;

    while low <= high {
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val = &a[mid_index];

        if val == target_value {
            return Some(mid_index);
        }

        // Search values that are greater than val - to right of current mid_index
        if val < target_value {
            low = mid + 1;
        }

        // Search values that are less than val - to the left of current mid_index
        if val > target_value {
            high = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_iterative() {
        let correct_arr = [
		1, 10, 20, 47, 59, 63, 75, 88, 99,
		107, 120, 133, 155, 162, 176, 188,
		199, 200, 210, 222
		];
        for i in 0..correct_arr.len() {
            assert_eq!(i, iterative(&correct_arr, correct_arr.len(), &correct_arr[i]).unwrap());
        }
    }
    #[test]
    fn incorrect_iterative() {
        let searched_arr = [
		1, 10, 20, 47, 59, 63, 75, 88, 99,
		107, 120, 133, 155, 162, 176, 188,
		199, 200, 210, 222
		];
        let incorrect_arr = [
		2, 22, 48, 58, 61, 73, 84, 90, 100,
		119, 132, 154, 160, 177, 187, 197,
		201, 211, 2242
		];
        for i in 0..incorrect_arr.len() {
            assert_eq!(None, iterative(&searched_arr, searched_arr.len(), &incorrect_arr[i]));
        }
    }
}

fn main() {
    let main_time = std::time::Instant::now();


    let arr = [
		1, 10, 20, 47, 59, 63, 75, 88, 99,
		107, 120, 133, 155, 162, 176, 188,
		199, 200, 210, 222
		];
    let target: i32 = 47;
    if let Some(result) = iterative(&arr, arr.len(), &target) {
        println!("{} found at index {}", target, result);
    } else {
        println!("{} not found.", target);
    }

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    let elapsed_seconds = elapsed_ms / 1000.0; // Convert milliseconds to seconds
    println!("\nExecution time: {:?} ({:?} ms) ({:.8} s)", duration, elapsed_ms, elapsed_seconds);
}
