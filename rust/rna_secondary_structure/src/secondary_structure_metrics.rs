///
/// ```
/// use crate::rna_secondary_structure::secondary_structure::from_dotbracketstring;
/// use crate::rna_secondary_structure::secondary_structure_metrics::get_mountain_vector;
/// let paired = from_dotbracketstring("(((...)))").unwrap();
/// let mountain_obs = get_mountain_vector(&paired);
/// let mountain_exp = vec![1.0, 2.0, 3.0, 3.0, 3.0, 3.0, 2.0, 1.0, 0.0];
/// assert_eq!(mountain_obs, mountain_exp);
/// ```
pub fn get_mountain_vector(paired : &Vec<i64>) -> Vec<f64> {
    let mut mountain = vec![0.0; paired.len()];
    for (i, j) in paired.iter().enumerate() {
        if i > 0 {
            mountain[i] = mountain[i-1];
        }

        if *j != 0 {
            if *j > (i as i64) {
                mountain[i] += 1.0;
            } else {
                mountain[i] -= 1.0;
            }
        }
    }
    mountain
}