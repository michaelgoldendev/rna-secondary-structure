use super::*;

#[test]
fn test_dotbracketstring() {
    let pairedsites = vec![10, 7, 6, 0, 0, 3, 2, 0, 0, 1, 0, 0];
    let dbs = String::from("(((..))..)..");
    let ss = SecondaryStructure {
        pairedsites: pairedsites.to_vec()
    };
    assert_eq!(ss.dotbracketstring(), dbs);

    let ss2 = from_dotbracketstring(&dbs);
    assert_eq!(ss2.pairedsites, pairedsites);
}