#[doc = "Register `ESR2` reader"]
pub type R = crate::R<Esr2Spec>;
#[doc = "Inactive Mailbox\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imb {
    #[doc = "0: If ESR2\\[VPS\\]
is asserted, the ESR2\\[LPTM\\]
is not an inactive Mailbox."]
    _0 = 0,
    #[doc = "1: If ESR2\\[VPS\\]
is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
    _1 = 1,
}
impl From<Imb> for bool {
    #[inline(always)]
    fn from(variant: Imb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMB` reader - Inactive Mailbox"]
pub type ImbR = crate::BitReader<Imb>;
impl ImbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imb {
        match self.bits {
            false => Imb::_0,
            true => Imb::_1,
        }
    }
    #[doc = "If ESR2\\[VPS\\]
is asserted, the ESR2\\[LPTM\\]
is not an inactive Mailbox."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Imb::_0
    }
    #[doc = "If ESR2\\[VPS\\]
is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Imb::_1
    }
}
#[doc = "Valid Priority Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vps {
    #[doc = "0: Contents of IMB and LPTM are invalid."]
    _0 = 0,
    #[doc = "1: Contents of IMB and LPTM are valid."]
    _1 = 1,
}
impl From<Vps> for bool {
    #[inline(always)]
    fn from(variant: Vps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VPS` reader - Valid Priority Status"]
pub type VpsR = crate::BitReader<Vps>;
impl VpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vps {
        match self.bits {
            false => Vps::_0,
            true => Vps::_1,
        }
    }
    #[doc = "Contents of IMB and LPTM are invalid."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vps::_0
    }
    #[doc = "Contents of IMB and LPTM are valid."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vps::_1
    }
}
#[doc = "Field `LPTM` reader - Lowest Priority Tx Mailbox"]
pub type LptmR = crate::FieldReader;
impl R {
    #[doc = "Bit 13 - Inactive Mailbox"]
    #[inline(always)]
    pub fn imb(&self) -> ImbR {
        ImbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Valid Priority Status"]
    #[inline(always)]
    pub fn vps(&self) -> VpsR {
        VpsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Lowest Priority Tx Mailbox"]
    #[inline(always)]
    pub fn lptm(&self) -> LptmR {
        LptmR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Error and Status 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esr2Spec;
impl crate::RegisterSpec for Esr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esr2::R`](R) reader structure"]
impl crate::Readable for Esr2Spec {}
#[doc = "`reset()` method sets ESR2 to value 0"]
impl crate::Resettable for Esr2Spec {
    const RESET_VALUE: u32 = 0;
}
