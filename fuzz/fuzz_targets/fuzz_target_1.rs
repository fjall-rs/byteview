#![no_main]
use byteview::ByteView;
use libfuzzer_sys::{
    arbitrary::{Arbitrary, Unstructured},
    fuzz_target,
};

fuzz_target!(|data: &[u8]| {
    let mut unstructured = Unstructured::new(data);

    // Generate two different fuzzed inputs
    if let (Ok(input1), Ok(input2)) = (
        <Vec<u8> as Arbitrary>::arbitrary(&mut unstructured),
        <Vec<u8> as Arbitrary>::arbitrary(&mut unstructured),
    ) {
        let a = ByteView::from(&*input1);
        let b = ByteView::from(&*input2);

        eprintln!("{a:?} <=> {b:?}");

        assert_eq!(input1 == input2, a == b);
        assert_eq!(input1 > input2, a > b);
    }
});
