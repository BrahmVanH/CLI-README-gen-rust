// format csv to list

#[cfg(test)]
mod tests {
    use super::*;

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
