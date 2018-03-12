static CHARSET: [char; 62] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn encode(v: u64) -> String {
    let base = 62;
    let mut n = v;
    let mut stack: Vec<char> = vec![];
    while n > 0 {
        let r = (n % base) as usize;
        n /= base;
        stack.push(CHARSET[r]);
    }

    if stack.len() == 0 {
        stack.push('0');
    }

    stack.reverse();
    return stack.into_iter().collect();
}

fn value(c: char) -> Option<usize> {
    return CHARSET.iter().position(|&x| x == c);
}

#[cfg(test)]
#[test]
fn test_encode() {
    assert!(encode(0) == "0");
    assert!(encode(34441886726) == "base62");
}

#[test]
fn test_value() {
    assert!(value('_') == None);
    assert!(value('0') == Some(0));
    assert!(value('A') == Some(10));
    assert!(value('a') == Some(36));
}