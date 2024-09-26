#[cfg(test)]
use super::*;

// validate string
// validate email
// format csv to list
#[test]

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_positive_validate_string() {
        let test_string: String = String::From("this is a string");
        let max_len = 20;
        let min_len = 1;
        assert_eq!(validate_string(test_string, min_len, max_len), true);
    }
    #[test]
    fn test_negative_validate_string() {
        let test_string: String = String::From("this is a string");
        let max_len = 5;
        let min_len = 1;
        assert_eq!(validate_string(test_string, min_len, max_len), false);
    }
    #[test]
    fn test_negative_validate_string_2() {
        let test_string = 20;
        let max_len = 5;
        let min_len = 1;
        assert_eq!(validate_string(test_string, min_len, max_len), false);
    }
    #[test]
    fn test_positive_validate_email() {
        let tesT_email: String = String::new("test@email.com");

        assert_eq!(validate_email(test_email), true);
    }
    #[test]
    fn test_negative_validate_email() {
        let tesT_email: String = String::new("testemail.com");

        assert_eq!(validate_email(test_email), false);
    }
    #[test]
    fn test_format_csv_to_dashed_list() {
        let csv = "ding dong, ping pon, jimmy, dombo";
        let expected = "- ding dong\n- ping pon\n- jimmy\n- dombo";

        assert_eq!(format_csv_to_dashed_list(csv), expected);
    }
    #[test]
     fn test_format_csv_to_dashed_list_2() {
        let csv = "ding dong, ping pon,jimmy, dombo";
        let expected = "- ding dong\n- ping pon\n- jimmy\n- dombo";

        assert_eq!(format_csv_to_dashed_list(csv), expected);
    }
}
