pub trait Ext {
    fn is_nul_terminated(&self) -> bool;
}

impl Ext for str {
    #[inline]
    fn is_nul_terminated(&self) -> bool {
        self.as_bytes().is_nul_terminated()
    }
}

impl Ext for [u8] {
    #[inline]
    fn is_nul_terminated(&self) -> bool {
        self.last().cloned() == Some(0)
    }
}
