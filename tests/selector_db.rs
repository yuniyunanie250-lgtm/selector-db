use selector_db::{is_consistent, selector_for, selector_of_calldata, signatures_for, KNOWN};

#[test]
fn the_table_is_well_formed() {
    assert!(is_consistent());
    assert!(KNOWN.len() >= 20);
}

#[test]
fn the_table_is_sorted_for_reading() {
    // sorted by selector so a human can scan it; not required for correctness
    let sel: Vec<&str> = KNOWN.iter().map(|(s, _)| *s).collect();
    let mut sorted = sel.clone();
    sorted.sort();
    assert_eq!(sel, sorted);
}

#[test]
fn looks_up_the_erc20_core() {
    assert_eq!(
        signatures_for("0xa9059cbb"),
        vec!["transfer(address,uint256)"]
    );
    assert_eq!(signatures_for("0x70a08231"), vec!["balanceOf(address)"]);
    assert_eq!(
        signatures_for("0x095ea7b3"),
        vec!["approve(address,uint256)"]
    );
}

#[test]
fn reverses_a_signature_to_its_selector() {
    assert_eq!(
        selector_for("transfer(address,uint256)"),
        Some("0xa9059cbb")
    );
    assert_eq!(selector_for("nope()"), None);
}

#[test]
fn prefix_and_case_do_not_matter() {
    assert_eq!(signatures_for("a9059cbb"), signatures_for("0xA9059CBB"));
}

#[test]
fn unknown_selectors_yield_nothing_rather_than_guessing() {
    assert!(signatures_for("0xdeadbeef").is_empty());
}

#[test]
fn extracts_the_selector_from_calldata() {
    let cd = "0xa9059cbb000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045\
              0000000000000000000000000000000000000000000000000de0b6b3a7640000";
    assert_eq!(selector_of_calldata(cd).as_deref(), Some("0xa9059cbb"));
    assert_eq!(selector_of_calldata("0xa905").as_deref(), None);
}

#[test]
fn round_trip_through_both_directions() {
    for (sel, sig) in KNOWN {
        assert_eq!(selector_for(sig), Some(*sel), "signature {}", sig);
        assert!(
            signatures_for(sel).contains(sig),
            "selector {} should map back to {}",
            sel,
            sig
        );
    }
}
