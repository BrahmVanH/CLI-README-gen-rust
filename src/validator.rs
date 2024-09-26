
use email_address::*;
use std::str::FromStr;


// This will take in a String and a max and mix usize
pub fn validate_string(str: &str, min_len: usize, max_len: usize) -> bool {
    min_len < str.len() && str.len() < max_len
}

pub fn validate_email(str: &str) {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_positive_validate_string() {
        let test_string = "this is a string";
        let max_len = 25;
        let min_len = 1;
        assert_eq!(validate_string(test_string, min_len, max_len), true);
    }
    #[test]
    fn test_negative_validate_string() {
        let test_string = "this is a string";
        let max_len = 5;
        let min_len = 1;
        assert_eq!(validate_string(test_string, min_len, max_len), false);
    }
    #[test]
    fn test_negative_validate_string_2() {
        let test_string = "bantongloy";
        let max_len = 5;
        let min_len = 10;
        assert_eq!(validate_string(test_string, min_len, max_len), false);
    }
    // #[test]
    // fn test_positive_validate_email() {
    //     let tesT_email: String = String::new("test@email.com");

    //     assert_eq!(validate_email(test_email), true);
    // }
    // #[test]
    // fn test_negative_validate_email() {
    //     let tesT_email: String = String::new("testemail.com");

    //     assert_eq!(validate_email(test_email), false);
    // }
}
