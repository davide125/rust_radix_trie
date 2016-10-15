use std::collections::HashSet;
use {Trie, TrieCommon};

const TEST_DATA: [(&'static str, u32); 7] = [
        ("abcdefgh", 19),
        ("abcdef", 18),
        ("abcd", 17),
        ("ab", 16),
        ("a", 15),
        ("acbdef", 30),
        ("bcdefgh", 29)
];

fn test_trie() -> Trie<&'static str, u32> {
    let mut trie = Trie::new();

    for &(key, val) in &TEST_DATA {
        trie.insert(key, val);
        assert!(trie.check_integrity());
    }

    trie
}

#[test]
fn get_nonexistant() {
    let trie = test_trie();
    assert!(trie.get(&"nonexistant").is_none());
    assert!(trie.get(&"").is_none());
}

#[test]
fn subtrie_nonexistant() {
    let trie = test_trie();
    assert!(trie.subtrie(&"nonexistant").is_none());
    assert!(trie.subtrie(&"").is_some());
}

#[test]
fn subtrie() {
    let mut trie = Trie::new();
    trie.insert("hello", 55);
    assert!(trie.subtrie(&"h").is_none());
    assert!(trie.subtrie(&"hello").is_some());
}
#[test]
fn subtrie_string() {
    let mut trie = Trie::new();
    trie.insert("hello".to_string(), 55);

    let h = "h".to_string();
    let hello = "hello".to_string();

    assert!(trie.subtrie(&h).is_none());
    assert!(trie.subtrie(&hello).is_some());
}

#[test]
fn empty_key() {
    let mut trie = test_trie();
    trie.insert(&"", 99);
    assert_eq!(*trie.get(&"").unwrap(), 99);
    assert_eq!(trie.remove(&""), Some(99));
}

#[test]
fn insert() {
    let trie = test_trie();

    for &(key, val) in &TEST_DATA {
        assert_eq!(*trie.get(&key).unwrap(), val);
    }

    assert!(trie.check_integrity());
    assert_eq!(trie.len(), TEST_DATA.len());
}

#[test]
fn insert_replace() {
    let mut trie = Trie::new();
    assert_eq!(trie.insert("haskell", 18), None);
    let length = trie.len();
    assert_eq!(trie.insert("haskell", 36), Some(18));
    assert_eq!(trie.len(), length);
}

#[test]
fn map_with_default() {
    let mut trie = test_trie();
    trie.map_with_default(&"abcd",{|x| *x = *x+1},42);
    assert_eq!(*trie.get(&"abcd").unwrap(),17+1);
    trie.map_with_default(&"zzz",{|x| *x = *x+1},42);
    assert_eq!(*trie.get(&"zzz").unwrap(),42);
}

#[test]
fn remove() {
    let mut trie = test_trie();

    // Remove.
    for &(key, val) in &TEST_DATA {
        let res = trie.remove(&key);
        assert_eq!(res, Some(val));
        assert!(trie.check_integrity());
    }

    // Check non-existance.
    for &(key, _) in &TEST_DATA {
        assert!(trie.get(&key).is_none());
    }
}

#[test]
fn remove_simple() {
    let mut trie = Trie::new();

    trie.insert("HELL", 66);
    trie.insert("HELLO", 77);
    let val = trie.remove(&"HELLO");
    assert_eq!(val, Some(77));
}

#[test]
fn nearest_ancestor_root() {
    let mut trie = Trie::new();
    trie.insert("", 55);
    assert_eq!(trie.get_ancestor_value(&""), Some(&55));
}

#[test]
fn nearest_ancestor() {
    let trie = test_trie();
    assert_eq!(trie.get_ancestor_value(&""), None);

    // Test identity prefixes.
    for &(key, val) in &TEST_DATA {
        assert_eq!(trie.get_ancestor_value(&key), Some(&val));
    }

    assert_eq!(trie.get_ancestor_value(&"abcdefg"), trie.get(&"abcdef"));
    assert_eq!(trie.get_ancestor_value(&"abcde"), trie.get(&"abcd"));
    assert_eq!(trie.get_ancestor_value(&"aauksdjk"), trie.get(&"a"));
}

#[test]
fn nearest_ancestor_no_child_fn() {
    let mut t = Trie::new();
    t.insert("ab", 5);
    let anc = t.get_ancestor(&"abc").unwrap();
    assert_eq!(*anc.value().unwrap(), 5);
}

#[test]
fn raw_ancestor() {
    let mut t = Trie::new();

    for &(key, _) in &TEST_DATA {
        assert_eq!(t.get_raw_ancestor(&key).key(), t.key());
    }

    t.insert("wow", 0);
    t.insert("hella", 1);
    t.insert("hellb", 2);

    // Ancestor should be "hell" node.
    let anc = t.get_raw_ancestor(&"hello");
    assert_eq!(anc.len(), 2);
}

#[test]
fn iter() {
    type Set = HashSet<(&'static str, u32)>;
    let trie = test_trie();
    let expected = TEST_DATA.iter().map(|&x| x).collect::<Set>();
    let observed = trie.iter().map(|(&k, &v)| (k, v)).collect::<Set>();
    assert_eq!(expected, observed);
}

#[test]
fn get_raw_descendant() {
    let trie = test_trie();
    assert_eq!(trie.get_raw_descendant(&"abcdefgh").and_then(|t| t.value()), Some(&19));
    assert_eq!(trie.get_raw_descendant(&"abcdefg").and_then(|t| t.value()), Some(&19));
    assert!(trie.get_raw_descendant(&"acbg").is_none());
}

#[test]
fn get_prefix_bug() {
    let mut trie = Trie::new();
    trie.insert("abdc", 5);
    trie.insert("abde", 6);
    assert!(trie.get(&"abc").is_none());
}

#[test]
fn get_ancestor_bug() {
    let mut trie = Trie::new();
    trie.insert("abc", 1);
    trie.insert("abcde", 2);
    assert_eq!(trie.get_ancestor_value(&"abcdz"), Some(&1));
}

#[test]
fn root_replace_bug() {
    let mut trie = Trie::new();
    trie.insert("a", ());
    trie.insert("p", ());
    trie.remove(&"a");
    assert_eq!(trie.len(), 1);
    trie.remove(&"p");
    assert_eq!(trie.len(), 0);
}

#[test]
fn subtrie_insert() {
    let mut trie = Trie::new();
    trie.insert("abc", 3);
    {
        let mut subtrie = trie.subtrie_mut(&"abc").unwrap();
        assert_eq!(subtrie.insert("somerandomshit", 666), Err(()));
        assert_eq!(subtrie.insert("abcdef", 6), Ok(None));
        assert_eq!(subtrie.insert("abc", 9), Ok(Some(3)));
    }
    assert_eq!(trie.get(&"abc"), Some(&9));
    assert_eq!(trie.get(&"abcdef"), Some(&6));
    assert_eq!(trie.len(), 2);
}

#[test]
fn subtrie_len() {
    let trie = test_trie();
    assert_eq!(trie.subtrie(&"ab").unwrap().len(), 4);
    assert_eq!(trie.subtrie(&"a").unwrap().len(), 6);
    assert_eq!(trie.subtrie(&"").unwrap().len(), trie.len());
    assert_eq!(trie.subtrie(&"bcdefgh").unwrap().len(), 1);
}

// Subtrie functions that return references should return references valid for
// the lifetime of the *original trie* from which they were borrowed, NOT
// just the lifetime of the subtrie (which may be shorter).
#[test]
fn subtrie_lifetime() {
    let trie = test_trie();
    let subtrie_value = {
        let subtrie = trie.subtrie(&"ab").unwrap();
        subtrie.value()
    };
    assert_eq!(*subtrie_value.unwrap(), 16);
}

#[test]
fn subtrie_mut_lifetime() {
    let mut trie = test_trie();
    let subtrie_value = {
        let mut subtrie = trie.subtrie_mut(&"ab").unwrap();
        *subtrie.value_mut().unwrap() = 999;
        subtrie.value()
    };
    assert_eq!(*subtrie_value.unwrap(), 999);
}
