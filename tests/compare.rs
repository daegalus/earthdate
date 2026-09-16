use std::process::{Command, Output};

fn compare(left: &str, right: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_earthdate"))
        .args(["compare", left, right])
        .output()
        .expect("run earthdate compare")
}

#[test]
fn compares_release_tags_in_both_directions() {
    for (left, right, expected) in [
        ("26U30.427", "26S11.500", "-1\n"),
        ("26S11.500", "26U30.427", "1\n"),
        ("26S11.500", "26S11.500", "0\n"),
    ] {
        let output = compare(left, right);
        assert!(output.status.success(), "{:?}", output);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
    }
}

#[test]
fn compares_all_months_in_calendar_order() {
    for months in b"JFMAYULGSOND".windows(2) {
        let left = format!("26{}28.999", months[0] as char);
        let right = format!("26{}01.0", months[1] as char);
        let output = compare(&left, &right);
        assert!(output.status.success(), "{:?}", output);
        assert_eq!(output.stdout, b"-1\n");
    }
}

#[test]
fn compares_years_days_and_integer_beats() {
    for (left, right, expected) in [
        ("26D31.999", "27J01.0", "-1\n"),
        ("99D31.999", "100J01.0", "-1\n"),
        ("26S09.999", "26S10.0", "-1\n"),
        ("26S11.9", "26S11.10", "-1\n"),
        ("26S11.99", "26S11.100", "-1\n"),
        ("26S11.099", "26S11.99", "0\n"),
        ("24F29.999", "24M01.0", "-1\n"),
    ] {
        let output = compare(left, right);
        assert!(output.status.success(), "{:?}", output);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
    }
}

#[test]
fn rejects_invalid_dates_and_formats() {
    for invalid in [
        "",
        "26X11.500",
        "26s11.500",
        "26S1.500",
        "26S00.500",
        "26S31.500",
        "25F29.500",
        "100F29.500",
        "26S11.1000",
        "26S11.-1",
        "26S11.+1",
        "26S11.",
        "26S11.5.0",
        "v26S11.500",
        "26S11.500-beta",
        "26S11.500\n",
        "26S11.５００",
        "éS11.500",
        "9999999999999999S11.500",
    ] {
        for (left, right) in [(invalid, "26S11.500"), ("26S11.500", invalid)] {
            let output = compare(left, right);
            assert!(!output.status.success(), "accepted {invalid:?}");
            assert!(output.stdout.is_empty(), "{:?}", output);
            assert!(!output.stderr.is_empty(), "{:?}", output);
        }
    }
}

#[test]
fn requires_exactly_two_operands() {
    for args in [
        vec!["compare"],
        vec!["compare", "26S11.500"],
        vec!["compare", "26S11.500", "26S11.500", "26S11.500"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_earthdate"))
            .args(args)
            .output()
            .unwrap();
        assert!(!output.status.success(), "{:?}", output);
    }
}
