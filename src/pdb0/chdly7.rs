#[doc = "Reader of register CH%sDLY7"]
pub type R = crate::R<u32, super::CHDLY7>;
#[doc = "Writer for register CH%sDLY7"]
pub type W = crate::W<u32, super::CHDLY7>;
#[doc = "Register CH%sDLY7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHDLY7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLY`"]
pub type DLY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DLY`"]
pub struct DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PDB Channel Delay"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Channel Delay"]
    #[inline(always)]
    pub fn dly(&mut self) -> DLY_W {
        DLY_W { w: self }
    }
}
