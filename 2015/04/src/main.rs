fn main() {
    println!("Part One: {}", hash("yzbqklnj", 5));
    println!("Part Two: {}", hash("yzbqklnj", 6));
}

fn hash(input: &str, zeros: u8) -> i32 {
    let mut out = String::new();
    let mut i = 0;
    while !out.starts_with(&"0".repeat(zeros as usize)) {
        let digest = md5::compute(format!("{}{}", input, i));
        out = format!("{:x}", digest);
        i += 1;
    }
    i - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("abcdef", 5), 609043);
        assert_eq!(hash("pqrstuv", 5), 1048970);
    }
}
