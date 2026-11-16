//! opendesk-proto crate.

#[cfg(test)]
mod tests {
    #[test]
    fn protocol_crate_is_included_in_the_workspace() {
        assert_eq!(env!("CARGO_PKG_NAME"), "opendesk-proto");
    }
}
