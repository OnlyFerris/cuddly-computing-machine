//! Given an N by N matrix, rotate it by 90 degrees clockwise.
//!
//! For example, given the following matrix:
//!
//! ```text
//! [[1, 2, 3],
//!  [4, 5, 6],
//!  [7, 8, 9]]
//! ```
//!
//! you should return:
//!
//! ```text
//! [[7, 4, 1],
//!  [8, 5, 2],
//!  [9, 6, 3]]
//! ```
//!
//! Follow-up: What if you couldn't use any extra space?

// An array of arrays is still contiguous.
// In this function we view teh argument as such.
// One array with len n^2 instead of a nested n*n matrix.
pub fn rotate_3x3(m: &mut [u64; 9]) {
    // [[1, 2, 3],
    //  [4, 5, 6],
    //  [7, 8, 9]]

    // [[3, 2, 1],
    //  [4, 5, 6],
    //  [7, 8, 9]]
    m.swap(0, 2);

    // [[9, 2, 1],
    //  [4, 5, 6],
    //  [7, 8, 3]]
    m.swap(0, 8);

    // [[7, 2, 1],
    //  [4, 5, 6],
    //  [9, 8, 3]]
    m.swap(0, 6);

    // [[7, 6, 1],
    //  [4, 5, 2],
    //  [9, 8, 3]]
    m.swap(1, 5);

    // [[7, 4, 1],
    //  [6, 5, 2],
    //  [9, 8, 3]]
    m.swap(1, 3);

    // [[7, 4, 1],
    //  [8, 5, 2],
    //  [9, 6, 3]]
    m.swap(3, 7);
}

// TODO: 4 * 4

// TODO: Arbitrary n*n
