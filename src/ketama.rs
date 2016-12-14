extern crate sha1;

use std::cmp::{Ordering, Ord, Eq, PartialEq, PartialOrd};



struct node {
    node: String,
    hash: i32
}

impl Eq for node {
}

impl PartialOrd for node {
    fn partial_cmp(&self, other: &node) -> Option<Ordering> {
        None
    }
}

impl PartialEq for node {
    fn eq(&self, other: &node) -> bool {
        false
    }
}

impl Ord for node {
    fn cmp(&self, other: &node) -> Ordering {
        if self.hash >= other.hash {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

//impl node {
//    fn new(n: String, hash: i32) -> node {
//
//    }
//}

type TickArray = Vec<node>;

pub struct HashRing {
    defaultSpots: i32,
    ticks: TickArray,
    length: usize
}

#[allow(exceeding_bitshifts)]
impl HashRing {
    pub fn new(n: i32) -> HashRing {
        let tk = Vec::new();
        let h = HashRing{defaultSpots: n, ticks: tk, length: 0};
        h
    }

    pub fn add(&mut self, n: String, s: i32) {
        let tSpots = self.defaultSpots * s;
        let mut hash = sha1::Sha1::new();

        for i in 0..tSpots {
            hash.reset();
            let ts: String = i.to_string();

            let b = format!("{}:{}", n, ts);
            hash.update(b.as_bytes());
            let hashBytes = hash.digest().bytes();
            let _node = node{node: n.clone(), hash: (hashBytes[19] | hashBytes[18] << 8 | hashBytes[17] << 16 | hashBytes[16] << 24) as i32};
            self.ticks.push(_node);
        }
    }

    pub fn bake(&mut self) {
        self.ticks.sort();
        self.length = self.ticks.len();
    }

    pub fn hash(&self, s: String) -> String {
        let mut h = sha1::Sha1::new();
        h.reset();
        h.update(s.as_bytes());
        let hashBytes = h.digest().bytes();
        let v = (hashBytes[19] | hashBytes[18] << 8 | hashBytes[17] << 16 | hashBytes[16] << 24) as i32;
        let _node = node{hash: v, node: "".to_string()};
        let nd: &node;
        match self.ticks.binary_search_by(|v| v.cmp(&_node)) {
            Ok(i) => {
                nd = &self.ticks[i];
            },
            _ => {
                nd = &self.ticks[0];
            }
        }
        nd.node.clone()
    }
}


#[cfg(test)]
mod tests {

}