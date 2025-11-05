#[allow(dead_code)]
fn testable() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use crate::testable;

    #[test]
    fn temp() {
        assert!(testable());
    }
}
