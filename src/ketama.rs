extern crate sha1;

use std::cmp::{Ordering, Ord, Eq, PartialEq, PartialOrd};



struct Node {
    node: String,
    hash: i32
}

impl Eq for Node {
}

#[allow(unused_variables)]
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        None
    }
}

#[allow(unused_variables)]
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        false
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        if self.hash == other.hash {
            Ordering::Greater
        }else if self.hash > other.hash {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl Node {
    fn self_cmp(&self, other: &Node) -> Ordering {
        if self.hash >= other.hash {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

type TickArray = Vec<Node>;

pub struct HashRing {
    default_spots: i32,
    ticks: TickArray,
    length: usize
}

#[allow(exceeding_bitshifts)]
impl HashRing {
    pub fn new(n: i32) -> HashRing {
        let tk = Vec::new();
        let h = HashRing{default_spots: n, ticks: tk, length: 0};
        h
    }

    pub fn add(&mut self, n: String, s: i32) {
        let spots = self.default_spots * s;
        let mut hash = sha1::Sha1::new();

        for i in 0..spots {
            hash.reset();
            let ts: String = i.to_string();

            let b = format!("{}:{}", n, ts);
            hash.update(b.as_bytes());
            let hash_bytes = hash.digest().bytes();
            let _node = Node{node: n.clone(), hash: ((hash_bytes[19] as u32) | (hash_bytes[18] as u32) << 8 | (hash_bytes[17] as u32) << 16 | (hash_bytes[16] as u32) << 24) as i32};
            self.ticks.push(_node);
        }
    }

    pub fn bake(&mut self) {
        self.ticks.sort_by(|a, b| a.cmp(&b));
        self.length = self.ticks.len();
    }

    pub fn hash(&self, s: String) -> String {
        let mut h = sha1::Sha1::new();
        h.reset();
        h.update(s.as_bytes());
        let hash_bytes = h.digest().bytes();
        let v = ((hash_bytes[19] as u32) | (hash_bytes[18] as u32) << 8 | (hash_bytes[17] as u32) << 16 | (hash_bytes[16] as u32) << 24) as i32;

        let _node = Node{hash: v, node: "".to_string()};
        match self.ticks.binary_search_by(|v| v.self_cmp(&_node)) {
            Ok(i) => {
                let nd = &self.ticks[i];
                nd.node.clone()
            },
            Err(_) => {
                // idx == self.length occur, reset the idx to 0
                let nd = &self.ticks[0];
                nd.node.clone()
            }
        }
    }
}


#[test]
fn test_simple() {
    let mut ring = HashRing::new(255);
    ring.add("node1".to_string(), 1);
    ring.add("node2".to_string(), 1);
    ring.add("node3".to_string(), 1);
    ring.bake();
    let i = ring.hash("liutaihua".to_string());
    println!("======= str: liutaihua, hash res:{:?} ========", i);
}


#[test]
fn bench() {
    let mut ring = HashRing::new(255);
    ring.add("node1".to_string(), 1);
    ring.add("node2".to_string(), 1);
    ring.add("node3".to_string(), 1);
    ring.add("node4".to_string(), 1);
    ring.add("node5".to_string(), 1);

    ring.bake();
    for i in 20000..30000 {
        println!("{} {}", i, ring.hash(i.to_string()));
    }
}
