#### ketama rust implement

#### Usage

[dependencies.ketama]
git = "https://github.com/liutaihua/rust-ketama"

<pre><code>
extern crate ketama;
use ketama::{HashRing};

fn main() {
    let mut ring = HashRing::new(255);
    ring.add("node1".to_string(), 1);
    ring.add("node2".to_string(), 1);
    ring.add("node3".to_string(), 1);
    ring.bake();
    let i = ring.hash("helloworld".to_string());
    println!("======= str: helloworld, hash res:{:?} ========", i);
}
</code></pre>

#### test
cargo test


#### todo
benchmark