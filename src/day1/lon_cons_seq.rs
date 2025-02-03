use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut vals = nums;
    vals.sort();

    let mut result = 0;
    let mut holder = 1;
    for window in vals.windows(2) {
        if window[0] == window[1] {
            continue;
        }

        if (window[0] - window[1]).abs() == 1 {
            holder += 1;
        } else {
            result = std::cmp::max(holder, result);
            holder = 1
        }
    }

    std::cmp::max(holder, result)
}

pub fn longest_consecutive_sec(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    
    let vals: HashSet<i32> = HashSet::from_iter(nums);
    let mut best = 0;

    for &val in &vals {
        let x = val - 1;
        if !vals.contains(&x) {
            let mut y = val + 1;
            while vals.contains(&y) {
                y += 1;
            }
            println!("Y {:?}", y);
            println!("Val {:?}", val);
            best = std::cmp::max(best, y - val);
        }
    }

    best 
}

#[cfg(test)]
mod tests {
    use crate::day1::lon_cons_seq::{longest_consecutive, longest_consecutive_sec};

    #[test]
    fn test_longest_consecutive_sec() {
        let input = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive_sec(input), 4);
        println!("===============================================");

        let input = vec![100, 4, 200, 1, 3, -2, -10, - 102, -1];
        assert_eq!(longest_consecutive_sec(input), 2);
        println!("===============================================");

        let input = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive_sec(input), 9);

    }

    
    #[test]
    fn test_longest_consecutive() {

        let input = vec![1, 2, 0, 1];
        assert_eq!(longest_consecutive(input), 3);
        println!("===============================================");

        let input = vec![];
        assert_eq!(longest_consecutive(input), 0);
        println!("===============================================");

        let input = vec![1];
        assert_eq!(longest_consecutive(input), 1);
        println!("===============================================");

        let input = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(input), 4);
        println!("===============================================");

        let input = vec![100, 4, 200, 1, 3, -2, -10, - 102, -1];
        assert_eq!(longest_consecutive(input), 2);
        println!("===============================================");

        let input = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive(input), 9);

    }
}
