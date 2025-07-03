mod common;

use common::*;
use indoc::indoc;

#[test]
fn test_match_traversal_pattern() {
    // Test the new -> traversal syntax
    let alice_envelope =
        run_cli_stdin(&["subject", "type", "string"], r#""Alice""#).unwrap();

    let alice_with_assertion = run_cli_stdin(
        &[
            "assertion",
            "add",
            "pred-obj",
            "string",
            "isA",
            "string",
            "Person",
        ],
        &alice_envelope,
    )
    .unwrap();

    // Test matching assertion predicate with new traversal syntax
    let match_result = run_cli_stdin(
        &["match", r#"node -> assertpred("isA")"#],
        &alice_with_assertion,
    )
    .unwrap();

    let expected = indoc! {r#"
        ea3bd24e NODE ""Alice"" [ "isA": "Person" ]
            242b24ff ASSERTION "isA": "Person"
    "#}.trim();
    assert_actual_expected!(match_result, expected);

    // Test matching assertion object with new traversal syntax
    let match_obj_result = run_cli_stdin(
        &["match", r#"node -> assertobj("Person")"#],
        &alice_with_assertion,
    )
    .unwrap();

    let expected_obj = indoc! {r#"
        ea3bd24e NODE ""Alice"" [ "isA": "Person" ]
            242b24ff ASSERTION "isA": "Person"
    "#}.trim();
    assert_actual_expected!(match_obj_result, expected_obj);

    // Test deeper traversal pattern
    let deep_match_result = run_cli_stdin(
        &[
            "match",
            r#"node -> assertpred("isA") -> obj("Person")"#,
        ],
        &alice_with_assertion,
    )
    .unwrap();
    let expected_deep = indoc! {r#"
    ea3bd24e NODE ""Alice"" [ "isA": "Person" ]
        242b24ff ASSERTION "isA": "Person"
            bd52917f LEAF "Person"
    "#}.trim();
    assert_actual_expected!(deep_match_result, expected_deep);
}

#[test]
fn test_match_numeric_comparison() {
    // Test that > still works for numeric comparisons
    let number_envelope =
        run_cli_stdin(&["subject", "type", "number"], "42").unwrap();

    let match_result =
        run_cli_stdin(&["match", ">40"], &number_envelope).unwrap();

    assert!(match_result.contains("42"));

    // Test that < also works
    let match_less_result =
        run_cli_stdin(&["match", "<50"], &number_envelope).unwrap();

    assert!(match_less_result.contains("42"));
}
