#[doc = "Reader of register FPROT1"]
pub type R = crate::R<u8, super::FPROT1>;
#[doc = "Writer for register FPROT1"]
pub type W = crate::W<u8, super::FPROT1>;
#[doc = "Register FPROT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FPROT1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROT`"]
pub type PROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROT`"]
pub struct PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W { w: self }
    }
}
