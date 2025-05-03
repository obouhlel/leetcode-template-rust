pub struct Solution;

impl Solution {
    pub fn change_me() -> &'static str {
        "Hello, world !"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = Solution::change_me();
        assert_eq!(result, "Hello, world !");
    }
}
