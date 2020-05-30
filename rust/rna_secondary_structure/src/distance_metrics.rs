//! A module for calculating distances between secondary structures.
//!
//! Implements the 'Mountain Metric' as defined in:
//! `Moulton, Vincent, et al. "Metrics on RNA secondary structures." Journal of Computational Biology 7.1-2 (2000): 277-292.`

use thiserror::Error;

use crate::secondary_structure::PairedSites;

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum SecondaryStructureMetricError {
    #[error("Secondary structures must be the same length.")]
    UnequalLength,
}

/// Returns a mountain vector from a list of paired sites
///
/// # Examples
/// ```rust
/// use rna_secondary_structure::secondary_structure::from_dotbracketstring;
/// use rna_secondary_structure::distance_metrics::get_mountain_vector;
/// let paired = from_dotbracketstring("(((...)))").unwrap();
/// let mountain_obs = get_mountain_vector(&paired);
/// let mountain_exp = vec![1.0, 2.0, 3.0, 3.0, 3.0, 3.0, 2.0, 1.0, 0.0];
/// assert_eq!(mountain_obs, mountain_exp);
/// ```
pub fn get_mountain_vector(paired: &dyn PairedSites) -> Vec<f64> {
    let paired = paired.paired();
    let mut mountain = vec![0.0; paired.len()];
    for (i, j) in paired.iter().enumerate() {
        if i > 0 {
            mountain[i] = mountain[i - 1];
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

/// Returns the mountain distance between two secondary structures.
pub fn get_mountain_distance(paired1: &dyn PairedSites, paired2: &dyn PairedSites, p: Option<f64>) -> Result<f64, SecondaryStructureMetricError> {
    let paired1 = paired1.paired();
    let paired2 = paired2.paired();

    // defaults
    let p = p.unwrap_or(1.0);

    if paired1.len() != paired2.len() {
        return Err(SecondaryStructureMetricError::UnequalLength);
    }

    let m1 = get_mountain_vector(paired1);
    let m2 = get_mountain_vector(paired2);
    let mut d = 0.0;
    for (a, b) in m1.iter().zip(m2) {
        d += (a - b).abs().powf(p);
    }
    Ok(d)
}

/// Returns the unique secondary structure configuration of the specified length that has the
/// maximal number of base-pairings.
///
/// # Examples
/// ```rust
/// use rna_secondary_structure::distance_metrics::get_structure_star;
/// use rna_secondary_structure::secondary_structure::from_dotbracketstring;
/// let structure_star = get_structure_star(10);
/// let paired_expected = from_dotbracketstring("((((..))))").unwrap();
/// assert_eq!(structure_star, paired_expected);
/// ```
pub fn get_structure_star(len: i64) -> Vec<i64> {
    let mut paired = vec![0; len as usize];
    let upper = len / 2 - ((len + 1) % 2);
    for i in 0..upper {
        let j = len - i - 1;
        paired[i as usize] = j + 1;
        paired[j as usize] = i + 1;
    }
    paired
}

/// Returns the unique secondary structure configuration of the specified length that has all
/// nucleotides unpaired.
///
/// # Examples
/// ```rust
/// use rna_secondary_structure::distance_metrics::get_structure_zero;
/// use rna_secondary_structure::secondary_structure::from_dotbracketstring;
/// let structure0 = get_structure_zero(10);
/// let paired_expected = from_dotbracketstring("..........").unwrap();
/// assert_eq!(structure0, paired_expected);
/// ```
pub fn get_structure_zero(len: i64) -> Vec<i64> {
    vec![0; len as usize]
}

/// Returns the maximal possible mountain distance between any two secondary structures of a given
/// length (this is the distance between [structure_star](fn.get_structure_star.html) and [structure_zero](fn.get_structure_zero.html)).
pub fn get_mountain_diameter(len: i64, p: Option<f64>) -> f64 {
    get_mountain_distance(&get_structure_star(len), &get_structure_zero(len), p).unwrap()
}

/// Returns the normalised mountain distance, d, between two secondary structure configurations, such
/// that 0.0 <= d <= 1.0.
///
/// If d equals 0.0 then the two structures are identical.
/// If d equals 1.0 the two structures are maximally distant, see  [get_mountain_diameter](fn.get_mountain_diameter.html).
///
/// # Examples
/// ```rust
/// use crate::rna_secondary_structure::distance_metrics;
///
/// // structure with maximal number of base-pairings: ((((..))))
/// let p1 = distance_metrics::get_structure_star(100);
/// // structure with all nucleotides unpaired: .........
/// let p2 = distance_metrics::get_structure_zero(100);
///
/// // structures p1 and p2 should be maximally distant
/// let max_distance = distance_metrics::get_normalised_mountain_distance(&p1, &p2, Some(2.0)).unwrap();
/// assert_eq!(max_distance, 1.0);
///
/// // structures p1 is identical to itself and therefore distance should be zero
/// let max_distance = distance_metrics::get_normalised_mountain_distance(&p1, &p1, Some(2.0)).unwrap();
/// assert_eq!(max_distance, 0.0);
/// ```
pub fn get_normalised_mountain_distance(paired1: &dyn PairedSites, paired2: &dyn PairedSites, p: Option<f64>) -> Result<f64, SecondaryStructureMetricError> {
    let paired1 = paired1.paired();
    let paired2 = paired2.paired();
    Ok(get_mountain_distance(paired1, paired2, p)? / get_mountain_diameter(paired1.len() as i64, p))
}

/// Returns a weighted mountain vector, where at base-paired positions the step up or down in
/// mountain height is inversely proportional to the number of nucleotides seperating the base-pairs.
pub fn get_weighted_mountain_vector(paired: &dyn PairedSites) -> Vec<f64> {
    let paired = paired.paired();
    let mut mountain = vec![0.0; paired.len()];
    for (i, j) in paired.iter().enumerate() {
        if i > 0 {
            mountain[i] = mountain[i - 1];
        }

        if *j != 0 {
            mountain[i] += 1.0 / ((*j - (i as i64)) as f64);
        }
    }
    mountain
}

/// Returns a weighted version of the mountain distance.
pub fn get_weighted_mountain_distance(paired1: &dyn PairedSites, paired2: &dyn PairedSites) -> Result<f64, SecondaryStructureMetricError> {
    let paired1 = paired1.paired();
    let paired2 = paired2.paired();

    if paired1.len() != paired2.len() {
        return Err(SecondaryStructureMetricError::UnequalLength);
    }

    let m1 = get_weighted_mountain_vector(paired1);
    let m2 = get_weighted_mountain_vector(paired2);
    let mut d = 0.0;
    for (a, b) in m1.iter().zip(m2) {
        d += (a - b).abs();
    }
    Ok(d)
}

/// Returns a weighted version of the mountain diameter.
pub fn get_weighted_mountain_diameter(len: i64) -> f64 {
    get_weighted_mountain_distance(&get_structure_star(len), &get_structure_zero(len)).unwrap()
}

/// Returns a weighted version of the normalised mountain distance.
pub fn get_normalised_weighted_mountain_distance(paired1: &dyn PairedSites, paired2: &dyn PairedSites) -> Result<f64, SecondaryStructureMetricError> {
    let paired1 = paired1.paired();
    let paired2 = paired2.paired();

    Ok(get_weighted_mountain_distance(paired1, paired2)? / get_weighted_mountain_diameter(paired1.len() as i64))
}