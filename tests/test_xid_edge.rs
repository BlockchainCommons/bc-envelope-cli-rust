mod common;
use common::*;

use anyhow::Result;

fn make_edge(subject: &str, is_a: &str, source_xid: &str, target_xid: &str) -> Result<String> {
    let edge = run_cli(&["subject", "type", "string", subject])?;
    let edge = run_cli(&["assertion", "add", "pred-obj", "known", "isA", "string", is_a, &edge])?;
    let edge = run_cli(&["assertion", "add", "pred-obj", "known", "source", "ur", source_xid, &edge])?;
    let edge = run_cli(&["assertion", "add", "pred-obj", "known", "target", "ur", target_xid, &edge])?;
    Ok(edge)
}

fn make_xid_doc() -> Result<String> {
    run_cli(&["xid", "new", ALICE_PUBKEYS])
}

fn make_xid_doc_with_private_keys() -> Result<String> {
    run_cli(&["xid", "new", ALICE_PRVKEYS])
}

fn alice_xid(xid_doc: &str) -> Result<String> {
    run_cli(&["xid", "id", xid_doc])
}

fn bob_xid() -> Result<String> {
    let bob_doc = run_cli(&["xid", "new", BOB_PUBKEYS])?;
    run_cli(&["xid", "id", &bob_doc])
}

#[test]
fn test_xid_edge_count_empty() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    run_cli_expect(&["xid", "edge", "count", &xid_doc], "0")?;
    Ok(())
}

#[test]
fn test_xid_edge_add_unsigned() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge = make_edge("credential-1", "foaf:Person", &xid, &xid)?;

    let xid_doc = run_cli(&["xid", "edge", "add", &edge, &xid_doc])?;
    run_cli_expect(&["xid", "edge", "count", &xid_doc], "1")?;
    Ok(())
}

#[test]
fn test_xid_edge_add_with_signing() -> Result<()> {
    let xid_doc = make_xid_doc_with_private_keys()?;
    let xid = alice_xid(&xid_doc)?;

    let edge = make_edge("credential-1", "foaf:Person", &xid, &xid)?;

    let xid_doc = run_cli(&[
        "xid", "edge", "add", &edge,
        "--sign", "inception",
        &xid_doc,
    ])?;

    run_cli_expect(&["xid", "edge", "count", &xid_doc], "1")?;

    // Verify signature is valid
    run_cli(&["xid", "id", "--verify", "inception", &xid_doc])?;
    Ok(())
}

#[test]
fn test_xid_edge_all() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge1 = make_edge("edge-1", "foaf:Person", &xid, &xid)?;
    let edge2 = make_edge("edge-2", "schema:Thing", &xid, &xid)?;

    let xid_doc = run_cli(&["xid", "edge", "add", &edge1, &xid_doc])?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge2, &xid_doc])?;

    run_cli_expect(&["xid", "edge", "count", &xid_doc], "2")?;

    let all = run_cli(&["xid", "edge", "all", &xid_doc])?;
    let lines: Vec<&str> = all.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);
    Ok(())
}

#[test]
fn test_xid_edge_at() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge1 = make_edge("edge-1", "foaf:Person", &xid, &xid)?;
    let edge2 = make_edge("edge-2", "schema:Thing", &xid, &xid)?;

    let xid_doc = run_cli(&["xid", "edge", "add", &edge1, &xid_doc])?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge2, &xid_doc])?;

    // Should be able to get edge at index 0 and 1
    let at0 = run_cli(&["xid", "edge", "at", "0", &xid_doc])?;
    assert!(!at0.is_empty());

    let at1 = run_cli(&["xid", "edge", "at", "1", &xid_doc])?;
    assert!(!at1.is_empty());

    // Index 2 should fail
    assert!(run_cli(&["xid", "edge", "at", "2", &xid_doc]).is_err());
    Ok(())
}

#[test]
fn test_xid_edge_remove() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge = make_edge("credential-1", "foaf:Person", &xid, &xid)?;

    let xid_doc = run_cli(&["xid", "edge", "add", &edge, &xid_doc])?;
    run_cli_expect(&["xid", "edge", "count", &xid_doc], "1")?;

    let xid_doc = run_cli(&["xid", "edge", "remove", &edge, &xid_doc])?;
    run_cli_expect(&["xid", "edge", "count", &xid_doc], "0")?;
    Ok(())
}

#[test]
fn test_xid_edge_remove_with_signing() -> Result<()> {
    let xid_doc = make_xid_doc_with_private_keys()?;
    let xid = alice_xid(&xid_doc)?;

    let edge = make_edge("credential-1", "foaf:Person", &xid, &xid)?;

    let xid_doc = run_cli(&[
        "xid", "edge", "add", &edge,
        "--sign", "inception",
        &xid_doc,
    ])?;

    let xid_doc = run_cli(&[
        "xid", "edge", "remove", &edge,
        "--verify", "inception",
        "--sign", "inception",
        &xid_doc,
    ])?;

    run_cli_expect(&["xid", "edge", "count", &xid_doc], "0")?;
    run_cli(&["xid", "id", "--verify", "inception", &xid_doc])?;
    Ok(())
}

#[test]
fn test_xid_edge_find_by_is_a() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge1 = make_edge("edge-1", "foaf:Person", &xid, &xid)?;
    let edge2 = make_edge("edge-2", "schema:Thing", &xid, &xid)?;

    let xid_doc = run_cli(&["xid", "edge", "add", &edge1, &xid_doc])?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge2, &xid_doc])?;

    let is_a_env = run_cli(&["subject", "type", "string", "foaf:Person"])?;
    let found = run_cli(&["xid", "edge", "find", "--is-a", &is_a_env, &xid_doc])?;
    let lines: Vec<&str> = found.trim().split('\n').collect();
    assert_eq!(lines.len(), 1);
    Ok(())
}

#[test]
fn test_xid_edge_find_by_target() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;
    let bob = bob_xid()?;

    let edge1 = make_edge("self-desc", "foaf:Person", &xid, &xid)?;
    let edge2 = make_edge("knows-bob", "schema:colleague", &xid, &bob)?;

    let xid_doc = run_cli(&["xid", "edge", "add", &edge1, &xid_doc])?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge2, &xid_doc])?;

    // Find by Bob's target
    let target_env = run_cli(&["subject", "type", "ur", &bob])?;
    let found = run_cli(&["xid", "edge", "find", "--target", &target_env, &xid_doc])?;
    let lines: Vec<&str> = found.trim().split('\n').collect();
    assert_eq!(lines.len(), 1);
    Ok(())
}

#[test]
fn test_xid_edge_find_by_subject() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge1 = make_edge("self-desc", "foaf:Person", &xid, &xid)?;
    let edge2 = make_edge("knows-bob", "schema:colleague", &xid, &xid)?;

    let xid_doc = run_cli(&["xid", "edge", "add", &edge1, &xid_doc])?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge2, &xid_doc])?;

    let subj_env = run_cli(&["subject", "type", "string", "self-desc"])?;
    let found = run_cli(&["xid", "edge", "find", "--subject", &subj_env, &xid_doc])?;
    let lines: Vec<&str> = found.trim().split('\n').collect();
    assert_eq!(lines.len(), 1);
    Ok(())
}

#[test]
fn test_xid_edge_find_combined() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge1 = make_edge("self-desc", "foaf:Person", &xid, &xid)?;
    let edge2 = make_edge("self-thing", "foaf:Person", &xid, &xid)?;
    let edge3 = make_edge("other", "schema:Thing", &xid, &xid)?;

    let xid_doc = run_cli(&["xid", "edge", "add", &edge1, &xid_doc])?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge2, &xid_doc])?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge3, &xid_doc])?;

    // Find by isA "foaf:Person" AND subject "self-desc"
    let is_a_env = run_cli(&["subject", "type", "string", "foaf:Person"])?;
    let subj_env = run_cli(&["subject", "type", "string", "self-desc"])?;
    let found = run_cli(&[
        "xid", "edge", "find",
        "--is-a", &is_a_env,
        "--subject", &subj_env,
        &xid_doc,
    ])?;
    let lines: Vec<&str> = found.trim().split('\n').collect();
    assert_eq!(lines.len(), 1);
    Ok(())
}

#[test]
fn test_xid_edge_format() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge = make_edge("credential-1", "foaf:Person", &xid, &xid)?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge, &xid_doc])?;

    let formatted = run_cli(&["format", &xid_doc])?;
    assert!(formatted.contains("'edge'"));
    assert!(formatted.contains("'isA'"));
    assert!(formatted.contains("'source'"));
    assert!(formatted.contains("'target'"));
    assert!(formatted.contains("\"credential-1\""));
    assert!(formatted.contains("\"foaf:Person\""));
    Ok(())
}

#[test]
fn test_xid_edge_persists_across_operations() -> Result<()> {
    let xid_doc = make_xid_doc()?;
    let xid = alice_xid(&xid_doc)?;

    let edge = make_edge("credential-1", "foaf:Person", &xid, &xid)?;
    let xid_doc = run_cli(&["xid", "edge", "add", &edge, &xid_doc])?;
    run_cli_expect(&["xid", "edge", "count", &xid_doc], "1")?;

    // Add a resolution method — edge should survive
    let xid_doc = run_cli(&[
        "xid", "method", "add", "https://example.com/resolve", &xid_doc,
    ])?;
    run_cli_expect(&["xid", "edge", "count", &xid_doc], "1")?;

    Ok(())
}

#[test]
fn test_xid_edge_with_signed_xid() -> Result<()> {
    let xid_doc = make_xid_doc_with_private_keys()?;
    let xid = alice_xid(&xid_doc)?;

    let edge = make_edge("credential-1", "foaf:Person", &xid, &xid)?;

    // Add edge with verify + sign
    let xid_doc = run_cli(&[
        "xid", "edge", "add", &edge,
        "--sign", "inception",
        &xid_doc,
    ])?;

    // Verify inception
    run_cli(&["xid", "id", "--verify", "inception", &xid_doc])?;
    run_cli_expect(&["xid", "edge", "count", &xid_doc], "1")?;

    // Add another operation with verify + sign, edge should persist
    let xid_doc = run_cli(&[
        "xid", "method", "add", "https://example.com/resolve",
        "--verify", "inception",
        "--sign", "inception",
        &xid_doc,
    ])?;

    run_cli(&["xid", "id", "--verify", "inception", &xid_doc])?;
    run_cli_expect(&["xid", "edge", "count", &xid_doc], "1")?;
    Ok(())
}
