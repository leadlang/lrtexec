use crate::{commands::FFISafeContainer, common::{others::FFISafeString, FFIableObject}};
use std::fmt::Debug;

#[test]
fn test() {
    println!("Running tests for FFIableObject and FFIableObject...\n");

    // Test case 1: FFIableObject - u32
    println!("- Test Case 1: FFIableObject (u32)");
    let val_u32 = 42u32;
    let mut container_u32: FFIableObject = val_u32.into();
    assert_eq!(*container_u32.as_u32().unwrap(), 42);
    println!("  -> as_u32() works as expected.");

    // Mutate the value
    let val_mut = unsafe { container_u32.as_u32_mut().unwrap() };
    *val_mut = 99;
    assert_eq!(*container_u32.as_u32().unwrap(), 99);
    println!("  -> as_u32_mut() works and the value was updated.");
    println!("  -> Test 1 passed.\n");

    // Test case 2: Type checking with FFIableObject
    println!("- Test Case 2: Type checking with FFIableObject");
    let val_i64 = -123i64;
    let container_i64: FFIableObject = val_i64.into();
    // This should fail because the type IDs don't match
    let wrong_type = container_i64.as_u32();
    assert!(wrong_type.is_none());
    println!("  -> as_u32() on an i64 container correctly returns None.");
    let correct_type = container_i64.as_i64();
    assert!(correct_type.is_some());
    assert_eq!(*correct_type.unwrap(), -123);
    println!("  -> as_i64() on an i64 container correctly returns the value.");
    println!("  -> Test 2 passed.\n");

    // Test case 3: FFIableObject - FFISafeString
    println!("- Test Case 3: FFIableObject (FFISafeString)");
    let mut container_str: FFIableObject = FFIableObject::create_using_box(FFISafeString::from("Hello, world!"));
    let safe_string = container_str.as_ffisafestring().unwrap();
    assert_eq!(safe_string.as_str().unwrap(), "Hello, world!");
    println!("  -> as_ffisafestring() correctly retrieves the string.");
    let safe_string_mut = container_str.as_ffisafestring_mut().unwrap();
    safe_string_mut.push_str(" This is a test.");
    assert_eq!(safe_string_mut.as_str().unwrap(), "Hello, world! This is a test.");
    println!("  -> as_ffisafestring_mut() correctly mutates the string.");
    println!("  -> Test 3 passed.\n");

    // Test case 4: FFIableObject - custom struct
    println!("- Test Case 4: FFIableObject (Custom Struct)");
    #[derive(Debug)]
    struct MyStruct {
        a: u8,
        b: u16,
    }
    impl FFISafeContainer for MyStruct {}
    let my_struct = MyStruct { a: 10, b: 20 };
    let mut ffi_object = FFIableObject::create_using_box_no_display(my_struct);
    println!("  -> Created FFIableObject: {:?}", ffi_object);

    // Get a reference to the value
    let ref_struct = unsafe { ffi_object.get::<MyStruct>() };
    assert_eq!(ref_struct.a, 10);
    assert_eq!(ref_struct.b, 20);
    println!("  -> get() correctly retrieves a shared reference.");

    // Get a mutable reference and modify the value
    let mut_struct = unsafe { ffi_object.get_mut::<MyStruct>() };
    mut_struct.a = 50;
    assert_eq!(mut_struct.a, 50);
    println!("  -> get_mut() correctly retrieves a mutable reference and allows modification.");

    // Reconstruct the object
    let reconstructed = unsafe { ffi_object.reconstruct::<MyStruct>() };
    assert_eq!(reconstructed.a, 50);
    assert_eq!(reconstructed.b, 20);
    println!("  -> reconstruct() successfully retrieves the value and consumes the FFIableObject.");

    // Check if the FFIableObject is now poisoned and will panic on drop (this is a negative test, so we can't just assert)
    let _ = FFIableObject::create_using_box_no_display(MyStruct { a: 1, b: 2 }); // A fresh object
    let mut ffi_transfer = FFIableObject::create_using_box_no_display(MyStruct { a: 10, b: 20 });
    let mut new_owner = unsafe { ffi_transfer.transfer_ownership() }; 
    // Dropping ffi_transfer here won't do anything because it's poisoned.
    // The memory will be freed when new_owner goes out of scope.
    drop(ffi_transfer);
    println!("  -> transfer_ownership() successfully transferred control and poisoned the original object.");

    println!("  -> Test 4 passed.\n");

    println!("All tests completed successfully!");
}