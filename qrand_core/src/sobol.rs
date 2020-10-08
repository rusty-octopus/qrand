#![no_std]
#![no_builtins]
#![no_implicit_prelude]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![forbid(unsafe_code)]

extern crate core;
use core::result::{Result, Result::Err, Result::Ok};

use crate::error::QrandCoreError;
use crate::low_discrepancy_sequence::LowDiscrepancySequence;

/// Creates a new LowDiscrepancySequence
#[cfg(not(tarpaulin_include))]
pub fn new_sequence(dimension: usize) -> impl LowDiscrepancySequence {
    Sobol::new()
}

#[cfg(not(tarpaulin_include))]
static DIRECTION_NUMBER: [u32; 64] = [
    0x80000000, 0x40000000, 0x20000000, 0x10000000, 0x08000000, 0x04000000, 0x02000000, 0x01000000,
    0x00800000, 0x00400000, 0x00200000, 0x00100000, 0x00080000, 0x00040000, 0x00020000, 0x00010000,
    0x00008000, 0x00004000, 0x00002000, 0x00001000, 0x00000800, 0x00000400, 0x00000200, 0x00000100,
    0x00000080, 0x00000040, 0x00000020, 0x00000010, 0x00000008, 0x00000004, 0x00000002, 0x00000001,
    0x80000000, 0xc0000000, 0xa0000000, 0xf0000000, 0x88000000, 0xcc000000, 0xaa000000, 0xff000000,
    0x80800000, 0xc0c00000, 0xa0a00000, 0xf0f00000, 0x88880000, 0xcccc0000, 0xaaaa0000, 0xffff0000,
    0x80008000, 0xc000c000, 0xa000a000, 0xf000f000, 0x88008800, 0xcc00cc00, 0xaa00aa00, 0xff00ff00,
    0x80808080, 0xc0c0c0c0, 0xa0a0a0a0, 0xf0f0f0f0, 0x88888888, 0xcccccccc, 0xaaaaaaaa, 0xffffffff,
];

#[cfg(not(tarpaulin_include))]
struct Sobol<'a> {
    dimension: usize,
    direction_numbers: &'a [u32],
}

#[cfg(not(tarpaulin_include))]
impl<'a> Sobol<'a> {
    fn new() -> Self {
        Sobol {
            dimension: 2,
            direction_numbers: &DIRECTION_NUMBER,
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl<'a> LowDiscrepancySequence for Sobol<'a> {
    fn element(&self, n: usize, dim: usize) -> Result<f64, QrandCoreError> {
        if dim < self.dimension {
            let mut n = n;
            let mut value: u32 = 0;
            let mut index = 0;
            while n > 0 {
                if n & 1 == 1 {
                    let direction_number = self.direction_numbers[32 * dim + index];
                    value ^= direction_number;
                }
                index += 1;
                n >>= 1;
            }
            let two_pow_32: u64 = 0x100000000;
            Ok(value as f64 / two_pow_32 as f64)
        } else {
            Err(QrandCoreError::create_point_element_not_existing())
        }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    extern crate core;
    use core::assert_eq;
    use core::panic;

    fn gray_code(n: usize) -> usize {
        n ^ (n >> 1)
    }

    #[test]
    fn test_gray_code() {
        assert_eq!(0, gray_code(0));
        assert_eq!(1, gray_code(1));

        assert_eq!(2, gray_code(3));
        assert_eq!(3, gray_code(2));

        assert_eq!(6, gray_code(4));
        assert_eq!(7, gray_code(5));
        assert_eq!(5, gray_code(6));
        assert_eq!(4, gray_code(7));

        assert_eq!(12, gray_code(8));
        assert_eq!(13, gray_code(9));
        assert_eq!(15, gray_code(10));
        assert_eq!(14, gray_code(11));
        assert_eq!(10, gray_code(12));
        assert_eq!(11, gray_code(13));
        assert_eq!(9, gray_code(14));
        assert_eq!(8, gray_code(15));
    }

    const SOBOL_2D: [(f64, f64); 10] = [
        (0.0, 0.0),
        (0.5, 0.5),
        (0.75, 0.25),
        (0.25, 0.75),
        (0.375, 0.375),
        (0.875, 0.875),
        (0.625, 0.125),
        (0.125, 0.625),
        (0.1875, 0.3125),
        (0.6875, 0.8125),
    ];

    #[test]
    fn sobol_values_for_2d() {
        let sobol = Sobol::new();
        for n in 0..10 {
            assert_eq!(SOBOL_2D[n].0, sobol.element(gray_code(n), 0).unwrap_or(1.1));
            assert_eq!(SOBOL_2D[n].1, sobol.element(gray_code(n), 1).unwrap_or(1.1));
        }
    }
}
