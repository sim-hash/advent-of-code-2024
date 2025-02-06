pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    let mut result = vec![];

    nums.iter().for_each(|&x| {
        let count = map.entry(x).or_insert(0);
        *count += 1;

//        if *count == k {
//            result.push(x)
//        }
    });


    println!("{:?}", map);


    result
}

#[cfg(test)]
mod tests {
    use crate::day1::k_freq_elem::top_k_frequent;

    #[test]
    fn top_k_freq_tests() {
        let ve = vec![1, 2];
        assert_eq!(top_k_frequent(ve, 2), vec![1, 2]);

        let ve = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(top_k_frequent(ve, 2), vec![1, 2]);

        let ve = vec![1];
        assert_eq!(top_k_frequent(ve, 1), vec![1]);

        let ve = vec![-1, -1, 1, 2, 2, 3];
        assert_eq!(top_k_frequent(ve, 2), vec![-1, 2]);
    }
}
