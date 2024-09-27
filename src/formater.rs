//  This will format a user=entered csv string to a formatted bulleted list string
pub fn format_csv_to_dashed_list(csv: &str) -> String {
    let values: Vec<String> = csv
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let result: Vec<String> = values
        .into_iter()
        .map(|value| format!("- {}", value.trim()))
        .collect();

    result.join("\n")
}

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
