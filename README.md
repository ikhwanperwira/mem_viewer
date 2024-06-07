# Overview

`mem_viewer` is a Rust crate that provides a macro `view_mem!` to view the memory content of an arbitrary variable. It supports viewing memory content of different data types including integers, floating-point numbers, strings, pointers, vectors, boxed variables, and structs.

## Usage

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
mem_viewer = "0.2.1"
```

Then, in your Rust code, you can use the `view_mem!` macro to view the memory content of a variable. Here's an example:

```rust
use mem_viewer::*;

let my_var: u16 = 69;
view_mem!(my_var);
```

This will print the memory content of `my_var` to the console.

## Example Output

```none
ame: my_var
Type: u16
Addr: 000000719ddfdce6
Size: 2 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-------------------Memory Content---------------
 000000719ddfdce6 | 45  | 069 | 01000101 |  E
 000000719ddfdce7 | 00  | 000 | 00000000 |  NUL
```

Read more for the macro usage: [https://docs.rs/mem_viewer/latest/mem_viewer/macro.view_mem.html](https://docs.rs/mem_viewer/latest/mem_viewer/macro.view_mem.html)

## Safe Usage
For safe mode, you can use the `safe_view_mem!` macro to view the memory content of a variable. Here's an example:
```rust
use mem_viewer::*;

let my_var: u16 = 69;
safe_view_mem!(&my_var);
```

## Example Safe Output
```none
Name          : &my_var
Type          : &u16
Addr          : 000000719ddfdce6
Size          : 8 bytes
Container Ptr : 00000260c1266610
Container Len : 2
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
00000260c1266610 | 45  | 069 | 01000101 | E
00000260c1266611 | 00  | 000 | 00000000 | NUL
```

Read more for safe usage: [https://docs.rs/mem_viewer/latest/mem_viewer/macro.safe_view_mem.html](https://docs.rs/mem_viewer/latest/mem_viewer/macro.safe_view_mem.html)

## License

This crate is licensed under the MIT License.

## Contributing

Contributions are welcome! If you find any issues or have any suggestions, please open an issue or submit a pull request on [GitHub](https://github.com/ikhwanperwira/mem_viewer).

# Unit Test Report

## Code Test:
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

## Output Test:
```none
running 8 tests
test tests::ptr_viewer ... ok
test tests::f32_viewer ... ok
test tests::box_viewer ... ok
test tests::str_viewer ... ok
test tests::struct_viewer ... ok
test tests::u16_viewer ... ok
test tests::vec_of_box_viewer ... ok
test tests::vec_viewer ... ok

successes:

---- tests::ptr_viewer stdout ----
This should print the memory of a pointer.

Currently pointer type is not supported by safe view memory.

Name: my_ptr
Type: *const u8
Addr: 000000d4ef4fe980
Size: 8 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef4fe980 | a8  | 168 | 10101000 |  ...
 000000d4ef4fe981 | 33  | 051 | 00110011 |  3
 000000d4ef4fe982 | f6  | 246 | 11110110 |  ...
 000000d4ef4fe983 | 9e  | 158 | 10011110 |  ...
 000000d4ef4fe984 | f7  | 247 | 11110111 |  ...
 000000d4ef4fe985 | 7f  | 127 | 01111111 |  DEL
 000000d4ef4fe986 | 00  | 000 | 00000000 |  NUL
 000000d4ef4fe987 | 00  | 000 | 00000000 |  NUL

Name: *my_ptr
Type: u8
Addr: 00007ff79ef633a8
Size: 1 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 00007ff79ef633a8 | 45  | 069 | 01000101 |  E


---- tests::f32_viewer stdout ----
This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.

Name: my_f32
Type: f32
Addr: 000000d4ef2fdbb4
Size: 4 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef2fdbb4 | c3  | 195 | 11000011 |  ...
 000000d4ef2fdbb5 | f5  | 245 | 11110101 |  ...
 000000d4ef2fdbb6 | 48  | 072 | 01001000 |  H
 000000d4ef2fdbb7 | 40  | 064 | 01000000 |  @

Name         : &my_f32
Type         : &f32
Addr         : 000000d4ef2fdbb4
Size         : 8 bytes
Container Ptr: 000002924a085200
Container Len: 4
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000002924a085200 | c3  | 195 | 11000011 | UNK
000002924a085201 | f5  | 245 | 11110101 | UNK
000002924a085202 | 48  | 072 | 01001000 | H
000002924a085203 | 40  | 064 | 01000000 | @


---- tests::box_viewer stdout ----
This should print the memory address of the box and the memory of its value.

Name         : &my_box
Type         : &alloc::boxed::Box<u8>
Addr         : 000000d4ef0fde48
Size         : 8 bytes
Container Ptr: 000002924a085210
Container Len: 1
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000002924a085210 | 45  | 069 | 01000101 | E

Name: &my_box
Type: &alloc::boxed::Box<u8>
Addr: 000000d4ef0fd7d8
Size: 8 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef0fd8c0 | 38  | 056 | 00111000 |  8
 000000d4ef0fd8c1 | d7  | 215 | 11010111 |  ...
 000000d4ef0fd8c2 | 0f  | 015 | 00001111 |  SI
 000000d4ef0fd8c3 | ef  | 239 | 11101111 |  ...
 000000d4ef0fd8c4 | d4  | 212 | 11010100 |  ...
 000000d4ef0fd8c5 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fd8c6 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fd8c7 | 00  | 000 | 00000000 |  NUL

Name: my_box
Type: alloc::boxed::Box<u8>
Addr: 000000d4ef0fd738
Size: 8 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef0fd738 | 30  | 048 | 00110000 |  0
 000000d4ef0fd739 | 51  | 081 | 01010001 |  Q
 000000d4ef0fd73a | 08  | 008 | 00001000 |  BS
 000000d4ef0fd73b | 4a  | 074 | 01001010 |  J
 000000d4ef0fd73c | 92  | 146 | 10010010 |  ...
 000000d4ef0fd73d | 02  | 002 | 00000010 |  STX
 000000d4ef0fd73e | 00  | 000 | 00000000 |  NUL
 000000d4ef0fd73f | 00  | 000 | 00000000 |  NUL

Name: *my_box
Type: u8
Addr: 000002924a085130
Size: 1 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000002924a085130 | 45  | 069 | 01000101 |  E


---- tests::str_viewer stdout ----
This should print the memory of 'Hello' and its address.

Name: my_str
Type: &str
Addr: 000000d4ef6fd228
Size: 16 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef6fd228 | 48  | 072 | 01001000 |  H
 000000d4ef6fd229 | 33  | 051 | 00110011 |  3
 000000d4ef6fd22a | f6  | 246 | 11110110 |  ...
 000000d4ef6fd22b | 9e  | 158 | 10011110 |  ...
 000000d4ef6fd22c | f7  | 247 | 11110111 |  ...
 000000d4ef6fd22d | 7f  | 127 | 01111111 |  DEL
 000000d4ef6fd22e | 00  | 000 | 00000000 |  NUL
 000000d4ef6fd22f | 00  | 000 | 00000000 |  NUL
 000000d4ef6fd230 | 05  | 005 | 00000101 |  ENQ
 000000d4ef6fd231 | 00  | 000 | 00000000 |  NUL
 000000d4ef6fd232 | 00  | 000 | 00000000 |  NUL
 000000d4ef6fd233 | 00  | 000 | 00000000 |  NUL
 000000d4ef6fd234 | 00  | 000 | 00000000 |  NUL
 000000d4ef6fd235 | 00  | 000 | 00000000 |  NUL
 000000d4ef6fd236 | 00  | 000 | 00000000 |  NUL
 000000d4ef6fd237 | 00  | 000 | 00000000 |  NUL

Name: *my_str
Type: str
Addr: 00007ff79ef63348
Size: 5 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 00007ff79ef63348 | 48  | 072 | 01001000 |  H
 00007ff79ef63349 | 65  | 101 | 01100101 |  e
 00007ff79ef6334a | 6c  | 108 | 01101100 |  l
 00007ff79ef6334b | 6c  | 108 | 01101100 |  l
 00007ff79ef6334c | 6f  | 111 | 01101111 |  o

Name         : &my_str
Type         : &&str
Addr         : 000000d4ef6fd228
Size         : 8 bytes
Container Ptr: 000002924a092bf0
Container Len: 5
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000002924a092bf0 | 48  | 072 | 01001000 | H
000002924a092bf1 | 65  | 101 | 01100101 | e
000002924a092bf2 | 6c  | 108 | 01101100 | l
000002924a092bf3 | 6c  | 108 | 01101100 | l
000002924a092bf4 | 6f  | 111 | 01101111 | o

Name         : my_str
Type         : &str
Addr         : 00007ff79ef63348
Size         : 16 bytes
Container Ptr: 000002924a092c00
Container Len: 5
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000002924a092c00 | 48  | 072 | 01001000 | H
000002924a092c01 | 65  | 101 | 01100101 | e
000002924a092c02 | 6c  | 108 | 01101100 | l
000002924a092c03 | 6c  | 108 | 01101100 | l
000002924a092c04 | 6f  | 111 | 01101111 | o


---- tests::struct_viewer stdout ----
This should print the memory address of the struct and the memory of its fields.

Name         : &my_serialized_struct
Type         : &mem_viewer::tests::MySerializedStruct
Addr         : 000000d4ef8fe508
Size         : 8 bytes
Container Ptr: 000002924a08df10
Container Len: 7
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000002924a08df10 | 45  | 069 | 01000101 | E
000002924a08df11 | ff  | 255 | 11111111 | UNK
000002924a08df12 | 00  | 000 | 00000000 | NUL
000002924a08df13 | 46  | 070 | 01000110 | F
000002924a08df14 | 00  | 000 | 00000000 | NUL
000002924a08df15 | 00  | 000 | 00000000 | NUL
000002924a08df16 | 00  | 000 | 00000000 | NUL

Name: &my_struct
Type: &mem_viewer::tests::MyStruct
Addr: 000000d4ef8fe058
Size: 8 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef8fe140 | b0  | 176 | 10110000 |  ...
 000000d4ef8fe141 | df  | 223 | 11011111 |  ...
 000000d4ef8fe142 | 8f  | 143 | 10001111 |  ...
 000000d4ef8fe143 | ef  | 239 | 11101111 |  ...
 000000d4ef8fe144 | d4  | 212 | 11010100 |  ...
 000000d4ef8fe145 | 00  | 000 | 00000000 |  NUL
 000000d4ef8fe146 | 00  | 000 | 00000000 |  NUL
 000000d4ef8fe147 | 00  | 000 | 00000000 |  NUL

Name: my_struct
Type: mem_viewer::tests::MyStruct
Addr: 000000d4ef8fdfb0
Size: 8 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef8fdfb0 | 46  | 070 | 01000110 |  F
 000000d4ef8fdfb1 | 00  | 000 | 00000000 |  NUL
 000000d4ef8fdfb2 | 00  | 000 | 00000000 |  NUL
 000000d4ef8fdfb3 | 00  | 000 | 00000000 |  NUL
 000000d4ef8fdfb4 | ff  | 255 | 11111111 |  ...
 000000d4ef8fdfb5 | 00  | 000 | 00000000 |  NUL
 000000d4ef8fdfb6 | 45  | 069 | 01000101 |  E
 000000d4ef8fdfb7 | 00  | 000 | 00000000 |  NUL


---- tests::u16_viewer stdout ----
This should print the memory of the holy number 69.

Name: my_u16
Type: u16
Addr: 000000d4efafe2a6
Size: 2 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4efafe2a6 | 45  | 069 | 01000101 |  E
 000000d4efafe2a7 | 00  | 000 | 00000000 |  NUL

Name         : &my_u16
Type         : &u16
Addr         : 000000d4efafe2a6
Size         : 8 bytes
Container Ptr: 000002924a085290
Container Len: 2
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000002924a085290 | 45  | 069 | 01000101 | E
000002924a085291 | 00  | 000 | 00000000 | NUL


---- tests::vec_of_box_viewer stdout ----
This should print the memory address of the vector of boxes and the memory of its elements.

Name         : &my_vec_of_box
Type         : &alloc::vec::Vec<alloc::boxed::Box<u8>>
Addr         : 000000d4ef0fe490
Size         : 8 bytes
Container Ptr: 000002924a07cd30
Container Len: 5
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000002924a07cd30 | 45  | 069 | 01000101 | E
000002924a07cd31 | ff  | 255 | 11111111 | UNK
000002924a07cd32 | fe  | 254 | 11111110 | UNK
000002924a07cd33 | fd  | 253 | 11111101 | UNK
000002924a07cd34 | 46  | 070 | 01000110 | F

Name: my_vec_of_box
Type: alloc::vec::Vec<alloc::boxed::Box<u8>>
Addr: 000000d4ef0ff030
Size: 24 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef0ff030 | 05  | 005 | 00000101 |  ENQ
 000000d4ef0ff031 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff032 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff033 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff034 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff035 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff036 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff037 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff038 | 30  | 048 | 00110000 |  0
 000000d4ef0ff039 | 2b  | 043 | 00101011 |  +
 000000d4ef0ff03a | 08  | 008 | 00001000 |  BS
 000000d4ef0ff03b | 4a  | 074 | 01001010 |  J
 000000d4ef0ff03c | 92  | 146 | 10010010 |  ...
 000000d4ef0ff03d | 02  | 002 | 00000010 |  STX
 000000d4ef0ff03e | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff03f | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff040 | 05  | 005 | 00000101 |  ENQ
 000000d4ef0ff041 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff042 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff043 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff044 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff045 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff046 | 00  | 000 | 00000000 |  NUL
 000000d4ef0ff047 | 00  | 000 | 00000000 |  NUL

Name: *my_vec_of_box
Type: [alloc::boxed::Box<u8>]
Addr: 000002924a082b30
Size: 40 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000002924a082b30 | 10  | 016 | 00010000 |  DLE
 000002924a082b31 | cd  | 205 | 11001101 |  ...
 000002924a082b32 | 07  | 007 | 00000111 |  BEL
 000002924a082b33 | 4a  | 074 | 01001010 |  J
 000002924a082b34 | 92  | 146 | 10010010 |  ...
 000002924a082b35 | 02  | 002 | 00000010 |  STX
 000002924a082b36 | 00  | 000 | 00000000 |  NUL
 000002924a082b37 | 00  | 000 | 00000000 |  NUL
 000002924a082b38 | d0  | 208 | 11010000 |  ...
 000002924a082b39 | cc  | 204 | 11001100 |  ...
 000002924a082b3a | 07  | 007 | 00000111 |  BEL
 000002924a082b3b | 4a  | 074 | 01001010 |  J
 000002924a082b3c | 92  | 146 | 10010010 |  ...
 000002924a082b3d | 02  | 002 | 00000010 |  STX
 000002924a082b3e | 00  | 000 | 00000000 |  NUL
 000002924a082b3f | 00  | 000 | 00000000 |  NUL
 000002924a082b40 | e0  | 224 | 11100000 |  ...
 000002924a082b41 | cc  | 204 | 11001100 |  ...
 000002924a082b42 | 07  | 007 | 00000111 |  BEL
 000002924a082b43 | 4a  | 074 | 01001010 |  J
 000002924a082b44 | 92  | 146 | 10010010 |  ...
 000002924a082b45 | 02  | 002 | 00000010 |  STX
 000002924a082b46 | 00  | 000 | 00000000 |  NUL
 000002924a082b47 | 00  | 000 | 00000000 |  NUL
 000002924a082b48 | 40  | 064 | 01000000 |  @
 000002924a082b49 | cd  | 205 | 11001101 |  ...
 000002924a082b4a | 07  | 007 | 00000111 |  BEL
 000002924a082b4b | 4a  | 074 | 01001010 |  J
 000002924a082b4c | 92  | 146 | 10010010 |  ...
 000002924a082b4d | 02  | 002 | 00000010 |  STX
 000002924a082b4e | 00  | 000 | 00000000 |  NUL
 000002924a082b4f | 00  | 000 | 00000000 |  NUL
 000002924a082b50 | c0  | 192 | 11000000 |  ...
 000002924a082b51 | cc  | 204 | 11001100 |  ...
 000002924a082b52 | 07  | 007 | 00000111 |  BEL
 000002924a082b53 | 4a  | 074 | 01001010 |  J
 000002924a082b54 | 92  | 146 | 10010010 |  ...
 000002924a082b55 | 02  | 002 | 00000010 |  STX
 000002924a082b56 | 00  | 000 | 00000000 |  NUL
 000002924a082b57 | 00  | 000 | 00000000 |  NUL

Name: *my_vec_of_box[0]
Type: u8
Addr: 000002924a07cd10
Size: 1 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000002924a07cd10 | 45  | 069 | 01000101 |  E


---- tests::vec_viewer stdout ----
This should print the memory address of the vector and the memory of its elements.

Name         : &my_vec
Type         : &alloc::vec::Vec<u8>
Addr         : 000000d4ef0fdf18
Size         : 8 bytes
Container Ptr: 000002924a0851e0
Container Len: 5
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000002924a0851e0 | 45  | 069 | 01000101 | E
000002924a0851e1 | ff  | 255 | 11111111 | UNK
000002924a0851e2 | fe  | 254 | 11111110 | UNK
000002924a0851e3 | fd  | 253 | 11111101 | UNK
000002924a0851e4 | 46  | 070 | 01000110 | F

Name: my_vec
Type: alloc::vec::Vec<u8>
Addr: 000000d4ef0fea90
Size: 24 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000000d4ef0fea90 | 05  | 005 | 00000101 |  ENQ
 000000d4ef0fea91 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fea92 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fea93 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fea94 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fea95 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fea96 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fea97 | 00  | 000 | 00000000 |  NUL
 000000d4ef0fea98 | 20  | 032 | 00100000 |  SPC
 000000d4ef0fea99 | 51  | 081 | 01010001 |  Q
 000000d4ef0fea9a | 08  | 008 | 00001000 |  BS
 000000d4ef0fea9b | 4a  | 074 | 01001010 |  J
 000000d4ef0fea9c | 92  | 146 | 10010010 |  ...
 000000d4ef0fea9d | 02  | 002 | 00000010 |  STX
 000000d4ef0fea9e | 00  | 000 | 00000000 |  NUL
 000000d4ef0fea9f | 00  | 000 | 00000000 |  NUL
 000000d4ef0feaa0 | 05  | 005 | 00000101 |  ENQ
 000000d4ef0feaa1 | 00  | 000 | 00000000 |  NUL
 000000d4ef0feaa2 | 00  | 000 | 00000000 |  NUL
 000000d4ef0feaa3 | 00  | 000 | 00000000 |  NUL
 000000d4ef0feaa4 | 00  | 000 | 00000000 |  NUL
 000000d4ef0feaa5 | 00  | 000 | 00000000 |  NUL
 000000d4ef0feaa6 | 00  | 000 | 00000000 |  NUL
 000000d4ef0feaa7 | 00  | 000 | 00000000 |  NUL

Name: *my_vec
Type: [u8]
Addr: 000002924a085120
Size: 5 bytes
     Address      | Hex | Dec |    Bin   | ASCII
-----------------Memory Content-----------------
 000002924a085120 | 45  | 069 | 01000101 |  E
 000002924a085121 | ff  | 255 | 11111111 |  ...
 000002924a085122 | fe  | 254 | 11111110 |  ...
 000002924a085123 | fd  | 253 | 11111101 |  ...
 000002924a085124 | 46  | 070 | 01000110 |  F



successes:
    tests::box_viewer
    tests::f32_viewer
    tests::ptr_viewer
    tests::str_viewer
    tests::struct_viewer
    tests::u16_viewer
    tests::vec_of_box_viewer
    tests::vec_viewer

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
