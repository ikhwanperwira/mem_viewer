# Overview

`mem_viewer` is a Rust crate that provides a macro `view_mem!` to view the memory content of an arbitrary variable. It supports viewing memory content of different data types including integers, floating-point numbers, strings, pointers, vectors, boxed variables, and structs.

## Usage

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
mem_viewer = "0.3.0"
```

Then, in your Rust code, you can use the `view_mem!` macro to view the memory content of a variable. Here's an example:

```rust
use mem_viewer::*;

let my_str: &str = "🦀Hello😃";
view_mem!(*my_str);
```

This will print the memory content of `my_var` to the console.

## Example Output

```none
Name: *my_str
Type: str
Addr: 00007ff7f23fa4b8
Size: 13 bytes
Aloc: Likely Stack
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00007ff7f23fa4b8 | f0  | 240 | 11110000 |  ...  | 🦀 
 00007ff7f23fa4b9 | 9f  | 159 | 10011111 |  ...  | ... 
 00007ff7f23fa4ba | a6  | 166 | 10100110 |  ...  | ... 
 00007ff7f23fa4bb | 80  | 128 | 10000000 |  ...  | ... 
 00007ff7f23fa4bc | 48  | 072 | 01001000 |   H   | Hell 
 00007ff7f23fa4bd | 65  | 101 | 01100101 |   e   | ello 
 00007ff7f23fa4be | 6c  | 108 | 01101100 |   l   | ... 
 00007ff7f23fa4bf | 6c  | 108 | 01101100 |   l   | ... 
 00007ff7f23fa4c0 | 6f  | 111 | 01101111 |   o   | ... 
 00007ff7f23fa4c1 | f0  | 240 | 11110000 |  ...  | 😃 
 00007ff7f23fa4c2 | 9f  | 159 | 10011111 |  ...  | XXX
 00007ff7f23fa4c3 | 98  | 152 | 10011000 |  ...  | XXX
 00007ff7f23fa4c4 | 83  | 131 | 10000011 |  ...  | XXX
```

Read more for the macro usage: [https://docs.rs/mem_viewer/latest/mem_viewer/macro.view_mem.html](https://docs.rs/mem_viewer/latest/mem_viewer/macro.view_mem.html)

## Safe Usage
For safe mode, you can use the `safe_view_mem!` macro to view the memory content of a variable. Here's an example:
```rust
use mem_viewer::*;

let my_str: &str = "🦀Hello😃";
view_mem!(my_str);
```

## Example Safe Output
```none
Name         : my_str
Type         : &str
Addr         : 00007ff7f23fa4b8
Size         : 16 bytes
Aloc         : Likely Stack
Container Ptr: 00000203f55753c0
Container Len: 13
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 00000203f55753c0 | f0  | 240 | 11110000 | ...   | 🦀
 00000203f55753c1 | 9f  | 159 | 10011111 | ...   | ...
 00000203f55753c2 | a6  | 166 | 10100110 | ...   | ...
 00000203f55753c3 | 80  | 128 | 10000000 | ...   | ...
 00000203f55753c4 | 48  | 072 | 01001000 |  H    | Hell
 00000203f55753c5 | 65  | 101 | 01100101 |  e    | ello
 00000203f55753c6 | 6c  | 108 | 01101100 |  l    | ...
 00000203f55753c7 | 6c  | 108 | 01101100 |  l    | ...
 00000203f55753c8 | 6f  | 111 | 01101111 |  o    | ...
 00000203f55753c9 | f0  | 240 | 11110000 | ...   | 😃
 00000203f55753ca | 9f  | 159 | 10011111 | ...   | XXX
 00000203f55753cb | 98  | 152 | 10011000 | ...   | XXX
 00000203f55753cc | 83  | 131 | 10000011 | ...   | XXX
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

    fn view_mem_utf8(my_str: &str) -> () {
        // Unsafe test
        view_mem!(*my_str);

        // Safe test
        safe_view_mem!(my_str);
    }

    fn myfunc() {
        println!("This is a function pointer.");
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

    #[test]
    fn utf8_viewer() {
        println!("This should print the memory of fixed content of string between holy number in ASCII and UTF-8 emoji.\n");
        assert_eq!(view_mem_utf8("😃6🦀9😃"), ());
    }

    #[test]
    fn functype_viewer() {
        println!("This should print the memory of function pointer.\n");

        // Safe test
        // Currently function pointer is not supported by safe view memory.

        // Unsafe test
        view_mem!(&myfunc);
    }

}
```

## Output Test:
```none
running 10 tests
test tests::box_viewer ... ok
test tests::f32_viewer ... ok
test tests::functype_viewer ... ok
test tests::ptr_viewer ... ok
test tests::str_viewer ... ok
test tests::struct_viewer ... ok
test tests::u16_viewer ... ok
test tests::utf8_viewer ... ok
test tests::vec_of_box_viewer ... ok
test tests::vec_viewer ... ok

successes:

---- tests::box_viewer stdout ----
This should print the memory address of the box and the memory of its value.

Name         : &my_box
Type         : &alloc::boxed::Box<u8>
Addr         : 00000038570fe198
Size         : 8 bytes
Aloc         : Likely Heap
Container Ptr: 000001d943845210
Container Len: 1
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d943845210 | 45  | 069 | 01000101 |  E    | XXX

Name: &my_box
Type: &alloc::boxed::Box<u8>
Addr: 00000038570fd548
Size: 8 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00000038570fd7c0 | b8  | 184 | 10111000 |  ...  | ...
 00000038570fd7c1 | d4  | 212 | 11010100 |  ...  | ...
 00000038570fd7c2 | 0f  | 015 | 00001111 |  SI   | W8
 00000038570fd7c3 | 57  | 087 | 01010111 |   W   | W8
 00000038570fd7c4 | 38  | 056 | 00111000 |   8   | 8
 00000038570fd7c5 | 00  | 000 | 00000000 |  NUL  | XXX
 00000038570fd7c6 | 00  | 000 | 00000000 |  NUL  | XXX
 00000038570fd7c7 | 00  | 000 | 00000000 |  NUL  | XXX

Name: my_box
Type: alloc::boxed::Box<u8>
Addr: 00000038570fd4b8
Size: 8 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00000038570fd4b8 | c0  | 192 | 11000000 |  ...  | ...
 00000038570fd4b9 | 52  | 082 | 01010010 |   R   | ...
 00000038570fd4ba | 84  | 132 | 10000100 |  ...  | ...
 00000038570fd4bb | 43  | 067 | 01000011 |   C   | ...
 00000038570fd4bc | d9  | 217 | 11011001 |  ...  | ...
 00000038570fd4bd | 01  | 001 | 00000001 |  SOH  | XXX
 00000038570fd4be | 00  | 000 | 00000000 |  NUL  | XXX
 00000038570fd4bf | 00  | 000 | 00000000 |  NUL  | XXX

Name: *my_box
Type: u8
Addr: 000001d9438452c0
Size: 1 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 000001d9438452c0 | 45  | 069 | 01000101 |   E   | XXX


---- tests::f32_viewer stdout ----
This should print the memory of pi in IEEE 754 representation, which is 0x4048f5c3.

Name: my_f32
Type: f32
Addr: 00000038572fd6fc
Size: 4 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00000038572fd6fc | c3  | 195 | 11000011 |  ...  | ...
 00000038572fd6fd | f5  | 245 | 11110101 |  ...  | XXX
 00000038572fd6fe | 48  | 072 | 01001000 |   H   | XXX
 00000038572fd6ff | 40  | 064 | 01000000 |   @   | XXX

Name         : &my_f32
Type         : &f32
Addr         : 00000038572fd6fc
Size         : 8 bytes
Aloc         : Likely Heap
Container Ptr: 000001d9438452c0
Container Len: 4
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d9438452c0 | c3  | 195 | 11000011 | ...   | ...
 000001d9438452c1 | f5  | 245 | 11110101 | ...   | XXX
 000001d9438452c2 | 48  | 072 | 01001000 |  H    | XXX
 000001d9438452c3 | 40  | 064 | 01000000 |  @    | XXX


---- tests::functype_viewer stdout ----
This should print the memory of function pointer.

Name: &myfunc
Type: &mem_viewer::tests::myfunc
Addr: 00007ff62661f5b0
Size: 8 bytes
Aloc: Likely Stack
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00007ff62661f5b0 | 38  | 056 | 00111000 |   8   | ...
 00007ff62661f5b1 | ea  | 234 | 11101010 |  ...  | ...
 00007ff62661f5b2 | 61  | 097 | 01100001 |   a   | ...
 00007ff62661f5b3 | 26  | 038 | 00100110 |   &   | ...
 00007ff62661f5b4 | f6  | 246 | 11110110 |  ...  | ...
 00007ff62661f5b5 | 7f  | 127 | 01111111 |  DEL  | XXX
 00007ff62661f5b6 | 00  | 000 | 00000000 |  NUL  | XXX
 00007ff62661f5b7 | 00  | 000 | 00000000 |  NUL  | XXX


---- tests::ptr_viewer stdout ----
This should print the memory of a pointer.

Currently pointer type is not supported by safe view memory.

Name: my_ptr
Type: *const u8
Addr: 00000038576fe8f8
Size: 8 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00000038576fe8f8 | 10  | 016 | 00010000 |  DLE  | ...
 00000038576fe8f9 | f1  | 241 | 11110001 |  ...  | ...
 00000038576fe8fa | 61  | 097 | 01100001 |   a   | ...
 00000038576fe8fb | 26  | 038 | 00100110 |   &   | ...
 00000038576fe8fc | f6  | 246 | 11110110 |  ...  | ...
 00000038576fe8fd | 7f  | 127 | 01111111 |  DEL  | XXX
 00000038576fe8fe | 00  | 000 | 00000000 |  NUL  | XXX
 00000038576fe8ff | 00  | 000 | 00000000 |  NUL  | XXX

Name: *my_ptr
Type: u8
Addr: 00007ff62661f110
Size: 1 bytes
Aloc: Likely Stack
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00007ff62661f110 | 45  | 069 | 01000101 |   E   | XXX


---- tests::str_viewer stdout ----
This should print the memory of 'Hello' and its address.

Name: my_str
Type: &str
Addr: 00000038578fc0b0
Size: 16 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00000038578fc0b0 | b0  | 176 | 10110000 |  ...  | ...
 00000038578fc0b1 | f0  | 240 | 11110000 |  ...  | ...
 00000038578fc0b2 | 61  | 097 | 01100001 |   a   | ...
 00000038578fc0b3 | 26  | 038 | 00100110 |   &   | ...
 00000038578fc0b4 | f6  | 246 | 11110110 |  ...  | ...
 00000038578fc0b5 | 7f  | 127 | 01111111 |  DEL  | ♣
 00000038578fc0b6 | 00  | 000 | 00000000 |  NUL  | ♣
 00000038578fc0b7 | 00  | 000 | 00000000 |  NUL  | ♣
 00000038578fc0b8 | 05  | 005 | 00000101 |  ENQ  | ♣
 00000038578fc0b9 | 00  | 000 | 00000000 |  NUL  |
 00000038578fc0ba | 00  | 000 | 00000000 |  NUL  |
 00000038578fc0bb | 00  | 000 | 00000000 |  NUL  |
 00000038578fc0bc | 00  | 000 | 00000000 |  NUL  |
 00000038578fc0bd | 00  | 000 | 00000000 |  NUL  | XXX
 00000038578fc0be | 00  | 000 | 00000000 |  NUL  | XXX
 00000038578fc0bf | 00  | 000 | 00000000 |  NUL  | XXX

Name: *my_str
Type: str
Addr: 00007ff62661f0b0
Size: 5 bytes
Aloc: Likely Stack
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00007ff62661f0b0 | 48  | 072 | 01001000 |   H   | Hell
 00007ff62661f0b1 | 65  | 101 | 01100101 |   e   | ello
 00007ff62661f0b2 | 6c  | 108 | 01101100 |   l   | XXX
 00007ff62661f0b3 | 6c  | 108 | 01101100 |   l   | XXX
 00007ff62661f0b4 | 6f  | 111 | 01101111 |   o   | XXX

Name         : &my_str
Type         : &&str
Addr         : 00000038578fc0b0
Size         : 8 bytes
Aloc         : Likely Heap
Container Ptr: 000001d943845390
Container Len: 5
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d943845390 | 48  | 072 | 01001000 |  H    | Hell
 000001d943845391 | 65  | 101 | 01100101 |  e    | ello
 000001d943845392 | 6c  | 108 | 01101100 |  l    | XXX
 000001d943845393 | 6c  | 108 | 01101100 |  l    | XXX
 000001d943845394 | 6f  | 111 | 01101111 |  o    | XXX

Name         : my_str
Type         : &str
Addr         : 00007ff62661f0b0
Size         : 16 bytes
Aloc         : Likely Stack
Container Ptr: 000001d9438451f0
Container Len: 5
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d9438451f0 | 48  | 072 | 01001000 |  H    | Hell
 000001d9438451f1 | 65  | 101 | 01100101 |  e    | ello
 000001d9438451f2 | 6c  | 108 | 01101100 |  l    | XXX
 000001d9438451f3 | 6c  | 108 | 01101100 |  l    | XXX
 000001d9438451f4 | 6f  | 111 | 01101111 |  o    | XXX


---- tests::struct_viewer stdout ----
This should print the memory address of the struct and the memory of its fields.

Name         : &my_serialized_struct
Type         : &mem_viewer::tests::MySerializedStruct
Addr         : 0000003857afe018
Size         : 8 bytes
Aloc         : Likely Heap
Container Ptr: 000001d9438452d0
Container Len: 7
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d9438452d0 | 45  | 069 | 01000101 |  E    | ...
 000001d9438452d1 | ff  | 255 | 11111111 | ...   | ...
 000001d9438452d2 | 00  | 000 | 00000000 | NUL   | F
 000001d9438452d3 | 46  | 070 | 01000110 |  F    | F
 000001d9438452d4 | 00  | 000 | 00000000 | NUL   | XXX
 000001d9438452d5 | 00  | 000 | 00000000 | NUL   | XXX
 000001d9438452d6 | 00  | 000 | 00000000 | NUL   | XXX

Name: &my_struct
Type: &mem_viewer::tests::MyStruct
Addr: 0000003857afd748
Size: 8 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 0000003857afd9c0 | b0  | 176 | 10110000 |  ...  | ...
 0000003857afd9c1 | d6  | 214 | 11010110 |  ...  | ֯W8
 0000003857afd9c2 | af  | 175 | 10101111 |  ...  | ...
 0000003857afd9c3 | 57  | 087 | 01010111 |   W   | W8
 0000003857afd9c4 | 38  | 056 | 00111000 |   8   | 8
 0000003857afd9c5 | 00  | 000 | 00000000 |  NUL  | XXX
 0000003857afd9c6 | 00  | 000 | 00000000 |  NUL  | XXX
 0000003857afd9c7 | 00  | 000 | 00000000 |  NUL  | XXX

Name: my_struct
Type: mem_viewer::tests::MyStruct
Addr: 0000003857afd6b0
Size: 8 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 0000003857afd6b0 | 46  | 070 | 01000110 |   F   | F
 0000003857afd6b1 | 00  | 000 | 00000000 |  NUL  | ...
 0000003857afd6b2 | 00  | 000 | 00000000 |  NUL  | ...
 0000003857afd6b3 | 00  | 000 | 00000000 |  NUL  | ...
 0000003857afd6b4 | ff  | 255 | 11111111 |  ...  | ...
 0000003857afd6b5 | 00  | 000 | 00000000 |  NUL  | XXX
 0000003857afd6b6 | 45  | 069 | 01000101 |   E   | XXX
 0000003857afd6b7 | 00  | 000 | 00000000 |  NUL  | XXX


---- tests::u16_viewer stdout ----
This should print the memory of the holy number 69.

Name: my_u16
Type: u16
Addr: 00000038570fdb9e
Size: 2 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00000038570fdb9e | 45  | 069 | 01000101 |   E   | XXX 
 00000038570fdb9f | 00  | 000 | 00000000 |  NUL  | XXX

Name         : &my_u16
Type         : &u16
Addr         : 00000038570fdb9e
Size         : 8 bytes
Aloc         : Likely Heap
Container Ptr: 000001d943845380
Container Len: 2
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d943845380 | 45  | 069 | 01000101 |  E    | XXX
 000001d943845381 | 00  | 000 | 00000000 | NUL   | XXX


---- tests::utf8_viewer stdout ----
This should print the memory of fixed content of string between holy number in ASCII and UTF-8 emoji.

Name: *my_str
Type: str
Addr: 00007ff62661f520
Size: 14 bytes
Aloc: Likely Stack
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00007ff62661f520 | f0  | 240 | 11110000 |  ...  | 😃
 00007ff62661f521 | 9f  | 159 | 10011111 |  ...  | ...
 00007ff62661f522 | 98  | 152 | 10011000 |  ...  | ...
 00007ff62661f523 | 83  | 131 | 10000011 |  ...  | ...
 00007ff62661f524 | 36  | 054 | 00110110 |   6   | ...
 00007ff62661f525 | f0  | 240 | 11110000 |  ...  | 🦀
 00007ff62661f526 | 9f  | 159 | 10011111 |  ...  | ...
 00007ff62661f527 | a6  | 166 | 10100110 |  ...  | ...
 00007ff62661f528 | 80  | 128 | 10000000 |  ...  | ...
 00007ff62661f529 | 39  | 057 | 00111001 |   9   | ...
 00007ff62661f52a | f0  | 240 | 11110000 |  ...  | 😃
 00007ff62661f52b | 9f  | 159 | 10011111 |  ...  | XXX
 00007ff62661f52c | 98  | 152 | 10011000 |  ...  | XXX
 00007ff62661f52d | 83  | 131 | 10000011 |  ...  | XXX

Name         : my_str
Type         : &str
Addr         : 00007ff62661f520
Size         : 16 bytes
Aloc         : Likely Stack
Container Ptr: 000001d943848570
Container Len: 14
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d943848570 | f0  | 240 | 11110000 | ...   | 😃
 000001d943848571 | 9f  | 159 | 10011111 | ...   | ...
 000001d943848572 | 98  | 152 | 10011000 | ...   | ...
 000001d943848573 | 83  | 131 | 10000011 | ...   | ...
 000001d943848574 | 36  | 054 | 00110110 |  6    | ...
 000001d943848575 | f0  | 240 | 11110000 | ...   | 🦀
 000001d943848576 | 9f  | 159 | 10011111 | ...   | ...
 000001d943848577 | a6  | 166 | 10100110 | ...   | ...
 000001d943848578 | 80  | 128 | 10000000 | ...   | ...
 000001d943848579 | 39  | 057 | 00111001 |  9    | ...
 000001d94384857a | f0  | 240 | 11110000 | ...   | 😃
 000001d94384857b | 9f  | 159 | 10011111 | ...   | XXX
 000001d94384857c | 98  | 152 | 10011000 | ...   | XXX
 000001d94384857d | 83  | 131 | 10000011 | ...   | XXX


---- tests::vec_of_box_viewer stdout ----
This should print the memory address of the vector of boxes and the memory of its elements.

Name         : &my_vec_of_box
Type         : &alloc::vec::Vec<alloc::boxed::Box<u8>>
Addr         : 00000038570fdae0
Size         : 8 bytes
Aloc         : Likely Heap
Container Ptr: 000001d943845360
Container Len: 5
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d943845360 | 45  | 069 | 01000101 |  E    | ...
 000001d943845361 | ff  | 255 | 11111111 | ...   | ...
 000001d943845362 | fe  | 254 | 11111110 | ...   | XXX
 000001d943845363 | fd  | 253 | 11111101 | ...   | XXX
 000001d943845364 | 46  | 070 | 01000110 |  F    | XXX

Name: my_vec_of_box
Type: alloc::vec::Vec<alloc::boxed::Box<u8>>
Addr: 00000038570fe910
Size: 24 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00000038570fe910 | 05  | 005 | 00000101 |  ENQ  | ♣
 00000038570fe911 | 00  | 000 | 00000000 |  NUL  |
 00000038570fe912 | 00  | 000 | 00000000 |  NUL  |
 00000038570fe913 | 00  | 000 | 00000000 |  NUL  |
 00000038570fe914 | 00  | 000 | 00000000 |  NUL  |
 00000038570fe915 | 00  | 000 | 00000000 |  NUL  | ...
 00000038570fe916 | 00  | 000 | 00000000 |  NUL  | ...
 00000038570fe917 | 00  | 000 | 00000000 |  NUL  | ...
 00000038570fe918 | a0  | 160 | 10100000 |  ...  | ...
 00000038570fe919 | 26  | 038 | 00100110 |   &   | ...
 00000038570fe91a | 84  | 132 | 10000100 |  ...  | ...
 00000038570fe91b | 43  | 067 | 01000011 |   C   | ...
 00000038570fe91c | d9  | 217 | 11011001 |  ...  | ...
 00000038570fe91d | 01  | 001 | 00000001 |  SOH  | ☺♣
 00000038570fe91e | 00  | 000 | 00000000 |  NUL  | ♣
 00000038570fe91f | 00  | 000 | 00000000 |  NUL  | ♣
 00000038570fe920 | 05  | 005 | 00000101 |  ENQ  | ♣
 00000038570fe921 | 00  | 000 | 00000000 |  NUL  |
 00000038570fe922 | 00  | 000 | 00000000 |  NUL  |
 00000038570fe923 | 00  | 000 | 00000000 |  NUL  |
 00000038570fe924 | 00  | 000 | 00000000 |  NUL  |
 00000038570fe925 | 00  | 000 | 00000000 |  NUL  | XXX
 00000038570fe926 | 00  | 000 | 00000000 |  NUL  | XXX
 00000038570fe927 | 00  | 000 | 00000000 |  NUL  | XXX

Name: *my_vec_of_box
Type: [alloc::boxed::Box<u8>]
Addr: 000001d9438426a0
Size: 40 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 000001d9438426a0 | 90  | 144 | 10010000 |  ...  | ...
 000001d9438426a1 | 52  | 082 | 01010010 |   R   | ...
 000001d9438426a2 | 84  | 132 | 10000100 |  ...  | ...
 000001d9438426a3 | 43  | 067 | 01000011 |   C   | ...
 000001d9438426a4 | d9  | 217 | 11011001 |  ...  | ...
 000001d9438426a5 | 01  | 001 | 00000001 |  SOH  | ☺0
 000001d9438426a6 | 00  | 000 | 00000000 |  NUL  | 0R
 000001d9438426a7 | 00  | 000 | 00000000 |  NUL  | ...
 000001d9438426a8 | 30  | 048 | 00110000 |   0   | ...
 000001d9438426a9 | 52  | 082 | 01010010 |   R   | ...
 000001d9438426aa | 84  | 132 | 10000100 |  ...  | ...
 000001d9438426ab | 43  | 067 | 01000011 |   C   | ...
 000001d9438426ac | d9  | 217 | 11011001 |  ...  | ...
 000001d9438426ad | 01  | 001 | 00000001 |  SOH  | ...
 000001d9438426ae | 00  | 000 | 00000000 |  NUL  | ...
 000001d9438426af | 00  | 000 | 00000000 |  NUL  | ...
 000001d9438426b0 | a0  | 160 | 10100000 |  ...  | ...
 000001d9438426b1 | 52  | 082 | 01010010 |   R   | ...
 000001d9438426b2 | 84  | 132 | 10000100 |  ...  | ...
 000001d9438426b3 | 43  | 067 | 01000011 |   C   | ...
 000001d9438426b4 | d9  | 217 | 11011001 |  ...  | ...
 000001d9438426b5 | 01  | 001 | 00000001 |  SOH  | ☺►
 000001d9438426b6 | 00  | 000 | 00000000 |  NUL  | ►S
 000001d9438426b7 | 00  | 000 | 00000000 |  NUL  | ...
 000001d9438426b8 | 10  | 016 | 00010000 |  DLE  | ...
 000001d9438426b9 | 53  | 083 | 01010011 |   S   | ...
 000001d9438426ba | 84  | 132 | 10000100 |  ...  | ...
 000001d9438426bb | 43  | 067 | 01000011 |   C   | ...
 000001d9438426bc | d9  | 217 | 11011001 |  ...  | ...
 000001d9438426bd | 01  | 001 | 00000001 |  SOH  | ...
 000001d9438426be | 00  | 000 | 00000000 |  NUL  | ...
 000001d9438426bf | 00  | 000 | 00000000 |  NUL  | ...
 000001d9438426c0 | f0  | 240 | 11110000 |  ...  | ...
 000001d9438426c1 | 51  | 081 | 01010001 |   Q   | ...
 000001d9438426c2 | 84  | 132 | 10000100 |  ...  | ...
 000001d9438426c3 | 43  | 067 | 01000011 |   C   | ...
 000001d9438426c4 | d9  | 217 | 11011001 |  ...  | ...
 000001d9438426c5 | 01  | 001 | 00000001 |  SOH  | XXX
 000001d9438426c6 | 00  | 000 | 00000000 |  NUL  | XXX
 000001d9438426c7 | 00  | 000 | 00000000 |  NUL  | XXX

Name: *my_vec_of_box[0]
Type: u8
Addr: 000001d943845290
Size: 1 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 000001d943845290 | 45  | 069 | 01000101 |   E   | XXX


---- tests::vec_viewer stdout ----
This should print the memory address of the vector and the memory of its elements.

Name         : &my_vec
Type         : &alloc::vec::Vec<u8>
Addr         : 00000038570fe028
Size         : 8 bytes
Aloc         : Likely Heap
Container Ptr: 000001d943845290
Container Len: 5
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
---------------------Container Content-------------------
 000001d943845290 | 45  | 069 | 01000101 |  E    | ...
 000001d943845291 | ff  | 255 | 11111111 | ...   | ...
 000001d943845292 | fe  | 254 | 11111110 | ...   | XXX
 000001d943845293 | fd  | 253 | 11111101 | ...   | XXX
 000001d943845294 | 46  | 070 | 01000110 |  F    | XXX

Name: my_vec
Type: alloc::vec::Vec<u8>
Addr: 00000038570fee30
Size: 24 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 00000038570fee30 | 05  | 005 | 00000101 |  ENQ  | ♣
 00000038570fee31 | 00  | 000 | 00000000 |  NUL  |
 00000038570fee32 | 00  | 000 | 00000000 |  NUL  |
 00000038570fee33 | 00  | 000 | 00000000 |  NUL  |
 00000038570fee34 | 00  | 000 | 00000000 |  NUL  |
 00000038570fee35 | 00  | 000 | 00000000 |  NUL  | ►
 00000038570fee36 | 00  | 000 | 00000000 |  NUL  | ►R
 00000038570fee37 | 00  | 000 | 00000000 |  NUL  | ...
 00000038570fee38 | 10  | 016 | 00010000 |  DLE  | ...
 00000038570fee39 | 52  | 082 | 01010010 |   R   | ... 
 00000038570fee3a | 84  | 132 | 10000100 |  ...  | ...
 00000038570fee3b | 43  | 067 | 01000011 |   C   | ...
 00000038570fee3c | d9  | 217 | 11011001 |  ...  | ...
 00000038570fee3d | 01  | 001 | 00000001 |  SOH  | ☺♣
 00000038570fee3e | 00  | 000 | 00000000 |  NUL  | ♣
 00000038570fee3f | 00  | 000 | 00000000 |  NUL  | ♣
 00000038570fee40 | 05  | 005 | 00000101 |  ENQ  | ♣
 00000038570fee41 | 00  | 000 | 00000000 |  NUL  |
 00000038570fee42 | 00  | 000 | 00000000 |  NUL  |
 00000038570fee43 | 00  | 000 | 00000000 |  NUL  |
 00000038570fee44 | 00  | 000 | 00000000 |  NUL  |
 00000038570fee45 | 00  | 000 | 00000000 |  NUL  | XXX
 00000038570fee46 | 00  | 000 | 00000000 |  NUL  | XXX
 00000038570fee47 | 00  | 000 | 00000000 |  NUL  | XXX

Name: *my_vec
Type: [u8]
Addr: 000001d943845210
Size: 5 bytes
Aloc: Likely Heap
     Address      | Hex | Dec |    Bin   | ASCII | UTF-8
----------------------Memory Content--------------------
 000001d943845210 | 45  | 069 | 01000101 |   E   | ...
 000001d943845211 | ff  | 255 | 11111111 |  ...  | ...
 000001d943845212 | fe  | 254 | 11111110 |  ...  | XXX
 000001d943845213 | fd  | 253 | 11111101 |  ...  | XXX
 000001d943845214 | 46  | 070 | 01000110 |   F   | XXX



successes:
    tests::box_viewer
    tests::f32_viewer
    tests::functype_viewer
    tests::ptr_viewer
    tests::str_viewer
    tests::struct_viewer
    tests::u16_viewer
    tests::utf8_viewer
    tests::vec_of_box_viewer
    tests::vec_viewer

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
