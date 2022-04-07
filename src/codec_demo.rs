use parity_scale_codec::{Encode, Decode};
use parity_scale_codec_derive::{Encode, Decode};

#[derive(Debug, PartialEq, Encode, Decode)]
enum EnumType {
	#[codec(index = 15)]
	A,
	B(u32, u64),
	C {
		a: u32,
		b: u64,
	},
}

pub fn run() {

let a = EnumType::A;
let b = EnumType::B(1, 2);
let c = EnumType::C { a: 513, b: 2 };

a.using_encoded(|ref slice| {
    println!("a.using_encoded {:?}", slice);
    assert_eq!(slice, &b"\x0f");
});

b.using_encoded(|ref slice| {
    println!("b.using_encoded {:?}", slice);
    assert_eq!(slice, &b"\x01\x01\0\0\0\x02\0\0\0\0\0\0\0");
});

c.using_encoded(|ref slice| {
    println!("c.using_encoded {:?}", slice);
    assert_eq!(slice, &b"\x02\x01\0\0\0\x02\0\0\0\0\0\0\0");
});



let mut da: &[u8] = b"\x0f";
assert_eq!(EnumType::decode(&mut da).ok(), Some(a));

let mut db: &[u8] = b"\x01\x01\0\0\0\x02\0\0\0\0\0\0\0";
assert_eq!(EnumType::decode(&mut db).ok(), Some(b));

let mut dc: &[u8] = b"\x02\x01\0\0\0\x02\0\0\0\0\0\0\0";
assert_eq!(EnumType::decode(&mut dc).ok(), Some(c));

let mut dz: &[u8] = &[0];
assert_eq!(EnumType::decode(&mut dz).ok(), None);

}

/* Compact type with HasCompact
use parity_scale_codec::{Encode, Decode, Compact, HasCompact};
use parity_scale_codec_derive::{Encode, Decode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test1CompactHasCompact<T: HasCompact> {
    #[codec(compact)]
    bar: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Test1HasCompact<T: HasCompact> {
    #[codec(encoded_as = "<T as HasCompact>::Type")]
    bar: T,
}

let test_val: (u64, usize) = (0u64, 1usize);

let encoded = Test1HasCompact { bar: test_val.0 }.encode();
assert_eq!(encoded.len(), test_val.1);
assert_eq!(<Test1CompactHasCompact<u64>>::decode(&mut &encoded[..]).unwrap().bar, test_val.0); */