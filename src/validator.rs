use email_address::*;

pub fn validate_email(email: &str) -> bool {
    EmailAddress::is_valid(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_validate_email() {
        let test_email = "test@email.com";

        assert_eq!(validate_email(test_email), true);
    }
    #[test]
    fn test_negative_validate_email() {
        let test_email = "testemail.com";

        assert_eq!(validate_email(test_email), false);
    }
}
