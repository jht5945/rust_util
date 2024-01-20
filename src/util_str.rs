/// Split string to lines, splited by '\r', '\n' or "\r\n"
pub fn read_str_to_lines(s: &str) -> Vec<String> {
    s.lines().map(|ln| ln.to_owned()).collect()
}

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

#[test]
fn test_read_str_to_lines() {
    {
        let s = "";
        let lines = read_str_to_lines(s);
        assert_eq!(lines.len(), 0);
    }
    {
        let s = "\n";
        let lines = read_str_to_lines(s);
        assert_eq!(lines.len(), 1);
    }
    {
        let s = "\r";
        let lines = read_str_to_lines(s);
        assert_eq!(lines.len(), 1);
    }
    {
        let s = "\r\n";
        let lines = read_str_to_lines(s);
        assert_eq!(lines.len(), 1);
    }
    {
        let s = "aa\r\nbb";
        let lines = read_str_to_lines(s);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "aa");
        assert_eq!(lines[1], "bb");
    }
    {
        let s = "aa\r\nbb\ncc";
        let lines = read_str_to_lines(s);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "aa");
        assert_eq!(lines[1], "bb");
        assert_eq!(lines[2], "cc");
    }
}
