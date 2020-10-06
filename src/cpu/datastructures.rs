use std::ops::Add;
use std::ops::Sub;
use std::ops::BitOr;
use std::ops::BitAnd;
use std::ops::BitXor;
use std::ops::Shl;
use std::ops::Shr;

pub const CARRY_BIT: i16 = (1 << 8);

#[inline]
fn carry_bit(val: doubleword) -> bool {
    (val & CARRY_BIT) != 0
}

#[inline]
fn carry_bit_u16(val: u16) -> bool {
    (val & (CARRY_BIT as u16)) != 0
}

#[inline]
fn get_bit_at_u8(data: u8, n: u8) -> Option<bool> {
    match n {
        0..=7 => Some(data & (1 << n) != 0),
        _ => None,
    }
}

#[inline]
fn get_bit_at_u16(data: u16, n: u8) -> Option<bool> {
    match n {
        0..=15 => Some(data & (1 << n) != 0),
        _ => None,
    }
}

#[inline]
fn get_bit_at_u32(data: u32, n: u8) -> Option<bool> {
    match n {
        0..=31 => Some(data & (1 << n) != 0),
        _ => None,
    }
}

#[inline]
fn get_bit_at_u64(data: u64, n: u8) -> Option<bool> {
    match n {
        0..=63 => Some(data & (1 << n) != 0),
        _ => None,
    }
}

#[inline]
fn get_bit_at_i8(data: i8, n: u8) -> Option<bool> {
    match n {
        0..=7 => Some(data & (1 << n) != 0),
        _ => None,
    }
}

#[inline]
fn get_bit_at_i16(data: i16, n: u8) -> Option<bool> {
    match n {
        0..=15 => Some(data & (1 << n) != 0),
        _ => None,
    }
}

#[inline]
fn get_bit_at_i32(data: i32, n: u8) -> Option<bool> {
    match n {
        0..=31 => Some(data & (1 << n) != 0),
        _ => None,
    }
}

#[inline]
fn get_bit_at_i64(data: i64, n: u8) -> Option<bool> {
    match n {
        0..=63 => Some(data & (1 << n) != 0),
        _ => None,
    }
}

/// Trait reperesenting a carry-less (CL) addition
/// Carry-less means that it must over- or under-flow silently
pub trait ClAdd<O: Sized>: Sized {
    type Output;
    fn cl_add(self, other: O) -> Self::Output;
}


/// Reperesents a word in 6502 (i.e. a single byte).
/// Currently stored in native endianness
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct word {
    value: u8,
}

impl word {
    pub fn bit_at(&self, n: u8) -> Option<bool> {
        get_bit_at_u8(self.value, n)
    }

    pub fn from_6502_bytes(data: u8) -> Self {
        Self {
            value: data,
        }
    }

    #[inline]
    pub fn native_value(&self) -> u8 {
        self.value
    }

    #[inline]
    pub fn native_value_signed(&self) -> i8 {
        self.value as i8
    }

    #[inline]
    pub fn cpu_value(&self) -> u8 {
        u8::to_le(self.value)
    }

    #[inline]
    pub fn cpu_value_signed(&self) -> i8 {
        i8::to_le(self.value as i8)
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            value: 0,
        }
    }

    #[inline]
    pub fn as_doubleword(&self) -> doubleword {
        doubleword {
            value: self.value as u16
        }
    }

    #[inline]
    // Performs an unchecked shift left, along with returning the carry value (i.e. bit '8' of the result)
    pub fn logical_shift_left_carry(self, rhs: u16) -> (word, bool) {
        let intermediate: u16 = ((self.value as u16) << rhs) as u16;

        let ret = Self {
            value: intermediate as u8,
        };
        (ret, carry_bit_u16(intermediate))
    }

    #[inline]
    // Performs an unchecked shift riht, along with returning the carry value (i.e. bit '8' of the result)
    pub fn logical_shift_right_carry(self, rhs: u32) -> (word, bool) {
        let intermediate: u16 = (self.value as u16).rotate_right(rhs);

        let ret = Self {
            value: intermediate as u8,
        };
        // Check if last bit of original rotation is 1, i.e. the 8-bit carry
        (ret, intermediate & (0b1 << 15) != 0)
    }

    #[inline]
    /// TODO: add descritpion
    pub fn arith_shift_left_carry(self, rhs: u8) -> (word, bool) {
        let intermediate: u16 = ((self.value as i16) << (rhs as i16)) as u16;

        let ret = Self {
            value: intermediate as u8,
        };
        (ret, carry_bit_u16(intermediate))
    }

    #[inline]
    /// TODO: add descritpion
    pub fn arith_shift_right_carry(self, rhs: u8) -> (word, bool) {
        let carry = self.value & 0b1 != 0;
        let intermediate = (self.value as i8) >> (rhs as i8);
        let ret = Self {
            value: intermediate as u8,
        };
        (ret, carry)
    }

    #[inline]
    /// TODO: add descritpion
    pub fn rotate_left_carry(self, data: word, carry_bit: bool) -> (word, bool) {
        let ret_carry = data.bit_at(7).unwrap(); // Static use of function, error handling not required
        let ret_val = (data << 1) | carry_bit;
        (ret_val, ret_carry)
    }

    #[inline]
    /// TODO: add descritpion
    pub fn rotate_right_carry(self, data: word, carry_bit: bool) -> (word, bool) {
        let ret_carry = data.bit_at(0).unwrap(); // Static use of function, error handling not required
        let ret_val = (data >> 1) | ((carry_bit as u8) << 7);
        (ret_val, ret_carry)
    }
}

impl ClAdd<Self> for word {
    type Output = Self;

    #[inline]
    fn cl_add(self, other: Self) -> Self {
        Self {
            value: self.value.wrapping_add(other.value),
        }
    }
}

impl Add<Self> for word {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Add<u8> for word {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        Self {
            value: self.value + other,
        }
    }
}

impl Sub<u8> for word {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        Self {
            value: self.value - other,
        }
    }
}

impl Sub<word> for word {
    type Output = Self;

    fn sub(self, other: word) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl BitOr<word> for word {
    type Output = Self;

    fn bitor(self, other: word) -> Self {
        Self {
            value: self.value | other.value,
        }
    }
}

impl BitOr<u8> for word {
    type Output = Self;

    fn bitor(self, other: u8) -> Self {
        Self {
            value: self.value | other,
        }
    }
}

impl BitOr<bool> for word {
    type Output = Self;

    fn bitor(self, other: bool) -> Self {
        Self {
            value: self.value | (other as u8),
        }
    }
}

impl BitAnd<word> for word {
    type Output = Self;

    fn bitand(self, other: word) -> Self {
        Self {
            value: self.value & other.value,
        }
    }
}

impl BitAnd<u8> for word {
    type Output = Self;

    fn bitand(self, other: u8) -> Self {
        Self {
            value: self.value & other,
        }
    }
}

impl BitAnd<bool> for word {
    type Output = Self;

    fn bitand(self, other: bool) -> Self {
        Self {
            value: self.value & (other as u8),
        }
    }
}

impl BitXor<word> for word {
    type Output = Self;

    fn bitxor(self, other: word) -> Self {
        Self {
            value: self.value ^ other.value,
        }
    }
}

impl BitXor<u8> for word {
    type Output = Self;

    fn bitxor(self, other: u8) -> Self {
        Self {
            value: self.value ^ other,
        }
    }
}

impl Shl<u8> for word {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self {
        Self {
            value: self.value << rhs,
        }
    }
}

impl Shr<u8> for word {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self {
        Self {
            value: self.value >> rhs,
        }
    }
}

impl From<u8> for word {
    fn from(val: u8) -> Self {
        Self {
            value: val,
        }
    }
}

impl From<u16> for word {
    fn from(val: u16) -> Self {
        Self {
            value: val as u8,
        }
    }
}

impl From<i16> for word {
    fn from(val: i16) -> Self {
        Self {
            value: val as u8,
        }
    }
}

/// Represents a doubleword in 6502 (used for addressing and the PC)
/// Currently stored in native endianness
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct doubleword {
    value: u16,
}

impl doubleword {
    pub fn bit_at(&self, n: u8) -> Option<bool> {
        get_bit_at_u16(self.value, n)
    }

    pub fn from_6502_bytes(data: u16) -> Self {
        Self {
            value: u16::from_le(data),
        }
    }

    #[inline]
    pub fn native_value(&self) -> u16 {
        self.value
    }

    #[inline]
    pub fn cpu_value(&self) -> u16 {
        u16::to_le(self.value)
    }

    #[inline]
    pub fn as_addr(&self) -> usize {
        self.value as usize
    }

    pub fn from_words(hi: word, lo: word) -> Self {
        Self {
            value: u16::from_le_bytes([lo.native_value(), hi.native_value()]),
        }
    }

    pub fn to_words(&self) -> [word; 2] {
        let ret = u16::to_le_bytes(self.value);
        [word{value: ret[0]}, word{value: ret[1]}]
    }

    pub fn zero() -> Self {
        Self {
            value: 0,
        }
    }
}

impl ClAdd<Self> for doubleword {
    type Output = Self;

    #[inline]
    fn cl_add(self, other: Self) -> Self {
        Self {
            value: self.value.wrapping_add(other.value),
        }
    }
}

impl ClAdd<word> for doubleword {
    type Output = Self;

    #[inline]
    fn cl_add(self, other: word) -> Self {
        Self {
            value: self.value.wrapping_add(other.value as u16),
        }
    }
}

impl Add<Self> for doubleword {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value, 
        }
    }
}

impl Add<u16> for doubleword {
    type Output = Self;
    fn add(self, other: u16) -> Self {
        Self {
            value: self.value + other, 
        }
    }
}

impl Add<word> for doubleword {
    type Output = Self;
    fn add(self, other: word) -> Self {
        Self {
            value: self.value + (other.value as u16), 
        }
    }
}

impl Add<u8> for doubleword {
    type Output = Self;
    fn add(self, other: u8) -> Self {
        Self {
            value: self.value + (other as u16), 
        }
    }
}

impl BitAnd<i16> for doubleword {
    type Output = Self;

    fn bitand(self, other: i16) -> Self {
        Self {
            value: self.value & (other as u16),
        }
    }
}

impl Add<i16> for doubleword {
    type Output = Self;
    fn add(self, other: i16) -> Self {
        Self {
            value: (self.value as i16 + other) as u16, 
        }
    }
}

impl From<u16> for doubleword {
    fn from(val: u16) -> Self {
        Self {
            value: val,
        }
    }
}

impl PartialEq<i16> for doubleword {
    fn eq(&self, other: &i16) -> bool {
        self.value == (*other as u16)
    }
}