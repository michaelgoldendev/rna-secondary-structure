use crate::secondary_structure;
use crate::secondary_structure_io;

#[test]
/// Tests both the the get_ct_string and parse_ct_string functions.
fn test_ct_strings() {
    let seq1 = "ATAGCATCTCGGA".to_string();
    let dbs1 = ".(((...))...)".to_string();
    let paired1 = secondary_structure::from_dotbracketstring(&dbs1).unwrap();

    let seq2 = "CCCCAAAAAAAAAAA".to_string();
    let dbs2 = "...............".to_string();
    let paired2 = secondary_structure::from_dotbracketstring(&dbs2).unwrap();

    let seq3 = "CCAAAAGG".to_string();
    let dbs3 = "((....))".to_string();
    let paired3 = secondary_structure::from_dotbracketstring(&dbs3).unwrap();

    let mut ct_string = "".to_string();
    ct_string.push_str(&secondary_structure_io::get_ct_string(&seq1, &paired1, &"example1".to_string()));
    ct_string.push_str(&secondary_structure_io::get_ct_string(&seq2, &paired2, &"example2".to_string()));
    ct_string.push_str("\n\n"); // test robustness to adding in blank lines
    ct_string.push_str(&secondary_structure_io::get_ct_string(&seq3, &paired3, &"example3".to_string()));

    let ls = secondary_structure_io::parse_ct_string(&ct_string);

    assert_eq!(ls[0].sequence, seq1);
    assert_eq!(ls[0].paired, paired1);
    assert_eq!(ls[1].sequence, seq2);
    assert_eq!(ls[1].paired, paired2);
    assert_eq!(ls[2].sequence, seq3);
    assert_eq!(ls[2].paired, paired3);
}