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
//! mem_viewer = "0.2.4"
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
//! Name: my_var
//! Type: u16
//! Addr: 000000719ddfdce6
//! Size: 2 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -------------------Memory Content-----------------
//!  000000719ddfdce6 | 45  | 069 | 01000101 |  E
//!  000000719ddfdce7 | 00  | 000 | 00000000 |  NUL
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
//! Name         : &my_var
//! Type         : &u16
//! Addr         : 000000719ddfdce6
//! Size         : 8 bytes
//! Container Ptr: 00000260c1266610
//! Container Len: 2
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
//! test tests::ptr_viewer ... ok
//! test tests::box_viewer ... ok
//! test tests::f32_viewer ... ok
//! test tests::str_viewer ... ok
//! test tests::struct_viewer ... ok
//! test tests::u16_viewer ... ok
//! test tests::vec_of_box_viewer ... ok
//! test tests::vec_viewer ... ok
//! 
//! successes:
//! 
//! ---- tests::ptr_viewer stdout ----
//! This should print the memory of a pointer.
//! 
//! Currently pointer type is not supported by safe view memory.
//! 
//! Name: my_ptr
//! Type: *const u8
//! Addr: 00000034619feb00
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00000034619feb00 | f8  | 248 | 11111000 |  ...
//!  00000034619feb01 | 2f  | 047 | 00101111 |  /
//!  00000034619feb02 | a6  | 166 | 10100110 |  ...
//!  00000034619feb03 | 20  | 032 | 00100000 |  SPC
//!  00000034619feb04 | f6  | 246 | 11110110 |  ...
//!  00000034619feb05 | 7f  | 127 | 01111111 |  DEL
//!  00000034619feb06 | 00  | 000 | 00000000 |  NUL
//!  00000034619feb07 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_ptr
//! Type: u8
//! Addr: 00007ff620a62ff8
//! Size: 1 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00007ff620a62ff8 | 45  | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::box_viewer stdout ----
//! This should print the memory address of the box and the memory of its value.
//! 
//! Name         : &my_box
//! Type         : &alloc::boxed::Box<u8>
//! Addr         : 00000034615fdfd8
//! Size         : 8 bytes
//! Container Ptr: 000001e2998cb200
//! Container Len: 1
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e2998cb200 | 45  | 069 | 01000101 | E
//! 
//! Name: &my_box
//! Type: &alloc::boxed::Box<u8>
//! Addr: 00000034615fd968
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00000034615fda50 | c8  | 200 | 11001000 |  ...
//!  00000034615fda51 | d8  | 216 | 11011000 |  ...
//!  00000034615fda52 | 5f  | 095 | 01011111 |  _
//!  00000034615fda53 | 61  | 097 | 01100001 |  a
//!  00000034615fda54 | 34  | 052 | 00110100 |  4
//!  00000034615fda55 | 00  | 000 | 00000000 |  NUL
//!  00000034615fda56 | 00  | 000 | 00000000 |  NUL
//!  00000034615fda57 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: my_box
//! Type: alloc::boxed::Box<u8>
//! Addr: 00000034615fd8c8
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00000034615fd8c8 | 00  | 000 | 00000000 |  NUL
//!  00000034615fd8c9 | b3  | 179 | 10110011 |  ...
//!  00000034615fd8ca | 8c  | 140 | 10001100 |  ...
//!  00000034615fd8cb | 99  | 153 | 10011001 |  ...
//!  00000034615fd8cc | e2  | 226 | 11100010 |  ...
//!  00000034615fd8cd | 01  | 001 | 00000001 |  SOH
//!  00000034615fd8ce | 00  | 000 | 00000000 |  NUL
//!  00000034615fd8cf | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_box
//! Type: u8
//! Addr: 000001e2998cb300
//! Size: 1 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000001e2998cb300 | 45  | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::f32_viewer stdout ----
//! This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.
//! 
//! Name: my_f32
//! Type: f32
//! Addr: 00000034617fdf14
//! Size: 4 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00000034617fdf14 | c3  | 195 | 11000011 |  ...
//!  00000034617fdf15 | f5  | 245 | 11110101 |  ...
//!  00000034617fdf16 | 48  | 072 | 01001000 |  H
//!  00000034617fdf17 | 40  | 064 | 01000000 |  @
//! 
//! Name         : &my_f32
//! Type         : &f32
//! Addr         : 00000034617fdf14
//! Size         : 8 bytes
//! Container Ptr: 000001e2998d1c40
//! Container Len: 4
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e2998d1c40 | c3  | 195 | 11000011 | ...
//! 000001e2998d1c41 | f5  | 245 | 11110101 | ...
//! 000001e2998d1c42 | 48  | 072 | 01001000 | H
//! 000001e2998d1c43 | 40  | 064 | 01000000 | @
//! 
//! 
//! ---- tests::str_viewer stdout ----
//! This should print the memory of 'Hello' and its address.
//! 
//! Name: my_str
//! Type: &str
//! Addr: 0000003461bfcaf8
//! Size: 16 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  0000003461bfcaf8 | 98  | 152 | 10011000 |  ...
//!  0000003461bfcaf9 | 2f  | 047 | 00101111 |  /
//!  0000003461bfcafa | a6  | 166 | 10100110 |  ...
//!  0000003461bfcafb | 20  | 032 | 00100000 |  SPC
//!  0000003461bfcafc | f6  | 246 | 11110110 |  ...
//!  0000003461bfcafd | 7f  | 127 | 01111111 |  DEL
//!  0000003461bfcafe | 00  | 000 | 00000000 |  NUL
//!  0000003461bfcaff | 00  | 000 | 00000000 |  NUL
//!  0000003461bfcb00 | 05  | 005 | 00000101 |  ENQ
//!  0000003461bfcb01 | 00  | 000 | 00000000 |  NUL
//!  0000003461bfcb02 | 00  | 000 | 00000000 |  NUL
//!  0000003461bfcb03 | 00  | 000 | 00000000 |  NUL
//!  0000003461bfcb04 | 00  | 000 | 00000000 |  NUL
//!  0000003461bfcb05 | 00  | 000 | 00000000 |  NUL
//!  0000003461bfcb06 | 00  | 000 | 00000000 |  NUL
//!  0000003461bfcb07 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_str
//! Type: str
//! Addr: 00007ff620a62f98
//! Size: 5 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00007ff620a62f98 | 48  | 072 | 01001000 |  H
//!  00007ff620a62f99 | 65  | 101 | 01100101 |  e
//!  00007ff620a62f9a | 6c  | 108 | 01101100 |  l
//!  00007ff620a62f9b | 6c  | 108 | 01101100 |  l
//!  00007ff620a62f9c | 6f  | 111 | 01101111 |  o
//! 
//! Name         : &my_str
//! Type         : &&str
//! Addr         : 0000003461bfcaf8
//! Size         : 8 bytes
//! Container Ptr: 000001e2998d1c30
//! Container Len: 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e2998d1c30 | 48  | 072 | 01001000 | H
//! 000001e2998d1c31 | 65  | 101 | 01100101 | e
//! 000001e2998d1c32 | 6c  | 108 | 01101100 | l
//! 000001e2998d1c33 | 6c  | 108 | 01101100 | l
//! 000001e2998d1c34 | 6f  | 111 | 01101111 | o
//! 
//! Name         : my_str
//! Type         : &str
//! Addr         : 00007ff620a62f98
//! Size         : 16 bytes
//! Container Ptr: 000001e2998d1c40
//! Container Len: 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e2998d1c40 | 48  | 072 | 01001000 | H
//! 000001e2998d1c41 | 65  | 101 | 01100101 | e
//! 000001e2998d1c42 | 6c  | 108 | 01101100 | l
//! 000001e2998d1c43 | 6c  | 108 | 01101100 | l
//! 000001e2998d1c44 | 6f  | 111 | 01101111 | o
//! 
//! 
//! ---- tests::struct_viewer stdout ----
//! This should print the memory address of the struct and the memory of its fields.
//! 
//! Name         : &my_serialized_struct
//! Type         : &mem_viewer::tests::MySerializedStruct
//! Addr         : 0000003461dfe168
//! Size         : 8 bytes
//! Container Ptr: 000001e2998cb350
//! Container Len: 7
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e2998cb350 | 45  | 069 | 01000101 | E
//! 000001e2998cb351 | ff  | 255 | 11111111 | ...
//! 000001e2998cb352 | 00  | 000 | 00000000 | NUL
//! 000001e2998cb353 | 46  | 070 | 01000110 | F
//! 000001e2998cb354 | 00  | 000 | 00000000 | NUL
//! 000001e2998cb355 | 00  | 000 | 00000000 | NUL
//! 000001e2998cb356 | 00  | 000 | 00000000 | NUL
//! 
//! Name: &my_struct
//! Type: &mem_viewer::tests::MyStruct
//! Addr: 0000003461dfdcb8
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  0000003461dfdda0 | 10  | 016 | 00010000 |  DLE
//!  0000003461dfdda1 | dc  | 220 | 11011100 |  ...
//!  0000003461dfdda2 | df  | 223 | 11011111 |  ...
//!  0000003461dfdda3 | 61  | 097 | 01100001 |  a
//!  0000003461dfdda4 | 34  | 052 | 00110100 |  4
//!  0000003461dfdda5 | 00  | 000 | 00000000 |  NUL
//!  0000003461dfdda6 | 00  | 000 | 00000000 |  NUL
//!  0000003461dfdda7 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: my_struct
//! Type: mem_viewer::tests::MyStruct
//! Addr: 0000003461dfdc10
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  0000003461dfdc10 | 46  | 070 | 01000110 |  F
//!  0000003461dfdc11 | 00  | 000 | 00000000 |  NUL
//!  0000003461dfdc12 | 00  | 000 | 00000000 |  NUL
//!  0000003461dfdc13 | 00  | 000 | 00000000 |  NUL
//!  0000003461dfdc14 | ff  | 255 | 11111111 |  ...
//!  0000003461dfdc15 | 00  | 000 | 00000000 |  NUL
//!  0000003461dfdc16 | 45  | 069 | 01000101 |  E
//!  0000003461dfdc17 | 00  | 000 | 00000000 |  NUL
//! 
//! 
//! ---- tests::u16_viewer stdout ----
//! This should print the memory of the holy number 69.
//! 
//! Name: my_u16
//! Type: u16
//! Addr: 0000003461ffe006
//! Size: 2 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  0000003461ffe006 | 45  | 069 | 01000101 |  E
//!  0000003461ffe007 | 00  | 000 | 00000000 |  NUL
//! 
//! Name         : &my_u16
//! Type         : &u16
//! Addr         : 0000003461ffe006
//! Size         : 8 bytes
//! Container Ptr: 000001e2998cb220
//! Container Len: 2
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e2998cb220 | 45  | 069 | 01000101 | E
//! 000001e2998cb221 | 00  | 000 | 00000000 | NUL
//! 
//! 
//! ---- tests::vec_of_box_viewer stdout ----
//! This should print the memory address of the vector of boxes and the memory of its elements.
//! 
//! Name         : &my_vec_of_box
//! Type         : &alloc::vec::Vec<alloc::boxed::Box<u8>>
//! Addr         : 00000034615fe530
//! Size         : 8 bytes
//! Container Ptr: 000001e2998d1c00
//! Container Len: 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e2998d1c00 | 45  | 069 | 01000101 | E
//! 000001e2998d1c01 | ff  | 255 | 11111111 | ...
//! 000001e2998d1c02 | fe  | 254 | 11111110 | ...
//! 000001e2998d1c03 | fd  | 253 | 11111101 | ...
//! 000001e2998d1c04 | 46  | 070 | 01000110 | F
//! 
//! Name: my_vec_of_box
//! Type: alloc::vec::Vec<alloc::boxed::Box<u8>>
//! Addr: 00000034615ff0d0
//! Size: 24 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00000034615ff0d0 | 05  | 005 | 00000101 |  ENQ
//!  00000034615ff0d1 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0d2 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0d3 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0d4 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0d5 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0d6 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0d7 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0d8 | 50  | 080 | 01010000 |  P
//!  00000034615ff0d9 | 85  | 133 | 10000101 |  ...
//!  00000034615ff0da | 8c  | 140 | 10001100 |  ...
//!  00000034615ff0db | 99  | 153 | 10011001 |  ...
//!  00000034615ff0dc | e2  | 226 | 11100010 |  ...
//!  00000034615ff0dd | 01  | 001 | 00000001 |  SOH
//!  00000034615ff0de | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0df | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0e0 | 05  | 005 | 00000101 |  ENQ
//!  00000034615ff0e1 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0e2 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0e3 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0e4 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0e5 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0e6 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff0e7 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec_of_box
//! Type: [alloc::boxed::Box<u8>]
//! Addr: 000001e2998c8550
//! Size: 40 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000001e2998c8550 | 90  | 144 | 10010000 |  ...
//!  000001e2998c8551 | 1c  | 028 | 00011100 |  FS
//!  000001e2998c8552 | 8d  | 141 | 10001101 |  ...
//!  000001e2998c8553 | 99  | 153 | 10011001 |  ...
//!  000001e2998c8554 | e2  | 226 | 11100010 |  ...
//!  000001e2998c8555 | 01  | 001 | 00000001 |  SOH
//!  000001e2998c8556 | 00  | 000 | 00000000 |  NUL
//!  000001e2998c8557 | 00  | 000 | 00000000 |  NUL
//!  000001e2998c8558 | 80  | 128 | 10000000 |  ...
//!  000001e2998c8559 | 1c  | 028 | 00011100 |  FS
//!  000001e2998c855a | 8d  | 141 | 10001101 |  ...
//!  000001e2998c855b | 99  | 153 | 10011001 |  ...
//!  000001e2998c855c | e2  | 226 | 11100010 |  ...
//!  000001e2998c855d | 01  | 001 | 00000001 |  SOH
//!  000001e2998c855e | 00  | 000 | 00000000 |  NUL
//!  000001e2998c855f | 00  | 000 | 00000000 |  NUL
//!  000001e2998c8560 | a0  | 160 | 10100000 |  ...
//!  000001e2998c8561 | 1c  | 028 | 00011100 |  FS
//!  000001e2998c8562 | 8d  | 141 | 10001101 |  ...
//!  000001e2998c8563 | 99  | 153 | 10011001 |  ...
//!  000001e2998c8564 | e2  | 226 | 11100010 |  ...
//!  000001e2998c8565 | 01  | 001 | 00000001 |  SOH
//!  000001e2998c8566 | 00  | 000 | 00000000 |  NUL
//!  000001e2998c8567 | 00  | 000 | 00000000 |  NUL
//!  000001e2998c8568 | 40  | 064 | 01000000 |  @
//!  000001e2998c8569 | 1c  | 028 | 00011100 |  FS
//!  000001e2998c856a | 8d  | 141 | 10001101 |  ...
//!  000001e2998c856b | 99  | 153 | 10011001 |  ...
//!  000001e2998c856c | e2  | 226 | 11100010 |  ...
//!  000001e2998c856d | 01  | 001 | 00000001 |  SOH
//!  000001e2998c856e | 00  | 000 | 00000000 |  NUL
//!  000001e2998c856f | 00  | 000 | 00000000 |  NUL
//!  000001e2998c8570 | 60  | 096 | 01100000 |  `
//!  000001e2998c8571 | 1c  | 028 | 00011100 |  FS
//!  000001e2998c8572 | 8d  | 141 | 10001101 |  ...
//!  000001e2998c8573 | 99  | 153 | 10011001 |  ...
//!  000001e2998c8574 | e2  | 226 | 11100010 |  ...
//!  000001e2998c8575 | 01  | 001 | 00000001 |  SOH
//!  000001e2998c8576 | 00  | 000 | 00000000 |  NUL
//!  000001e2998c8577 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec_of_box[0]
//! Type: u8
//! Addr: 000001e2998d1c90
//! Size: 1 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000001e2998d1c90 | 45  | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::vec_viewer stdout ----
//! This should print the memory address of the vector and the memory of its elements.
//! 
//! Name         : &my_vec
//! Type         : &alloc::vec::Vec<u8>
//! Addr         : 00000034615fe508
//! Size         : 8 bytes
//! Container Ptr: 000001e2998d1c60
//! Container Len: 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000001e2998d1c60 | 45  | 069 | 01000101 | E
//! 000001e2998d1c61 | ff  | 255 | 11111111 | ...
//! 000001e2998d1c62 | fe  | 254 | 11111110 | ...
//! 000001e2998d1c63 | fd  | 253 | 11111101 | ...
//! 000001e2998d1c64 | 46  | 070 | 01000110 | F
//! 
//! Name: my_vec
//! Type: alloc::vec::Vec<u8>
//! Addr: 00000034615ff080
//! Size: 24 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00000034615ff080 | 05  | 005 | 00000101 |  ENQ
//!  00000034615ff081 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff082 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff083 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff084 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff085 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff086 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff087 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff088 | 40  | 064 | 01000000 |  @
//!  00000034615ff089 | 1c  | 028 | 00011100 |  FS
//!  00000034615ff08a | 8d  | 141 | 10001101 |  ...
//!  00000034615ff08b | 99  | 153 | 10011001 |  ...
//!  00000034615ff08c | e2  | 226 | 11100010 |  ...
//!  00000034615ff08d | 01  | 001 | 00000001 |  SOH
//!  00000034615ff08e | 00  | 000 | 00000000 |  NUL
//!  00000034615ff08f | 00  | 000 | 00000000 |  NUL
//!  00000034615ff090 | 05  | 005 | 00000101 |  ENQ
//!  00000034615ff091 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff092 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff093 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff094 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff095 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff096 | 00  | 000 | 00000000 |  NUL
//!  00000034615ff097 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec
//! Type: [u8]
//! Addr: 000001e2998d1c40
//! Size: 5 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000001e2998d1c40 | 45  | 069 | 01000101 |  E
//!  000001e2998d1c41 | ff  | 255 | 11111111 |  ...
//!  000001e2998d1c42 | fe  | 254 | 11111110 |  ...
//!  000001e2998d1c43 | fd  | 253 | 11111101 |  ...
//!  000001e2998d1c44 | 46  | 070 | 01000110 |  F
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
/// * `&var` - The variable whose memory content needs to be viewed.
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
/// Name          : &my_var
/// Type          : &u16
/// Addr          : 000000acf3bfdc86
/// Size          : 8 bytes
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
		println!("Name         : {}", stringify!($var));
		println!("Type         : {}", _get_type_of($var));
        println!("Addr         : {:016x}", $var as *const _ as *const u8 as usize);
		println!("Size         : {} bytes", size);

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
        println!("Container Ptr: {:016x}", container.as_ptr() as usize);
		println!("Container Len: {}", container.len());

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
				format!(" {} ", *byte as char)
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
							_   => "...",
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
/// * `var` - The variable whose memory content needs to be viewed.
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
///      Address      | Hex | Dec |    Bin   | ASCII
/// ------------------Memory Content----------------
///  00000007f88fdc56 | 45 | 069  | 01000101 |  E
///  00000007f88fdc57 | 00 | 000  | 00000000 |  NUL
/// ```
macro_rules! view_mem {
    ($var: expr) => {
        // Get the size of the variable
        // let size = std::mem::size_of_val(&$var);

        // Print metadata of var: var_name, size, type, separated by a new line for each meta
        println!("Name: {}", stringify!($var));
        _print_type_of(&$var);
        println!("Addr: {:016x}", &$var as *const _ as *const u8 as usize);
        println!("Size: {} bytes", std::mem::size_of_val(&$var));

        _show_memory_content(&$var as *const _ as *const u8, std::mem::size_of_val(&$var));
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

    let mut ptr: *const u8 = src_ptr;
    let end: *const u8 = unsafe { src_ptr.add(len) };

    println!("     Address      | Hex | Dec |    Bin   | ASCII");
    println!("-----------------Memory Content-----------------");
    while ptr < end {
        let byte = unsafe {*ptr};

        let ascii = if byte.is_ascii_graphic() {
            format!(" {} ", byte as char)
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
                _   => "...",
            }.to_string()
        };

        println!(" {:016x} | {:02x}  | {:03} | {:08b} |  {}", ptr as usize, byte as u8, byte as u8, byte as u8, ascii);

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

