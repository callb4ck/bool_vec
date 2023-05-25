# Description

This library exposes a vector called BoolVec which allows you to store 8 booleans in a single byte.
Basically a boolean only occupies a single bit.

# How to use

Please check out the documentation on [docs.rs](https://docs.rs/bool_vec/latest/) and the examples below

## Examples

### Initializing an empty BoolVec

To create a new `BoolVec` you can use either `BoolVec::new()` or `BoolVec::default()`:
```rust
use bool_vec::BoolVec;

let bv1 = BoolVec::new();
let bv2 = BoolVec::default();

assert_eq!(bv1, bv2);
```

Or, if you already know your desired capacity you can use `BoolVec::with_capacity(cap)`:
```rust
use bool_vec::BoolVec;

let bv = BoolVec::with_capacity(3);

assert!(bv.capacity() > 0);
```

### Initializing BoolVec from a Vec or slice

You can initialize a `BoolVec` from anything that implements `AsRef<[bool]>` with `BoolVec::from(S)`.
This includes vectors and slices:
```rust
use bool_vec::BoolVec;

let bv1 = BoolVec::from([true, false, true]);
let bv2 = BoolVec::from(vec![true, false, true]);

assert_eq!(bv1, bv2);
```

### Initializing using boolvec![] macro

Just like `Vec` with the `vec![]` macro, you can initialize a `BoolVec` with the `boolvec![]` macro:
```rust
use bool_vec::{BoolVec, boolvec};

let bv1 = BoolVec::new();
let bv2 = boolvec![];

assert_eq!(bv1, bv2);

let bv3 = boolvec![true, true, true];
let bv4 = boolvec![true; 3];

assert_eq!(bv3, bv4);
```

### Pushing values into the BoolVec

You can push booleans to the back of the `BoolVec` just like you would with a normal `Vec`:

```rust
use bool_vec::boolvec;

let mut bv = boolvec![true, false, true];

bv.push(true);

assert_eq!(bv, boolvec![true, false, true, true]);
```

### Popping values off the BoolVec

Again, just like with a normal `Vec`, you can remove items at the end of a `BoolVec` with `BoolVec.pop()`.
Do note that just like with `Vec`, removed values will be returned. If no value is found, `None` is returned instead:
```rust
use bool_vec::boolvec;

let mut bv1 = boolvec![true, false, true];
let mut bv2 = boolvec![];

assert_eq!(bv1.pop(), Some(true));
assert_eq!(bv2.pop(), None);

assert_eq!(bv1, boolvec![true, false]);
```

### Getting values from a BoolVec

You can get a value from a `BoolVec` with the `BoolVec.get(index)` method.
`None` will be returned if `index` is invalid:
```rust
use bool_vec::boolvec;

let bv = boolvec![true, false, true];

assert_eq!(bv.get(1), Some(false));
assert_eq!(bv.get(3), None);
```

### Changing values in a BoolVec

You can change the value of any `bool` inside a `BoolVec` with the `BoolVec.set(index, value)` method.
Just like with `BoolVec.get(index)`, `None` will be returned if `index` is invalid.
```rust
use bool_vec::boolvec;

let mut bv = boolvec![true, false, true];

assert_eq!(bv.set(0, false), Some(()) );
assert_eq!(bv.get(0), Some(false));

assert_eq!(bv.set(3, false), None);
```

### Negating values in a BoolVec

Negating a value is simple with the `BoolVec.negate(index)` method.
This will update your value in the BoolVec,
changing it either from `true` to `false`, or from `false` to `true`, and then return the negated value.
Again, `None` will be returned if `index` is invalid.
```rust
use bool_vec::boolvec;

let mut bv = boolvec![true, false, true];

assert_eq!(bv.negate(0), Some(false));
assert_eq!(bv.get(0), Some(false));

assert_eq!(bv.negate(3), None);
```

### Getting a Vec from a BoolVec
You can get a `Vec<bool>` from a `BoolVec` with the `BoolVec.into_vec()` method:
```rust
use bool_vec::boolvec;

let bv = boolvec![true, false, true];

let vector = vec![true, false, true];

assert_eq!(bv.into_vec(), vector);
```

*WARNING: It's recommended to try and work with `BoolVec` when possible. Converting to `Vec<bool>` might drastically increase your memory usage*

### Iteration
You can iterate using a for loop or convert your `BoolVec` into a `BoolVecIter` directly using `BoolVec.into_iter()`:
```rust
use bool_vec::boolvec;

let bv = boolvec![true; 3];

for boolean in &bv {
    assert_eq!(boolean, true);
}

let mut bv_iter = bv.into_iter();

while let Some(boolean) = bv_iter.next() {
    assert_eq!(boolean, true);
}
```

### Printing

You can either debug print and pretty print your `BoolVec`:
```rust
use bool_vec::boolvec;

let bv = boolvec![true; 3];

println!("{bv:?}");

println!("{bv:#?}"); // This will print up to 8 booleans in a single line
```

Or print the underlying bytes of your `BoolVec`:
```rust
use bool_vec::boolvec;

let mut bv = boolvec![true; 9];
bv.set(2, false).unwrap();

assert_eq!(format!("{bv:b}"), "[11011111, 10000000]")
```

It's ok if you don't understand this le latter, it's mostly for debug purposes and you don't need to concern with it.

### Other
Other methods you might already know from `Vec` are implemented, such as:
- `BoolVec.len()` to get the current length of the `BoolVec`;
- `BoolVec.capacity()` to get the capacity;
- `BoolVec.is_empty()` to check whether the `BoolVec` is empty or not;
