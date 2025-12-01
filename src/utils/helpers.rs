// Helper functions for address validation and formatting

pub fn format_address(address: &str, chain: &str) -> String {
    match chain {
        "ethereum" => {
            if address.starts_with("0x") {
                format!("0x{}", &address[2..].to_lowercase())
            } else {
                address.to_lowercase()
            }
        }
        "solana" => address.to_string(),
        _ => address.to_string(),
    }
}

pub fn truncate_address(address: &str, start: usize, end: usize) -> String {
    if address.len() <= start + end {
        return address.to_string();
    }
    format!("{}...{}", &address[..start], &address[address.len() - end..])
}

