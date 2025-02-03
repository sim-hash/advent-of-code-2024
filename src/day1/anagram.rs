pub fn is_anagram_unicode(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map = std::collections::HashMap::new();
    s.chars().for_each(|x| *map.entry(x).or_insert(0) += 1);
    t.chars().for_each(|x| *map.entry(x).or_insert(0) -= 1);

    !map.into_values().any(|x| x != 0)
}

pub fn is_anagram_byte(s: String, t: String) -> bool {
    let n = s.len();
    let m = t.len();
    if n != m {
        return false;
    }
    let (s, t) = (s.as_bytes(), t.as_bytes());
    let mut count = [0; 26];
    for i in 0..n {
        count[(s[i] - b'a') as usize] += 1;
        count[(t[i] - b'a') as usize] -= 1;
    }
    count.iter().all(|&c| c == 0)
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut holder = [0; 36];
    for ch in t.chars() {
        let val: usize = ch.to_digit(36).unwrap() as usize;
        holder[val] = holder[val] + 1;
    }

    for ch in s.chars() {
        let val: usize = ch.to_digit(36).unwrap() as usize; holder[val] = holder[val] - 1;
        if holder[val] < 0 {
            return false;
        }
    }

    if holder.iter().all(|&x| x == 0) {
        return true
    }

    return false
}

pub fn is_anagram_sort(s: String, t:String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut t_vec: Vec<char> = t.chars().collect();
    let mut s_vec: Vec<char> = s.chars().collect();

    t_vec.sort();
    s_vec.sort();

    t_vec == s_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram_unicode() {

        let s = String::from("anagram");
        let t = String::from("nagrama");

        assert_eq!(is_anagram_unicode(s, t), true);
    }

    #[test]
    fn test_is_anagram_byte() {

        let s = String::from("anagram");
        let t = String::from("nagrama");

        assert_eq!(is_anagram_byte(s, t), true);
    }

    #[test]
    fn test_is_anagram() {

        let s = String::from("anagram");
        let t = String::from("nagrama");

        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn test_is_anangram_sort() {
        let s = String::from("anagram");
        let t = String::from("nagrama");

        assert_eq!(is_anagram_sort(s, t), true);
    }
}
