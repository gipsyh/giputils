#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptionU32(u32);

impl OptionU32 {
    pub const NONE: Self = Self(u32::MAX);

    #[inline]
    pub fn some(v: u32) -> Self {
        Self(v)
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        *self != Self::NONE
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }
}

impl Default for OptionU32 {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}
