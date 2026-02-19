#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `ODD` reader - this bit is set if the last Buffer Descriptor updated was in the odd bank of the BDT."]
pub type OddR = crate::BitReader;
#[doc = "Transmit Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: The most recent transaction was a Receive operation."]
    _0 = 0,
    #[doc = "1: The most recent transaction was a Transmit operation."]
    _1 = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - Transmit Indicator"]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            false => Tx::_0,
            true => Tx::_1,
        }
    }
    #[doc = "The most recent transaction was a Receive operation."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tx::_0
    }
    #[doc = "The most recent transaction was a Transmit operation."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tx::_1
    }
}
#[doc = "Field `ENDP` reader - This four-bit field encodes the endpoint address that received or transmitted the previous token"]
pub type EndpR = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - this bit is set if the last Buffer Descriptor updated was in the odd bank of the BDT."]
    #[inline(always)]
    pub fn odd(&self) -> OddR {
        OddR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Indicator"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - This four-bit field encodes the endpoint address that received or transmitted the previous token"]
    #[inline(always)]
    pub fn endp(&self) -> EndpR {
        EndpR::new((self.bits >> 4) & 0x0f)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u8 = 0;
}
