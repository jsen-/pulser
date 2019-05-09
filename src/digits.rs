use ryu::Buffer;

pub trait Digits {
    type IntoIter;
    fn digits(&self) -> Self::IntoIter;
}

trait NumberOfDigits {
    fn number_of_digits(&self) -> u8;
}

macro_rules! impl_number_of_digits {
    ($ty:ident) => {
        impl NumberOfDigits for $ty {
            fn number_of_digits(&self) -> u8 {
                match *self {
                    0 => 1,
                    std::$ty::MAX => (std::$ty::MAX as f64).log10().ceil() as _,
                    _ => ((self + 1) as f64).log10().ceil() as _,
                }
            }
        }
    };
}

impl_number_of_digits!(usize);
impl_number_of_digits!(u8);
impl_number_of_digits!(u16);
impl_number_of_digits!(u32);
impl_number_of_digits!(u64);
impl_number_of_digits!(u128);

macro_rules! digit_iterator {
    ($name:ident, $ty:ty) => {
        pub struct $name {
            digits: u8,
            number: $ty,
        }
        impl Iterator for $name {
            type Item = u8;
            fn next(&mut self) -> Option<Self::Item> {
                if self.digits == 0 {
                    None
                } else {
                    self.digits -= 1;
                    let exp = (10 as $ty).pow(self.digits as u32) as $ty;
                    let ret = self.number / exp;
                    self.number %= exp;
                    Some(ret as u8 + b'0')
                }
            }
        }
        impl Digits for $ty {
            type IntoIter = $name;
            fn digits(&self) -> Self::IntoIter {
                $name {
                    digits: self.number_of_digits(),
                    number: *self,
                }
            }
        }
    };
    ($name:ident, $ty:ty, $output:ty) => {
        pub struct $name {
            digits: u8,
            number: $output,
        }
        impl Iterator for $name {
            type Item = u8;
            fn next(&mut self) -> Option<Self::Item> {
                if self.digits == 0 {
                    None
                } else {
                    if self.digits & 128 != 0 {
                        self.digits ^= 128;
                        Some(b'-')
                    } else {
                        self.digits -= 1;
                        let exp = (10 as $ty).pow(self.digits as u32) as $output;
                        let ret = self.number / exp;
                        self.number %= exp;
                        Some(ret as u8 + b'0')
                    }
                }
            }
        }
        impl Digits for $ty {
            type IntoIter = $name;
            fn digits(&self) -> Self::IntoIter {
                let number = *self;
                if number < 0 {
                    let number = number.abs() as $output;
                    $name {
                        digits: number.number_of_digits() | 128,
                        number,
                    }
                } else {
                    let number = number as $output;
                    $name {
                        digits: number.number_of_digits(),
                        number,
                    }
                }
            }
        }
    };
}

digit_iterator!(UsizeIterator, usize);
digit_iterator!(IsizeIterator, isize, usize);
digit_iterator!(U8Iterator, u8);
digit_iterator!(I8Iterator, i8, u8);
digit_iterator!(U16Iterator, u16);
digit_iterator!(I16Iterator, i16, u16);
digit_iterator!(U32Iterator, u32);
digit_iterator!(I32Iterator, i32, u32);
digit_iterator!(U64Iterator, u64);
digit_iterator!(I64Iterator, i64, u64);
digit_iterator!(U128Iterator, u128);
digit_iterator!(I128Iterator, i128, u128);

pub struct F64Iterator {
    buffer: [u8; 24],
    len: u8,
}

impl Digits for f64 {
    type IntoIter = F64Iterator;
    fn digits(&self) -> Self::IntoIter {
        let mut b = unsafe { std::mem::zeroed::<[u8; 24]>() };

        let len = if self.is_nan() {
            if self.is_sign_negative() {
                b[0..4].copy_from_slice("NaN-".as_bytes());
                4
            } else {
                b[0..3].copy_from_slice("NaN".as_bytes());
                3
            }
        } else if self.is_infinite() {
            if self.is_sign_negative() {
                b[0..9].copy_from_slice("ytinifnI-".as_bytes());
                9
            } else {
                b[0..8].copy_from_slice("ytinifnI".as_bytes());
                8
            }
        } else {
            let mut buffer = Buffer::new();
            let s = buffer.format(*self);

            // this is safe as u8 has no destructor
            for (offset, byte) in s.bytes().enumerate() {
                b[s.len() - offset] = byte;
            }
            s.len() as u8
        };

        F64Iterator {
            buffer: b,
            len: len,
        }
    }
}

impl Iterator for F64Iterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let x = self.buffer[self.len as usize];
            Some(x)
        }
    }
}

impl Digits for f32 {
    type IntoIter = F64Iterator;
    fn digits(&self) -> Self::IntoIter {
        (*self as f64).digits()
    }
}
