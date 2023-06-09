// MIT License
//
// Copyright (c) 2023 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use std::borrow::Cow;

use crate::assert_error;
use crate::error::Error;
use crate::reader::Reader;

#[test]
fn i8() {
    let mut reader = Reader::new([0xff, 0, 1].as_slice());

    assert_eq!(reader.read_i8().unwrap(), -1);
    assert_eq!(reader.as_ref(), &[0, 1]);

    assert_eq!(reader.read_i8().unwrap(), 0);
    assert_eq!(reader.as_ref(), &[1]);

    assert_eq!(reader.read_i8().unwrap(), 1);
    assert_eq!(reader.as_ref(), &[]);

    let err = reader.read_i8().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[]);
}

#[test]
fn u8() {
    let mut reader = Reader::new([1, 2, 3].as_slice());

    assert_eq!(reader.read_u8().unwrap(), 1);
    assert_eq!(reader.as_ref(), &[2, 3]);

    assert_eq!(reader.read_u8().unwrap(), 2);
    assert_eq!(reader.as_ref(), &[3]);

    assert_eq!(reader.read_u8().unwrap(), 3);
    assert_eq!(reader.as_ref(), &[]);

    let err = reader.read_u8().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[]);
}

#[test]
fn i16() {
    let mut reader = Reader::new([0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x02].as_slice());

    assert_eq!(reader.read_i16().unwrap(), -1);
    assert_eq!(reader.as_ref(), &[0x00, 0x00, 0x00, 0x01, 0x02]);

    assert_eq!(reader.read_i16().unwrap(), 0);
    assert_eq!(reader.as_ref(), &[0x00, 0x01, 0x02]);

    assert_eq!(reader.read_i16().unwrap(), 1);
    assert_eq!(reader.as_ref(), &[0x02]);

    let err = reader.read_i16().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[0x02]);
}

#[test]
fn u16() {
    let mut reader = Reader::new([1, 2, 3, 4, 5].as_slice());

    assert_eq!(reader.read_u16().unwrap(), 0x0102);
    assert_eq!(reader.as_ref(), &[3, 4, 5]);

    assert_eq!(reader.read_u16().unwrap(), 0x0304);
    assert_eq!(reader.as_ref(), &[5]);

    let err = reader.read_u16().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[5]);
}

#[test]
fn i32() {
    let mut reader = Reader::new(
        [
            0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02,
        ]
        .as_slice(),
    );

    assert_eq!(reader.read_i32().unwrap(), -1);
    assert_eq!(
        reader.as_ref(),
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02]
    );

    assert_eq!(reader.read_i32().unwrap(), 0);
    assert_eq!(reader.as_ref(), &[0x00, 0x00, 0x00, 0x01, 0x02]);

    assert_eq!(reader.read_i32().unwrap(), 1);
    assert_eq!(reader.as_ref(), &[0x02]);

    let err = reader.read_u32().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[0x02]);
}

#[test]
fn u32() {
    let mut reader = Reader::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].as_slice());

    assert_eq!(reader.read_u32().unwrap(), 0x01020304);
    assert_eq!(reader.as_ref(), &[5, 6, 7, 8, 9, 10, 11]);

    assert_eq!(reader.read_u32().unwrap(), 0x05060708);
    assert_eq!(reader.as_ref(), &[9, 10, 11]);

    let err = reader.read_u32().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[9, 10, 11]);
}

#[test]
fn i64() {
    let mut reader = Reader::new(
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02,
        ]
        .as_slice(),
    );

    assert_eq!(reader.read_i64().unwrap(), -1);
    assert_eq!(
        reader.as_ref(),
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x02
        ]
    );

    assert_eq!(reader.read_i64().unwrap(), 0);
    assert_eq!(
        reader.as_ref(),
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02]
    );

    assert_eq!(reader.read_i64().unwrap(), 1);
    assert_eq!(reader.as_ref(), &[0x02]);

    let err = reader.read_i64().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[0x02]);
}

#[test]
fn u64() {
    let mut reader = Reader::new(
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        ]
        .as_slice(),
    );

    assert_eq!(reader.read_u64().unwrap(), 0x0102030405060708);
    assert_eq!(
        reader.as_ref(),
        &[9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,]
    );

    assert_eq!(reader.read_u64().unwrap(), 0x090A0B0C0D0E0F10);
    assert_eq!(reader.as_ref(), &[17, 18, 19, 20, 21, 22, 23]);

    let err = reader.read_u64().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[17, 18, 19, 20, 21, 22, 23]);
}

#[test]
fn i128() {
    let mut reader = Reader::new(
        [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02,
        ]
        .as_slice(),
    );

    assert_eq!(reader.read_i128().unwrap(), -1);
    assert_eq!(
        reader.as_ref(),
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x02
        ]
    );

    assert_eq!(reader.read_i128().unwrap(), 0);
    assert_eq!(
        reader.as_ref(),
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x02
        ]
    );

    assert_eq!(reader.read_i128().unwrap(), 1);
    assert_eq!(reader.as_ref(), &[0x02]);

    let err = reader.read_i128().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[0x02]);
}

#[test]
fn u128() {
    let mut reader = Reader::new(
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47,
        ]
        .as_slice(),
    );

    assert_eq!(
        reader.read_u128().unwrap(),
        0x0102030405060708090A0B0C0D0E0F10
    );
    assert_eq!(
        reader.as_ref(),
        &[
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
            39, 40, 41, 42, 43, 44, 45, 46, 47,
        ]
    );

    assert_eq!(
        reader.read_u128().unwrap(),
        0x1112131415161718191a1b1c1d1e1f20
    );
    assert_eq!(
        reader.as_ref(),
        &[33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,]
    );

    let err = reader.read_u128().unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(
        reader.as_ref(),
        &[33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,]
    );
}

#[test]
fn read_bytes() {
    let mut reader = Reader::new([1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice());

    assert_eq!(reader.read_bytes(0).unwrap(), Cow::Borrowed(&[]));
    assert_eq!(reader.as_ref(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(reader.read_bytes(1).unwrap(), Cow::Borrowed(&[1]));
    assert_eq!(reader.as_ref(), &[2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(reader.read_bytes(2).unwrap(), Cow::Borrowed(&[2, 3]));
    assert_eq!(reader.as_ref(), &[4, 5, 6, 7, 8, 9]);

    assert_eq!(reader.read_bytes(3).unwrap(), Cow::Borrowed(&[4, 5, 6]));
    assert_eq!(reader.as_ref(), &[7, 8, 9]);

    let err = reader.read_bytes(4).unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(reader.as_ref(), &[7, 8, 9]);
}

#[test]
fn read_bytes_to() {
    let mut reader = Reader::new([1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice());

    let mut buf = [];
    reader.read_bytes_to(&mut buf).unwrap();
    assert_eq!(reader.as_ref(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 1];
    reader.read_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [1]);
    assert_eq!(reader.as_ref(), &[2, 3, 4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 2];
    reader.read_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [2, 3]);
    assert_eq!(reader.as_ref(), &[4, 5, 6, 7, 8, 9]);

    let mut buf = [0; 3];
    reader.read_bytes_to(&mut buf).unwrap();
    assert_eq!(buf, [4, 5, 6]);
    assert_eq!(reader.as_ref(), &[7, 8, 9]);

    let mut buf = [0; 4];
    let err = reader.read_bytes_to(&mut buf).unwrap_err();
    assert_error!(err, Error::Eof(|cause| cause.is_none()));
    assert_eq!(buf, [0, 0, 0, 0]);
    assert_eq!(reader.as_ref(), &[7, 8, 9]);
}
