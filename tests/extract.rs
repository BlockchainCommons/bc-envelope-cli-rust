mod common;
use common::*;

const ALICE_KNOWS_BOB: &str = "ur:envelope/lftpcsihfpjziniaihoytpcsihjejtjlktjktpcsiafwjliddssngwct";

#[test]
fn test_extract_arid() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "arid", "ur:envelope/tpcstansgshdcxpfflioesssvlnlrhfrgtrsnndslpcacmcwdmfscarpoygwinhekkdtgdonkpahrofnvaisih"],
        "ur:arid/hdcxpfflioesssvlnlrhfrgtrsnndslpcacmcwdmfscarpoygwinhekkdtgdonkpahrozmcmutis"
    )
}

#[test]
fn test_extract_assertion() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_cbor() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_data() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_date() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_digest() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_envelope() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "envelope", ALICE_KNOWS_BOB],
        "ur:envelope/tpcsihfpjziniaihnsrsnyue"
    )
}

#[test]
fn test_extract_known() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_number() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_object() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_predicate() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_string() -> anyhow::Result<()> {
    run_cli_expect(
        &["extract", "string", ALICE_KNOWS_BOB],
        "Alice"
    )
}

#[test]
fn test_extract_ur() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_uri() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_uuid() -> anyhow::Result<()> {
    todo!()
}

#[test]
fn test_extract_wrapped() -> anyhow::Result<()> {
    todo!()
}
