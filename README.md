# Overview

`mem_viewer` is a Rust crate that provides a macro `view_mem!` to view the memory content of an arbitrary variable. It supports viewing memory content of different data types including integers, floating-point numbers, strings, pointers, vectors, boxed variables, and structs.

## Usage

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
mem_viewer = "0.1.3"
```

Then, in your Rust code, you can use the `view_mem!` macro to view the memory content of a variable. Here's an example:

```rust
use mem_viewer::*;

let my_var: u32 = 69;
view_mem!(my_var);
```

This will print the memory content of `my_var` to the console.

## Example Output

```none
Name: my_var
Type: u32
Size: 4 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x000019a20ff104  | 0x45 | 069  |  E
  0x000019a20ff105  | 0x00 | 000  |  NUL
  0x000019a20ff106  | 0x00 | 000  |  NUL
  0x000019a20ff107  | 0x00 | 000  |  NUL
```

## License

This crate is licensed under the MIT License.

## Contributing

Contributions are welcome! If you find any issues or have any suggestions, please open an issue or submit a pull request on [GitHub](https://github.com/ikhwanperwira/mem_viewer).

# Unit Test Report

## Code:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// Displays the memory content of a u64 variable.
    fn view_mem_u64(my_u64: u64) -> () {
        view_mem!(my_u64);
    }

    /// Displays the memory content of a f32 variable.
    fn view_mem_f32(my_f32: f32) -> () {
        view_mem!(my_f32);
    }

    /// Displays the memory content of a string variable.
    fn view_mem_str(my_str: &str) -> () {
        view_mem!(my_str); // Print address of the first character of the my_str
        view_mem!(*my_str); // Print actual content of my_str
    }

    /// Displays the memory content of a null pointer.
    fn view_mem_ptr<T>(my_ptr: *const T) -> () {
        view_mem!(my_ptr);
        unsafe { view_mem!(*my_ptr); }
    }

    /// Displays the memory content of a vector variable.
    fn view_mem_vec<T>(my_vec: Vec<T>) -> () {
        view_mem!(my_vec);
        view_mem!(*my_vec);
    }

    /// Displays the memory content of a boxed variable.
    fn view_mem_box<T>(my_box: Box<T>) -> () {
        view_mem!(my_box);
        view_mem!(*my_box);
    }

    /// Displays the memory content of a vector of boxed variables.
    fn view_mem_vec_of_box<T>(my_vec_of_box: Vec<Box<T>>) -> () {
        view_mem!(my_vec_of_box);
        view_mem!(*my_vec_of_box);
        view_mem!(*my_vec_of_box[0]);
    }

    /// Displays the memory content of a struct variable.
    fn view_mem_struct<T>(my_struct: T) -> () {
        view_mem!(&my_struct);
        view_mem!(my_struct);
    }

    #[allow(dead_code)]
    struct MyStruct {
        a: u8,
        b: u16,
        c: u32,
    }

    #[test]
    fn u64_viewer() {
        println!("This should print the memory of the holy number 69.\n");
        assert_eq!(view_mem_u64(69), ());
    }

    #[test]
    fn f32_viewer() {
        println!("This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.\n");
        assert_eq!(view_mem_f32(3.14), ());
    }

    #[test]
    fn str_viewer() {
        println!("This should print the memory of 'Hello' and its address.\n");
        assert_eq!(view_mem_str("Hello"), ());
    }

    #[test]
    fn ptr_viewer() {
        println!("This should print the memory of a pointer.\n");
        assert_eq!(view_mem_ptr(&69), ());
    }

    #[test]
    fn vec_viewer() {
        println!("This should print the memory address of the vector and the memory of its elements.\n");
        assert_eq!(view_mem_vec(vec![69, 255, 254, 253, 70]), ());
    }

    #[test]
    fn box_viewer() {
        println!("This should print the memory address of the box and the memory of its value.\n");
        assert_eq!(view_mem_box(Box::new(69)), ());
    }

    #[test]
    fn vec_of_box_viewer() {
        println!("This should print the memory address of the vector of boxes and the memory of its elements.\n");
        assert_eq!(view_mem_vec_of_box(vec![Box::new(69), Box::new(255), Box::new(254), Box::new(253), Box::new(70)]), ());
    }

    #[test]
    fn struct_viewer() {
        println!("This should print the memory address of the struct and the memory of its fields.\n");
        assert_eq!(view_mem_struct(
            MyStruct {
                a: 69,
                b: 255,
                c: 70,
            }), ());
    }
}
```

## Output:
```none
running 8 tests
test tests::f32_viewer ... ok
test tests::box_viewer ... ok
test tests::str_viewer ... ok
test tests::u64_viewer ... ok
test tests::struct_viewer ... ok
test tests::ptr_viewer ... ok
test tests::vec_of_box_viewer ... ok
test tests::vec_viewer ... ok

successes:

---- tests::f32_viewer stdout ----
This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.

Name: my_f32
Type: f32
Size: 4 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x00004428fff274  | 0xc3 | 195  |  ...
  0x00004428fff275  | 0xf5 | 245  |  ...
  0x00004428fff276  | 0x48 | 072  |  H
  0x00004428fff277  | 0x40 | 064  |  @


---- tests::box_viewer stdout ----
This should print the memory address of the box and the memory of its value.

Name: my_box
Type: alloc::boxed::Box<i32>
Size: 8 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x00004428dfed80  | 0x80 | 128  |  ...
  0x00004428dfed81  | 0x5c | 092  |  \
  0x00004428dfed82  | 0xd6 | 214  |  ...
  0x00004428dfed83  | 0x46 | 070  |  F
  0x00004428dfed84  | 0x02 | 002  |  STX
  0x00004428dfed85  | 0x02 | 002  |  STX
  0x00004428dfed86  | 0x00 | 000  |  NUL
  0x00004428dfed87  | 0x00 | 000  |  NUL

Name: *my_box
Type: i32
Size: 4 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x00020246d65c80  | 0x45 | 069  |  E
  0x00020246d65c81  | 0x00 | 000  |  NUL
  0x00020246d65c82  | 0x00 | 000  |  NUL
  0x00020246d65c83  | 0x00 | 000  |  NUL


---- tests::str_viewer stdout ----
This should print the memory of 'Hello' and its address.

Name: my_str
Type: &str
Size: 16 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x000044293ff268  | 0x50 | 080  |  P
  0x000044293ff269  | 0x94 | 148  |  ...
  0x000044293ff26a  | 0xec | 236  |  ...
  0x000044293ff26b  | 0x1d | 029  |  GS
  0x000044293ff26c  | 0xf6 | 246  |  ...
  0x000044293ff26d  | 0x7f | 127  |  DEL
  0x000044293ff26e  | 0x00 | 000  |  NUL
  0x000044293ff26f  | 0x00 | 000  |  NUL
  0x000044293ff270  | 0x05 | 005  |  ENQ
  0x000044293ff271  | 0x00 | 000  |  NUL
  0x000044293ff272  | 0x00 | 000  |  NUL
  0x000044293ff273  | 0x00 | 000  |  NUL
  0x000044293ff274  | 0x00 | 000  |  NUL
  0x000044293ff275  | 0x00 | 000  |  NUL
  0x000044293ff276  | 0x00 | 000  |  NUL
  0x000044293ff277  | 0x00 | 000  |  NUL

Name: *my_str
Type: str
Size: 5 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x007ff61dec9450  | 0x48 | 072  |  H
  0x007ff61dec9451  | 0x65 | 101  |  e
  0x007ff61dec9452  | 0x6c | 108  |  l
  0x007ff61dec9453  | 0x6c | 108  |  l
  0x007ff61dec9454  | 0x6f | 111  |  o


---- tests::u64_viewer stdout ----
This should print the memory of the holy number 69.

Name: my_u64
Type: u64
Size: 8 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x000044297fef60  | 0x45 | 069  |  E
  0x000044297fef61  | 0x00 | 000  |  NUL
  0x000044297fef62  | 0x00 | 000  |  NUL
  0x000044297fef63  | 0x00 | 000  |  NUL
  0x000044297fef64  | 0x00 | 000  |  NUL
  0x000044297fef65  | 0x00 | 000  |  NUL
  0x000044297fef66  | 0x00 | 000  |  NUL
  0x000044297fef67  | 0x00 | 000  |  NUL


---- tests::struct_viewer stdout ----
This should print the memory address of the struct and the memory of its fields.

Name: &my_struct
Type: &mem_viewer::tests::MyStruct
Size: 8 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x000044295ff3b0  | 0x10 | 016  |  DLE
  0x000044295ff3b1  | 0xf3 | 243  |  ...
  0x000044295ff3b2  | 0x5f | 095  |  _
  0x000044295ff3b3  | 0x29 | 041  |  )
  0x000044295ff3b4  | 0x44 | 068  |  D
  0x000044295ff3b5  | 0x00 | 000  |  NUL
  0x000044295ff3b6  | 0x00 | 000  |  NUL
  0x000044295ff3b7  | 0x00 | 000  |  NUL

Name: my_struct
Type: mem_viewer::tests::MyStruct
Size: 8 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x000044295ff310  | 0x46 | 070  |  F
  0x000044295ff311  | 0x00 | 000  |  NUL
  0x000044295ff312  | 0x00 | 000  |  NUL
  0x000044295ff313  | 0x00 | 000  |  NUL
  0x000044295ff314  | 0xff | 255  |  ...
  0x000044295ff315  | 0x00 | 000  |  NUL
  0x000044295ff316  | 0x45 | 069  |  E
  0x000044295ff317  | 0x00 | 000  |  NUL


---- tests::ptr_viewer stdout ----
This should print the memory of a pointer.

Name: my_ptr
Type: *const i32
Size: 8 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x000044291fefa0  | 0xb0 | 176  |  ...
  0x000044291fefa1  | 0x94 | 148  |  ...
  0x000044291fefa2  | 0xec | 236  |  ...
  0x000044291fefa3  | 0x1d | 029  |  GS
  0x000044291fefa4  | 0xf6 | 246  |  ...
  0x000044291fefa5  | 0x7f | 127  |  DEL
  0x000044291fefa6  | 0x00 | 000  |  NUL
  0x000044291fefa7  | 0x00 | 000  |  NUL

Name: *my_ptr
Type: i32
Size: 4 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x007ff61dec94b0  | 0x45 | 069  |  E
  0x007ff61dec94b1  | 0x00 | 000  |  NUL
  0x007ff61dec94b2  | 0x00 | 000  |  NUL
  0x007ff61dec94b3  | 0x00 | 000  |  NUL


---- tests::vec_of_box_viewer stdout ----
This should print the memory address of the vector of boxes and the memory of its elements.

Name: my_vec_of_box
Type: alloc::vec::Vec<alloc::boxed::Box<i32>>
Size: 24 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x00004428dfed98  | 0x05 | 005  |  ENQ
  0x00004428dfed99  | 0x00 | 000  |  NUL
  0x00004428dfed9a  | 0x00 | 000  |  NUL
  0x00004428dfed9b  | 0x00 | 000  |  NUL
  0x00004428dfed9c  | 0x00 | 000  |  NUL
  0x00004428dfed9d  | 0x00 | 000  |  NUL
  0x00004428dfed9e  | 0x00 | 000  |  NUL
  0x00004428dfed9f  | 0x00 | 000  |  NUL
  0x00004428dfeda0  | 0x50 | 080  |  P
  0x00004428dfeda1  | 0x34 | 052  |  4
  0x00004428dfeda2  | 0xd6 | 214  |  ...
  0x00004428dfeda3  | 0x46 | 070  |  F
  0x00004428dfeda4  | 0x02 | 002  |  STX
  0x00004428dfeda5  | 0x02 | 002  |  STX
  0x00004428dfeda6  | 0x00 | 000  |  NUL
  0x00004428dfeda7  | 0x00 | 000  |  NUL
  0x00004428dfeda8  | 0x05 | 005  |  ENQ
  0x00004428dfeda9  | 0x00 | 000  |  NUL
  0x00004428dfedaa  | 0x00 | 000  |  NUL
  0x00004428dfedab  | 0x00 | 000  |  NUL
  0x00004428dfedac  | 0x00 | 000  |  NUL
  0x00004428dfedad  | 0x00 | 000  |  NUL
  0x00004428dfedae  | 0x00 | 000  |  NUL
  0x00004428dfedaf  | 0x00 | 000  |  NUL

Name: *my_vec_of_box
Type: [alloc::boxed::Box<i32>]
Size: 40 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x00020246d63450  | 0x10 | 016  |  DLE
  0x00020246d63451  | 0x5d | 093  |  ]
  0x00020246d63452  | 0xd6 | 214  |  ...
  0x00020246d63453  | 0x46 | 070  |  F
  0x00020246d63454  | 0x02 | 002  |  STX
  0x00020246d63455  | 0x02 | 002  |  STX
  0x00020246d63456  | 0x00 | 000  |  NUL
  0x00020246d63457  | 0x00 | 000  |  NUL
  0x00020246d63458  | 0x80 | 128  |  ...
  0x00020246d63459  | 0x5c | 092  |  \
  0x00020246d6345a  | 0xd6 | 214  |  ...
  0x00020246d6345b  | 0x46 | 070  |  F
  0x00020246d6345c  | 0x02 | 002  |  STX
  0x00020246d6345d  | 0x02 | 002  |  STX
  0x00020246d6345e  | 0x00 | 000  |  NUL
  0x00020246d6345f  | 0x00 | 000  |  NUL
  0x00020246d63460  | 0xa0 | 160  |  ...
  0x00020246d63461  | 0x5d | 093  |  ]
  0x00020246d63462  | 0xd6 | 214  |  ...
  0x00020246d63463  | 0x46 | 070  |  F
  0x00020246d63464  | 0x02 | 002  |  STX
  0x00020246d63465  | 0x02 | 002  |  STX
  0x00020246d63466  | 0x00 | 000  |  NUL
  0x00020246d63467  | 0x00 | 000  |  NUL
  0x00020246d63468  | 0x80 | 128  |  ...
  0x00020246d63469  | 0x5d | 093  |  ]
  0x00020246d6346a  | 0xd6 | 214  |  ...
  0x00020246d6346b  | 0x46 | 070  |  F
  0x00020246d6346c  | 0x02 | 002  |  STX
  0x00020246d6346d  | 0x02 | 002  |  STX
  0x00020246d6346e  | 0x00 | 000  |  NUL
  0x00020246d6346f  | 0x00 | 000  |  NUL
  0x00020246d63470  | 0xc0 | 192  |  ...
  0x00020246d63471  | 0x5c | 092  |  \
  0x00020246d63472  | 0xd6 | 214  |  ...
  0x00020246d63473  | 0x46 | 070  |  F
  0x00020246d63474  | 0x02 | 002  |  STX
  0x00020246d63475  | 0x02 | 002  |  STX
  0x00020246d63476  | 0x00 | 000  |  NUL
  0x00020246d63477  | 0x00 | 000  |  NUL

Name: *my_vec_of_box[0]
Type: i32
Size: 4 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x00020246d65d10  | 0x45 | 069  |  E
  0x00020246d65d11  | 0x00 | 000  |  NUL
  0x00020246d65d12  | 0x00 | 000  |  NUL
  0x00020246d65d13  | 0x00 | 000  |  NUL


---- tests::vec_viewer stdout ----
This should print the memory address of the vector and the memory of its elements.

Name: my_vec
Type: alloc::vec::Vec<i32>
Size: 24 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x00004428dff068  | 0x05 | 005  |  ENQ
  0x00004428dff069  | 0x00 | 000  |  NUL
  0x00004428dff06a  | 0x00 | 000  |  NUL
  0x00004428dff06b  | 0x00 | 000  |  NUL
  0x00004428dff06c  | 0x00 | 000  |  NUL
  0x00004428dff06d  | 0x00 | 000  |  NUL
  0x00004428dff06e  | 0x00 | 000  |  NUL
  0x00004428dff06f  | 0x00 | 000  |  NUL
  0x00004428dff070  | 0x90 | 144  |  ...
  0x00004428dff071  | 0x8f | 143  |  ...
  0x00004428dff072  | 0xd6 | 214  |  ...
  0x00004428dff073  | 0x46 | 070  |  F
  0x00004428dff074  | 0x02 | 002  |  STX
  0x00004428dff075  | 0x02 | 002  |  STX
  0x00004428dff076  | 0x00 | 000  |  NUL
  0x00004428dff077  | 0x00 | 000  |  NUL
  0x00004428dff078  | 0x05 | 005  |  ENQ
  0x00004428dff079  | 0x00 | 000  |  NUL
  0x00004428dff07a  | 0x00 | 000  |  NUL
  0x00004428dff07b  | 0x00 | 000  |  NUL
  0x00004428dff07c  | 0x00 | 000  |  NUL
  0x00004428dff07d  | 0x00 | 000  |  NUL
  0x00004428dff07e  | 0x00 | 000  |  NUL
  0x00004428dff07f  | 0x00 | 000  |  NUL

Name: *my_vec
Type: [i32]
Size: 20 bytes
       Address      | Hex  | Dec  | ASCII
-----------------------------------------
  0x00020246d68f90  | 0x45 | 069  |  E
  0x00020246d68f91  | 0x00 | 000  |  NUL
  0x00020246d68f92  | 0x00 | 000  |  NUL
  0x00020246d68f93  | 0x00 | 000  |  NUL
  0x00020246d68f94  | 0xff | 255  |  ...
  0x00020246d68f95  | 0x00 | 000  |  NUL
  0x00020246d68f96  | 0x00 | 000  |  NUL
  0x00020246d68f97  | 0x00 | 000  |  NUL
  0x00020246d68f98  | 0xfe | 254  |  ...
  0x00020246d68f99  | 0x00 | 000  |  NUL
  0x00020246d68f9a  | 0x00 | 000  |  NUL
  0x00020246d68f9b  | 0x00 | 000  |  NUL
  0x00020246d68f9c  | 0xfd | 253  |  ...
  0x00020246d68f9d  | 0x00 | 000  |  NUL
  0x00020246d68f9e  | 0x00 | 000  |  NUL
  0x00020246d68f9f  | 0x00 | 000  |  NUL
  0x00020246d68fa0  | 0x46 | 070  |  F
  0x00020246d68fa1  | 0x00 | 000  |  NUL
  0x00020246d68fa2  | 0x00 | 000  |  NUL
  0x00020246d68fa3  | 0x00 | 000  |  NUL


successes:
    tests::box_viewer
    tests::f32_viewer
    tests::ptr_viewer
    tests::str_viewer
    tests::struct_viewer
    tests::u64_viewer
    tests::vec_of_box_viewer
    tests::vec_viewer

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
