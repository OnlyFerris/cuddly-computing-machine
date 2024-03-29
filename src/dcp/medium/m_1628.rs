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

    // [[3, 2, 1],
    //  [6, 5, 4],
    //  [7, 8, 9]]
    m.swap(3, 5);

    // [[3, 2, 1],
    //  [6, 5, 4],
    //  [9, 8, 7]]
    m.swap(6, 8);

    // [[7, 2, 1],
    //  [6, 5, 4],
    //  [9, 8, 3]]
    m.swap(0, 8);

    // [[7, 4, 1],
    //  [6, 5, 2],
    //  [9, 8, 3]]
    m.swap(1, 5);

    // [[7, 4, 1],
    //  [8, 5, 2],
    //  [9, 6, 3]]
    m.swap(3, 7);
}

#[test]
fn rotates_3x3_as_expected() {
    let m = &mut [1, 2, 3, 4, 5, 6, 7, 8, 9];

    rotate_3x3(m);

    let x = &[7, 4, 1, 8, 5, 2, 9, 6, 3];

    assert_eq!(m, x);
}

pub fn rotate_4x4(_m: &mut [u64; 16]) {
    // TODO: Implement

    // [[1, 2, 3, 4],
    //  [5, 6, 7, 8],
    //  [9, A, B, C],
    //  [D, E, F, G]]

    // --- Reverse every row
    // TODO: Is there a shorthand for that, or do we need to swap
    //       [a, b, c, d] -> [d, b, c, a] and then
    //       [d, b, c, a] -> [d, c, b, a]

    // [[4, 3, 2, 1],
    //  [5, 6, 7, 8],
    //  [9, A, B, C],
    //  [D, E, F, G]]

    // [[4, 3, 2, 1],
    //  [8, 7, 6, 5],
    //  [9, A, B, C],
    //  [D, E, F, G]]

    // [[4, 3, 2, 1],
    //  [8, 7, 6, 5],
    //  [C, B, A, 9],
    //  [D, E, F, G]]

    // [[4, 3, 2, 1],
    //  [8, 7, 6, 5],
    //  [C, B, A, 9],
    //  [G, F, E, D]]

    // --- Make swaps in outer "ring"

    // [[4, 3, 5, 1],
    //  [8, 7, 6, 2],
    //  [C, B, A, 9],
    //  [G, F, E, D]]

    // [[4, 9, 5, 1],
    //  [8, 7, 6, 2],
    //  [C, B, A, 3],
    //  [G, F, E, D]]

    // [[D, 9, 5, 1],
    //  [8, 7, 6, 2],
    //  [C, B, A, 3],
    //  [G, F, E, 4]]

    // [[D, 9, 5, 1],
    //  [E, 7, 6, 2],
    //  [C, B, A, 3],
    //  [G, F, 8, 4]]

    // [[D, 9, 5, 1],
    //  [E, 7, 6, 2],
    //  [F, B, A, 3],
    //  [G, C, 8, 4]]

    // --- Make swaps in inner "ring"

    // [[D, 9, 5, 1],
    //  [E, A, 6, 2],
    //  [F, B, 7, 3],
    //  [G, C, 8, 4]]
}

// TODO: Arbitrary n*n
