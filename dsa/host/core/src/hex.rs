//! Bytes as lower-case hexadecimal text, the form hashes and keys are written in.

/// `bytes` as lower-case hexadecimal, two digits each.
pub fn encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::encode;

    #[test]
    fn d9_bytes_are_written_as_two_lower_case_digits_each() {
        assert_eq!(encode(&[0x00, 0x0f, 0xa5, 0xff]), "000fa5ff");
        assert_eq!(encode(&[]), "");
    }
}
