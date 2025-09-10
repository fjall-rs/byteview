use crate::ByteView;

/// A builder for a [`ByteView`] that allows mutation before freezing it.
pub struct Builder(ByteView);

impl Builder {
    /// Creates a new builder.
    #[must_use]
    pub const fn new(inner: ByteView) -> Self {
        Self(inner)
    }

    /// Converts the builder into a [`ByteView`], making it immutable.
    #[must_use]
    pub fn freeze(mut self) -> ByteView {
        self.0.update_prefix();
        self.0
    }
}

impl std::ops::Deref for Builder {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.get_mut_slice()
    }
}
