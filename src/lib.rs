// Copyright © 2019 Andre Mukhsia
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;

/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```
pub fn mean(nums: &[f64]) -> Option<f64> {
    // Empty list, mean is 0.0
    if nums.len() == 0 {
        Some(0.0_f64)
    }
    else
    {
        // Non empty list, get sum of elements and divide by length of array
        let mut sum: f64 = 0.0;
        for num in nums {
            sum += num;
        };
        sum = sum / nums.len() as f64;
        Some(sum)
    }
}

// Example test 1
#[test]
fn test_mean_example1() {
    assert_eq!(0.0, mean(&[]).unwrap());
}

// Example test 2
#[test]
fn test_mean_example2() {
    assert_eq!(0.0, mean(&[-1.0,1.0]).unwrap());
}

// Added Test
#[test]
fn test_mean_added() {
    assert_eq!(1.7, mean(&[-6.0,14.5,2.2,-8.4,6.2]).unwrap())
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```
pub fn stddev(nums: &[f64]) -> Option<f64> {
    // Empty list, stddev is None
    if nums.len() == 0 {
        None
    }
    else
    {
        // Non empty list, Compute calculation similar to https://en.wikipedia.org/wiki/Standard_deviation#Population_standard_deviation_of_grades_of_eight_students
        let mean_nums = mean(nums).unwrap();
        let mut sum: f64 = 0.0;
        for num in nums {
            sum += (num-mean_nums).powi(2);
        };
        sum = sum / nums.len() as f64;
        Some(sum.sqrt())
    }
}

// Example test 1
#[test]
fn test_stddev_example1() {
    assert_eq!(None, stddev(&[]));
}

// Example test 2
#[test]
fn test_stddev_example2() {
    assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
}

// Added test
#[test]
fn test_stddev_added() {
    assert_eq!(Some(1.5), stddev(&[3.5,3.5,3.5,6.5,6.5,6.5]));
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Case |nums| = 0: return undefined
    // Case Odd |nums|: Get mean of middle elements
    // Case Even |nums|: Get middle element
    unimplemented!("no median yet")
}

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
pub fn l2(nums: &[f64]) -> Option<f64> {
    unimplemented!("no l2 yet")
}
