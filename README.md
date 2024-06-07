# Overview

`mem_viewer` is a Rust crate that provides a macro `view_mem!` to view the memory content of an arbitrary variable. It supports viewing memory content of different data types including integers, floating-point numbers, strings, pointers, vectors, boxed variables, and structs.

## Usage

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
mem_viewer = "0.2.0"
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
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000719ddfdce6 | 45 | 069 | 01000101 |  E
 000000719ddfdce7 | 00 | 000 | 00000000 |  NUL
```

## Safe Usage
For safe mode, you can use the `safe_view_mem!` macro to view the memory content of a variable. Here's an example:
```rust
use mem_viewer::*;

let my_var: u16 = 69;
safe_view_mem!(&my_var);
```

## Example Safe Output
```none
Name: &my_var
Type: &u16
Addr: 000000719ddfdce6
Size: 8 bytes
Container Ptr : 00000260c1266610
Container Len : 2
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
00000260c1266610 | 45  | 069 | 01000101 | E
00000260c1266611 | 00  | 000 | 00000000 | NUL
```

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
test tests::f32_viewer ... ok
test tests::box_viewer ... ok
test tests::ptr_viewer ... ok
test tests::str_viewer ... ok
test tests::u16_viewer ... ok
test tests::struct_viewer ... ok
test tests::vec_of_box_viewer ... ok
test tests::vec_viewer ... ok

successes:

---- tests::f32_viewer stdout ----
This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.

Name: my_f32
Type: f32
Addr: 000000dc288fdbf4
Size: 4 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc288fdbf4 | c3 | 195 | 11000011 |  ...
 000000dc288fdbf5 | f5 | 245 | 11110101 |  ...
 000000dc288fdbf6 | 48 | 072 | 01001000 |  H
 000000dc288fdbf7 | 40 | 064 | 01000000 |  @

Name: &my_f32
Type: &f32
Addr: 000000dc288fdbf4
Size: 8 bytes
Container Ptr : 000001e1f8704690
Container Len : 4
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000001e1f8704690 | c3  | 195 | 11000011 | UNK
000001e1f8704691 | f5  | 245 | 11110101 | UNK
000001e1f8704692 | 48  | 072 | 01001000 | H
000001e1f8704693 | 40  | 064 | 01000000 | @


---- tests::box_viewer stdout ----
This should print the memory address of the box and the memory of its value.

Name: &my_box
Type: &alloc::boxed::Box<u8>
Addr: 000000dc286fe348
Size: 8 bytes
Container Ptr : 000001e1f8704680
Container Len : 1
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000001e1f8704680 | 45  | 069 | 01000101 | E

Name: &my_box
Type: &alloc::boxed::Box<u8>
Addr: 000000dc286fdcd8
Size: 8 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc286fddc0 | 38 | 056 | 00111000 |  8
 000000dc286fddc1 | dc | 220 | 11011100 |  ...
 000000dc286fddc2 | 6f | 111 | 01101111 |  o
 000000dc286fddc3 | 28 | 040 | 00101000 |  (
 000000dc286fddc4 | dc | 220 | 11011100 |  ...
 000000dc286fddc5 | 00 | 000 | 00000000 |  NUL
 000000dc286fddc6 | 00 | 000 | 00000000 |  NUL
 000000dc286fddc7 | 00 | 000 | 00000000 |  NUL

Name: my_box
Type: alloc::boxed::Box<u8>
Addr: 000000dc286fdc38
Size: 8 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc286fdc38 | 40 | 064 | 01000000 |  @
 000000dc286fdc39 | 45 | 069 | 01000101 |  E
 000000dc286fdc3a | 70 | 112 | 01110000 |  p
 000000dc286fdc3b | f8 | 248 | 11111000 |  ...
 000000dc286fdc3c | e1 | 225 | 11100001 |  ...
 000000dc286fdc3d | 01 | 001 | 00000001 |  SOH
 000000dc286fdc3e | 00 | 000 | 00000000 |  NUL
 000000dc286fdc3f | 00 | 000 | 00000000 |  NUL

Name: *my_box
Type: u8
Addr: 000001e1f8704540
Size: 1 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000001e1f8704540 | 45 | 069 | 01000101 |  E


---- tests::ptr_viewer stdout ----
This should print the memory of a pointer.

Currently pointer type is not supported by safe view memory.

Name: my_ptr
Type: *const u8
Addr: 000000dc28afe9d0
Size: 8 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc28afe9d0 | 38 | 056 | 00111000 |  8
 000000dc28afe9d1 | 28 | 040 | 00101000 |  (
 000000dc28afe9d2 | 2c | 044 | 00101100 |  ,
 000000dc28afe9d3 | 38 | 056 | 00111000 |  8
 000000dc28afe9d4 | f6 | 246 | 11110110 |  ...
 000000dc28afe9d5 | 7f | 127 | 01111111 |  DEL
 000000dc28afe9d6 | 00 | 000 | 00000000 |  NUL
 000000dc28afe9d7 | 00 | 000 | 00000000 |  NUL

Name: *my_ptr
Type: u8
Addr: 00007ff6382c2838
Size: 1 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 00007ff6382c2838 | 45 | 069 | 01000101 |  E


---- tests::str_viewer stdout ----
This should print the memory of 'Hello' and its address.

Name: my_str
Type: &str
Addr: 000000dc28cfd0b8
Size: 16 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc28cfd0b8 | d8 | 216 | 11011000 |  ...
 000000dc28cfd0b9 | 27 | 039 | 00100111 |  '
 000000dc28cfd0ba | 2c | 044 | 00101100 |  ,
 000000dc28cfd0bb | 38 | 056 | 00111000 |  8
 000000dc28cfd0bc | f6 | 246 | 11110110 |  ...
 000000dc28cfd0bd | 7f | 127 | 01111111 |  DEL
 000000dc28cfd0be | 00 | 000 | 00000000 |  NUL
 000000dc28cfd0bf | 00 | 000 | 00000000 |  NUL
 000000dc28cfd0c0 | 05 | 005 | 00000101 |  ENQ
 000000dc28cfd0c1 | 00 | 000 | 00000000 |  NUL
 000000dc28cfd0c2 | 00 | 000 | 00000000 |  NUL
 000000dc28cfd0c3 | 00 | 000 | 00000000 |  NUL
 000000dc28cfd0c4 | 00 | 000 | 00000000 |  NUL
 000000dc28cfd0c5 | 00 | 000 | 00000000 |  NUL
 000000dc28cfd0c6 | 00 | 000 | 00000000 |  NUL
 000000dc28cfd0c7 | 00 | 000 | 00000000 |  NUL

Name: *my_str
Type: str
Addr: 00007ff6382c27d8
Size: 5 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 00007ff6382c27d8 | 48 | 072 | 01001000 |  H
 00007ff6382c27d9 | 65 | 101 | 01100101 |  e
 00007ff6382c27da | 6c | 108 | 01101100 |  l
 00007ff6382c27db | 6c | 108 | 01101100 |  l
 00007ff6382c27dc | 6f | 111 | 01101111 |  o

Name: &my_str
Type: &&str
Addr: 000000dc28cfd0b8
Size: 8 bytes
Container Ptr : 000001e1f8703c50
Container Len : 5
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000001e1f8703c50 | 48  | 072 | 01001000 | H
000001e1f8703c51 | 65  | 101 | 01100101 | e
000001e1f8703c52 | 6c  | 108 | 01101100 | l
000001e1f8703c53 | 6c  | 108 | 01101100 | l
000001e1f8703c54 | 6f  | 111 | 01101111 | o

Name: my_str
Type: &str
Addr: 00007ff6382c27d8
Size: 16 bytes
Container Ptr : 000001e1f8703cc0
Container Len : 5
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000001e1f8703cc0 | 48  | 072 | 01001000 | H
000001e1f8703cc1 | 65  | 101 | 01100101 | e
000001e1f8703cc2 | 6c  | 108 | 01101100 | l
000001e1f8703cc3 | 6c  | 108 | 01101100 | l
000001e1f8703cc4 | 6f  | 111 | 01101111 | o


---- tests::u16_viewer stdout ----
This should print the memory of the holy number 69.

Name: my_u16
Type: u16
Addr: 000000dc290fe126
Size: 2 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc290fe126 | 45 | 069 | 01000101 |  E
 000000dc290fe127 | 00 | 000 | 00000000 |  NUL

Name: &my_u16
Type: &u16
Addr: 000000dc290fe126
Size: 8 bytes
Container Ptr : 000001e1f8704570
Container Len : 2
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000001e1f8704570 | 45  | 069 | 01000101 | E
000001e1f8704571 | 00  | 000 | 00000000 | NUL


---- tests::struct_viewer stdout ----
This should print the memory address of the struct and the memory of its fields.

Name: &my_serialized_struct
Type: &mem_viewer::tests::MySerializedStruct
Addr: 000000dc28efe068
Size: 8 bytes
Container Ptr : 000001e1f8704510
Container Len : 7
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000001e1f8704510 | 45  | 069 | 01000101 | E
000001e1f8704511 | ff  | 255 | 11111111 | UNK
000001e1f8704512 | 00  | 000 | 00000000 | NUL
000001e1f8704513 | 46  | 070 | 01000110 | F
000001e1f8704514 | 00  | 000 | 00000000 | NUL
000001e1f8704515 | 00  | 000 | 00000000 | NUL
000001e1f8704516 | 00  | 000 | 00000000 | NUL

Name: &my_struct
Type: &mem_viewer::tests::MyStruct
Addr: 000000dc28efdbb8
Size: 8 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc28efdca0 | 10 | 016 | 00010000 |  DLE
 000000dc28efdca1 | db | 219 | 11011011 |  ...
 000000dc28efdca2 | ef | 239 | 11101111 |  ...
 000000dc28efdca3 | 28 | 040 | 00101000 |  (
 000000dc28efdca4 | dc | 220 | 11011100 |  ...
 000000dc28efdca5 | 00 | 000 | 00000000 |  NUL
 000000dc28efdca6 | 00 | 000 | 00000000 |  NUL
 000000dc28efdca7 | 00 | 000 | 00000000 |  NUL

Name: my_struct
Type: mem_viewer::tests::MyStruct
Addr: 000000dc28efdb10
Size: 8 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc28efdb10 | 46 | 070 | 01000110 |  F
 000000dc28efdb11 | 00 | 000 | 00000000 |  NUL
 000000dc28efdb12 | 00 | 000 | 00000000 |  NUL
 000000dc28efdb13 | 00 | 000 | 00000000 |  NUL
 000000dc28efdb14 | ff | 255 | 11111111 |  ...
 000000dc28efdb15 | 00 | 000 | 00000000 |  NUL
 000000dc28efdb16 | 45 | 069 | 01000101 |  E
 000000dc28efdb17 | 00 | 000 | 00000000 |  NUL


---- tests::vec_of_box_viewer stdout ----
This should print the memory address of the vector of boxes and the memory of its elements.

Name: &my_vec_of_box
Type: &alloc::vec::Vec<alloc::boxed::Box<u8>>
Addr: 000000dc286fdd90
Size: 8 bytes
Container Ptr : 000001e1f8703c70
Container Len : 5
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000001e1f8703c70 | 45  | 069 | 01000101 | E
000001e1f8703c71 | ff  | 255 | 11111111 | UNK
000001e1f8703c72 | fe  | 254 | 11111110 | UNK
000001e1f8703c73 | fd  | 253 | 11111101 | UNK
000001e1f8703c74 | 46  | 070 | 01000110 | F

Name: my_vec_of_box
Type: alloc::vec::Vec<alloc::boxed::Box<u8>>
Addr: 000000dc286fe930
Size: 24 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc286fe930 | 05 | 005 | 00000101 |  ENQ
 000000dc286fe931 | 00 | 000 | 00000000 |  NUL
 000000dc286fe932 | 00 | 000 | 00000000 |  NUL
 000000dc286fe933 | 00 | 000 | 00000000 |  NUL
 000000dc286fe934 | 00 | 000 | 00000000 |  NUL
 000000dc286fe935 | 00 | 000 | 00000000 |  NUL
 000000dc286fe936 | 00 | 000 | 00000000 |  NUL
 000000dc286fe937 | 00 | 000 | 00000000 |  NUL
 000000dc286fe938 | 90 | 144 | 10010000 |  ...
 000000dc286fe939 | 77 | 119 | 01110111 |  w
 000000dc286fe93a | 70 | 112 | 01110000 |  p
 000000dc286fe93b | f8 | 248 | 11111000 |  ...
 000000dc286fe93c | e1 | 225 | 11100001 |  ...
 000000dc286fe93d | 01 | 001 | 00000001 |  SOH
 000000dc286fe93e | 00 | 000 | 00000000 |  NUL
 000000dc286fe93f | 00 | 000 | 00000000 |  NUL
 000000dc286fe940 | 05 | 005 | 00000101 |  ENQ
 000000dc286fe941 | 00 | 000 | 00000000 |  NUL
 000000dc286fe942 | 00 | 000 | 00000000 |  NUL
 000000dc286fe943 | 00 | 000 | 00000000 |  NUL
 000000dc286fe944 | 00 | 000 | 00000000 |  NUL
 000000dc286fe945 | 00 | 000 | 00000000 |  NUL
 000000dc286fe946 | 00 | 000 | 00000000 |  NUL
 000000dc286fe947 | 00 | 000 | 00000000 |  NUL

Name: *my_vec_of_box
Type: [alloc::boxed::Box<u8>]
Addr: 000001e1f8707790
Size: 40 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000001e1f8707790 | 50 | 080 | 01010000 |  P
 000001e1f8707791 | 3c | 060 | 00111100 |  <
 000001e1f8707792 | 70 | 112 | 01110000 |  p
 000001e1f8707793 | f8 | 248 | 11111000 |  ...
 000001e1f8707794 | e1 | 225 | 11100001 |  ...
 000001e1f8707795 | 01 | 001 | 00000001 |  SOH
 000001e1f8707796 | 00 | 000 | 00000000 |  NUL
 000001e1f8707797 | 00 | 000 | 00000000 |  NUL
 000001e1f8707798 | d0 | 208 | 11010000 |  ...
 000001e1f8707799 | 3c | 060 | 00111100 |  <
 000001e1f870779a | 70 | 112 | 01110000 |  p
 000001e1f870779b | f8 | 248 | 11111000 |  ...
 000001e1f870779c | e1 | 225 | 11100001 |  ...
 000001e1f870779d | 01 | 001 | 00000001 |  SOH
 000001e1f870779e | 00 | 000 | 00000000 |  NUL
 000001e1f870779f | 00 | 000 | 00000000 |  NUL
 000001e1f87077a0 | e0 | 224 | 11100000 |  ...
 000001e1f87077a1 | 3c | 060 | 00111100 |  <
 000001e1f87077a2 | 70 | 112 | 01110000 |  p
 000001e1f87077a3 | f8 | 248 | 11111000 |  ...
 000001e1f87077a4 | e1 | 225 | 11100001 |  ...
 000001e1f87077a5 | 01 | 001 | 00000001 |  SOH
 000001e1f87077a6 | 00 | 000 | 00000000 |  NUL
 000001e1f87077a7 | 00 | 000 | 00000000 |  NUL
 000001e1f87077a8 | 40 | 064 | 01000000 |  @
 000001e1f87077a9 | 3c | 060 | 00111100 |  <
 000001e1f87077aa | 70 | 112 | 01110000 |  p
 000001e1f87077ab | f8 | 248 | 11111000 |  ...
 000001e1f87077ac | e1 | 225 | 11100001 |  ...
 000001e1f87077ad | 01 | 001 | 00000001 |  SOH
 000001e1f87077ae | 00 | 000 | 00000000 |  NUL
 000001e1f87077af | 00 | 000 | 00000000 |  NUL
 000001e1f87077b0 | 60 | 096 | 01100000 |  `
 000001e1f87077b1 | 3c | 060 | 00111100 |  <
 000001e1f87077b2 | 70 | 112 | 01110000 |  p
 000001e1f87077b3 | f8 | 248 | 11111000 |  ...
 000001e1f87077b4 | e1 | 225 | 11100001 |  ...
 000001e1f87077b5 | 01 | 001 | 00000001 |  SOH
 000001e1f87077b6 | 00 | 000 | 00000000 |  NUL
 000001e1f87077b7 | 00 | 000 | 00000000 |  NUL

Name: *my_vec_of_box[0]
Type: u8
Addr: 000001e1f8703c50
Size: 1 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000001e1f8703c50 | 45 | 069 | 01000101 |  E


---- tests::vec_viewer stdout ----
This should print the memory address of the vector and the memory of its elements.

Name: &my_vec
Type: &alloc::vec::Vec<u8>
Addr: 000000dc286fe438
Size: 8 bytes
Container Ptr : 000001e1f87046a0
Container Len : 5
     Address     | Hex | Dec |    Bin   | ASCII
---------------Container Content---------------
000001e1f87046a0 | 45  | 069 | 01000101 | E
000001e1f87046a1 | ff  | 255 | 11111111 | UNK
000001e1f87046a2 | fe  | 254 | 11111110 | UNK
000001e1f87046a3 | fd  | 253 | 11111101 | UNK
000001e1f87046a4 | 46  | 070 | 01000110 | F

Name: my_vec
Type: alloc::vec::Vec<u8>
Addr: 000000dc286fefb0
Size: 24 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000000dc286fefb0 | 05 | 005 | 00000101 |  ENQ
 000000dc286fefb1 | 00 | 000 | 00000000 |  NUL
 000000dc286fefb2 | 00 | 000 | 00000000 |  NUL
 000000dc286fefb3 | 00 | 000 | 00000000 |  NUL
 000000dc286fefb4 | 00 | 000 | 00000000 |  NUL
 000000dc286fefb5 | 00 | 000 | 00000000 |  NUL
 000000dc286fefb6 | 00 | 000 | 00000000 |  NUL
 000000dc286fefb7 | 00 | 000 | 00000000 |  NUL
 000000dc286fefb8 | 20 | 032 | 00100000 |  SPC
 000000dc286fefb9 | 46 | 070 | 01000110 |  F
 000000dc286fefba | 70 | 112 | 01110000 |  p
 000000dc286fefbb | f8 | 248 | 11111000 |  ...
 000000dc286fefbc | e1 | 225 | 11100001 |  ...
 000000dc286fefbd | 01 | 001 | 00000001 |  SOH
 000000dc286fefbe | 00 | 000 | 00000000 |  NUL
 000000dc286fefbf | 00 | 000 | 00000000 |  NUL
 000000dc286fefc0 | 05 | 005 | 00000101 |  ENQ
 000000dc286fefc1 | 00 | 000 | 00000000 |  NUL
 000000dc286fefc2 | 00 | 000 | 00000000 |  NUL
 000000dc286fefc3 | 00 | 000 | 00000000 |  NUL
 000000dc286fefc4 | 00 | 000 | 00000000 |  NUL
 000000dc286fefc5 | 00 | 000 | 00000000 |  NUL
 000000dc286fefc6 | 00 | 000 | 00000000 |  NUL
 000000dc286fefc7 | 00 | 000 | 00000000 |  NUL

Name: *my_vec
Type: [u8]
Addr: 000001e1f8704620
Size: 5 bytes
     Address     | Hex | Dec |    Bin   | ASCII
-------------------Memory Content-----------------
 000001e1f8704620 | 45 | 069 | 01000101 |  E
 000001e1f8704621 | ff | 255 | 11111111 |  ...
 000001e1f8704622 | fe | 254 | 11111110 |  ...
 000001e1f8704623 | fd | 253 | 11111101 |  ...
 000001e1f8704624 | 46 | 070 | 01000110 |  F



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
