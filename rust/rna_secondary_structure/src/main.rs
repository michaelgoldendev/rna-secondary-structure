mod secondary_structure;

fn main() {
    let ss = secondary_structure::SecondaryStructure {
        pairedsites: vec![5,4,0,0,2,1]
    };
    println!("{}", ss);
}
