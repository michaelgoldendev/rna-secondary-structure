use crate::secondary_structure;

#[test]
fn test_to_dotbracketstring() {
    let pairedsites = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
    let ss = secondary_structure::SecondaryStructure::new(pairedsites.to_vec());
    assert_eq!(ss.dotbracketstring(), "(((..))..)..");
}