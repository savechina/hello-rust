//!
//! futures Sample
//!
use base64::prelude::*;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use bitvec::prelude::*;
use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::Cursor;

fn bytes_create() {
    let mut mem = Bytes::from("Hello world");
    let a = mem.slice(0..5);

    assert_eq!(a, "Hello");

    let b = mem.split_to(6);

    assert_eq!(mem, "world");
    assert_eq!(b, "Hello ");

    assert_eq!(b.len(), 6);

    let mut a = Bytes::from(&b"hello world"[..]);
    let b = a.split_off(5);

    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b" world");
}

fn bytes_buffer() {
    let hello = Bytes::new();
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);

    let a = buf.split();
    assert_eq!(a, b"hello world\x04\xD2"[..]);

    buf.put(&b"goodbye world"[..]);

    let b = buf.split();
    assert_eq!(b, b"goodbye world"[..]);

    assert_eq!(buf.capacity(), 998);
}

fn bytes_search() {
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    assert_eq!(s.binary_search(&13), Ok(9));
    assert_eq!(s.binary_search(&4), Err(7));
    assert_eq!(s.binary_search(&100), Err(13));
    let r = s.binary_search(&1);
    assert!(match r {
        Ok(1..=4) => true,
        _ => false,
    });

    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    let seek = 4;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
    let seek = 100;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
    let seek = 1;
    let r = s.binary_search_by(|probe| probe.cmp(&seek));
    assert!(match r {
        Ok(1..=4) => true,
        _ => false,
    });
}

fn bytes_order() {
    let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
    // Note that we use type parameters to indicate which kind of byte order
    // we want!
    assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
    assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
}

fn bytes_bitvets() {
    // All data-types have macro
    // constructors.
    let arr = bitarr![u32, Lsb0; 0; 80];
    let bits = bits![u16, Msb0; 0; 40];

    // Unsigned integers (scalar, array,
    // and slice) can be borrowed.
    let data = 0x2021u16;
    let bits = data.view_bits::<Msb0>();
    let data = [0xA5u8, 0x3C];
    let bits = data.view_bits::<Lsb0>();

    // Bit-slices can split anywhere.
    let (head, rest) = bits.split_at(4);
    assert_eq!(head, bits[..4]);
    assert_eq!(rest, bits[4..]);

    // And they are writable!
    let mut data = [0u8; 2];
    let bits = data.view_bits_mut::<Lsb0>();
    // l and r each own one byte.
    let (l, r) = bits.split_at_mut(8);

    // but now a, b, c, and d own a nibble!
    let ((a, b), (c, d)) = (l.split_at_mut(4), r.split_at_mut(4));

    // and all four of them are writable.
    a.set(0, true);
    b.set(1, true);
    c.set(2, true);
    d.set(3, true);

    assert!(bits[0]); // a[0]
    assert!(bits[5]); // b[1]
    assert!(bits[10]); // c[2]
    assert!(bits[15]); // d[3]

    // `BitSlice` is accessed by reference,
    // which means it respects NLL styles.
    assert_eq!(data, [0x21u8, 0x84]);

    // Furthermore, bit-slices can store
    // ordinary integers:
    let eight = [0u8, 4, 8, 12, 16, 20, 24, 28];
    //           a    b  c  d   e   f   g   h
    let mut five = [0u8; 5];
    for (slot, byte) in five
        .view_bits_mut::<Msb0>()
        .chunks_mut(5)
        .zip(eight.iter().copied())
    {
        slot.store_be(byte);
        assert_eq!(slot.load_be::<u8>(), byte);
    }

    assert_eq!(
        five,
        [
            0b00000_001,
            //  aaaaa bbb
            0b00_01000_0,
            //  bb ccccc d
            0b1100_1000,
            //  dddd eeee
            0b0_10100_11,
            //  e fffff gg
            0b000_11100,
            //  ggg hhhhh
        ]
    );
}

fn bytes_base64() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
    assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");

    assert_eq!(URL_SAFE.decode(b"-uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
    assert_eq!(URL_SAFE.encode(b"\xFF\xEC\x20\x55\0"), "_-wgVQA=");

    let context = b"hello world!";

    let result = BASE64_STANDARD.encode(context);

    println!("encode:{}", result);

    let origin = BASE64_STANDARD.decode(result).unwrap();

    println!("decode:{:?}", String::from_utf8(origin));

    Ok(())
}

fn bytes_base64_write() {
    use base64::engine::general_purpose;
    use std::io::Write;

    // use a vec as the simplest possible `Write` -- in real code this is probably a file, etc.
    let mut enc = base64::write::EncoderWriter::new(Vec::new(), &general_purpose::STANDARD);

    // handle errors as you normally would
    enc.write_all(b"asdf").unwrap();

    // could leave this out to be called by Drop, if you don't care
    // about handling errors or getting the delegate writer back
    let delegate = enc.finish().unwrap();

    // base64 was written to the writer
    assert_eq!(b"YXNkZg==", &delegate[..]);

    println!("'asdf' is base64 : {:?}", String::from_utf8(delegate));
}

///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_bytes() {
        bytes_create();

        bytes_buffer();

        bytes_search();

        bytes_order();
    }

    #[test]
    fn test_bitvet() {
        bytes_bitvets();
    }

    #[test]
    fn test_base64() {
        bytes_base64();

        bytes_base64_write();
    }
}
