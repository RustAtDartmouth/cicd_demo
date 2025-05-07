// From Bogdan's outstanding "Let's Get Rusty" video on CI/CD
// https://www.youtube.com/watch?v=bw_kseQYxto
// 
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_with_negative_number() {
        assert_eq!(add(-1, -2), -3);
    }
}
