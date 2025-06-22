use std::time::Duration;

pub fn should_show_blinking_lines(countdown: Duration) -> bool {
    countdown.as_millis() % 400 > 200
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Duration::from_millis(1000), false)]
    #[case(Duration::from_millis(800), false)]
    #[case(Duration::from_millis(0), false)]
    #[case(Duration::from_millis(999), false)]
    #[case(Duration::from_millis(801), false)]
    #[case(Duration::from_millis(601), true)]
    #[case(Duration::from_millis(199), false)]
    fn should_show_blinking_lines_returns_correct_state(
        #[case] countdown: Duration,
        #[case] expected: bool,
    ) {
        let result = should_show_blinking_lines(countdown);
        assert_eq!(result, expected);
    }
}
