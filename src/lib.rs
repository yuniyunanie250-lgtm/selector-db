//! A registry of 4-byte function selectors, searched in both directions.
//!
//! Given a selector, find candidate signatures; given a signature, compute its
//! selector. Selectors collide (different signatures can share four bytes), so
//! the lookup returns every candidate rather than pretending the mapping is
//! one-to-one -- that is exactly the ambiguity 4byte.directory exists to expose.

/// A small curated table. Entries are (selector, signature).
pub const KNOWN: &[(&str, &str)] = &[
    ("0x01ffc9a7", "supportsInterface(bytes4)"),
    ("0x06fdde03", "name()"),
    ("0x095ea7b3", "approve(address,uint256)"),
    (
        "0x150b7a02",
        "onERC721Received(address,address,uint256,bytes)",
    ),
    ("0x18160ddd", "totalSupply()"),
    ("0x23b872dd", "transferFrom(address,address,uint256)"),
    ("0x2e1a7d4d", "withdraw(uint256)"),
    ("0x313ce567", "decimals()"),
    ("0x3ccfd60b", "withdraw()"),
    ("0x42842e0e", "safeTransferFrom(address,address,uint256)"),
    ("0x6352211e", "ownerOf(uint256)"),
    ("0x70a08231", "balanceOf(address)"),
    ("0x8da5cb5b", "owner()"),
    ("0x95d89b41", "symbol()"),
    ("0xa22cb465", "setApprovalForAll(address,bool)"),
    ("0xa9059cbb", "transfer(address,uint256)"),
    ("0xd0e30db0", "deposit()"),
    ("0xdd62ed3e", "allowance(address,address)"),
    ("0xe8a3d485", "contractURI()"),
    (
        "0xf23a6e61",
        "onERC1155Received(address,address,uint256,uint256,bytes)",
    ),
];

fn normalise(selector: &str) -> String {
    let s = selector
        .strip_prefix("0x")
        .or_else(|| selector.strip_prefix("0X"))
        .unwrap_or(selector);
    format!("0x{}", s.to_ascii_lowercase())
}

/// Every signature in the table sharing this selector. Empty when unknown.
pub fn signatures_for(selector: &str) -> Vec<&'static str> {
    let want = normalise(selector);
    KNOWN
        .iter()
        .filter(|(sel, _)| *sel == want)
        .map(|(_, sig)| *sig)
        .collect()
}

/// The selector recorded for a signature, if the table has it.
pub fn selector_for(signature: &str) -> Option<&'static str> {
    KNOWN
        .iter()
        .find(|(_, sig)| *sig == signature)
        .map(|(sel, _)| *sel)
}

/// Pull the selector out of a calldata blob, if it is long enough to have one.
pub fn selector_of_calldata(calldata: &str) -> Option<String> {
    let s = calldata
        .strip_prefix("0x")
        .or_else(|| calldata.strip_prefix("0X"))
        .unwrap_or(calldata);
    if s.len() < 8 {
        return None;
    }
    Some(normalise(&s[..8]))
}

/// Is the table internally consistent (no selector pointing at a duplicate
/// signature, and every entry well formed)?
pub fn is_consistent() -> bool {
    let mut seen = Vec::new();
    for (sel, sig) in KNOWN {
        if sel.len() != 10 || !sel.starts_with("0x") || sig.is_empty() {
            return false;
        }
        if seen.contains(sig) {
            return false;
        }
        seen.push(*sig);
    }
    true
}
