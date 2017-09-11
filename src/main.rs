extern crate farmhash;

fn main() {
    let strs = ["hello world", "farmhash", "foo", "bar", "x"];
    for s in strs.iter() {
        println!("{}: {}", s, farmhash::hash64(s.as_bytes()));
    }
}
