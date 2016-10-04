// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(deprecated)]

use test::{Bencher, black_box};

use core::hash::{Hash, Hasher};
use core::hash::{SipHasher, SipHasher13, SipHasher24};

// Hash just the bytes of the slice, without length prefix
struct Bytes<'a>(&'a [u8]);

impl<'a> Hash for Bytes<'a> {
    #[allow(unused_must_use)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Bytes(v) = *self;
        state.write(v);
    }
}

macro_rules! u8to64_le {
    ($buf:expr, $i:expr) =>
    ($buf[0+$i] as u64 |
     ($buf[1+$i] as u64) << 8 |
     ($buf[2+$i] as u64) << 16 |
     ($buf[3+$i] as u64) << 24 |
     ($buf[4+$i] as u64) << 32 |
     ($buf[5+$i] as u64) << 40 |
     ($buf[6+$i] as u64) << 48 |
     ($buf[7+$i] as u64) << 56);
    ($buf:expr, $i:expr, $len:expr) =>
    ({
        let mut t = 0;
        let mut out = 0;
        while t < $len {
            out |= ($buf[t+$i] as u64) << t*8;
            t += 1;
        }
        out
    });
}

fn hash_with<H: Hasher, T: Hash>(mut st: H, x: &T) -> u64 {
    x.hash(&mut st);
    st.finish()
}

fn hash<T: Hash>(x: &T) -> u64 {
    hash_with(SipHasher::new(), x)
}

fn hash_bytes<H: Hasher>(mut s: H, x: &[u8]) -> u64 {
    Hasher::write(&mut s, x);
    s.finish()
}

#[test]
#[allow(unused_must_use)]
fn test_siphash_1_3() {
    let vecs : [[u8; 8]; 64] = [
        [ 0xdc, 0xc4, 0x0f, 0x05, 0x58, 0x01, 0xac, 0xab ],
        [ 0x93, 0xca, 0x57, 0x7d, 0xf3, 0x9b, 0xf4, 0xc9 ],
        [ 0x4d, 0xd4, 0xc7, 0x4d, 0x02, 0x9b, 0xcb, 0x82 ],
        [ 0xfb, 0xf7, 0xdd, 0xe7, 0xb8, 0x0a, 0xf8, 0x8b ],
        [ 0x28, 0x83, 0xd3, 0x88, 0x60, 0x57, 0x75, 0xcf ],
        [ 0x67, 0x3b, 0x53, 0x49, 0x2f, 0xd5, 0xf9, 0xde ],
        [ 0xa7, 0x22, 0x9f, 0xc5, 0x50, 0x2b, 0x0d, 0xc5 ],
        [ 0x40, 0x11, 0xb1, 0x9b, 0x98, 0x7d, 0x92, 0xd3 ],
        [ 0x8e, 0x9a, 0x29, 0x8d, 0x11, 0x95, 0x90, 0x36 ],
        [ 0xe4, 0x3d, 0x06, 0x6c, 0xb3, 0x8e, 0xa4, 0x25 ],
        [ 0x7f, 0x09, 0xff, 0x92, 0xee, 0x85, 0xde, 0x79 ],
        [ 0x52, 0xc3, 0x4d, 0xf9, 0xc1, 0x18, 0xc1, 0x70 ],
        [ 0xa2, 0xd9, 0xb4, 0x57, 0xb1, 0x84, 0xa3, 0x78 ],
        [ 0xa7, 0xff, 0x29, 0x12, 0x0c, 0x76, 0x6f, 0x30 ],
        [ 0x34, 0x5d, 0xf9, 0xc0, 0x11, 0xa1, 0x5a, 0x60 ],
        [ 0x56, 0x99, 0x51, 0x2a, 0x6d, 0xd8, 0x20, 0xd3 ],
        [ 0x66, 0x8b, 0x90, 0x7d, 0x1a, 0xdd, 0x4f, 0xcc ],
        [ 0x0c, 0xd8, 0xdb, 0x63, 0x90, 0x68, 0xf2, 0x9c ],
        [ 0x3e, 0xe6, 0x73, 0xb4, 0x9c, 0x38, 0xfc, 0x8f ],
        [ 0x1c, 0x7d, 0x29, 0x8d, 0xe5, 0x9d, 0x1f, 0xf2 ],
        [ 0x40, 0xe0, 0xcc, 0xa6, 0x46, 0x2f, 0xdc, 0xc0 ],
        [ 0x44, 0xf8, 0x45, 0x2b, 0xfe, 0xab, 0x92, 0xb9 ],
        [ 0x2e, 0x87, 0x20, 0xa3, 0x9b, 0x7b, 0xfe, 0x7f ],
        [ 0x23, 0xc1, 0xe6, 0xda, 0x7f, 0x0e, 0x5a, 0x52 ],
        [ 0x8c, 0x9c, 0x34, 0x67, 0xb2, 0xae, 0x64, 0xf4 ],
        [ 0x79, 0x09, 0x5b, 0x70, 0x28, 0x59, 0xcd, 0x45 ],
        [ 0xa5, 0x13, 0x99, 0xca, 0xe3, 0x35, 0x3e, 0x3a ],
        [ 0x35, 0x3b, 0xde, 0x4a, 0x4e, 0xc7, 0x1d, 0xa9 ],
        [ 0x0d, 0xd0, 0x6c, 0xef, 0x02, 0xed, 0x0b, 0xfb ],
        [ 0xf4, 0xe1, 0xb1, 0x4a, 0xb4, 0x3c, 0xd9, 0x88 ],
        [ 0x63, 0xe6, 0xc5, 0x43, 0xd6, 0x11, 0x0f, 0x54 ],
        [ 0xbc, 0xd1, 0x21, 0x8c, 0x1f, 0xdd, 0x70, 0x23 ],
        [ 0x0d, 0xb6, 0xa7, 0x16, 0x6c, 0x7b, 0x15, 0x81 ],
        [ 0xbf, 0xf9, 0x8f, 0x7a, 0xe5, 0xb9, 0x54, 0x4d ],
        [ 0x3e, 0x75, 0x2a, 0x1f, 0x78, 0x12, 0x9f, 0x75 ],
        [ 0x91, 0x6b, 0x18, 0xbf, 0xbe, 0xa3, 0xa1, 0xce ],
        [ 0x06, 0x62, 0xa2, 0xad, 0xd3, 0x08, 0xf5, 0x2c ],
        [ 0x57, 0x30, 0xc3, 0xa3, 0x2d, 0x1c, 0x10, 0xb6 ],
        [ 0xa1, 0x36, 0x3a, 0xae, 0x96, 0x74, 0xf4, 0xb3 ],
        [ 0x92, 0x83, 0x10, 0x7b, 0x54, 0x57, 0x6b, 0x62 ],
        [ 0x31, 0x15, 0xe4, 0x99, 0x32, 0x36, 0xd2, 0xc1 ],
        [ 0x44, 0xd9, 0x1a, 0x3f, 0x92, 0xc1, 0x7c, 0x66 ],
        [ 0x25, 0x88, 0x13, 0xc8, 0xfe, 0x4f, 0x70, 0x65 ],
        [ 0xa6, 0x49, 0x89, 0xc2, 0xd1, 0x80, 0xf2, 0x24 ],
        [ 0x6b, 0x87, 0xf8, 0xfa, 0xed, 0x1c, 0xca, 0xc2 ],
        [ 0x96, 0x21, 0x04, 0x9f, 0xfc, 0x4b, 0x16, 0xc2 ],
        [ 0x23, 0xd6, 0xb1, 0x68, 0x93, 0x9c, 0x6e, 0xa1 ],
        [ 0xfd, 0x14, 0x51, 0x8b, 0x9c, 0x16, 0xfb, 0x49 ],
        [ 0x46, 0x4c, 0x07, 0xdf, 0xf8, 0x43, 0x31, 0x9f ],
        [ 0xb3, 0x86, 0xcc, 0x12, 0x24, 0xaf, 0xfd, 0xc6 ],
        [ 0x8f, 0x09, 0x52, 0x0a, 0xd1, 0x49, 0xaf, 0x7e ],
        [ 0x9a, 0x2f, 0x29, 0x9d, 0x55, 0x13, 0xf3, 0x1c ],
        [ 0x12, 0x1f, 0xf4, 0xa2, 0xdd, 0x30, 0x4a, 0xc4 ],
        [ 0xd0, 0x1e, 0xa7, 0x43, 0x89, 0xe9, 0xfa, 0x36 ],
        [ 0xe6, 0xbc, 0xf0, 0x73, 0x4c, 0xb3, 0x8f, 0x31 ],
        [ 0x80, 0xe9, 0xa7, 0x70, 0x36, 0xbf, 0x7a, 0xa2 ],
        [ 0x75, 0x6d, 0x3c, 0x24, 0xdb, 0xc0, 0xbc, 0xb4 ],
        [ 0x13, 0x15, 0xb7, 0xfd, 0x52, 0xd8, 0xf8, 0x23 ],
        [ 0x08, 0x8a, 0x7d, 0xa6, 0x4d, 0x5f, 0x03, 0x8f ],
        [ 0x48, 0xf1, 0xe8, 0xb7, 0xe5, 0xd0, 0x9c, 0xd8 ],
        [ 0xee, 0x44, 0xa6, 0xf7, 0xbc, 0xe6, 0xf4, 0xf6 ],
        [ 0xf2, 0x37, 0x18, 0x0f, 0xd8, 0x9a, 0xc5, 0xae ],
        [ 0xe0, 0x94, 0x66, 0x4b, 0x15, 0xf6, 0xb2, 0xc3 ],
        [ 0xa8, 0xb3, 0xbb, 0xb7, 0x62, 0x90, 0x19, 0x9d ]
    ];

    let k0 = 0x_07_06_05_04_03_02_01_00;
    let k1 = 0x_0f_0e_0d_0c_0b_0a_09_08;
    let mut buf = Vec::new();
    let mut t = 0;
    let mut state_inc = SipHasher13::new_with_keys(k0, k1);

    while t < 64 {
        let vec = u8to64_le!(vecs[t], 0);
        let out = hash_with(SipHasher13::new_with_keys(k0, k1), &Bytes(&buf));
        assert_eq!(vec, out);

        let full = hash_with(SipHasher13::new_with_keys(k0, k1), &Bytes(&buf));
        let i = state_inc.finish();

        assert_eq!(full, i);
        assert_eq!(full, vec);

        buf.push(t as u8);
        Hasher::write(&mut state_inc, &[t as u8]);

        t += 1;
    }
}

#[test]
#[allow(unused_must_use)]
fn test_siphash_2_4() {
    let vecs : [[u8; 8]; 64] = [
        [ 0x31, 0x0e, 0x0e, 0xdd, 0x47, 0xdb, 0x6f, 0x72, ],
        [ 0xfd, 0x67, 0xdc, 0x93, 0xc5, 0x39, 0xf8, 0x74, ],
        [ 0x5a, 0x4f, 0xa9, 0xd9, 0x09, 0x80, 0x6c, 0x0d, ],
        [ 0x2d, 0x7e, 0xfb, 0xd7, 0x96, 0x66, 0x67, 0x85, ],
        [ 0xb7, 0x87, 0x71, 0x27, 0xe0, 0x94, 0x27, 0xcf, ],
        [ 0x8d, 0xa6, 0x99, 0xcd, 0x64, 0x55, 0x76, 0x18, ],
        [ 0xce, 0xe3, 0xfe, 0x58, 0x6e, 0x46, 0xc9, 0xcb, ],
        [ 0x37, 0xd1, 0x01, 0x8b, 0xf5, 0x00, 0x02, 0xab, ],
        [ 0x62, 0x24, 0x93, 0x9a, 0x79, 0xf5, 0xf5, 0x93, ],
        [ 0xb0, 0xe4, 0xa9, 0x0b, 0xdf, 0x82, 0x00, 0x9e, ],
        [ 0xf3, 0xb9, 0xdd, 0x94, 0xc5, 0xbb, 0x5d, 0x7a, ],
        [ 0xa7, 0xad, 0x6b, 0x22, 0x46, 0x2f, 0xb3, 0xf4, ],
        [ 0xfb, 0xe5, 0x0e, 0x86, 0xbc, 0x8f, 0x1e, 0x75, ],
        [ 0x90, 0x3d, 0x84, 0xc0, 0x27, 0x56, 0xea, 0x14, ],
        [ 0xee, 0xf2, 0x7a, 0x8e, 0x90, 0xca, 0x23, 0xf7, ],
        [ 0xe5, 0x45, 0xbe, 0x49, 0x61, 0xca, 0x29, 0xa1, ],
        [ 0xdb, 0x9b, 0xc2, 0x57, 0x7f, 0xcc, 0x2a, 0x3f, ],
        [ 0x94, 0x47, 0xbe, 0x2c, 0xf5, 0xe9, 0x9a, 0x69, ],
        [ 0x9c, 0xd3, 0x8d, 0x96, 0xf0, 0xb3, 0xc1, 0x4b, ],
        [ 0xbd, 0x61, 0x79, 0xa7, 0x1d, 0xc9, 0x6d, 0xbb, ],
        [ 0x98, 0xee, 0xa2, 0x1a, 0xf2, 0x5c, 0xd6, 0xbe, ],
        [ 0xc7, 0x67, 0x3b, 0x2e, 0xb0, 0xcb, 0xf2, 0xd0, ],
        [ 0x88, 0x3e, 0xa3, 0xe3, 0x95, 0x67, 0x53, 0x93, ],
        [ 0xc8, 0xce, 0x5c, 0xcd, 0x8c, 0x03, 0x0c, 0xa8, ],
        [ 0x94, 0xaf, 0x49, 0xf6, 0xc6, 0x50, 0xad, 0xb8, ],
        [ 0xea, 0xb8, 0x85, 0x8a, 0xde, 0x92, 0xe1, 0xbc, ],
        [ 0xf3, 0x15, 0xbb, 0x5b, 0xb8, 0x35, 0xd8, 0x17, ],
        [ 0xad, 0xcf, 0x6b, 0x07, 0x63, 0x61, 0x2e, 0x2f, ],
        [ 0xa5, 0xc9, 0x1d, 0xa7, 0xac, 0xaa, 0x4d, 0xde, ],
        [ 0x71, 0x65, 0x95, 0x87, 0x66, 0x50, 0xa2, 0xa6, ],
        [ 0x28, 0xef, 0x49, 0x5c, 0x53, 0xa3, 0x87, 0xad, ],
        [ 0x42, 0xc3, 0x41, 0xd8, 0xfa, 0x92, 0xd8, 0x32, ],
        [ 0xce, 0x7c, 0xf2, 0x72, 0x2f, 0x51, 0x27, 0x71, ],
        [ 0xe3, 0x78, 0x59, 0xf9, 0x46, 0x23, 0xf3, 0xa7, ],
        [ 0x38, 0x12, 0x05, 0xbb, 0x1a, 0xb0, 0xe0, 0x12, ],
        [ 0xae, 0x97, 0xa1, 0x0f, 0xd4, 0x34, 0xe0, 0x15, ],
        [ 0xb4, 0xa3, 0x15, 0x08, 0xbe, 0xff, 0x4d, 0x31, ],
        [ 0x81, 0x39, 0x62, 0x29, 0xf0, 0x90, 0x79, 0x02, ],
        [ 0x4d, 0x0c, 0xf4, 0x9e, 0xe5, 0xd4, 0xdc, 0xca, ],
        [ 0x5c, 0x73, 0x33, 0x6a, 0x76, 0xd8, 0xbf, 0x9a, ],
        [ 0xd0, 0xa7, 0x04, 0x53, 0x6b, 0xa9, 0x3e, 0x0e, ],
        [ 0x92, 0x59, 0x58, 0xfc, 0xd6, 0x42, 0x0c, 0xad, ],
        [ 0xa9, 0x15, 0xc2, 0x9b, 0xc8, 0x06, 0x73, 0x18, ],
        [ 0x95, 0x2b, 0x79, 0xf3, 0xbc, 0x0a, 0xa6, 0xd4, ],
        [ 0xf2, 0x1d, 0xf2, 0xe4, 0x1d, 0x45, 0x35, 0xf9, ],
        [ 0x87, 0x57, 0x75, 0x19, 0x04, 0x8f, 0x53, 0xa9, ],
        [ 0x10, 0xa5, 0x6c, 0xf5, 0xdf, 0xcd, 0x9a, 0xdb, ],
        [ 0xeb, 0x75, 0x09, 0x5c, 0xcd, 0x98, 0x6c, 0xd0, ],
        [ 0x51, 0xa9, 0xcb, 0x9e, 0xcb, 0xa3, 0x12, 0xe6, ],
        [ 0x96, 0xaf, 0xad, 0xfc, 0x2c, 0xe6, 0x66, 0xc7, ],
        [ 0x72, 0xfe, 0x52, 0x97, 0x5a, 0x43, 0x64, 0xee, ],
        [ 0x5a, 0x16, 0x45, 0xb2, 0x76, 0xd5, 0x92, 0xa1, ],
        [ 0xb2, 0x74, 0xcb, 0x8e, 0xbf, 0x87, 0x87, 0x0a, ],
        [ 0x6f, 0x9b, 0xb4, 0x20, 0x3d, 0xe7, 0xb3, 0x81, ],
        [ 0xea, 0xec, 0xb2, 0xa3, 0x0b, 0x22, 0xa8, 0x7f, ],
        [ 0x99, 0x24, 0xa4, 0x3c, 0xc1, 0x31, 0x57, 0x24, ],
        [ 0xbd, 0x83, 0x8d, 0x3a, 0xaf, 0xbf, 0x8d, 0xb7, ],
        [ 0x0b, 0x1a, 0x2a, 0x32, 0x65, 0xd5, 0x1a, 0xea, ],
        [ 0x13, 0x50, 0x79, 0xa3, 0x23, 0x1c, 0xe6, 0x60, ],
        [ 0x93, 0x2b, 0x28, 0x46, 0xe4, 0xd7, 0x06, 0x66, ],
        [ 0xe1, 0x91, 0x5f, 0x5c, 0xb1, 0xec, 0xa4, 0x6c, ],
        [ 0xf3, 0x25, 0x96, 0x5c, 0xa1, 0x6d, 0x62, 0x9f, ],
        [ 0x57, 0x5f, 0xf2, 0x8e, 0x60, 0x38, 0x1b, 0xe5, ],
        [ 0x72, 0x45, 0x06, 0xeb, 0x4c, 0x32, 0x8a, 0x95, ]
    ];

    let k0 = 0x_07_06_05_04_03_02_01_00;
    let k1 = 0x_0f_0e_0d_0c_0b_0a_09_08;
    let mut buf = Vec::new();
    let mut t = 0;
    let mut state_inc = SipHasher24::new_with_keys(k0, k1);

    while t < 64 {
        let vec = u8to64_le!(vecs[t], 0);
        let out = hash_with(SipHasher24::new_with_keys(k0, k1), &Bytes(&buf));
        assert_eq!(vec, out);

        let full = hash_with(SipHasher24::new_with_keys(k0, k1), &Bytes(&buf));
        let i = state_inc.finish();

        assert_eq!(full, i);
        assert_eq!(full, vec);

        buf.push(t as u8);
        Hasher::write(&mut state_inc, &[t as u8]);

        t += 1;
    }
}
#[test] #[cfg(target_arch = "arm")]
fn test_hash_usize() {
    let val = 0xdeadbeef_deadbeef_u64;
    assert!(hash(&(val as u64)) != hash(&(val as usize)));
    assert_eq!(hash(&(val as u32)), hash(&(val as usize)));
}
#[test] #[cfg(target_arch = "x86_64")]
fn test_hash_usize() {
    let val = 0xdeadbeef_deadbeef_u64;
    assert_eq!(hash(&(val as u64)), hash(&(val as usize)));
    assert!(hash(&(val as u32)) != hash(&(val as usize)));
}
#[test] #[cfg(target_arch = "x86")]
fn test_hash_usize() {
    let val = 0xdeadbeef_deadbeef_u64;
    assert!(hash(&(val as u64)) != hash(&(val as usize)));
    assert_eq!(hash(&(val as u32)), hash(&(val as usize)));
}

#[test]
fn test_hash_idempotent() {
    let val64 = 0xdeadbeef_deadbeef_u64;
    assert_eq!(hash(&val64), hash(&val64));
    let val32 = 0xdeadbeef_u32;
    assert_eq!(hash(&val32), hash(&val32));
}

#[test]
fn test_hash_no_bytes_dropped_64() {
    let val = 0xdeadbeef_deadbeef_u64;

    assert!(hash(&val) != hash(&zero_byte(val, 0)));
    assert!(hash(&val) != hash(&zero_byte(val, 1)));
    assert!(hash(&val) != hash(&zero_byte(val, 2)));
    assert!(hash(&val) != hash(&zero_byte(val, 3)));
    assert!(hash(&val) != hash(&zero_byte(val, 4)));
    assert!(hash(&val) != hash(&zero_byte(val, 5)));
    assert!(hash(&val) != hash(&zero_byte(val, 6)));
    assert!(hash(&val) != hash(&zero_byte(val, 7)));

    fn zero_byte(val: u64, byte: usize) -> u64 {
        assert!(byte < 8);
        val & !(0xff << (byte * 8))
    }
}

#[test]
fn test_hash_no_bytes_dropped_32() {
    let val = 0xdeadbeef_u32;

    assert!(hash(&val) != hash(&zero_byte(val, 0)));
    assert!(hash(&val) != hash(&zero_byte(val, 1)));
    assert!(hash(&val) != hash(&zero_byte(val, 2)));
    assert!(hash(&val) != hash(&zero_byte(val, 3)));

    fn zero_byte(val: u32, byte: usize) -> u32 {
        assert!(byte < 4);
        val & !(0xff << (byte * 8))
    }
}

#[test]
fn test_hash_no_concat_alias() {
    let s = ("aa", "bb");
    let t = ("aabb", "");
    let u = ("a", "abb");

    assert!(s != t && t != u);
    assert!(hash(&s) != hash(&t) && hash(&s) != hash(&u));

    let u = [1, 0, 0, 0];
    let v = (&u[..1], &u[1..3], &u[3..]);
    let w = (&u[..], &u[4..4], &u[4..4]);

    assert!(v != w);
    assert!(hash(&v) != hash(&w));
}

#[bench]
fn bench_str_under_8_bytes(b: &mut Bencher) {
    let s = "foo";
    b.iter(|| {
        assert_eq!(hash(&s), 16262950014981195938);
    })
}

#[bench]
fn bench_str_of_8_bytes(b: &mut Bencher) {
    let s = "foobar78";
    b.iter(|| {
        assert_eq!(hash(&s), 4898293253460910787);
    })
}

#[bench]
fn bench_str_over_8_bytes(b: &mut Bencher) {
    let s = "foobarbaz0";
    b.iter(|| {
        assert_eq!(hash(&s), 10581415515220175264);
    })
}

#[bench]
fn bench_long_str(b: &mut Bencher) {
    let s = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor \
incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute \
irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui \
officia deserunt mollit anim id est laborum.";
    b.iter(|| {
        assert_eq!(hash(&s), 17717065544121360093);
    })
}

#[bench]
fn bench_u32(b: &mut Bencher) {
    let u = 162629500u32;
    let u = black_box(u);
    b.iter(|| {
        hash(&u)
    });
    b.bytes = 8;
}

#[bench]
fn bench_u32_keyed(b: &mut Bencher) {
    let u = 162629500u32;
    let u = black_box(u);
    let k1 = black_box(0x1);
    let k2 = black_box(0x2);
    b.iter(|| {
        hash_with(SipHasher::new_with_keys(k1, k2), &u)
    });
    b.bytes = 8;
}

#[bench]
fn bench_u64(b: &mut Bencher) {
    let u = 16262950014981195938u64;
    let u = black_box(u);
    b.iter(|| {
        hash(&u)
    });
    b.bytes = 8;
}

#[bench]
fn bench_bytes_4(b: &mut Bencher) {
    let data = black_box([b' '; 4]);
    b.iter(|| {
        hash_bytes(SipHasher::default(), &data)
    });
    b.bytes = 4;
}

#[bench]
fn bench_bytes_7(b: &mut Bencher) {
    let data = black_box([b' '; 7]);
    b.iter(|| {
        hash_bytes(SipHasher::default(), &data)
    });
    b.bytes = 7;
}

#[bench]
fn bench_bytes_8(b: &mut Bencher) {
    let data = black_box([b' '; 8]);
    b.iter(|| {
        hash_bytes(SipHasher::default(), &data)
    });
    b.bytes = 8;
}

#[bench]
fn bench_bytes_a_16(b: &mut Bencher) {
    let data = black_box([b' '; 16]);
    b.iter(|| {
        hash_bytes(SipHasher::default(), &data)
    });
    b.bytes = 16;
}

#[bench]
fn bench_bytes_b_32(b: &mut Bencher) {
    let data = black_box([b' '; 32]);
    b.iter(|| {
        hash_bytes(SipHasher::default(), &data)
    });
    b.bytes = 32;
}

#[bench]
fn bench_bytes_c_128(b: &mut Bencher) {
    let data = black_box([b' '; 128]);
    b.iter(|| {
        hash_bytes(SipHasher::default(), &data)
    });
    b.bytes = 128;
}
