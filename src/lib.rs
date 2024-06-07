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
//! mem_viewer = "0.2.1"
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
//! Name          : &my_var
//! Type          : &u16
//! Addr          : 000000719ddfdce6
//! Size          : 8 bytes
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
//! test tests::ptr_viewer ... ok
//! test tests::f32_viewer ... ok
//! test tests::box_viewer ... ok
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
//! Addr: 000000d4ef4fe980
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef4fe980 | a8  | 168 | 10101000 |  ...
//!  000000d4ef4fe981 | 33  | 051 | 00110011 |  3
//!  000000d4ef4fe982 | f6  | 246 | 11110110 |  ...
//!  000000d4ef4fe983 | 9e  | 158 | 10011110 |  ...
//!  000000d4ef4fe984 | f7  | 247 | 11110111 |  ...
//!  000000d4ef4fe985 | 7f  | 127 | 01111111 |  DEL
//!  000000d4ef4fe986 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef4fe987 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_ptr
//! Type: u8
//! Addr: 00007ff79ef633a8
//! Size: 1 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00007ff79ef633a8 | 45  | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::f32_viewer stdout ----
//! This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.
//! 
//! Name: my_f32
//! Type: f32
//! Addr: 000000d4ef2fdbb4
//! Size: 4 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef2fdbb4 | c3  | 195 | 11000011 |  ...
//!  000000d4ef2fdbb5 | f5  | 245 | 11110101 |  ...
//!  000000d4ef2fdbb6 | 48  | 072 | 01001000 |  H
//!  000000d4ef2fdbb7 | 40  | 064 | 01000000 |  @
//! 
//! Name         : &my_f32
//! Type         : &f32
//! Addr         : 000000d4ef2fdbb4
//! Size         : 8 bytes
//! Container Ptr: 000002924a085200
//! Container Len: 4
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000002924a085200 | c3  | 195 | 11000011 | UNK
//! 000002924a085201 | f5  | 245 | 11110101 | UNK
//! 000002924a085202 | 48  | 072 | 01001000 | H
//! 000002924a085203 | 40  | 064 | 01000000 | @
//! 
//! 
//! ---- tests::box_viewer stdout ----
//! This should print the memory address of the box and the memory of its value.
//! 
//! Name         : &my_box
//! Type         : &alloc::boxed::Box<u8>
//! Addr         : 000000d4ef0fde48
//! Size         : 8 bytes
//! Container Ptr: 000002924a085210
//! Container Len: 1
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000002924a085210 | 45  | 069 | 01000101 | E
//! 
//! Name: &my_box
//! Type: &alloc::boxed::Box<u8>
//! Addr: 000000d4ef0fd7d8
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef0fd8c0 | 38  | 056 | 00111000 |  8
//!  000000d4ef0fd8c1 | d7  | 215 | 11010111 |  ...
//!  000000d4ef0fd8c2 | 0f  | 015 | 00001111 |  SI
//!  000000d4ef0fd8c3 | ef  | 239 | 11101111 |  ...
//!  000000d4ef0fd8c4 | d4  | 212 | 11010100 |  ...
//!  000000d4ef0fd8c5 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fd8c6 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fd8c7 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: my_box
//! Type: alloc::boxed::Box<u8>
//! Addr: 000000d4ef0fd738
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef0fd738 | 30  | 048 | 00110000 |  0
//!  000000d4ef0fd739 | 51  | 081 | 01010001 |  Q
//!  000000d4ef0fd73a | 08  | 008 | 00001000 |  BS
//!  000000d4ef0fd73b | 4a  | 074 | 01001010 |  J
//!  000000d4ef0fd73c | 92  | 146 | 10010010 |  ...
//!  000000d4ef0fd73d | 02  | 002 | 00000010 |  STX
//!  000000d4ef0fd73e | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fd73f | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_box
//! Type: u8
//! Addr: 000002924a085130
//! Size: 1 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000002924a085130 | 45  | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::str_viewer stdout ----
//! This should print the memory of 'Hello' and its address.
//! 
//! Name: my_str
//! Type: &str
//! Addr: 000000d4ef6fd228
//! Size: 16 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef6fd228 | 48  | 072 | 01001000 |  H
//!  000000d4ef6fd229 | 33  | 051 | 00110011 |  3
//!  000000d4ef6fd22a | f6  | 246 | 11110110 |  ...
//!  000000d4ef6fd22b | 9e  | 158 | 10011110 |  ...
//!  000000d4ef6fd22c | f7  | 247 | 11110111 |  ...
//!  000000d4ef6fd22d | 7f  | 127 | 01111111 |  DEL
//!  000000d4ef6fd22e | 00  | 000 | 00000000 |  NUL
//!  000000d4ef6fd22f | 00  | 000 | 00000000 |  NUL
//!  000000d4ef6fd230 | 05  | 005 | 00000101 |  ENQ
//!  000000d4ef6fd231 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef6fd232 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef6fd233 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef6fd234 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef6fd235 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef6fd236 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef6fd237 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_str
//! Type: str
//! Addr: 00007ff79ef63348
//! Size: 5 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  00007ff79ef63348 | 48  | 072 | 01001000 |  H
//!  00007ff79ef63349 | 65  | 101 | 01100101 |  e
//!  00007ff79ef6334a | 6c  | 108 | 01101100 |  l
//!  00007ff79ef6334b | 6c  | 108 | 01101100 |  l
//!  00007ff79ef6334c | 6f  | 111 | 01101111 |  o
//! 
//! Name         : &my_str
//! Type         : &&str
//! Addr         : 000000d4ef6fd228
//! Size         : 8 bytes
//! Container Ptr: 000002924a092bf0
//! Container Len: 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000002924a092bf0 | 48  | 072 | 01001000 | H
//! 000002924a092bf1 | 65  | 101 | 01100101 | e
//! 000002924a092bf2 | 6c  | 108 | 01101100 | l
//! 000002924a092bf3 | 6c  | 108 | 01101100 | l
//! 000002924a092bf4 | 6f  | 111 | 01101111 | o
//! 
//! Name         : my_str
//! Type         : &str
//! Addr         : 00007ff79ef63348
//! Size         : 16 bytes
//! Container Ptr: 000002924a092c00
//! Container Len: 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000002924a092c00 | 48  | 072 | 01001000 | H
//! 000002924a092c01 | 65  | 101 | 01100101 | e
//! 000002924a092c02 | 6c  | 108 | 01101100 | l
//! 000002924a092c03 | 6c  | 108 | 01101100 | l
//! 000002924a092c04 | 6f  | 111 | 01101111 | o
//! 
//! 
//! ---- tests::struct_viewer stdout ----
//! This should print the memory address of the struct and the memory of its fields.
//! 
//! Name         : &my_serialized_struct
//! Type         : &mem_viewer::tests::MySerializedStruct
//! Addr         : 000000d4ef8fe508
//! Size         : 8 bytes
//! Container Ptr: 000002924a08df10
//! Container Len: 7
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000002924a08df10 | 45  | 069 | 01000101 | E
//! 000002924a08df11 | ff  | 255 | 11111111 | UNK
//! 000002924a08df12 | 00  | 000 | 00000000 | NUL
//! 000002924a08df13 | 46  | 070 | 01000110 | F
//! 000002924a08df14 | 00  | 000 | 00000000 | NUL
//! 000002924a08df15 | 00  | 000 | 00000000 | NUL
//! 000002924a08df16 | 00  | 000 | 00000000 | NUL
//! 
//! Name: &my_struct
//! Type: &mem_viewer::tests::MyStruct
//! Addr: 000000d4ef8fe058
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef8fe140 | b0  | 176 | 10110000 |  ...
//!  000000d4ef8fe141 | df  | 223 | 11011111 |  ...
//!  000000d4ef8fe142 | 8f  | 143 | 10001111 |  ...
//!  000000d4ef8fe143 | ef  | 239 | 11101111 |  ...
//!  000000d4ef8fe144 | d4  | 212 | 11010100 |  ...
//!  000000d4ef8fe145 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef8fe146 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef8fe147 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: my_struct
//! Type: mem_viewer::tests::MyStruct
//! Addr: 000000d4ef8fdfb0
//! Size: 8 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef8fdfb0 | 46  | 070 | 01000110 |  F
//!  000000d4ef8fdfb1 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef8fdfb2 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef8fdfb3 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef8fdfb4 | ff  | 255 | 11111111 |  ...
//!  000000d4ef8fdfb5 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef8fdfb6 | 45  | 069 | 01000101 |  E
//!  000000d4ef8fdfb7 | 00  | 000 | 00000000 |  NUL
//! 
//! 
//! ---- tests::u16_viewer stdout ----
//! This should print the memory of the holy number 69.
//! 
//! Name: my_u16
//! Type: u16
//! Addr: 000000d4efafe2a6
//! Size: 2 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4efafe2a6 | 45  | 069 | 01000101 |  E
//!  000000d4efafe2a7 | 00  | 000 | 00000000 |  NUL
//! 
//! Name         : &my_u16
//! Type         : &u16
//! Addr         : 000000d4efafe2a6
//! Size         : 8 bytes
//! Container Ptr: 000002924a085290
//! Container Len: 2
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000002924a085290 | 45  | 069 | 01000101 | E
//! 000002924a085291 | 00  | 000 | 00000000 | NUL
//! 
//! 
//! ---- tests::vec_of_box_viewer stdout ----
//! This should print the memory address of the vector of boxes and the memory of its elements.
//! 
//! Name         : &my_vec_of_box
//! Type         : &alloc::vec::Vec<alloc::boxed::Box<u8>>
//! Addr         : 000000d4ef0fe490
//! Size         : 8 bytes
//! Container Ptr: 000002924a07cd30
//! Container Len: 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000002924a07cd30 | 45  | 069 | 01000101 | E
//! 000002924a07cd31 | ff  | 255 | 11111111 | UNK
//! 000002924a07cd32 | fe  | 254 | 11111110 | UNK
//! 000002924a07cd33 | fd  | 253 | 11111101 | UNK
//! 000002924a07cd34 | 46  | 070 | 01000110 | F
//! 
//! Name: my_vec_of_box
//! Type: alloc::vec::Vec<alloc::boxed::Box<u8>>
//! Addr: 000000d4ef0ff030
//! Size: 24 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef0ff030 | 05  | 005 | 00000101 |  ENQ
//!  000000d4ef0ff031 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff032 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff033 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff034 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff035 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff036 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff037 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff038 | 30  | 048 | 00110000 |  0
//!  000000d4ef0ff039 | 2b  | 043 | 00101011 |  +
//!  000000d4ef0ff03a | 08  | 008 | 00001000 |  BS
//!  000000d4ef0ff03b | 4a  | 074 | 01001010 |  J
//!  000000d4ef0ff03c | 92  | 146 | 10010010 |  ...
//!  000000d4ef0ff03d | 02  | 002 | 00000010 |  STX
//!  000000d4ef0ff03e | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff03f | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff040 | 05  | 005 | 00000101 |  ENQ
//!  000000d4ef0ff041 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff042 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff043 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff044 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff045 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff046 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0ff047 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec_of_box
//! Type: [alloc::boxed::Box<u8>]
//! Addr: 000002924a082b30
//! Size: 40 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000002924a082b30 | 10  | 016 | 00010000 |  DLE
//!  000002924a082b31 | cd  | 205 | 11001101 |  ...
//!  000002924a082b32 | 07  | 007 | 00000111 |  BEL
//!  000002924a082b33 | 4a  | 074 | 01001010 |  J
//!  000002924a082b34 | 92  | 146 | 10010010 |  ...
//!  000002924a082b35 | 02  | 002 | 00000010 |  STX
//!  000002924a082b36 | 00  | 000 | 00000000 |  NUL
//!  000002924a082b37 | 00  | 000 | 00000000 |  NUL
//!  000002924a082b38 | d0  | 208 | 11010000 |  ...
//!  000002924a082b39 | cc  | 204 | 11001100 |  ...
//!  000002924a082b3a | 07  | 007 | 00000111 |  BEL
//!  000002924a082b3b | 4a  | 074 | 01001010 |  J
//!  000002924a082b3c | 92  | 146 | 10010010 |  ...
//!  000002924a082b3d | 02  | 002 | 00000010 |  STX
//!  000002924a082b3e | 00  | 000 | 00000000 |  NUL
//!  000002924a082b3f | 00  | 000 | 00000000 |  NUL
//!  000002924a082b40 | e0  | 224 | 11100000 |  ...
//!  000002924a082b41 | cc  | 204 | 11001100 |  ...
//!  000002924a082b42 | 07  | 007 | 00000111 |  BEL
//!  000002924a082b43 | 4a  | 074 | 01001010 |  J
//!  000002924a082b44 | 92  | 146 | 10010010 |  ...
//!  000002924a082b45 | 02  | 002 | 00000010 |  STX
//!  000002924a082b46 | 00  | 000 | 00000000 |  NUL
//!  000002924a082b47 | 00  | 000 | 00000000 |  NUL
//!  000002924a082b48 | 40  | 064 | 01000000 |  @
//!  000002924a082b49 | cd  | 205 | 11001101 |  ...
//!  000002924a082b4a | 07  | 007 | 00000111 |  BEL
//!  000002924a082b4b | 4a  | 074 | 01001010 |  J
//!  000002924a082b4c | 92  | 146 | 10010010 |  ...
//!  000002924a082b4d | 02  | 002 | 00000010 |  STX
//!  000002924a082b4e | 00  | 000 | 00000000 |  NUL
//!  000002924a082b4f | 00  | 000 | 00000000 |  NUL
//!  000002924a082b50 | c0  | 192 | 11000000 |  ...
//!  000002924a082b51 | cc  | 204 | 11001100 |  ...
//!  000002924a082b52 | 07  | 007 | 00000111 |  BEL
//!  000002924a082b53 | 4a  | 074 | 01001010 |  J
//!  000002924a082b54 | 92  | 146 | 10010010 |  ...
//!  000002924a082b55 | 02  | 002 | 00000010 |  STX
//!  000002924a082b56 | 00  | 000 | 00000000 |  NUL
//!  000002924a082b57 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec_of_box[0]
//! Type: u8
//! Addr: 000002924a07cd10
//! Size: 1 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000002924a07cd10 | 45  | 069 | 01000101 |  E
//! 
//! 
//! ---- tests::vec_viewer stdout ----
//! This should print the memory address of the vector and the memory of its elements.
//! 
//! Name         : &my_vec
//! Type         : &alloc::vec::Vec<u8>
//! Addr         : 000000d4ef0fdf18
//! Size         : 8 bytes
//! Container Ptr: 000002924a0851e0
//! Container Len: 5
//!      Address     | Hex | Dec |    Bin   | ASCII
//! ---------------Container Content---------------
//! 000002924a0851e0 | 45  | 069 | 01000101 | E
//! 000002924a0851e1 | ff  | 255 | 11111111 | UNK
//! 000002924a0851e2 | fe  | 254 | 11111110 | UNK
//! 000002924a0851e3 | fd  | 253 | 11111101 | UNK
//! 000002924a0851e4 | 46  | 070 | 01000110 | F
//! 
//! Name: my_vec
//! Type: alloc::vec::Vec<u8>
//! Addr: 000000d4ef0fea90
//! Size: 24 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000000d4ef0fea90 | 05  | 005 | 00000101 |  ENQ
//!  000000d4ef0fea91 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fea92 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fea93 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fea94 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fea95 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fea96 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fea97 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fea98 | 20  | 032 | 00100000 |  SPC
//!  000000d4ef0fea99 | 51  | 081 | 01010001 |  Q
//!  000000d4ef0fea9a | 08  | 008 | 00001000 |  BS
//!  000000d4ef0fea9b | 4a  | 074 | 01001010 |  J
//!  000000d4ef0fea9c | 92  | 146 | 10010010 |  ...
//!  000000d4ef0fea9d | 02  | 002 | 00000010 |  STX
//!  000000d4ef0fea9e | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0fea9f | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0feaa0 | 05  | 005 | 00000101 |  ENQ
//!  000000d4ef0feaa1 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0feaa2 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0feaa3 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0feaa4 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0feaa5 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0feaa6 | 00  | 000 | 00000000 |  NUL
//!  000000d4ef0feaa7 | 00  | 000 | 00000000 |  NUL
//! 
//! Name: *my_vec
//! Type: [u8]
//! Addr: 000002924a085120
//! Size: 5 bytes
//!      Address      | Hex | Dec |    Bin   | ASCII
//! -----------------Memory Content-----------------
//!  000002924a085120 | 45  | 069 | 01000101 |  E
//!  000002924a085121 | ff  | 255 | 11111111 |  ...
//!  000002924a085122 | fe  | 254 | 11111110 |  ...
//!  000002924a085123 | fd  | 253 | 11111101 |  ...
//!  000002924a085124 | 46  | 070 | 01000110 |  F
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

    println!("     Address      | Hex | Dec |    Bin   | ASCII");
    println!("-----------------Memory Content-----------------");
    while ptr < end {
        value = unsafe {*ptr};

        print!(" {:016x} | {:02x}  | {:03} | {:08b} |", ptr as usize, value as usize, value as usize, value as usize);

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

