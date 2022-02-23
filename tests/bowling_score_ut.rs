#[cfg(test)]
mod tests {
    use bowling_score::Rolls;
    use bowling_score::report_score;

    #[test]
    fn test_report_score() {
        assert_eq!(report_score(Rolls{}), 0);
    }
}
