use crate::syn::{ReadableType, Reader, WritableType, Writer};
use core::marker::PhantomData;

pub struct Utf8String<C: Constraint = NoConstraint>(PhantomData<C>);

impl<C: Constraint> Default for Utf8String<C> {
    fn default() -> Self {
        Utf8String(Default::default())
    }
}

pub trait Constraint {
    const MIN: Option<usize> = None;
    const MAX: Option<usize> = None;
}

#[derive(Default)]
pub struct NoConstraint;
impl Constraint for NoConstraint {}

impl<C: Constraint> WritableType for Utf8String<C> {
    type Type = String;

    #[inline]
    fn write_value<W: Writer>(writer: &mut W, value: &Self::Type) -> Result<(), W::Error> {
        writer.write_utf8string::<C>(value.as_str())
    }
}

impl<C: Constraint> ReadableType for Utf8String<C> {
    type Type = String;

    #[inline]
    fn read_value<R: Reader>(reader: &mut R) -> Result<Self::Type, <R as Reader>::Error> {
        reader.read_utf8string::<C>()
    }
}
