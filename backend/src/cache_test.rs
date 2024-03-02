#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]

    fn test_kube_cache_hash_is_not_order_sensitive() {
        let mut items = vec![];
        items.push(KubeId::new("a"));
        items.push(KubeId::new("b"));
        items.push(KubeId::new("b"));
        items.push(KubeId::new("c"));
        let recipe1 = Recipe::new(items.clone());

        items.reverse();
        let recipe2 = Recipe::new(items.clone());
        assert_eq!(recipe1.hash(), recipe2.hash());
    }
}
