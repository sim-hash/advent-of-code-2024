pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result = std::collections::HashMap::new();

    for str in strs {
        let mut ch: Vec<char> = str.chars().collect();
        ch.sort();
        result.entry(ch).or_insert(vec![]).push(str);
    }

    result.values().cloned().collect()
}


fn is_anagram(s: &str, t: &str) -> bool {
    let mut map = std::collections::HashMap::new();
    
    s.chars().for_each(|x| *map.entry(x).or_insert(0) += 1);
    t.chars().for_each(|x| *map.entry(x).or_insert(0) -= 1);

    return map.into_values().all(|x| x == 0);
}

#[cfg(test)]
mod tests {
    use crate::day1::group_anagrams::group_anagrams;


#[test]
    fn test_group_anagrams() {
        let strs = vec![String::from("")];
        assert_eq!(group_anagrams(strs), vec![vec![String::from("")]]);

        let strs = vec![String::from("a")];
        assert_eq!(group_anagrams(strs), vec![vec![String::from("a")]]);

        let strs = vec![String::from("eat"), String::from("tea"), String::from("tan"), String::from("ate"), String::from("nat"), String::from("bat")];
        assert_eq!(group_anagrams(strs), vec![vec![String::from("bat")], vec![String::from("nat"), String::from("tan")], vec![String::from("ate"), String::from("eat"), String::from("tea")]]);
    }
}
