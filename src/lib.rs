//! # Overview
//!
//! `mem_viewer` is a Rust crate that provides a macro `view_mem!` to view the memory content of an arbitrary variable. It supports viewing memory content of different data types including integers, floating-point numbers, strings, pointers, vectors, boxed variables, and structs.
//!
//! ## Usage
//!
//! Add the following line to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! mem_viewer = "0.1.4"
//! ```
//!
//! Then, in your Rust code, you can use the `view_mem!` macro to view the memory content of a variable. Here's an example:
//!
//! ```rust
//! use mem_viewer::*;
//!
//! let my_var: u32 = 69;
//! view_mem!(my_var);
//! ```
//!
//! This will print the memory content of `my_var` to the console.
//!
//! ## Example Output
//!
//! ```none
//! Name: my_var
//! Type: u32
//! Size: 4 bytes
//!      Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//! 0x0000cd9fafef50 | 0x45 | 069 | 0b01000101 |  E
//! 0x0000cd9fafef51 | 0x00 | 000 | 0b00000000 |  NUL
//! 0x0000cd9fafef52 | 0x00 | 000 | 0b00000000 |  NUL
//! 0x0000cd9fafef53 | 0x00 | 000 | 0b00000000 |  NUL
//! ```
//!
//! ## License
//!
//! This crate is licensed under the MIT License.
//!
//! ## Contributing
//!
//! Contributions are welcome! If you find any issues or have any suggestions, please open an issue or submit a pull request on [GitHub](https://github.com/ikhwanperwira/mem_viewer).
//!
//! ## Authors
//!
//! - Muhammad Ikhwan Perwira <ikhwanperwira@gmail.com>
//! 
//! # Unit Test Report
//! 
//! ## Code:
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     /// Displays the memory content of a u64 variable.
//!     fn view_mem_u64(my_u64: u64) -> () {
//!         view_mem!(my_u64);
//!     }
//!
//!     /// Displays the memory content of a f32 variable.
//!     fn view_mem_f32(my_f32: f32) -> () {
//!         view_mem!(my_f32);
//!     }
//!
//!     /// Displays the memory content of a string variable.
//!     fn view_mem_str(my_str: &str) -> () {
//!         view_mem!(my_str); // Print address of the first character of the my_str
//!         view_mem!(*my_str); // Print actual content of my_str
//!     }
//!
//!     /// Displays the memory content of a null pointer.
//!     fn view_mem_ptr<T>(my_ptr: *const T) -> () {
//!         view_mem!(my_ptr);
//!         unsafe { view_mem!(*my_ptr); }
//!     }
//!
//!     /// Displays the memory content of a vector variable.
//!     fn view_mem_vec<T>(my_vec: Vec<T>) -> () {
//!         view_mem!(my_vec);
//!         view_mem!(*my_vec);
//!     }
//!
//!     /// Displays the memory content of a boxed variable.
//!     fn view_mem_box<T>(my_box: Box<T>) -> () {
//!         view_mem!(my_box);
//!         view_mem!(*my_box);
//!     }
//!
//!     /// Displays the memory content of a vector of boxed variables.
//!     fn view_mem_vec_of_box<T>(my_vec_of_box: Vec<Box<T>>) -> () {
//!         view_mem!(my_vec_of_box);
//!         view_mem!(*my_vec_of_box);
//!         view_mem!(*my_vec_of_box[0]);
//!     }
//!
//!     /// Displays the memory content of a struct variable.
//!     fn view_mem_struct<T>(my_struct: T) -> () {
//!         view_mem!(&my_struct);
//!         view_mem!(my_struct);
//!     }
//!
//!     #[allow(dead_code)]
//!     struct MyStruct {
//!         a: u8,
//!         b: u16,
//!         c: u32,
//!     }
//!
//!     #[test]
//!     fn u64_viewer() {
//!         println!("This should print the memory of the holy number 69.\n");
//!         assert_eq!(view_mem_u64(69), ());
//!     }
//!
//!     #[test]
//!     fn f32_viewer() {
//!         println!("This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.\n");
//!         assert_eq!(view_mem_f32(3.14), ());
//!     }
//!
//!     #[test]
//!     fn str_viewer() {
//!         println!("This should print the memory of 'Hello' and its address.\n");
//!         assert_eq!(view_mem_str("Hello"), ());
//!     }
//!
//!     #[test]
//!     fn ptr_viewer() {
//!         println!("This should print the memory of a pointer.\n");
//!         assert_eq!(view_mem_ptr(&69), ());
//!     }
//!
//!     #[test]
//!     fn vec_viewer() {
//!         println!("This should print the memory address of the vector and the memory of its elements.\n");
//!         assert_eq!(view_mem_vec(vec![69, 255, 254, 253, 70]), ());
//!     }
//!
//!     #[test]
//!     fn box_viewer() {
//!         println!("This should print the memory address of the box and the memory of its value.\n");
//!         assert_eq!(view_mem_box(Box::new(69)), ());
//!     }
//!
//!     #[test]
//!     fn vec_of_box_viewer() {
//!         println!("This should print the memory address of the vector of boxes and the memory of its elements.\n");
//!         assert_eq!(view_mem_vec_of_box(vec![Box::new(69), Box::new(255), Box::new(254), Box::new(253), Box::new(70)]), ());
//!     }
//!
//!     #[test]
//!     fn struct_viewer() {
//!         println!("This should print the memory address of the struct and the memory of its fields.\n");
//!         assert_eq!(view_mem_struct(
//!             MyStruct {
//!                 a: 69,
//!                 b: 255,
//!                 c: 70,
//!             }), ());
//!     }
//! }
//! ```
//! 
//! ## Output:
//! ```none
//! running 8 tests
//! test tests::f32_viewer ... ok
//! test tests::box_viewer ... ok
//! test tests::ptr_viewer ... ok
//! test tests::str_viewer ... ok
//! test tests::u64_viewer ... ok
//! test tests::struct_viewer ... ok
//! test tests::vec_of_box_viewer ... ok
//! test tests::vec_viewer ... ok
//! 
//! successes:
//! 
//! ---- tests::f32_viewer stdout ----
//! This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.
//! 
//! Name: my_f32
//! Type: f32
//! Size: 4 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9f2fec24 | 0xc3 | 195 | 0b11000011 |  ...
//!  0x0000cd9f2fec25 | 0xf5 | 245 | 0b11110101 |  ...
//!  0x0000cd9f2fec26 | 0x48 | 072 | 0b01001000 |  H
//!  0x0000cd9f2fec27 | 0x40 | 064 | 0b01000000 |  @
//! 
//! 
//! ---- tests::box_viewer stdout ----
//! This should print the memory address of the box and the memory of its value.
//! 
//! Name: my_box
//! Type: alloc::boxed::Box<i32>
//! Size: 8 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9f0feb50 | 0x10 | 016 | 0b00010000 |  DLE
//!  0x0000cd9f0feb51 | 0x60 | 096 | 0b01100000 |  `
//!  0x0000cd9f0feb52 | 0xba | 186 | 0b10111010 |  ...
//!  0x0000cd9f0feb53 | 0xf2 | 242 | 0b11110010 |  ...
//!  0x0000cd9f0feb54 | 0xf4 | 244 | 0b11110100 |  ...
//!  0x0000cd9f0feb55 | 0x01 | 001 | 0b00000001 |  SOH
//!  0x0000cd9f0feb56 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0feb57 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! Name: *my_box
//! Type: i32
//! Size: 4 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0001f4f2ba6010 | 0x45 | 069 | 0b01000101 |  E
//!  0x0001f4f2ba6011 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba6012 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba6013 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! 
//! ---- tests::ptr_viewer stdout ----
//! This should print the memory of a pointer.
//! 
//! Name: my_ptr
//! Type: *const i32
//! Size: 8 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9f4fef50 | 0x50 | 080 | 0b01010000 |  P
//!  0x0000cd9f4fef51 | 0x95 | 149 | 0b10010101 |  ...
//!  0x0000cd9f4fef52 | 0x69 | 105 | 0b01101001 |  i
//!  0x0000cd9f4fef53 | 0x94 | 148 | 0b10010100 |  ...
//!  0x0000cd9f4fef54 | 0xf6 | 246 | 0b11110110 |  ...
//!  0x0000cd9f4fef55 | 0x7f | 127 | 0b01111111 |  DEL
//!  0x0000cd9f4fef56 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f4fef57 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! Name: *my_ptr
//! Type: i32
//! Size: 4 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x007ff694699550 | 0x45 | 069 | 0b01000101 |  E
//!  0x007ff694699551 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x007ff694699552 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x007ff694699553 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! 
//! ---- tests::str_viewer stdout ----
//! This should print the memory of 'Hello' and its address.
//! 
//! Name: my_str
//! Type: &str
//! Size: 16 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9f6feea8 | 0xf0 | 240 | 0b11110000 |  ...
//!  0x0000cd9f6feea9 | 0x94 | 148 | 0b10010100 |  ...
//!  0x0000cd9f6feeaa | 0x69 | 105 | 0b01101001 |  i
//!  0x0000cd9f6feeab | 0x94 | 148 | 0b10010100 |  ...
//!  0x0000cd9f6feeac | 0xf6 | 246 | 0b11110110 |  ...
//!  0x0000cd9f6feead | 0x7f | 127 | 0b01111111 |  DEL
//!  0x0000cd9f6feeae | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f6feeaf | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f6feeb0 | 0x05 | 005 | 0b00000101 |  ENQ
//!  0x0000cd9f6feeb1 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f6feeb2 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f6feeb3 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f6feeb4 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f6feeb5 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f6feeb6 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f6feeb7 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! Name: *my_str
//! Type: str
//! Size: 5 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x007ff6946994f0 | 0x48 | 072 | 0b01001000 |  H
//!  0x007ff6946994f1 | 0x65 | 101 | 0b01100101 |  e
//!  0x007ff6946994f2 | 0x6c | 108 | 0b01101100 |  l
//!  0x007ff6946994f3 | 0x6c | 108 | 0b01101100 |  l
//!  0x007ff6946994f4 | 0x6f | 111 | 0b01101111 |  o
//! 
//! 
//! ---- tests::u64_viewer stdout ----
//! This should print the memory of the holy number 69.
//! 
//! Name: my_u64
//! Type: u64
//! Size: 8 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9fafef50 | 0x45 | 069 | 0b01000101 |  E
//!  0x0000cd9fafef51 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9fafef52 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9fafef53 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9fafef54 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9fafef55 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9fafef56 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9fafef57 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! 
//! ---- tests::struct_viewer stdout ----
//! This should print the memory address of the struct and the memory of its fields.
//! 
//! Name: &my_struct
//! Type: &mem_viewer::tests::MyStruct
//! Size: 8 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9f8ff110 | 0x70 | 112 | 0b01110000 |  p
//!  0x0000cd9f8ff111 | 0xf0 | 240 | 0b11110000 |  ...
//!  0x0000cd9f8ff112 | 0x8f | 143 | 0b10001111 |  ...
//!  0x0000cd9f8ff113 | 0x9f | 159 | 0b10011111 |  ...
//!  0x0000cd9f8ff114 | 0xcd | 205 | 0b11001101 |  ...
//!  0x0000cd9f8ff115 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f8ff116 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f8ff117 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! Name: my_struct
//! Type: mem_viewer::tests::MyStruct
//! Size: 8 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9f8ff070 | 0x46 | 070 | 0b01000110 |  F
//!  0x0000cd9f8ff071 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f8ff072 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f8ff073 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f8ff074 | 0xff | 255 | 0b11111111 |  ...
//!  0x0000cd9f8ff075 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f8ff076 | 0x45 | 069 | 0b01000101 |  E
//!  0x0000cd9f8ff077 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! 
//! ---- tests::vec_of_box_viewer stdout ----
//! This should print the memory address of the vector of boxes and the memory of its elements.
//! 
//! Name: my_vec_of_box
//! Type: alloc::vec::Vec<alloc::boxed::Box<i32>>
//! Size: 24 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9f0ff178 | 0x05 | 005 | 0b00000101 |  ENQ
//!  0x0000cd9f0ff179 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff17a | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff17b | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff17c | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff17d | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff17e | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff17f | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff180 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff181 | 0x37 | 055 | 0b00110111 |  7
//!  0x0000cd9f0ff182 | 0xba | 186 | 0b10111010 |  ...
//!  0x0000cd9f0ff183 | 0xf2 | 242 | 0b11110010 |  ...
//!  0x0000cd9f0ff184 | 0xf4 | 244 | 0b11110100 |  ...
//!  0x0000cd9f0ff185 | 0x01 | 001 | 0b00000001 |  SOH
//!  0x0000cd9f0ff186 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff187 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff188 | 0x05 | 005 | 0b00000101 |  ENQ
//!  0x0000cd9f0ff189 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff18a | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff18b | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff18c | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff18d | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff18e | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff18f | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! Name: *my_vec_of_box
//! Type: [alloc::boxed::Box<i32>]
//! Size: 40 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0001f4f2ba3700 | 0x20 | 032 | 0b00100000 |  SPC
//!  0x0001f4f2ba3701 | 0x60 | 096 | 0b01100000 |  `
//!  0x0001f4f2ba3702 | 0xba | 186 | 0b10111010 |  ...
//!  0x0001f4f2ba3703 | 0xf2 | 242 | 0b11110010 |  ...
//!  0x0001f4f2ba3704 | 0xf4 | 244 | 0b11110100 |  ...
//!  0x0001f4f2ba3705 | 0x01 | 001 | 0b00000001 |  SOH
//!  0x0001f4f2ba3706 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba3707 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba3708 | 0xf0 | 240 | 0b11110000 |  ...
//!  0x0001f4f2ba3709 | 0x5f | 095 | 0b01011111 |  _
//!  0x0001f4f2ba370a | 0xba | 186 | 0b10111010 |  ...
//!  0x0001f4f2ba370b | 0xf2 | 242 | 0b11110010 |  ...
//!  0x0001f4f2ba370c | 0xf4 | 244 | 0b11110100 |  ...
//!  0x0001f4f2ba370d | 0x01 | 001 | 0b00000001 |  SOH
//!  0x0001f4f2ba370e | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba370f | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba3710 | 0xb0 | 176 | 0b10110000 |  ...
//!  0x0001f4f2ba3711 | 0x60 | 096 | 0b01100000 |  `
//!  0x0001f4f2ba3712 | 0xba | 186 | 0b10111010 |  ...
//!  0x0001f4f2ba3713 | 0xf2 | 242 | 0b11110010 |  ...
//!  0x0001f4f2ba3714 | 0xf4 | 244 | 0b11110100 |  ...
//!  0x0001f4f2ba3715 | 0x01 | 001 | 0b00000001 |  SOH
//!  0x0001f4f2ba3716 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba3717 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba3718 | 0x40 | 064 | 0b01000000 |  @
//!  0x0001f4f2ba3719 | 0x60 | 096 | 0b01100000 |  `
//!  0x0001f4f2ba371a | 0xba | 186 | 0b10111010 |  ...
//!  0x0001f4f2ba371b | 0xf2 | 242 | 0b11110010 |  ...
//!  0x0001f4f2ba371c | 0xf4 | 244 | 0b11110100 |  ...
//!  0x0001f4f2ba371d | 0x01 | 001 | 0b00000001 |  SOH
//!  0x0001f4f2ba371e | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba371f | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba3720 | 0xc0 | 192 | 0b11000000 |  ...
//!  0x0001f4f2ba3721 | 0x60 | 096 | 0b01100000 |  `
//!  0x0001f4f2ba3722 | 0xba | 186 | 0b10111010 |  ...
//!  0x0001f4f2ba3723 | 0xf2 | 242 | 0b11110010 |  ...
//!  0x0001f4f2ba3724 | 0xf4 | 244 | 0b11110100 |  ...
//!  0x0001f4f2ba3725 | 0x01 | 001 | 0b00000001 |  SOH
//!  0x0001f4f2ba3726 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba3727 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! Name: *my_vec_of_box[0]
//! Type: i32
//! Size: 4 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0001f4f2ba6020 | 0x45 | 069 | 0b01000101 |  E
//!  0x0001f4f2ba6021 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba6022 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba6023 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! 
//! ---- tests::vec_viewer stdout ----
//! This should print the memory address of the vector and the memory of its elements.
//! 
//! Name: my_vec
//! Type: alloc::vec::Vec<i32>
//! Size: 24 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0000cd9f0ff2b8 | 0x05 | 005 | 0b00000101 |  ENQ
//!  0x0000cd9f0ff2b9 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2ba | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2bb | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2bc | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2bd | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2be | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2bf | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2c0 | 0xb0 | 176 | 0b10110000 |  ...
//!  0x0000cd9f0ff2c1 | 0x8f | 143 | 0b10001111 |  ...
//!  0x0000cd9f0ff2c2 | 0xba | 186 | 0b10111010 |  ...
//!  0x0000cd9f0ff2c3 | 0xf2 | 242 | 0b11110010 |  ...
//!  0x0000cd9f0ff2c4 | 0xf4 | 244 | 0b11110100 |  ...
//!  0x0000cd9f0ff2c5 | 0x01 | 001 | 0b00000001 |  SOH
//!  0x0000cd9f0ff2c6 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2c7 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2c8 | 0x05 | 005 | 0b00000101 |  ENQ
//!  0x0000cd9f0ff2c9 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2ca | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2cb | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2cc | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2cd | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2ce | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0000cd9f0ff2cf | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! Name: *my_vec
//! Type: [i32]
//! Size: 20 bytes
//!       Address     | Hex  | Dec |    Bin     | ASCII
//! ---------------------------------------------------
//!  0x0001f4f2ba8fb0 | 0x45 | 069 | 0b01000101 |  E
//!  0x0001f4f2ba8fb1 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fb2 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fb3 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fb4 | 0xff | 255 | 0b11111111 |  ...
//!  0x0001f4f2ba8fb5 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fb6 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fb7 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fb8 | 0xfe | 254 | 0b11111110 |  ...
//!  0x0001f4f2ba8fb9 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fba | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fbb | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fbc | 0xfd | 253 | 0b11111101 |  ...
//!  0x0001f4f2ba8fbd | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fbe | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fbf | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fc0 | 0x46 | 070 | 0b01000110 |  F
//!  0x0001f4f2ba8fc1 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fc2 | 0x00 | 000 | 0b00000000 |  NUL
//!  0x0001f4f2ba8fc3 | 0x00 | 000 | 0b00000000 |  NUL
//! 
//! 
//! 
//! successes:
//!     tests::box_viewer
//!     tests::f32_viewer
//!     tests::ptr_viewer
//!     tests::str_viewer
//!     tests::struct_viewer
//!     tests::u64_viewer
//!     tests::vec_of_box_viewer
//!     tests::vec_viewer
//! 
//! test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//! ```
//! 

#![allow(dead_code)]
#![allow(unused_assignments)]


#[macro_export]
/// Macro to view the memory content of an arbitrary variable.
/// 
/// It supports viewing memory content of different data types including integers, floating-point numbers, strings, pointers, vectors, boxed variables, and structs.
///
/// # Argument
///
/// * `$var` - The variable whose memory content needs to be viewed.
///
/// # Example
///
/// ```rust
/// let my_var: u32 = 69;
/// view_mem!(my_var);
/// ```
/// 
/// # Output
/// 
/// ```none
/// Name: my_var
/// Type: u32
/// Size: 4 bytes
///      Address     | Hex  | Dec |    Bin     | ASCII
/// ---------------------------------------------------
/// 0x0000cd9fafef50 | 0x45 | 069 | 0b01000101 |  E
/// 0x0000cd9fafef51 | 0x00 | 000 | 0b00000000 |  NUL
/// 0x0000cd9fafef52 | 0x00 | 000 | 0b00000000 |  NUL
/// 0x0000cd9fafef53 | 0x00 | 000 | 0b00000000 |  NUL
/// ```
macro_rules! view_mem {
    ($var: expr) => {
        // Get the size of the variable
        let size = std::mem::size_of_val(&$var);

        // Print metadata of var: var_name, size, type, separated by a new line for each meta
        println!("Name: {}", stringify!($var));
        _print_type_of(&$var);
        println!("Size: {} bytes", size);

        _show_memory_content(&$var as *const _ as *const u8, size);
    };
}

/// Prints the type of a variable.
///
/// (This is supposed to be private usage!)
/// 
/// # Argument
///
/// * `_: T` - The variable whose type needs to be printed.
pub fn _print_type_of<T>(_: T) {
    let type_name = &std::any::type_name::<T>()[1..]; // Remove `&` at first character
    println!("Type: {}", type_name);
}

/// Displays the memory content of a given memory address.
///
/// 
/// (This is supposed to be private usage!)
/// 
/// # Arguments
///
/// * `src_ptr` - The memory address to start displaying from.
/// * `len` - The number of bytes to display.
pub fn _show_memory_content(src_ptr: *const u8, len: usize) { // This supposed to be private usage.
    // Display the memory and its value for every byte from src_ptr to src_ptr + len

    let mut ascii_display = String::with_capacity(3);
    let mut ptr: *const u8 = src_ptr;
    let mut value: u8;

    let end: *const u8 = unsafe { src_ptr.add(len) };

    println!("      Address     | Hex  | Dec |    Bin     | ASCII");
    println!("---------------------------------------------------");
    while ptr < end {
        value = unsafe {*ptr};

        print!(" {:#016p} | {:#04x} | {:03} | {:#010b} |", ptr, value, value, value);

        ascii_display = "...".to_string(); // Not ASCII

        if value.is_ascii() {
            if value.is_ascii_graphic() { // Graphic ASCII
                ascii_display = (value as char).to_string();
            } else {
                ascii_display = match value { // Non-graphic ASCII
                    0   => "NUL",
                    1   => "SOH",
                    2   => "STX",
                    3   => "ETX",
                    4   => "EOT",
                    5   => "ENQ",
                    6   => "ACK",
                    7   => "BEL",
                    8   => "BS ",
                    9   => "HT ",
                    10  => "LF ",
                    11  => "VT ",
                    12  => "FF ",
                    13  => "CR ",
                    14  => "SO ",
                    15  => "SI ",
                    16  => "DLE",
                    17  => "DC1",
                    18  => "DC2",
                    19  => "DC3",
                    20  => "DC4",
                    21  => "NAK",
                    22  => "SYN",
                    23  => "ETB",
                    24  => "CAN",
                    25  => "EM ",
                    26  => "SUB",
                    27  => "ESC",
                    28  => "FS ",
                    29  => "GS ",
                    30  => "RS ",
                    31  => "US ",
                    32  => "SPC",
                    127 => "DEL",
                    _   => "UNK",
                }
                .to_string()
            }
        }

        println!("  {}", ascii_display);

        ptr = unsafe { ptr.add(1) };
    }

    println!();
}

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

