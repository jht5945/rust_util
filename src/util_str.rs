

pub fn split_kv(s: &str, split: char) -> (String, String) {
    let mut k = String::new();
    let mut v = String::new();

    let mut is_splited = false;
    let cs = s.chars();
    for c in cs {
        if is_splited {
            v.push(c);
        } else if c == split {
            is_splited = true;
        } else {
            k.push(c);
        }
    }

    (k, v)
}


#[test]
fn test_split_kv() {
    assert_eq!(("".to_owned(), "".to_owned()), split_kv("", '='));
    assert_eq!(("aaaa".to_owned(), "".to_owned()), split_kv("aaaa", '='));
    assert_eq!(("".to_owned(), "aaaa".to_owned()), split_kv("=aaaa", '='));
    assert_eq!(("aa".to_owned(), "bb".to_owned()), split_kv("aa=bb", '='));
    assert_eq!(("aa".to_owned(), "bb".to_owned()), split_kv("aa:bb", ':'));
}
