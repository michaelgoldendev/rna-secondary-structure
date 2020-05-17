use super::*;

#[test]
fn test_to_dotbracketstring() {
    let pairedsites = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
    let ss = SecondaryStructure {
        pairedsites: pairedsites.to_vec()
    };
    assert_eq!(ss.dotbracketstring(), "(((..))..)..");
}

#[test]
fn test_from_dotbracketstring() {
    let pairedsites = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
    let ss : SecondaryStructure = "(((..))..)..".parse().unwrap();
    assert_eq!(ss.pairedsites, pairedsites);
}