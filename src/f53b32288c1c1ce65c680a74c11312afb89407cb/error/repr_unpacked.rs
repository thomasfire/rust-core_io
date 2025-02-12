//! This is a fairly simple unpacked error representation that's used on
//! non-64bit targets, where the packed 64 bit representation wouldn't work, and
//! would have no benefit.

use super::{Custom, ErrorData, ErrorKind, SimpleMessage};

#[cfg(feature="alloc")] use alloc::boxed::Box;
#[cfg(not(feature="alloc"))] use ::FakeBox as Box;

#[cfg(feature="alloc")] type Inner = ErrorData<Box<Custom>>;
#[cfg(not(feature="alloc"))] type Inner = ErrorData<Custom>;

pub(super) struct Repr(Inner);

impl Repr {
    #[inline]
    pub(super) fn new(dat: Inner) -> Self {
        Self(dat)
    }
    #[cfg(feature="alloc")]
    pub(super) fn new_custom(b: Box<Custom>) -> Self {
        Self(Inner::Custom(b))
    }
    #[cfg(not(feature="alloc"))]
    pub(super) fn new_custom(b: Custom) -> Self {
        Self(Inner::Custom(b))
    }
    #[inline]
    pub(super) fn new_os(code: i32) -> Self {
        Self(Inner::Os(code))
    }
    #[inline]
    pub(super) fn new_simple(kind: ErrorKind) -> Self {
        Self(Inner::Simple(kind))
    }
    #[inline]
    pub(super) const fn new_simple_message(m: &'static SimpleMessage) -> Self {
        Self(Inner::SimpleMessage(m))
    }
    #[inline]
    pub(super) fn into_data(self) -> Inner {
        self.0
    }

    #[inline]
    #[cfg(feature="alloc")]
    pub(super) fn data(&self) -> ErrorData<&Custom> {
        match &self.0 {
            Inner::Os(c) => ErrorData::Os(*c),
            Inner::Simple(k) => ErrorData::Simple(*k),
            Inner::SimpleMessage(m) => ErrorData::SimpleMessage(*m),
            Inner::Custom(m) => ErrorData::Custom(&*m),
        }
    }
    #[inline]
    #[cfg(feature="alloc")]
    pub(super) fn data_mut(&mut self) -> ErrorData<&mut Custom> {
        match &mut self.0 {
            Inner::Os(c) => ErrorData::Os(*c),
            Inner::Simple(k) => ErrorData::Simple(*k),
            Inner::SimpleMessage(m) => ErrorData::SimpleMessage(*m),
            Inner::Custom(m) => ErrorData::Custom(&mut *m),
        }
    }

    #[inline]
    #[cfg(not(feature="alloc"))]
    pub(super) fn data(&self) -> ErrorData<&Custom> {
        match &self.0 {
            Inner::Os(c) => ErrorData::Os(*c),
            Inner::Simple(k) => ErrorData::Simple(*k),
            Inner::SimpleMessage(m) => ErrorData::SimpleMessage(*m),
            Inner::Custom(m) => ErrorData::Custom(&*m),
        }
    }
    #[inline]
    #[cfg(not(feature="alloc"))]
    pub(super) fn data_mut(&mut self) -> ErrorData<&mut Custom> {
        match &mut self.0 {
            Inner::Os(c) => ErrorData::Os(*c),
            Inner::Simple(k) => ErrorData::Simple(*k),
            Inner::SimpleMessage(m) => ErrorData::SimpleMessage(*m),
            Inner::Custom(m) => ErrorData::Custom(&mut *m),
        }
    }
}
