use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize,Debug,PartialEq)]
struct MyStruct { 
    id:u64,
    data:String,
    v:Vec<u32>
}
fn main(){
    let original = MyStruct{
        id:42,
        data : ("Hello,From Struct").into(),
        v:vec![1,4,6,7]
    };
    let mut buffer:Vec<u8> = Vec::new();
    original.serialize(&mut buffer).unwrap();
    println!("{:?}",buffer);
    let deserialized = MyStruct::try_from_slice(&mut buffer).unwrap();
    println!("{:?}",size_of_val(&deserialized));
    println!("{:?}",deserialized);

}