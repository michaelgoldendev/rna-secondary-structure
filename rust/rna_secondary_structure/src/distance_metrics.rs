//! A module for calculating distances between secondary structures.
//!
//! Implements the 'Mountain Metric' as defined in:
//! `Moulton, Vincent, et al. "Metrics on RNA secondary structures." Journal of Computational Biology 7.1-2 (2000): 277-292.`

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecondaryStructureMetricError {
    #[error("Secondary structures must be the same length.")]
    UnequalLength,
}

/// Returns a mountain vector from a vector representing a list of paired sites.
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
pub fn get_mountain_vector(paired: &Vec<i64>) -> Vec<f64> {
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

pub fn get_mountain_distance(paired1: &Vec<i64>, paired2: &Vec<i64>, p: f64) -> Result<f64, SecondaryStructureMetricError> {
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
/// length (this is just the distance between structure_star and structure_zero).
pub fn get_mountain_diameter(len: i64, p: f64) -> f64 {
    get_mountain_distance(&get_structure_star(len), &get_structure_zero(len), p).unwrap()
}

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
/// let max_distance = distance_metrics::get_normalised_mountain_distance(&p1, &p2, 2.0).unwrap();
/// assert_eq!(max_distance, 1.0);
///
/// // structures p1 is identical to itself and therefore distance should be zero
/// let max_distance = distance_metrics::get_normalised_mountain_distance(&p1, &p1, 2.0).unwrap();
/// assert_eq!(max_distance, 0.0);
/// ```
pub fn get_normalised_mountain_distance(paired1: &Vec<i64>, paired2: &Vec<i64>, p: f64) -> Result<f64, SecondaryStructureMetricError> {
    Ok(get_mountain_distance(paired1, paired2, p)? / get_mountain_diameter(paired1.len() as i64, p))
}

pub fn get_weighted_mountain_vector(paired: &Vec<i64>) -> Vec<f64> {
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

pub fn get_weighted_mountain_distance(paired1: &Vec<i64>, paired2: &Vec<i64>) -> Result<f64, SecondaryStructureMetricError> {
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

pub fn get_weighted_mountain_diameter(len: i64) -> f64 {
    get_weighted_mountain_distance(&get_structure_star(len), &get_structure_zero(len)).unwrap()
}

pub fn get_normalised_weighted_mountain_distance(paired1: &Vec<i64>, paired2: &Vec<i64>) -> Result<f64, SecondaryStructureMetricError> {
    Ok(get_weighted_mountain_distance(paired1, paired2)? / get_weighted_mountain_diameter(paired1.len() as i64))
}