use indexmap::IndexMap;

fn main() {
    let mut letters = IndexMap::new();
    letters.insert(1, 1);
    assert!(*letters.get(&1).unwrap()==1, "1");
    letters.insert(1, 2);
    assert!(*letters.get(&1).unwrap()==2, "2");
    print!("{:?}", letters.get(&1).unwrap());
    
}
