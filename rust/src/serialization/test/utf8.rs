//use crate::serialization::test::de_serialize;
use crate::serialization::test::serialize;
use crate::serialization::tutf8::TUtf8;
use crate::serialization::slice_reader::MemReader;
use crate::serialization::core::Reader;

#[test]
fn small_strings_utf8() {
   // assert_string_serde_eq("");
    //assert_string_serde_eq("1");
    //assert_string_serde_eq("1è");
    //assert_string_serde_eq("1èf");
    //assert_string_serde_eq("1èfö");
    assert_string_serde_eq("hello");
    //assert_string_serde_eq("hello6");
    /*assert_string_serde_eq("hello67");
    assert_string_serde_eq("hello678");*/
}

fn assert_string_serde_eq(string : &'static str) {
    let binary = serialize::<TUtf8>(string);
    
    //let restored_string = de_serialize::<TUtf8>(&binary);

        let mut reader = MemReader::from(binary.as_slice());    
    let restored = reader.read::<TUtf8>().unwrap();


    assert_eq!(string, restored);
}