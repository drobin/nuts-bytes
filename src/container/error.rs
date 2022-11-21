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

use std::{error, fmt, result};

use crate::backend::Backend;
use crate::bytes;

/// Error type used by this module.
pub enum ContainerError<B: Backend> {
    /// An error occured in the attached backend.
    Backend(B::Err),

    /// Error while (de-) serializing binary data.
    Bytes(bytes::Error),
}

impl<B: Backend> fmt::Display for ContainerError<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContainerError::Backend(cause) => fmt::Display::fmt(cause, fmt),
            ContainerError::Bytes(cause) => fmt::Display::fmt(cause, fmt),
        }
    }
}

impl<B: Backend> fmt::Debug for ContainerError<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContainerError::Backend(cause) => fmt::Debug::fmt(cause, fmt),
            ContainerError::Bytes(cause) => fmt::Debug::fmt(cause, fmt),
        }
    }
}

impl<B: Backend + 'static> error::Error for ContainerError<B> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ContainerError::Backend(cause) => Some(cause),
            ContainerError::Bytes(cause) => Some(cause),
        }
    }
}

impl<B: Backend> From<bytes::Error> for ContainerError<B> {
    fn from(cause: bytes::Error) -> Self {
        ContainerError::Bytes(cause)
    }
}

pub type ContainerResult<T, B> = result::Result<T, ContainerError<B>>;
