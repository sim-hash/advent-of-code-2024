use std::collections::HashMap;
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return false;
    }

    let mut holder = HashMap::new();

    for num in nums {
        if holder.get(&num).is_some() {
            return true;
        }
        holder.insert(num, 1);
    }
    return false;
}

pub fn contains_duplicate_set(nums: Vec<i32>) -> bool {
    println!("{:?}", nums);
    let mut holder: HashSet<i32> = HashSet::with_capacity(nums.len());

    for num in nums {
        if !holder.insert(num) {
            return true;
        }
    }

    //println!("{:?}", nums);
    false

}

pub fn contains_duplicate_set_2(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}

pub fn contains_duplicate_hash_map(nums: Vec<i32>) -> bool {
    let mut occurs = HashMap::new();

    for n in nums {
        match occurs.insert(n, n) {
            Some(x) => return true,
            None    => {}
        }
    }
    false
}
