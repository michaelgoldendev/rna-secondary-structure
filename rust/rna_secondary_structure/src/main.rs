extern crate rna_secondary_structure;

use std::path::Path;
use std::str::FromStr;

use rna_secondary_structure::*;
use rna_secondary_structure::io;
use rna_secondary_structure::secondary_structure::get_dot_bracket_string;

fn main() {
    let paired = vec![5, 7, 6, 9, 1, 3, 2, 10, 4, 8];
    println!("{:?}", paired);
    //let paired = secondary_structure::from_dotbracketstring("(<.)>").unwrap();
    let mountain = distance_metrics::get_mountain_vector(&paired);
    println!("{:?}", mountain);
    let paired2 = distance_metrics::invert_mountain_vector(&mountain);
    println!("{:?}", paired2);

    println!("{}", get_dot_bracket_string(&paired).unwrap());
    let p1 = distance_metrics::get_structure_star(100);
    let p2 = distance_metrics::get_structure_zero(100);
    let dist3 = distance_metrics::get_normalised_mountain_distance(&p1, &p2, None).unwrap();
    let dist1 = distance_metrics::get_mountain_distance(&p1, &p2, None).unwrap();
    let dist2 = distance_metrics::get_mountain_diameter(100, None);
    println!("{:?} {:?} {:?}", dist1, dist2, dist3);


    let ct_string_expected = ">example
    1	C	0	2	8	1
    2	G	1	3	5	2
    3	A	2	4	0	3
    4	A	3	5	0	4
    5	C	4	6	2	5
    6	A	5	7	0	6
    7	A	6	8	0	7
    8	G	7	9	1	8";
    let ls = io::parse_ct_string(&ct_string_expected.parse().unwrap()).unwrap();
    println!("[{}]", ls.iter().fold(String::new(), |acc, num| acc + &num.to_string() + ", "));

    let mut ss = secondary_structure::SecondaryStructureRecord::from_str("(({....))").unwrap_or_else(|err| {
        panic!("{}", err)
    });
    ss.sequence = "AAAA".to_string();
    println!("{}", ss);

    let ss: secondary_structure::SecondaryStructureRecord = "(..)".parse().unwrap();
    println!("{}", ss);

    // let ss : secondary_structure::SecondaryStructure = "(".parse().unwrap();
    // println!("{}", ss);

    let ss: secondary_structure::SecondaryStructureRecord = "((..)..)".parse().unwrap();
    io::write_ct_file(Path::new("test.ct"), &ss).unwrap();
    println!("{}", ss);
}
