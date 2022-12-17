// MIT License
//
// Copyright (c) 2022 Robin Doer
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

mod current;
mod insert;
mod insert_front;
mod payload;
mod walk;

use std::io::{Cursor, Write};

use crate::backend::{Backend, BlockId};
use crate::bytes::ToBytesExt;
use crate::container::{Cipher, Container, CreateOptionsBuilder};
use crate::memory::{MemOptions, MemoryBackend};

macro_rules! next {
    ($stream:expr) => {
        $stream.next_block().unwrap().unwrap()
    };
}

macro_rules! prev {
    ($stream:expr) => {
        $stream.prev_block().unwrap().unwrap()
    };
}

macro_rules! assert_next_is_none {
    ($stream:expr) => {
        assert!($stream.next_block().is_none())
    };
}

macro_rules! assert_prev_is_none {
    ($stream:expr) => {
        assert!($stream.prev_block().is_none())
    };
}

use {assert_next_is_none, assert_prev_is_none, next, prev};

fn setup_container() -> Container<MemoryBackend> {
    let options = CreateOptionsBuilder::<MemoryBackend>::new(MemOptions::new(), Cipher::None)
        .build()
        .unwrap();

    Container::create(options).unwrap()
}

fn make_block<B: Backend>(
    container: &mut Container<B>,
    id: &B::Id,
    first: bool,
    prev: &B::Id,
    next: &B::Id,
    payload: &[u8],
) {
    let mut cursor = Cursor::new(vec![]);

    if first {
        cursor.write_bytes(b"stream0").unwrap();
    } else {
        cursor.write_bytes(b"streamn").unwrap();
    }

    cursor.to_bytes(prev).unwrap(); // prev
    cursor.to_bytes(next).unwrap(); // next
    cursor.to_bytes(&(payload.len() as u32)).unwrap(); // length
    cursor.write_bytes(payload).unwrap();
    cursor.flush().unwrap();

    container.write(&id, &cursor.into_inner()).unwrap();
}

fn setup_one<B: Backend>(container: &mut Container<B>) -> B::Id {
    let id = container.aquire().unwrap();
    let next = B::Id::null();

    make_block(container, &id, true, &id, &next, &[1, 2, 3]);

    id
}

fn setup_two<B: Backend>(container: &mut Container<B>) -> (B::Id, B::Id) {
    let id1 = container.aquire().unwrap();
    let id2 = container.aquire().unwrap();
    let null = B::Id::null();

    make_block(container, &id1, true, &id2, &id2, &[1, 2, 3]);
    make_block(container, &id2, false, &id1, &null, &[4, 5, 6]);

    (id1, id2)
}

fn setup_three<B: Backend>(container: &mut Container<B>) -> (B::Id, B::Id, B::Id) {
    let id1 = container.aquire().unwrap();
    let id2 = container.aquire().unwrap();
    let id3 = container.aquire().unwrap();
    let null = B::Id::null();

    make_block(container, &id1, true, &id3, &id2, &[1, 2, 3]);
    make_block(container, &id2, false, &id1, &id3, &[4, 5, 6]);
    make_block(container, &id3, false, &id2, &null, &[7, 8, 9]);

    (id1, id2, id3)
}