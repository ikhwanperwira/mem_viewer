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
//! mem_viewer = "0.2.0"
//! ```
//! 
//! Then, in your Rust code, you can use the `view_mem!` macro to view the memory content of a variable. Here's an example:
//! 
//! ```rust
//! use mem_viewer::*;
//! 
//! let my_var: u16 = 69;
//! view_mem!(my_var);
//! ```
//! 
//! This will print the memory content of `my_var` to the console.
//! 
//! ## Example Output
//! 
//! ```none
//! ame: my_var
//! Type: u16
//! Addr: 000000719ddfdce6
//! Size: 2 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000719ddfdce6 | 45 | 069 | 01000101 |  E
//!  000000719ddfdce7 | 00 | 000 | 00000000 |  NUL
//! ```
//! 
//! ## Safe Usage
//! For safe mode, you can use the `safe_view_mem!` macro to view the memory content of a variable. Here's an example:
//! ```rust
//! use mem_viewer::*;
//! 
//! let my_var: u16 = 69;
//! safe_view_mem!(&my_var);
//! ```
//! 
//! ## Example Safe Output
//! ```none
//! Name: &my_var
//! Type: &u16
//! Addr: 000000719ddfdce6
//! Size: 8 bytes
//! Container Ptr : 00000260c1266610
//! Container Len : 2
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 00000260c1266610 | 45  | 069 | 01000101 | E
//! 00000260c1266611 | 00  | 000 | 00000000 | NUL
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
//! 
//! # Unit Test Report
//! 
//! ## Code:
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//! 
//!     /// Display the memopry content of a u16 variable.
//!     fn view_mem_u16(my_u16: u16) -> () {
//!         // Unsafe test
//!         view_mem!(my_u16);
//! 
//!         // Safe test
//!         safe_view_mem!(&my_u16);
//!     }
//! 
//!     /// Displays the memory content of a u64 variable.
//!     fn view_mem_u64(my_u64: u64) -> () {
//!         // Unsafe test
//!         view_mem!(my_u64);
//! 
//!         // Safe test
//!         safe_view_mem!(&my_u64);
//!     }
//! 
//!     /// Displays the memory content of a f32 variable.
//!     fn view_mem_f32(my_f32: f32) -> () {
//!         // Unsafe test
//!         view_mem!(my_f32);
//! 
//!         // Safe test
//!         safe_view_mem!(&my_f32);
//!     }
//! 
//!     /// Displays the memory content of a string variable.
//!     fn view_mem_str(my_str: &str) -> () {
//!         // Unsafe test
//!         view_mem!(my_str); // Print address of the first character of the my_str
//!         view_mem!(*my_str); // Print actual content of my_str
//! 
//!         // Safe test
//!         safe_view_mem!(&my_str);
//!         safe_view_mem!(my_str);
//!     }
//! 
//!     /// Displays the memory content of a pointer.
//!     fn view_mem_ptr<T>(my_ptr: *const T) -> () {
//!         // Unsafe test
//!         view_mem!(my_ptr);
//!         unsafe { view_mem!(*my_ptr); }
//! 
//!         // Safe test
//!         // Parameterized type is not supported for safe view.
//!     }
//! 
//!     /// Displays the memory content of a vector variable.
//!     fn view_mem_vec<T>(my_vec: Vec<T>) -> () {
//!         // Unsafe test
//!         view_mem!(my_vec);
//!         view_mem!(*my_vec);
//! 
//!         // Safe test
//!         // Parameterized type is not supported for safe view.
//!     }
//! 
//!     /// Displays the memory content of a boxed variable.
//!     fn view_mem_box<T>(my_box: Box<T>) -> () {
//!         // Unsafe test
//!         view_mem!(&my_box);
//!         view_mem!(my_box);
//!         view_mem!(*my_box);
//! 
//!         // Safe test
//!         // Parameterized type is not supported for safe view.
//!     }
//! 
//!     /// Displays the memory content of a vector of boxed variables.
//!     fn view_mem_vec_of_box<T>(my_vec_of_box: Vec<Box<T>>) -> () {
//!         // Unsafe test
//!         view_mem!(my_vec_of_box);
//!         view_mem!(*my_vec_of_box);
//!         view_mem!(*my_vec_of_box[0]);
//! 
//!         // Safe test
//!         // Parameterized type is not supported for safe view.
//!     }
//! 
//!     /// Displays the memory content of a struct variable.
//!     fn view_mem_struct<T>(my_struct: T) -> () {
//!         // Unsafe test
//!         view_mem!(&my_struct);
//!         view_mem!(my_struct);
//! 
//!         // Parameterized type is not supported for safe view.
//!     }
//!      
//!     struct MyStruct {
//!         a: u8,
//!         b: u16,
//!         c: u32,
//!     }
//! 
//!     #[derive(Serialize)]
//!     struct MySerializedStruct {
//!         a: u8,
//!         b: u16,
//!         c: u32,
//!     }
//! 
//!     #[test]
//!     fn u16_viewer() {
//!         println!("This should print the memory of the holy number 69.\n");
//!         assert_eq!(view_mem_u16(69), ());
//!     }
//! 
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
//!         let my_ptr: *const u8 = &69;
//! 
//!         // Safe test
//!         println!("Currently pointer type is not supported by safe view memory.\n");
//! 
//!         // Unsafe test
//!         assert_eq!(view_mem_ptr(my_ptr), ());
//!     }
//! 
//!     #[test]
//!     fn vec_viewer() {
//!         println!("This should print the memory address of the vector and the memory of its elements.\n");
//!         let my_vec: Vec<u8> = vec![69, 255, 254, 253, 70];
//! 
//!         // Safe test
//!         safe_view_mem!(&my_vec);
//! 
//!         // Unsafe test
//!         assert_eq!(view_mem_vec(my_vec), ());
//!     }
//! 
//!     #[test]
//!     fn box_viewer() {
//!         println!("This should print the memory address of the box and the memory of its value.\n");
//!         let my_box: Box<u8> = Box::new(69);
//! 
//!         // Safe test
//!         safe_view_mem!(&my_box);
//! 
//! 
//!         // Unsafe test
//!         assert_eq!(view_mem_box(my_box), ());
//!     }
//! 
//!     #[test]
//!     fn vec_of_box_viewer() {
//!         println!("This should print the memory address of the vector of boxes and the memory of its elements.\n");
//!         let my_vec_of_box: Vec<Box<u8>> = vec![Box::new(69), Box::new(255), Box::new(254), Box::new(253), Box::new(70)];
//! 
//!         // Safe test
//!         safe_view_mem!(&my_vec_of_box);
//! 
//!         // Unsafe test
//!         assert_eq!(view_mem_vec_of_box(my_vec_of_box), ());
//!     }
//! 
//!     #[test]
//!     fn struct_viewer() {
//!         println!("This should print the memory address of the struct and the memory of its fields.\n");
//!         let my_struct = MyStruct {
//!             a: 69,
//!             b: 255,
//!             c: 70,
//!         };
//! 
//!         let my_serialized_struct = MySerializedStruct {
//!             a: 69,
//!             b: 255,
//!             c: 70,
//!         };
//! 
//!         // Safe test
//!         safe_view_mem!(&my_serialized_struct);
//! 
//!         // Unsafe test
//!         assert_eq!(view_mem_struct(my_struct), ());
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
//! test tests::u16_viewer ... ok
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
//! Addr: 000000dc288fdbf4
//! Size: 4 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc288fdbf4 | c3 | 195 | 11000011 |  ...
//!  000000dc288fdbf5 | f5 | 245 | 11110101 |  ...
//!  000000dc288fdbf6 | 48 | 072 | 01001000 |  H
//!  000000dc288fdbf7 | 40 | 064 | 01000000 |  @
//! 
//! Name: &my_f32
//! Type: &f32
//! Addr: 000000dc288fdbf4
//! Size: 8 bytes
//! Container Ptr : 000001e1f8704690
//! Container Len : 4
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e1f8704690 | c3  | 195 | 11000011 | UNK
//! 000001e1f8704691 | f5  | 245 | 11110101 | UNK
//! 000001e1f8704692 | 48  | 072 | 01001000 | H
//! 000001e1f8704693 | 40  | 064 | 01000000 | @
//! 
//! 
//! ---- tests::box_viewer stdout ----
//! This should print the memory address of the box and the memory of its value.
//! 
//! Name: &my_box
//! Type: &alloc::boxed::Box<u8>
//! Addr: 000000dc286fe348
//! Size: 8 bytes
//! Container Ptr : 000001e1f8704680
//! Container Len : 1
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e1f8704680 | 45  | 069 | 01000101 | E
//! 
//! Name: &my_box
//! Type: &alloc::boxed::Box<u8>
//! Addr: 000000dc286fdcd8
//! Size: 8 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc286fddc0 | 38 | 056 | 00111000 |  8
//!  000000dc286fddc1 | dc | 220 | 11011100 |  ...
//!  000000dc286fddc2 | 6f | 111 | 01101111 |  o
//!  000000dc286fddc3 | 28 | 040 | 00101000 |  (
//!  000000dc286fddc4 | dc | 220 | 11011100 |  ...
//!  000000dc286fddc5 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fddc6 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fddc7 | 00 | 000 | 00000000 |  NUL
//! 
//! Name: my_box
//! Type: alloc::boxed::Box<u8>
//! Addr: 000000dc286fdc38
//! Size: 8 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc286fdc38 | 40 | 064 | 01000000 |  @
//!  000000dc286fdc39 | 45 | 069 | 01000101 |  E
//!  000000dc286fdc3a | 70 | 112 | 01110000 |  p
//!  000000dc286fdc3b | f8 | 248 | 11111000 |  ...
//!  000000dc286fdc3c | e1 | 225 | 11100001 |  ...
//!  000000dc286fdc3d | 01 | 001 | 00000001 |  SOH
//!  000000dc286fdc3e | 00 | 000 | 00000000 |  NUL
//!  000000dc286fdc3f | 00 | 000 | 00000000 |  NUL
//! 
//! Name: *my_box
//! Type: u8
//! Addr: 000001e1f8704540
//! Size: 1 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000001e1f8704540 | 45 | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::ptr_viewer stdout ----
//! This should print the memory of a pointer.
//! 
//! Currently pointer type is not supported by safe view memory.
//! 
//! Name: my_ptr
//! Type: *const u8
//! Addr: 000000dc28afe9d0
//! Size: 8 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc28afe9d0 | 38 | 056 | 00111000 |  8
//!  000000dc28afe9d1 | 28 | 040 | 00101000 |  (
//!  000000dc28afe9d2 | 2c | 044 | 00101100 |  ,
//!  000000dc28afe9d3 | 38 | 056 | 00111000 |  8
//!  000000dc28afe9d4 | f6 | 246 | 11110110 |  ...
//!  000000dc28afe9d5 | 7f | 127 | 01111111 |  DEL
//!  000000dc28afe9d6 | 00 | 000 | 00000000 |  NUL
//!  000000dc28afe9d7 | 00 | 000 | 00000000 |  NUL
//! 
//! Name: *my_ptr
//! Type: u8
//! Addr: 00007ff6382c2838
//! Size: 1 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  00007ff6382c2838 | 45 | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::str_viewer stdout ----
//! This should print the memory of 'Hello' and its address.
//! 
//! Name: my_str
//! Type: &str
//! Addr: 000000dc28cfd0b8
//! Size: 16 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc28cfd0b8 | d8 | 216 | 11011000 |  ...
//!  000000dc28cfd0b9 | 27 | 039 | 00100111 |  '
//!  000000dc28cfd0ba | 2c | 044 | 00101100 |  ,
//!  000000dc28cfd0bb | 38 | 056 | 00111000 |  8
//!  000000dc28cfd0bc | f6 | 246 | 11110110 |  ...
//!  000000dc28cfd0bd | 7f | 127 | 01111111 |  DEL
//!  000000dc28cfd0be | 00 | 000 | 00000000 |  NUL
//!  000000dc28cfd0bf | 00 | 000 | 00000000 |  NUL
//!  000000dc28cfd0c0 | 05 | 005 | 00000101 |  ENQ
//!  000000dc28cfd0c1 | 00 | 000 | 00000000 |  NUL
//!  000000dc28cfd0c2 | 00 | 000 | 00000000 |  NUL
//!  000000dc28cfd0c3 | 00 | 000 | 00000000 |  NUL
//!  000000dc28cfd0c4 | 00 | 000 | 00000000 |  NUL
//!  000000dc28cfd0c5 | 00 | 000 | 00000000 |  NUL
//!  000000dc28cfd0c6 | 00 | 000 | 00000000 |  NUL
//!  000000dc28cfd0c7 | 00 | 000 | 00000000 |  NUL
//! 
//! Name: *my_str
//! Type: str
//! Addr: 00007ff6382c27d8
//! Size: 5 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  00007ff6382c27d8 | 48 | 072 | 01001000 |  H
//!  00007ff6382c27d9 | 65 | 101 | 01100101 |  e
//!  00007ff6382c27da | 6c | 108 | 01101100 |  l
//!  00007ff6382c27db | 6c | 108 | 01101100 |  l
//!  00007ff6382c27dc | 6f | 111 | 01101111 |  o
//! 
//! Name: &my_str
//! Type: &&str
//! Addr: 000000dc28cfd0b8
//! Size: 8 bytes
//! Container Ptr : 000001e1f8703c50
//! Container Len : 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e1f8703c50 | 48  | 072 | 01001000 | H
//! 000001e1f8703c51 | 65  | 101 | 01100101 | e
//! 000001e1f8703c52 | 6c  | 108 | 01101100 | l
//! 000001e1f8703c53 | 6c  | 108 | 01101100 | l
//! 000001e1f8703c54 | 6f  | 111 | 01101111 | o
//! 
//! Name: my_str
//! Type: &str
//! Addr: 00007ff6382c27d8
//! Size: 16 bytes
//! Container Ptr : 000001e1f8703cc0
//! Container Len : 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e1f8703cc0 | 48  | 072 | 01001000 | H
//! 000001e1f8703cc1 | 65  | 101 | 01100101 | e
//! 000001e1f8703cc2 | 6c  | 108 | 01101100 | l
//! 000001e1f8703cc3 | 6c  | 108 | 01101100 | l
//! 000001e1f8703cc4 | 6f  | 111 | 01101111 | o
//! 
//! 
//! ---- tests::u16_viewer stdout ----
//! This should print the memory of the holy number 69.
//! 
//! Name: my_u16
//! Type: u16
//! Addr: 000000dc290fe126
//! Size: 2 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc290fe126 | 45 | 069 | 01000101 |  E
//!  000000dc290fe127 | 00 | 000 | 00000000 |  NUL
//! 
//! Name: &my_u16
//! Type: &u16
//! Addr: 000000dc290fe126
//! Size: 8 bytes
//! Container Ptr : 000001e1f8704570
//! Container Len : 2
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e1f8704570 | 45  | 069 | 01000101 | E
//! 000001e1f8704571 | 00  | 000 | 00000000 | NUL
//! 
//! 
//! ---- tests::struct_viewer stdout ----
//! This should print the memory address of the struct and the memory of its fields.
//! 
//! Name: &my_serialized_struct
//! Type: &mem_viewer::tests::MySerializedStruct
//! Addr: 000000dc28efe068
//! Size: 8 bytes
//! Container Ptr : 000001e1f8704510
//! Container Len : 7
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e1f8704510 | 45  | 069 | 01000101 | E
//! 000001e1f8704511 | ff  | 255 | 11111111 | UNK
//! 000001e1f8704512 | 00  | 000 | 00000000 | NUL
//! 000001e1f8704513 | 46  | 070 | 01000110 | F
//! 000001e1f8704514 | 00  | 000 | 00000000 | NUL
//! 000001e1f8704515 | 00  | 000 | 00000000 | NUL
//! 000001e1f8704516 | 00  | 000 | 00000000 | NUL
//! 
//! Name: &my_struct
//! Type: &mem_viewer::tests::MyStruct
//! Addr: 000000dc28efdbb8
//! Size: 8 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc28efdca0 | 10 | 016 | 00010000 |  DLE
//!  000000dc28efdca1 | db | 219 | 11011011 |  ...
//!  000000dc28efdca2 | ef | 239 | 11101111 |  ...
//!  000000dc28efdca3 | 28 | 040 | 00101000 |  (
//!  000000dc28efdca4 | dc | 220 | 11011100 |  ...
//!  000000dc28efdca5 | 00 | 000 | 00000000 |  NUL
//!  000000dc28efdca6 | 00 | 000 | 00000000 |  NUL
//!  000000dc28efdca7 | 00 | 000 | 00000000 |  NUL
//! 
//! Name: my_struct
//! Type: mem_viewer::tests::MyStruct
//! Addr: 000000dc28efdb10
//! Size: 8 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc28efdb10 | 46 | 070 | 01000110 |  F
//!  000000dc28efdb11 | 00 | 000 | 00000000 |  NUL
//!  000000dc28efdb12 | 00 | 000 | 00000000 |  NUL
//!  000000dc28efdb13 | 00 | 000 | 00000000 |  NUL
//!  000000dc28efdb14 | ff | 255 | 11111111 |  ...
//!  000000dc28efdb15 | 00 | 000 | 00000000 |  NUL
//!  000000dc28efdb16 | 45 | 069 | 01000101 |  E
//!  000000dc28efdb17 | 00 | 000 | 00000000 |  NUL
//! 
//! 
//! ---- tests::vec_of_box_viewer stdout ----
//! This should print the memory address of the vector of boxes and the memory of its elements.
//! 
//! Name: &my_vec_of_box
//! Type: &alloc::vec::Vec<alloc::boxed::Box<u8>>
//! Addr: 000000dc286fdd90
//! Size: 8 bytes
//! Container Ptr : 000001e1f8703c70
//! Container Len : 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e1f8703c70 | 45  | 069 | 01000101 | E
//! 000001e1f8703c71 | ff  | 255 | 11111111 | UNK
//! 000001e1f8703c72 | fe  | 254 | 11111110 | UNK
//! 000001e1f8703c73 | fd  | 253 | 11111101 | UNK
//! 000001e1f8703c74 | 46  | 070 | 01000110 | F
//! 
//! Name: my_vec_of_box
//! Type: alloc::vec::Vec<alloc::boxed::Box<u8>>
//! Addr: 000000dc286fe930
//! Size: 24 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc286fe930 | 05 | 005 | 00000101 |  ENQ
//!  000000dc286fe931 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe932 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe933 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe934 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe935 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe936 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe937 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe938 | 90 | 144 | 10010000 |  ...
//!  000000dc286fe939 | 77 | 119 | 01110111 |  w
//!  000000dc286fe93a | 70 | 112 | 01110000 |  p
//!  000000dc286fe93b | f8 | 248 | 11111000 |  ...
//!  000000dc286fe93c | e1 | 225 | 11100001 |  ...
//!  000000dc286fe93d | 01 | 001 | 00000001 |  SOH
//!  000000dc286fe93e | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe93f | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe940 | 05 | 005 | 00000101 |  ENQ
//!  000000dc286fe941 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe942 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe943 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe944 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe945 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe946 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fe947 | 00 | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec_of_box
//! Type: [alloc::boxed::Box<u8>]
//! Addr: 000001e1f8707790
//! Size: 40 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000001e1f8707790 | 50 | 080 | 01010000 |  P
//!  000001e1f8707791 | 3c | 060 | 00111100 |  <
//!  000001e1f8707792 | 70 | 112 | 01110000 |  p
//!  000001e1f8707793 | f8 | 248 | 11111000 |  ...
//!  000001e1f8707794 | e1 | 225 | 11100001 |  ...
//!  000001e1f8707795 | 01 | 001 | 00000001 |  SOH
//!  000001e1f8707796 | 00 | 000 | 00000000 |  NUL
//!  000001e1f8707797 | 00 | 000 | 00000000 |  NUL
//!  000001e1f8707798 | d0 | 208 | 11010000 |  ...
//!  000001e1f8707799 | 3c | 060 | 00111100 |  <
//!  000001e1f870779a | 70 | 112 | 01110000 |  p
//!  000001e1f870779b | f8 | 248 | 11111000 |  ...
//!  000001e1f870779c | e1 | 225 | 11100001 |  ...
//!  000001e1f870779d | 01 | 001 | 00000001 |  SOH
//!  000001e1f870779e | 00 | 000 | 00000000 |  NUL
//!  000001e1f870779f | 00 | 000 | 00000000 |  NUL
//!  000001e1f87077a0 | e0 | 224 | 11100000 |  ...
//!  000001e1f87077a1 | 3c | 060 | 00111100 |  <
//!  000001e1f87077a2 | 70 | 112 | 01110000 |  p
//!  000001e1f87077a3 | f8 | 248 | 11111000 |  ...
//!  000001e1f87077a4 | e1 | 225 | 11100001 |  ...
//!  000001e1f87077a5 | 01 | 001 | 00000001 |  SOH
//!  000001e1f87077a6 | 00 | 000 | 00000000 |  NUL
//!  000001e1f87077a7 | 00 | 000 | 00000000 |  NUL
//!  000001e1f87077a8 | 40 | 064 | 01000000 |  @
//!  000001e1f87077a9 | 3c | 060 | 00111100 |  <
//!  000001e1f87077aa | 70 | 112 | 01110000 |  p
//!  000001e1f87077ab | f8 | 248 | 11111000 |  ...
//!  000001e1f87077ac | e1 | 225 | 11100001 |  ...
//!  000001e1f87077ad | 01 | 001 | 00000001 |  SOH
//!  000001e1f87077ae | 00 | 000 | 00000000 |  NUL
//!  000001e1f87077af | 00 | 000 | 00000000 |  NUL
//!  000001e1f87077b0 | 60 | 096 | 01100000 |  `
//!  000001e1f87077b1 | 3c | 060 | 00111100 |  <
//!  000001e1f87077b2 | 70 | 112 | 01110000 |  p
//!  000001e1f87077b3 | f8 | 248 | 11111000 |  ...
//!  000001e1f87077b4 | e1 | 225 | 11100001 |  ...
//!  000001e1f87077b5 | 01 | 001 | 00000001 |  SOH
//!  000001e1f87077b6 | 00 | 000 | 00000000 |  NUL
//!  000001e1f87077b7 | 00 | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec_of_box[0]
//! Type: u8
//! Addr: 000001e1f8703c50
//! Size: 1 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000001e1f8703c50 | 45 | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::vec_viewer stdout ----
//! This should print the memory address of the vector and the memory of its elements.
//! 
//! Name: &my_vec
//! Type: &alloc::vec::Vec<u8>
//! Addr: 000000dc286fe438
//! Size: 8 bytes
//! Container Ptr : 000001e1f87046a0
//! Container Len : 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e1f87046a0 | 45  | 069 | 01000101 | E
//! 000001e1f87046a1 | ff  | 255 | 11111111 | UNK
//! 000001e1f87046a2 | fe  | 254 | 11111110 | UNK
//! 000001e1f87046a3 | fd  | 253 | 11111101 | UNK
//! 000001e1f87046a4 | 46  | 070 | 01000110 | F
//! 
//! Name: my_vec
//! Type: alloc::vec::Vec<u8>
//! Addr: 000000dc286fefb0
//! Size: 24 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000dc286fefb0 | 05 | 005 | 00000101 |  ENQ
//!  000000dc286fefb1 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefb2 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefb3 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefb4 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefb5 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefb6 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefb7 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefb8 | 20 | 032 | 00100000 |  SPC
//!  000000dc286fefb9 | 46 | 070 | 01000110 |  F
//!  000000dc286fefba | 70 | 112 | 01110000 |  p
//!  000000dc286fefbb | f8 | 248 | 11111000 |  ...
//!  000000dc286fefbc | e1 | 225 | 11100001 |  ...
//!  000000dc286fefbd | 01 | 001 | 00000001 |  SOH
//!  000000dc286fefbe | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefbf | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefc0 | 05 | 005 | 00000101 |  ENQ
//!  000000dc286fefc1 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefc2 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefc3 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefc4 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefc5 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefc6 | 00 | 000 | 00000000 |  NUL
//!  000000dc286fefc7 | 00 | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec
//! Type: [u8]
//! Addr: 000001e1f8704620
//! Size: 5 bytes
//!      Address     | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000001e1f8704620 | 45 | 069 | 01000101 |  E
//!  000001e1f8704621 | ff | 255 | 11111111 |  ...
//!  000001e1f8704622 | fe | 254 | 11111110 |  ...
//!  000001e1f8704623 | fd | 253 | 11111101 |  ...
//!  000001e1f8704624 | 46 | 070 | 01000110 |  F
//! 
//! 
//! 
//! successes:
//!     tests::box_viewer
//!     tests::f32_viewer
//!     tests::ptr_viewer
//!     tests::str_viewer
//!     tests::struct_viewer
//!     tests::u16_viewer
//!     tests::vec_of_box_viewer
//!     tests::vec_viewer
//! 
//! test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//! ```
//! 

#![allow(dead_code)]
#![allow(unused_assignments)]
pub use bincode::serialize_into;
pub use serde::Serialize;

#[macro_export]
/// Macro to view the memory content of almost any arbitrary variable in safe way
/// 
/// It's safe because it's isolated on container with serializing into `Vec<u8>` by `serde`. Hence, the address and size displayed is not accurate and actually it's virtual address of the container instead of reported by OS.
/// 
/// It supports viewing memory content of different data types including integers, floating-point numbers, strings, vectors, boxed variables, and structs. Except pointer.
/// 
/// For struct, it's mandatory to add `#[derive(Serialize)]` to the struct definition.
/// 
/// For example:
/// ```rust
/// #[derive(Serialize)]
/// struct MyStruct {
///    a: i32,
///    b: f32,
/// }
/// ```
/// 
/// # Argument
/// 
/// * `$var` - The variable whose memory content needs to be viewed.
/// 
/// # Example
/// 
/// ```rust
/// let my_var: u16 = 69;
/// safe_view_mem!(&my_var);
/// ```
/// 
/// # Output
/// 
/// ```none
/// Name: &my_var
/// Type: &u16
/// Addr: 000000acf3bfdc86
/// Size: 8 bytes
/// Container Ptr : 0000027f45f05290
/// Container Len : 2
///      Address     | Hex | Dec |    Bin   | ASCII
/// ---------------Container Content---------------
/// 0000027f45f05290 | 45  | 069 | 01000101 | E
/// 0000027f45f05291 | 00  | 000 | 00000000 | NUL
/// ```
macro_rules! safe_view_mem  {
	($var: expr) => {
        let size = std::mem::size_of_val(&$var);

		// Print variable metadata
		println!("Name: {}", stringify!($var));
		println!("Type: {}", _get_type_of($var));
        println!("Addr: {:016x}", $var as *const _ as *const u8 as usize);
		println!("Size: {} bytes", size);

		// Isolate on container
		let mut container: Vec<u8> = Vec::new();
		serialize_into(&mut container, $var).unwrap();
		// println!("Container Raw : {:?}", container);
		if container.len() >= 8 && container.len() != size {
			 // If not same, then there is header of serializer with size 8 bytes, exclude it!
			container = (&container[8..]).to_vec();
		}

		// Print container metadata
		// println!("Container Val : {:?}", container);
        println!("Container Ptr : {:016x}", container.as_ptr() as usize);
		println!("Container Len : {}", container.len());

		// Print container content
		println!("     Address     | Hex | Dec |    Bin   | ASCII");
		println!("---------------Container Content---------------");
		// Iterate over Vec<u8>
		for (_, byte) in container.iter().enumerate() {
			let addr = byte as *const u8 as usize;
			let hex = format!("{:02x}", byte);
			let dec = format!("{:03}", byte);
			let bin = format!("{:08b}", byte);
			let ascii = if byte.is_ascii_graphic() {
				format!("{}  ", *byte as char)
			} else {
				match byte {
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
					}.to_string()
			};
			println!("{:016x} | {}  | {} | {} | {}", addr, hex, dec, bin, ascii);
		};
		println!();
		}
	}


#[macro_export]
/// Macro to view the memory content of an arbitrary variable.
/// 
/// It supports viewing memory content of different data types including integers, floating-point numbers, strings, pointers, vectors, boxed variables, and structs.
/// 
/// This is more **unsafe** than safe_view_mem! macro, because it directly reads the memory content of the variable.
/// 
/// As a result, it can be used to view the memory content of a pointer.
/// 
/// You can dereference a variable as many as you want as long compiler allows it, of course this is **unsafe** operation.
///
/// # Argument
///
/// * `$var` - The variable whose memory content needs to be viewed.
///
/// # Example
///
/// ```rust
/// let my_var: u16 = 69;
/// view_mem!(my_var);
/// ```
/// 
/// # Output
/// 
/// ```none
/// Name: my_var
/// Type: u16
/// Addr: 00000007f88fdc56
/// Size: 2 bytes
///      Address     | Hex | Dec |    Bin   | ASCII
/// -------------------Memory Content-----------------
///  00000007f88fdc56 | 45 | 069 | 01000101 |  E
///  00000007f88fdc57 | 00 | 000 | 00000000 |  NUL
/// ```
macro_rules! view_mem {
    ($var: expr) => {
        // Get the size of the variable
        let size = std::mem::size_of_val(&$var);

        // Print metadata of var: var_name, size, type, separated by a new line for each meta
        println!("Name: {}", stringify!($var));
        _print_type_of(&$var);
        println!("Addr: {:016x}", &$var as *const _ as *const u8 as usize);
        println!("Size: {} bytes", size);

        _show_memory_content(&$var as *const _ as *const u8, size);
    };
}

/// Returns the type of a variable as a string.
/// 
/// (This is supposed to be private usage of safe_view_mem! macro usage.)
/// 
/// # Argument
/// 
/// * `_: T` - The variable whose type needs to be returned.
pub fn _get_type_of<T>(_: T) -> &'static str {
	std::any::type_name::<T>()
}

/// Prints the type of a variable.
///
/// (This is supposed to be private usage for unsafe view_mem! macro usage.)
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
/// (This is supposed to be private usage for unsafe view_mem! macro usage.)
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

    println!("     Address     | Hex | Dec |    Bin   | ASCII");
    println!("-------------------Memory Content-----------------");
    while ptr < end {
        value = unsafe {*ptr};

        print!(" {:016x} | {:02x} | {:03} | {:08b} |", ptr as usize, value as usize, value as usize, value as usize);

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

    /// Display the memopry content of a u16 variable.
    fn view_mem_u16(my_u16: u16) -> () {
        // Unsafe test
        view_mem!(my_u16);

        // Safe test
        safe_view_mem!(&my_u16);
    }

    /// Displays the memory content of a u64 variable.
    fn view_mem_u64(my_u64: u64) -> () {
        // Unsafe test
        view_mem!(my_u64);

        // Safe test
        safe_view_mem!(&my_u64);
    }

    /// Displays the memory content of a f32 variable.
    fn view_mem_f32(my_f32: f32) -> () {
        // Unsafe test
        view_mem!(my_f32);

        // Safe test
        safe_view_mem!(&my_f32);
    }

    /// Displays the memory content of a string variable.
    fn view_mem_str(my_str: &str) -> () {
        // Unsafe test
        view_mem!(my_str); // Print address of the first character of the my_str
        view_mem!(*my_str); // Print actual content of my_str

        // Safe test
        safe_view_mem!(&my_str);
        safe_view_mem!(my_str);
    }

    /// Displays the memory content of a pointer.
    fn view_mem_ptr<T>(my_ptr: *const T) -> () {
        // Unsafe test
        view_mem!(my_ptr);
        unsafe { view_mem!(*my_ptr); }

        // Safe test
        // Parameterized type is not supported for safe view.
    }

    /// Displays the memory content of a vector variable.
    fn view_mem_vec<T>(my_vec: Vec<T>) -> () {
        // Unsafe test
        view_mem!(my_vec);
        view_mem!(*my_vec);

        // Safe test
        // Parameterized type is not supported for safe view.
    }

    /// Displays the memory content of a boxed variable.
    fn view_mem_box<T>(my_box: Box<T>) -> () {
        // Unsafe test
        view_mem!(&my_box);
        view_mem!(my_box);
        view_mem!(*my_box);

        // Safe test
        // Parameterized type is not supported for safe view.
    }

    /// Displays the memory content of a vector of boxed variables.
    fn view_mem_vec_of_box<T>(my_vec_of_box: Vec<Box<T>>) -> () {
        // Unsafe test
        view_mem!(my_vec_of_box);
        view_mem!(*my_vec_of_box);
        view_mem!(*my_vec_of_box[0]);

        // Safe test
        // Parameterized type is not supported for safe view.
    }

    /// Displays the memory content of a struct variable.
    fn view_mem_struct<T>(my_struct: T) -> () {
        // Unsafe test
        view_mem!(&my_struct);
        view_mem!(my_struct);

        // Parameterized type is not supported for safe view.
    }
     
    struct MyStruct {
        a: u8,
        b: u16,
        c: u32,
    }

    #[derive(Serialize)]
    struct MySerializedStruct {
        a: u8,
        b: u16,
        c: u32,
    }

    #[test]
    fn u16_viewer() {
        println!("This should print the memory of the holy number 69.\n");
        assert_eq!(view_mem_u16(69), ());
    }

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
        let my_ptr: *const u8 = &69;

        // Safe test
        println!("Currently pointer type is not supported by safe view memory.\n");

        // Unsafe test
        assert_eq!(view_mem_ptr(my_ptr), ());
    }

    #[test]
    fn vec_viewer() {
        println!("This should print the memory address of the vector and the memory of its elements.\n");
        let my_vec: Vec<u8> = vec![69, 255, 254, 253, 70];

        // Safe test
        safe_view_mem!(&my_vec);

        // Unsafe test
        assert_eq!(view_mem_vec(my_vec), ());
    }

    #[test]
    fn box_viewer() {
        println!("This should print the memory address of the box and the memory of its value.\n");
        let my_box: Box<u8> = Box::new(69);

        // Safe test
        safe_view_mem!(&my_box);


        // Unsafe test
        assert_eq!(view_mem_box(my_box), ());
    }

    #[test]
    fn vec_of_box_viewer() {
        println!("This should print the memory address of the vector of boxes and the memory of its elements.\n");
        let my_vec_of_box: Vec<Box<u8>> = vec![Box::new(69), Box::new(255), Box::new(254), Box::new(253), Box::new(70)];

        // Safe test
        safe_view_mem!(&my_vec_of_box);

        // Unsafe test
        assert_eq!(view_mem_vec_of_box(my_vec_of_box), ());
    }

    #[test]
    fn struct_viewer() {
        println!("This should print the memory address of the struct and the memory of its fields.\n");
        let my_struct = MyStruct {
            a: 69,
            b: 255,
            c: 70,
        };

        let my_serialized_struct = MySerializedStruct {
            a: 69,
            b: 255,
            c: 70,
        };

        // Safe test
        safe_view_mem!(&my_serialized_struct);

        // Unsafe test
        assert_eq!(view_mem_struct(my_struct), ());
    }
}

