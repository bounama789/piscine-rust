pub fn delete_prefix<'s>(prefix: &str, s: &'s str) -> Option<&'s str> {
    if s.starts_with(prefix) {   
        return Some(&s[prefix.len()..]);
    }
    return None;
}
