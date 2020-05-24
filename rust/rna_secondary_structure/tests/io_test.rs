extern crate rna_secondary_structure;

use rna_secondary_structure::secondary_structure::SecondaryStructureRecord;

use crate::rna_secondary_structure::io;
use crate::rna_secondary_structure::secondary_structure;

#[test]
/// Tests both the the get_ct_string and parse_ct_string functions.
fn test_ct_strings() {
    let dbs1 = ".(((...))...)".to_string();
    let paired1 = secondary_structure::from_dotbracketstring(&dbs1).unwrap();
    let ss1 = SecondaryStructureRecord {
        name: "example1".to_string(),
        paired: paired1,
        sequence: "ATAGCATCTCGGA".to_string(),
    };

    let dbs2 = "...............".to_string();
    let paired2 = secondary_structure::from_dotbracketstring(&dbs2).unwrap();
    let ss2 = SecondaryStructureRecord {
        name: "example2".to_string(),
        paired: paired2,
        sequence: "CCCCAAAAAAAAAAA".to_string(),
    };

    let dbs3 = "((....))".to_string();
    let paired3 = secondary_structure::from_dotbracketstring(&dbs3).unwrap();
    let ss3 = SecondaryStructureRecord {
        name: "example3".to_string(),
        paired: paired3,
        sequence: "CCAAAAGG".to_string(),
    };

    let mut ct_string = "".to_string();
    ct_string.push_str(&io::get_ct_string(&ss1));
    ct_string.push_str(&io::get_ct_string(&ss2));
    ct_string.push_str("\n\n"); // test robustness to adding in blank lines
    ct_string.push_str(&io::get_ct_string(&ss3));

    let ls = io::parse_ct_string(&ct_string);

    assert_eq!(ls[0].sequence, ss1.sequence);
    assert_eq!(ls[0].paired, ss1.paired);
    assert_eq!(ls[1].sequence, ss2.sequence);
    assert_eq!(ls[1].paired, ss2.paired);
    assert_eq!(ls[2].sequence, ss3.sequence);
    assert_eq!(ls[2].paired, ss3.paired);
}