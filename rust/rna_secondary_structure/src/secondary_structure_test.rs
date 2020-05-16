use super::*;

#[test]
fn test_dotbracketstring() {
    let ss = SecondaryStructure {
        pairedsites: vec![5,4,0,0,2,1]
    };
    print!("{}", ss);
    assert_eq!(ss.dotbracketstring(), "((..))");
}