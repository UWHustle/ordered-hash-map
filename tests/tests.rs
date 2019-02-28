extern crate clashmap;

#[cfg(test)]
mod tests {

    use clashmap::ClashMap;

//    #[test]
//    fn iter() {
//        let mut map = ClashMap::new();
//        map.insert("a", 1);
//        map.insert("b", 2);
//        map.insert("c", 3);
//
//        let mut iter = map.iter();
//        assert_eq!(iter.next(), Some((&"a", &1)));
//        assert_eq!(iter.next(), Some((&"b", &2)));
//        assert_eq!(iter.next(), Some((&"c", &3)));
//    }
//
    #[test]
    fn len() {
        let a = ClashMap::new();
        assert_eq!(a.len(), 0);
        a.insert(1, "a");
        assert_eq!(a.len(), 1);
    }

    #[test]
    fn is_empty() {
        let a = ClashMap::new();
        assert!(a.is_empty());
        a.insert(1, "a");
        assert!(!a.is_empty());
    }

    #[test]
    fn clear() {
        let a = ClashMap::new();
        a.insert(1, "a");
        a.clear();
        assert!(a.is_empty());
    }
//
//    #[test]
//    fn get() {
//        let map = ClashMap::new();
//        map.insert(1, "a");
//        assert_eq!(map.get(&1).unwrap(), &"a");
//        assert!(map.get(&2).is_none());
//    }
//
//    #[test]
//    fn insert() {
//        let map = ClashMap::new();
//        assert_eq!(map.insert(37, "a"), None);
//        assert_eq!(map.is_empty(), false);
//
//        map.insert(37, "b");
//        assert_eq!(map.insert(37, "c"), Some("b"));
//        assert_eq!(map.get(&37).unwrap(), "c");
//    }
//
//    #[test]
//    fn remove() {
//        let map = ClashMap::new();
//        map.insert(1, "a");
//        assert_eq!(map.remove(&1), Some("a"));
//        assert_eq!(map.remove(&1), None);
//    }
}
