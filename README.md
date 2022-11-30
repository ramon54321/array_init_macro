<div align="center">
    <span><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1920px-Rust_programming_language_black_logo.svg.png" width="100"></span>
</div>

### Array Init Macro

Provides a simple macro to initialize an array given a type, size and value.

[Crate on Crates.io](https://crates.io/crates/array_init_macro)

```rust
use array_init_macro::arr;

fn main() {
    // [1, 1, 1, 1]
    let array0 = arr![u8; 4; 1];
    
    // [1, 1, 1, 1]
    let array1 = arr![u32; 4; 1];

    // [Vec[1, 2], Vec[1, 2], Vec[1, 2], Vec[1, 2]]
    let array2 = arr![Vec<u8>; 4; vec![1, 2]];
}
```

