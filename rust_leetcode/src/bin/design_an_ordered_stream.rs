use std::collections::HashMap;
struct OrderedStream {
    ptr: i32,
    map: HashMap<i32, String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            ptr: 1,
            map: HashMap::with_capacity(n as usize),
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let mut ans = Vec::new();
        let s = value.clone();

        self.map.insert(id_key, value);

        if self.ptr == id_key {
            ans.push(s);

            let mut i = 1;
            'l: loop {
                match self.map.get(&(id_key + i)) {
                    Some(s) => {
                        ans.push(s.clone());
                        i += 1;
                    }
                    None => break 'l,
                }
            }
            self.ptr = id_key + i;
        }
        ans
    }
}

fn main() {
    let mut os = OrderedStream::new(5);

    println!("{:?}", os.insert(3, "ccccc".to_string()));
    println!("{:?}", os.insert(1, "aaaaa".to_string()));
    println!("{:?}", os.insert(2, "bbbbb".to_string()));
    println!("{:?}", os.insert(5, "eeeee".to_string()));
    println!("{:?}", os.insert(4, "ddddd".to_string()));
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use crate::OrderedStream;

        let mut os = OrderedStream::new(5);
        assert!(os.insert(3, "ccccc".to_string()).is_empty());
        assert_eq!(os.insert(1, "aaaaa".to_string()), vec!["aaaaa".to_string()]);
        assert_eq!(
            os.insert(2, "bbbbb".to_string()),
            vec!["bbbbb".to_string(), "ccccc".to_string()]
        );
        assert!(os.insert(5, "eeeee".to_string()).is_empty());
        assert_eq!(
            os.insert(4, "ddddd".to_string()),
            vec!["ddddd".to_string(), "eeeee".to_string()]
        );
    }
}
