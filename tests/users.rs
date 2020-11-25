#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::users::*;

    #[test]
    fn test_users() {
        let _ = get_users().unwrap();
    }
}
