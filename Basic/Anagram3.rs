fn AnagramCount(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut count = [0; 25];

    for c in s1.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    for c in s2.chars() {
        count[(c as u8 - b'a') as usize] -= 1;
    }

    for i in count {
        if i != 0 {
            return false;
        }
    }

    true
}
