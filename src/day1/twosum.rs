use std::{collections::HashMap, usize};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
     let mut map = std::collections::HashMap::new();

     for (i, el) in nums.iter().enumerate() {
         let complement = target - el;
         if map.contains_key(&complement) {
             return vec![*map.get(&complement).unwrap(), i as i32];
         }
         map.entry(el).or_insert(i as i32);
     }
     return vec![0, 0];
}

pub fn two_sum_two_pointers(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        if left == right {
            left = 0;
            right -= 1;
        }

        if nums[left] + nums[right] == target {
            return vec![left as i32, right as i32];
        }

        left += 1;
    }

    return vec![0, 0];
}

pub fn two_sum_ref(nums: Vec<i32>, target: i32) -> Vec<i32> {
     let mut map = std::collections::HashMap::new();

     for (i, &el) in nums.iter().enumerate() {
         let complement = target - el;
         if let Some(&index) = map.get(&complement) {
             return vec![index as i32, i as i32];
         }
         map.insert(el, i);
     }
     return vec![];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let ve = vec![2, 7, 11, 15];
        assert_eq!(two_sum(ve, 9), vec![0, 1]);

        let ve = vec![3, 3];
        assert_eq!(two_sum(ve, 6), vec![0, 1]);

        let ve = vec![3, 2, 4];
        assert_eq!(two_sum(ve, 6), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_pointers() {
        let ve = vec![2, 7, 11, 15];
        assert_eq!(two_sum_two_pointers(ve, 9), vec![0, 1]);

        let ve = vec![3, 3];
        assert_eq!(two_sum_two_pointers(ve, 6), vec![0, 1]);

        let ve = vec![3, 2, 4];
        assert_eq!(two_sum_two_pointers(ve, 6), vec![1, 2]);
    }
}
