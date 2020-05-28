extern crate rna_secondary_structure;

use crate::rna_secondary_structure::secondary_structure::SecondaryStructureRecord;

#[test]
fn test_to_dotbracketstring() {
    let paired = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
    let ss = SecondaryStructureRecord::new(paired);
    assert_eq!(ss.get_dotbracketstring(), "(((..))..)..");
}

#[test]
fn test_from_dotbracketstring() {
    let ss : SecondaryStructureRecord = "<(a..A)..>..".parse().unwrap();
    let paired = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
    assert_eq!(ss.paired,  paired);
}