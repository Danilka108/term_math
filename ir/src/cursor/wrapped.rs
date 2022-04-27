use std::ops::{Deref, DerefMut};

pub struct WrappedStr<'s>(&'s str);

impl<'s> Deref for WrappedStr<'s> {
    type Target = &'s str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> DerefMut for WrappedStr<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'s> WrappedStr<'s> {
    pub fn wrap(val: &'s str) -> Self {
        Self(val)
    }
}

pub struct WrappedString(String);

impl Deref for WrappedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WrappedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WrappedString {
    pub fn wrap(val: String) -> Self {
        Self(val)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
