#[doc = "Register `ES` reader"]
pub type R = crate::R<EsSpec>;
#[doc = "Destination Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbe {
    #[doc = "0: No destination bus error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a bus error on a destination write"]
    _1 = 1,
}
impl From<Dbe> for bool {
    #[inline(always)]
    fn from(variant: Dbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBE` reader - Destination Bus Error"]
pub type DbeR = crate::BitReader<Dbe>;
impl DbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbe {
        match self.bits {
            false => Dbe::_0,
            true => Dbe::_1,
        }
    }
    #[doc = "No destination bus error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dbe::_0
    }
    #[doc = "The last recorded error was a bus error on a destination write"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dbe::_1
    }
}
#[doc = "Source Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbe {
    #[doc = "0: No source bus error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a bus error on a source read"]
    _1 = 1,
}
impl From<Sbe> for bool {
    #[inline(always)]
    fn from(variant: Sbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBE` reader - Source Bus Error"]
pub type SbeR = crate::BitReader<Sbe>;
impl SbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbe {
        match self.bits {
            false => Sbe::_0,
            true => Sbe::_1,
        }
    }
    #[doc = "No source bus error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sbe::_0
    }
    #[doc = "The last recorded error was a bus error on a source read"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sbe::_1
    }
}
#[doc = "Scatter/Gather Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sge {
    #[doc = "0: No scatter/gather configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\\[ESG\\]
is enabled. TCDn_DLASTSGA is not on a 32 byte boundary."]
    _1 = 1,
}
impl From<Sge> for bool {
    #[inline(always)]
    fn from(variant: Sge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGE` reader - Scatter/Gather Configuration Error"]
pub type SgeR = crate::BitReader<Sge>;
impl SgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sge {
        match self.bits {
            false => Sge::_0,
            true => Sge::_1,
        }
    }
    #[doc = "No scatter/gather configuration error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sge::_0
    }
    #[doc = "The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\\[ESG\\]
is enabled. TCDn_DLASTSGA is not on a 32 byte boundary."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sge::_1
    }
}
#[doc = "NBYTES/CITER Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nce {
    #[doc = "0: No NBYTES/CITER configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\\[SSIZE\\]
and TCDn_ATTR\\[DSIZE\\], or TCDn_CITER\\[CITER\\]
is equal to zero, or TCDn_CITER\\[ELINK\\]
is not equal to TCDn_BITER\\[ELINK\\]"]
    _1 = 1,
}
impl From<Nce> for bool {
    #[inline(always)]
    fn from(variant: Nce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCE` reader - NBYTES/CITER Configuration Error"]
pub type NceR = crate::BitReader<Nce>;
impl NceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nce {
        match self.bits {
            false => Nce::_0,
            true => Nce::_1,
        }
    }
    #[doc = "No NBYTES/CITER configuration error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nce::_0
    }
    #[doc = "The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\\[SSIZE\\]
and TCDn_ATTR\\[DSIZE\\], or TCDn_CITER\\[CITER\\]
is equal to zero, or TCDn_CITER\\[ELINK\\]
is not equal to TCDn_BITER\\[ELINK\\]"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nce::_1
    }
}
#[doc = "Destination Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe {
    #[doc = "0: No destination offset configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    _1 = 1,
}
impl From<Doe> for bool {
    #[inline(always)]
    fn from(variant: Doe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE` reader - Destination Offset Error"]
pub type DoeR = crate::BitReader<Doe>;
impl DoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe {
        match self.bits {
            false => Doe::_0,
            true => Doe::_1,
        }
    }
    #[doc = "No destination offset configuration error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Doe::_0
    }
    #[doc = "The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Doe::_1
    }
}
#[doc = "Destination Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dae {
    #[doc = "0: No destination address configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    _1 = 1,
}
impl From<Dae> for bool {
    #[inline(always)]
    fn from(variant: Dae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAE` reader - Destination Address Error"]
pub type DaeR = crate::BitReader<Dae>;
impl DaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dae {
        match self.bits {
            false => Dae::_0,
            true => Dae::_1,
        }
    }
    #[doc = "No destination address configuration error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dae::_0
    }
    #[doc = "The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dae::_1
    }
}
#[doc = "Source Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soe {
    #[doc = "0: No source offset configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    _1 = 1,
}
impl From<Soe> for bool {
    #[inline(always)]
    fn from(variant: Soe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOE` reader - Source Offset Error"]
pub type SoeR = crate::BitReader<Soe>;
impl SoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soe {
        match self.bits {
            false => Soe::_0,
            true => Soe::_1,
        }
    }
    #[doc = "No source offset configuration error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Soe::_0
    }
    #[doc = "The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Soe::_1
    }
}
#[doc = "Source Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sae {
    #[doc = "0: No source address configuration error."]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    _1 = 1,
}
impl From<Sae> for bool {
    #[inline(always)]
    fn from(variant: Sae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAE` reader - Source Address Error"]
pub type SaeR = crate::BitReader<Sae>;
impl SaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sae {
        match self.bits {
            false => Sae::_0,
            true => Sae::_1,
        }
    }
    #[doc = "No source address configuration error."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sae::_0
    }
    #[doc = "The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sae::_1
    }
}
#[doc = "Field `ERRCHN` reader - Error Channel Number or Cancelled Channel Number"]
pub type ErrchnR = crate::FieldReader;
#[doc = "Channel Priority Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpe {
    #[doc = "0: No channel priority error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error in the channel priorities. Channel priorities are not unique."]
    _1 = 1,
}
impl From<Cpe> for bool {
    #[inline(always)]
    fn from(variant: Cpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPE` reader - Channel Priority Error"]
pub type CpeR = crate::BitReader<Cpe>;
impl CpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpe {
        match self.bits {
            false => Cpe::_0,
            true => Cpe::_1,
        }
    }
    #[doc = "No channel priority error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpe::_0
    }
    #[doc = "The last recorded error was a configuration error in the channel priorities. Channel priorities are not unique."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpe::_1
    }
}
#[doc = "Transfer Cancelled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecx {
    #[doc = "0: No cancelled transfers"]
    _0 = 0,
    #[doc = "1: The last recorded entry was a cancelled transfer by the error cancel transfer input"]
    _1 = 1,
}
impl From<Ecx> for bool {
    #[inline(always)]
    fn from(variant: Ecx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECX` reader - Transfer Cancelled"]
pub type EcxR = crate::BitReader<Ecx>;
impl EcxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecx {
        match self.bits {
            false => Ecx::_0,
            true => Ecx::_1,
        }
    }
    #[doc = "No cancelled transfers"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecx::_0
    }
    #[doc = "The last recorded entry was a cancelled transfer by the error cancel transfer input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecx::_1
    }
}
#[doc = "Logical OR of all ERR status bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vld {
    #[doc = "0: No ERR bits are set"]
    _0 = 0,
    #[doc = "1: At least one ERR bit is set indicating a valid error exists that has not been cleared"]
    _1 = 1,
}
impl From<Vld> for bool {
    #[inline(always)]
    fn from(variant: Vld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLD` reader - Logical OR of all ERR status bits"]
pub type VldR = crate::BitReader<Vld>;
impl VldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vld {
        match self.bits {
            false => Vld::_0,
            true => Vld::_1,
        }
    }
    #[doc = "No ERR bits are set"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vld::_0
    }
    #[doc = "At least one ERR bit is set indicating a valid error exists that has not been cleared"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vld::_1
    }
}
impl R {
    #[doc = "Bit 0 - Destination Bus Error"]
    #[inline(always)]
    pub fn dbe(&self) -> DbeR {
        DbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source Bus Error"]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub fn sge(&self) -> SgeR {
        SgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub fn nce(&self) -> NceR {
        NceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Destination Offset Error"]
    #[inline(always)]
    pub fn doe(&self) -> DoeR {
        DoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Address Error"]
    #[inline(always)]
    pub fn dae(&self) -> DaeR {
        DaeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Offset Error"]
    #[inline(always)]
    pub fn soe(&self) -> SoeR {
        SoeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source Address Error"]
    #[inline(always)]
    pub fn sae(&self) -> SaeR {
        SaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Error Channel Number or Cancelled Channel Number"]
    #[inline(always)]
    pub fn errchn(&self) -> ErrchnR {
        ErrchnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Channel Priority Error"]
    #[inline(always)]
    pub fn cpe(&self) -> CpeR {
        CpeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer Cancelled"]
    #[inline(always)]
    pub fn ecx(&self) -> EcxR {
        EcxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Logical OR of all ERR status bits"]
    #[inline(always)]
    pub fn vld(&self) -> VldR {
        VldR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`es::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsSpec;
impl crate::RegisterSpec for EsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`es::R`](R) reader structure"]
impl crate::Readable for EsSpec {}
#[doc = "`reset()` method sets ES to value 0"]
impl crate::Resettable for EsSpec {
    const RESET_VALUE: u32 = 0;
}
