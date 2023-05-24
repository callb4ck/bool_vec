use std::fmt::{Binary, Debug, Write};

/// A vector that can store 8 booleans in a single byte
///
/// You can use the `BoolVec::new()`, `BoolVec::default()` and `BoolVec::with_size()`
/// functions to initialize a new BoolVec.
///
/// Or you can use the `bool_vec::boolvec![]` macro,
/// which works exactly like the `vec![]` macro.
///
/// ---
///
/// # Capacity
/// Capacity is always a multiple of 8, check BoolVec.capacity() docs for more infos.
///
/// # Length
/// Length works like the length of a normal Vec.
///
/// ---
/// # Formatting specifiers
///
/// ## You can debug print and pretty print:
/// ```rust
/// use bool_vec::boolvec;
///
/// let bv = boolvec![true; 16];
///
/// // Debug print the vector
/// println!("{bv:?}");
///
/// // Pretty-print the vector. Prints 8 values for each line
/// println!("{bv:#?}");
/// ```
///
/// ## You can print the underlying bytes of the BoolVec if you need them
/// ```rust
/// use bool_vec::boolvec;
///
/// let mut bv = boolvec![true; 9];
/// bv.set(2, false).unwrap();
///
/// assert_eq!(format!("{bv:b}"), "[11011111, 10000000]")
/// // You can also apply padding and pretty printing. See formatting specifiers.
/// ```
#[derive(Default)]
pub struct BoolVec {
    /// The underlying vector holding the bytes
    bytes: Vec<u8>,

    /// The maximum capacity of the vector in bits (how many values the BoolVec can hold without reallocating)
    capacity: usize,

    /// The length of the vector in bits (the number of values in the BoolVec)
    length: usize,
}

/// Value used for indexing bytes inside BoolVec.bytes and Bits inside
/// a single element of BoolVec.bytes
struct BoolIndex {
    /// Index for BoolVec.bytes
    byte_index: usize,

    /// Index for bits inside a single element of BoolVec.bytes
    bit_index: u8,
}

/// Iterator referencing to a BoolVec
pub struct BoolVecIter<'a> {
    vec: &'a BoolVec,
    counter: usize,
}

impl BoolIndex {
    /// Create a BoolIndex from a "typical" intenger index
    fn from(int_index: usize) -> Self {
        BoolIndex {
            byte_index: int_index / 8,
            bit_index: (int_index % 8) as u8,
        }
    }
}

impl BoolVec {
    /// Allocate empty BoolVec with capacity: 0 and len: 0
    /// ```rust
    /// use bool_vec::{boolvec, BoolVec};
    /// let bv = BoolVec::new();
    ///
    /// assert_eq!(bv, boolvec![]);
    /// ```
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            capacity: 0,
            length: 0,
        }
    }

    #[allow(clippy::slow_vector_initialization)]
    /// Allocate empty BoolVec with specified capacity and len: 0
    /// ```rust
    /// use bool_vec::{boolvec, BoolVec};
    /// let bv1 = BoolVec::with_capacity(3);
    /// let mut bv2 = boolvec![true; 3];
    /// bv2.pop();
    /// bv2.pop();
    /// bv2.pop();
    ///
    /// assert_eq!(bv1.capacity(), 8);
    /// assert_eq!(bv1.capacity(), bv2.capacity());
    /// ```
    /// To see why capacity in this case is 8 please do check BoolVec::capacity() documentation
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        let byte_capacity = ((capacity - 1) / 8) + 1;
        let mut bytes = Vec::with_capacity(byte_capacity);

        bytes.resize(byte_capacity, 0);

        Self {
            bytes,
            capacity: byte_capacity * 8,
            length: 0,
        }
    }

    /// Create BoolVec from a slice or vector of booleans
    /// ```rust
    /// use bool_vec::{boolvec, BoolVec};
    /// let bv = BoolVec::from([true, false, true]);
    ///
    /// assert_eq!(bv, boolvec![true, false, true]);
    /// ```
    pub fn from<S: AsRef<[bool]>>(slice: S) -> Self {
        let slice = slice.as_ref();

        let mut bool_vec = Self::with_capacity(slice.len());

        for b in slice {
            bool_vec.push(*b);
        }

        bool_vec
    }

    /// Get bool value from a BoolVec. Returns None if index overflows BoolVec.len()
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let bv = boolvec![true, false, true];
    ///
    /// assert_eq!(bv.get(0), Some(true));
    /// assert_eq!(bv.get(4), None);
    /// ```
    pub fn get(&self, int_index: usize) -> Option<bool> {
        let index = BoolIndex::from(int_index);

        if int_index >= self.length {
            return None;
        }

        Some((self.bytes[index.byte_index] << index.bit_index) & 128 == 128)
    }

    /// Set bool value in vector. Returns None if index overflows BoolVec.len()
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let mut bv = boolvec![true, false, true];
    /// bv.set(0, false);
    ///
    /// assert_eq!(bv.get(0), Some(false));
    /// assert_eq!(bv.set(4, true), None);
    /// ```
    pub fn set(&mut self, int_index: usize, value: bool) -> Option<()> {
        let index = BoolIndex::from(int_index);

        if int_index >= self.length {
            return None;
        }

        let byte = &mut self.bytes[index.byte_index];

        if value {
            // Assign one to that single bit
            *byte |= 1 << (7 - index.bit_index);
        } else {
            // Assign zero to that single bit
            *byte &= !(1 << (7 - index.bit_index));
        }

        Some(())
    }

    /// Alters BoolVec by negating the value at the specified index.
    /// Returns None if index overflows BoolVec.len(), otherwise returns Some(negated value)
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let mut bv = boolvec![true, false, true];
    ///
    /// assert_eq!(bv.negate(0), Some(false));
    /// assert_eq!(bv.get(0), Some(false));
    /// ```
    pub fn negate(&mut self, int_index: usize) -> Option<bool> {
        // Index overflow check is done by self.get()
        let negated_value = !self.get(int_index)?;
        let _ = self.set(int_index, negated_value);

        Some(negated_value)
    }

    /// Appends a bool to the back of a BoolVec
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let mut bv = boolvec![true; 8];
    ///
    /// assert_eq!(bv.capacity(), bv.bytes_capacity()*8);
    /// assert_eq!(bv.len(), 8);
    /// assert_eq!(bv.bytes_len(), 1);
    ///
    /// bv.push(false);
    ///
    /// assert_eq!(bv.capacity(), bv.bytes_capacity()*8);
    /// assert_eq!(bv.len(), 9);
    /// assert_eq!(bv.bytes_len(), 2);
    ///
    /// assert_eq!(bv.get(8), Some(false));
    /// ```
    pub fn push(&mut self, value: bool) {
        self.length += 1;

        if self.length > self.capacity {
            self.bytes.push(0);
            self.capacity = self.bytes.capacity()*8;
        }

        let _ = self.set(self.length - 1, value);
    }

    /// Removes the last element from a BoolVec and returns it, or None if it is empty
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let mut bv = boolvec![true; 8];
    ///
    /// assert_eq!(bv.capacity(), 8);
    /// assert_eq!(bv.len(), 8);
    ///
    /// assert_eq!(bv.pop(), Some(true));
    ///
    /// assert_eq!(bv.capacity(), 8);
    /// assert_eq!(bv.len(), 7);
    /// ```
    pub fn pop(&mut self) -> Option<bool> {
        // self.get()'s check cannot be trusted here since it would require 0u8-1
        if self.length == 0 {
            return None;
        }

        let to_return = self.get(self.length - 1);

        // This self.set() is necessary for PartialEq to keep working properly after a pop() operation
        self.set(self.length - 1, false)?;

        // The previous methods need this attribe to stay unchanged
        self.length -= 1;

        to_return
    }

    /// Returns vector capacity (how many booleans the BoolVec can hold without reallocating).
    /// Capacity will always be a power of 8 since a single boolean takes 1 bit of space and you
    /// can only allocate a minimum of 1 byte at a time.
    ///
    /// For example 8 booleans here take a single byte of space.
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    ///
    /// let bv = boolvec![true; 10];
    ///
    /// assert_eq!(bv.capacity(), 16);
    /// assert_eq!(bv.bytes_len(), 2);
    /// ```
    /// In the example 2 bytes are currently allocated, please read BoolVec::bytes_len()
    /// documentation for further details.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns BoolVec's length (the number of booleans in the BoolVec)
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    ///
    /// let bv = boolvec![true; 10];
    ///
    /// assert_eq!(bv.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns true if the BoolVec contains no booleans
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let bv = boolvec![];
    ///
    /// assert!(bv.is_empty());
    /// ```
    ///
    /// ---
    ///
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let mut bv = boolvec![true];
    /// bv.pop();
    ///
    /// assert!(bv.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the capacity of the underlying vector that stores the values.
    /// This function is here just for testing purposes
    ///
    /// This capacity is expressed in bytes, while the normal capacity of BoolVec is expressed in bits.
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let mut bv = boolvec![true; 8];
    /// bv.push(true);
    ///
    /// assert_eq!(bv.capacity(), bv.bytes_capacity()*8);
    /// ```
    pub fn bytes_capacity(&self) -> usize {
        self.bytes.capacity()
    }

    /// Returns the number of elements of the underlying vector that stores the values.
    /// This function is here just for testing purposes
    ///
    /// This length is expressed in bytes, while the normal length of BoolVec is expressed in bits.
    /// ```rust
    /// use bool_vec::boolvec;
    ///
    /// let mut bv = boolvec![true; 8];
    ///
    /// assert_eq!(bv.len(), 8);
    /// assert_eq!(bv.bytes_len(), 1);
    ///
    /// bv.push(true);
    ///
    /// assert_eq!(bv.len(), 9);
    /// assert_eq!(bv.bytes_len(), 2);
    /// ```
    pub fn bytes_len(&self) -> usize {
        self.bytes.len()
    }

    /// Copies BoolVec data into a Vec<bool>
    /// ```rust
    /// use bool_vec::boolvec;
    /// 
    /// let bv = boolvec![true, false, true];
    ///
    /// let vector = bv.into_vector();
    ///
    /// assert_eq!(vector, vec![true, false, true]);
    /// ```
    ///
    /// To achieve the opposite (getting a `BoolVec` from a `Vec<bool>` or a `&[bool]`),
    /// please see BoolVec::from()
    ///
    /// *WARNING: This might require a non indifferent amount of memory if you are working
    /// with a large amount of data, please consider trying to work with BoolVec directly
    /// if you can*
    pub fn into_vector(&self) -> Vec<bool> {
        let mut new_vec = Vec::with_capacity(self.len());

        for b in self {
            new_vec.push(b);
        }

        new_vec
    }
}

impl PartialEq for BoolVec {
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.length {
            return false;
        }

        self.bytes == other.bytes
    }
}

impl Debug for BoolVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.length == 0 {
            return f.write_str("[]");
        }

        if !f.alternate() {
            return f.debug_list().entries(self).finish();
        }

        f.write_str("[\n")?;

        let mut counter = 0;

        for b in self {
            if counter % 8 == 0 {
                f.write_str("    ")?;
            }

            f.write_str(b.to_string().as_str())?;

            if b {
                f.write_char(' ')?;
            }

            counter += 1;
            if counter < self.length {
                f.write_str(", ")?;
            }

            if counter % 8 == 0 {
                f.write_char('\n')?;
            }
        }

        f.write_str("\n]")
    }
}

impl Binary for BoolVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.length == 0 {
            return f.write_str("[]");
        }

        let mut counter = 0;

        f.write_char('[')?;

        if f.alternate() {
            f.write_char('\n')?;
        }

        for byte in self.bytes.iter() {
            if f.alternate() {
                f.write_str("    ")?;
            }

            std::fmt::Binary::fmt(byte, f)?;

            counter += 1;
            if counter < self.bytes.len() {
                f.write_str(", ")?;
                if f.alternate() {
                    f.write_char('\n')?;
                }
            }
        }

        if f.alternate() {
            f.write_char('\n')?;
        }

        f.write_char(']')
    }
}

impl<'a> IntoIterator for &'a BoolVec {
    type Item = bool;

    type IntoIter = BoolVecIter<'a>;

    /// Convert BoolVec into an iterator
    fn into_iter(self) -> Self::IntoIter {
        BoolVecIter {
            vec: self,
            counter: 0,
        }
    }
}

impl<'a> Iterator for BoolVecIter<'a> {
    type Item = bool;

    /// Convert return next
    /// Advances the iterator and returns the next value.
    /// Returns None when iteration is finished.
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.get(self.counter)?;
        self.counter += 1;

        Some(item)
    }
}

#[macro_export]
/// Works exactly like vec![] macro.
///
/// ```rust
/// use bool_vec::{BoolVec, boolvec};
/// let mut bv1 = BoolVec::with_capacity(3);
/// bv1.push(true);
/// bv1.push(true);
/// bv1.push(true);
///
/// let bv2 = boolvec![true, true, true];
///
/// let bv3 = boolvec![true; 3];
///
/// assert_eq!(bv1, bv2);
/// assert_eq!(bv2, bv3);
/// assert_eq!(bv3, bv1);
/// ```
macro_rules! boolvec {
    [$($val:expr),*] => [
        {
            count_macro::count!{
                $(
                    let _ = $val;
                    let _ = _int_;
                )*

                    //Using count macro to be able to use BoolVec::with_capacity()
                    let mut vec = bool_vec::BoolVec::with_capacity(_int_);
            }

            $(
                vec.push($val);
            )*

        vec
        }
    ];

    [$val:expr; $length:expr] => [
        {
            let mut vec = bool_vec::BoolVec::with_capacity($length);

            for x in 0..$length {
                vec.push($val);
            }

        vec
        }
    ];
}
