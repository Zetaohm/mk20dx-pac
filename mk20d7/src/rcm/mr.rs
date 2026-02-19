#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "EZP_MS_B pin state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EzpMs {
    #[doc = "0: Pin negated (logic 1)"]
    _0 = 0,
    #[doc = "1: Pin asserted (logic 0)"]
    _1 = 1,
}
impl From<EzpMs> for bool {
    #[inline(always)]
    fn from(variant: EzpMs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EZP_MS` reader - EZP_MS_B pin state"]
pub type EzpMsR = crate::BitReader<EzpMs>;
impl EzpMsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EzpMs {
        match self.bits {
            false => EzpMs::_0,
            true => EzpMs::_1,
        }
    }
    #[doc = "Pin negated (logic 1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EzpMs::_0
    }
    #[doc = "Pin asserted (logic 0)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EzpMs::_1
    }
}
impl R {
    #[doc = "Bit 1 - EZP_MS_B pin state"]
    #[inline(always)]
    pub fn ezp_ms(&self) -> EzpMsR {
        EzpMsR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u8 = 0;
}
