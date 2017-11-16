use std::fmt;

fn main() {
    let mut bits = BitVec::with_capacity(17);
    bits.set(0, true);
    bits.set(1, true);
    bits.set(3, true);
    bits.set(8, true);
    bits.set(10, true);
    bits.set(16, true);
    println!("{}", bits);
    bits.set(0, false);
    bits.set(8, false);
    println!("{}", bits);
}

/// The number of bits in a single element of the bytes Vec in BitVec
/// Set to 8 because BitVec stores u8 values. If we stored u16, we would use 16 here
const BITS: usize = 8;

/// An implementation of a vector of bits that can be easily set individually
/// Note that this is meant to be a teaching implementation and so is not necessarily the most
/// sophisticated or performant way to do everything. The goal is to make the code understandable
/// and correct.
/// See the bit-vec crate and others on crates.io for more advanced implementations.
#[derive(Debug, Clone, Default)]
pub struct BitVec {
    bytes: Vec<u8>,
}

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // EXERCISE: Only print the bytes up to the length of bytes that have been pushed, not
        // the entire capacity. (Similar to how Vec prints its len, not its capacity)
        for byte in &self.bytes {
            write!(f, "{:0width$b}", byte, width = BITS)?;
        }
        Ok(())
    }
}

impl BitVec {
    /// Create a new BitVec with the given size in bits
    /// Note that the actual size of the vector is in bytes so `bits` will be rounded up to the
    /// nearest multiple of 8 >= bits.
    pub fn with_capacity(bits: usize) -> Self {
        if bits > 0 {
            // Ceiling division of bits / BITS so that we get >= bits capacity
            // From: https://stackoverflow.com/a/2745086/551904
            let capacity = 1 + ((bits - 1) / BITS);
            Self {
                bytes: vec![0; capacity],
            }
        }
        else {
            Default::default()
        }
    }

    /// Returns the size in bits of the array
    pub fn len(&self) -> usize {
        self.len_bytes() * BITS
    }

    /// Returns the size in bytes of the array
    pub fn len_bytes(&self) -> usize {
        self.bytes.len()
    }

    /// Set the bit at the given index to the given value
    ///
    /// Bits are set left to right, much like how an actual Vec might be indexed
    ///
    /// # Panics
    /// Panics if index >= len()
    pub fn set(&mut self, index: usize, value: bool) {
        let byte_index = index / BITS;
        let byte = &mut self.bytes[byte_index];
        let bit_index = index % BITS;
        // We want to make sure that bits are set left to right
        // This depends on the platform's endianess
        //TODO: Check this somehow
        let bit_index = if cfg!(target_endian = "big") {
            bit_index
        }
        else {
            BITS - 1 - bit_index
        };
        if value {
            // Set the bit to 1 by performing a bitwise OR with each bit in the byte.
            // `1 << bit_index` will make sure the bit_index bit is 1.
            // 1 OR anything will always result in 1, so the bit will be set to 1.
            *byte |= 1 << bit_index;
        }
        else {
            // Set the bit to 0 by performing a bitwise AND with each bit
            // `!(1 << bit_index)` sets the bit_index bit to 0 and the other bits to 1.
            // 1 AND x will always leave x as is.
            // 0 AND x will set the bit to 0.
            *byte &= !(1 << bit_index);
        }
    }
}
