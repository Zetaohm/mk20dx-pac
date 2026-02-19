#[doc = "Register `TCD%s_ATTR` reader"]
pub type R = crate::R<TcdAttrSpec>;
#[doc = "Register `TCD%s_ATTR` writer"]
pub type W = crate::W<TcdAttrSpec>;
#[doc = "Field `DSIZE` reader - Destination Data Transfer Size"]
pub type DsizeR = crate::FieldReader;
#[doc = "Field `DSIZE` writer - Destination Data Transfer Size"]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMOD` reader - Destination Address Modulo"]
pub type DmodR = crate::FieldReader;
#[doc = "Field `DMOD` writer - Destination Address Modulo"]
pub type DmodW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Source data transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssize {
    #[doc = "0: 8-bit"]
    _000 = 0,
    #[doc = "1: 16-bit"]
    _001 = 1,
    #[doc = "2: 32-bit"]
    _010 = 2,
    #[doc = "4: 16-byte"]
    _100 = 4,
}
impl From<Ssize> for u8 {
    #[inline(always)]
    fn from(variant: Ssize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssize {
    type Ux = u8;
}
impl crate::IsEnum for Ssize {}
#[doc = "Field `SSIZE` reader - Source data transfer size"]
pub type SsizeR = crate::FieldReader<Ssize>;
impl SsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssize> {
        match self.bits {
            0 => Some(Ssize::_000),
            1 => Some(Ssize::_001),
            2 => Some(Ssize::_010),
            4 => Some(Ssize::_100),
            _ => None,
        }
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ssize::_000
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ssize::_001
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ssize::_010
    }
    #[doc = "16-byte"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ssize::_100
    }
}
#[doc = "Field `SSIZE` writer - Source data transfer size"]
pub type SsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ssize>;
impl<'a, REG> SsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ssize::_000)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ssize::_001)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ssize::_010)
    }
    #[doc = "16-byte"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ssize::_100)
    }
}
#[doc = "Source Address Modulo.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smod {
    #[doc = "0: Source address modulo feature is disabled"]
    _0 = 0,
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(variant: Smod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smod {
    type Ux = u8;
}
impl crate::IsEnum for Smod {}
#[doc = "Field `SMOD` reader - Source Address Modulo."]
pub type SmodR = crate::FieldReader<Smod>;
impl SmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smod> {
        match self.bits {
            0 => Some(Smod::_0),
            _ => None,
        }
    }
    #[doc = "Source address modulo feature is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Smod::_0
    }
}
#[doc = "Field `SMOD` writer - Source Address Modulo."]
pub type SmodW<'a, REG> = crate::FieldWriter<'a, REG, 5, Smod>;
impl<'a, REG> SmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source address modulo feature is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::_0)
    }
}
impl R {
    #[doc = "Bits 0:2 - Destination Data Transfer Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&self) -> DmodR {
        DmodR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&self) -> SsizeR {
        SsizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - Source Address Modulo."]
    #[inline(always)]
    pub fn smod(&self) -> SmodR {
        SmodR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Destination Data Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DsizeW<TcdAttrSpec> {
        DsizeW::new(self, 0)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    #[must_use]
    pub fn dmod(&mut self) -> DmodW<TcdAttrSpec> {
        DmodW::new(self, 3)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn ssize(&mut self) -> SsizeW<TcdAttrSpec> {
        SsizeW::new(self, 8)
    }
    #[doc = "Bits 11:15 - Source Address Modulo."]
    #[inline(always)]
    #[must_use]
    pub fn smod(&mut self) -> SmodW<TcdAttrSpec> {
        SmodW::new(self, 11)
    }
}
#[doc = "TCD Transfer Attributes\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcdAttrSpec;
impl crate::RegisterSpec for TcdAttrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcd_attr::R`](R) reader structure"]
impl crate::Readable for TcdAttrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcd_attr::W`](W) writer structure"]
impl crate::Writable for TcdAttrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TCD%s_ATTR to value 0"]
impl crate::Resettable for TcdAttrSpec {
    const RESET_VALUE: u16 = 0;
}
