#[doc = "Register `PFB1CR` reader"]
pub type R = crate::R<Pfb1crSpec>;
#[doc = "Register `PFB1CR` writer"]
pub type W = crate::W<Pfb1crSpec>;
#[doc = "Bank 1 Single Entry Buffer Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1sebe {
    #[doc = "0: Single entry buffer is disabled."]
    _0 = 0,
    #[doc = "1: Single entry buffer is enabled."]
    _1 = 1,
}
impl From<B1sebe> for bool {
    #[inline(always)]
    fn from(variant: B1sebe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B1SEBE` reader - Bank 1 Single Entry Buffer Enable"]
pub type B1sebeR = crate::BitReader<B1sebe>;
impl B1sebeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B1sebe {
        match self.bits {
            false => B1sebe::_0,
            true => B1sebe::_1,
        }
    }
    #[doc = "Single entry buffer is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1sebe::_0
    }
    #[doc = "Single entry buffer is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1sebe::_1
    }
}
#[doc = "Field `B1SEBE` writer - Bank 1 Single Entry Buffer Enable"]
pub type B1sebeW<'a, REG> = crate::BitWriter<'a, REG, B1sebe>;
impl<'a, REG> B1sebeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single entry buffer is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B1sebe::_0)
    }
    #[doc = "Single entry buffer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B1sebe::_1)
    }
}
#[doc = "Bank 1 Instruction Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1ipe {
    #[doc = "0: Do not prefetch in response to instruction fetches."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to instruction fetches."]
    _1 = 1,
}
impl From<B1ipe> for bool {
    #[inline(always)]
    fn from(variant: B1ipe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B1IPE` reader - Bank 1 Instruction Prefetch Enable"]
pub type B1ipeR = crate::BitReader<B1ipe>;
impl B1ipeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B1ipe {
        match self.bits {
            false => B1ipe::_0,
            true => B1ipe::_1,
        }
    }
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1ipe::_0
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1ipe::_1
    }
}
#[doc = "Field `B1IPE` writer - Bank 1 Instruction Prefetch Enable"]
pub type B1ipeW<'a, REG> = crate::BitWriter<'a, REG, B1ipe>;
impl<'a, REG> B1ipeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B1ipe::_0)
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B1ipe::_1)
    }
}
#[doc = "Bank 1 Data Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1dpe {
    #[doc = "0: Do not prefetch in response to data references."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to data references."]
    _1 = 1,
}
impl From<B1dpe> for bool {
    #[inline(always)]
    fn from(variant: B1dpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B1DPE` reader - Bank 1 Data Prefetch Enable"]
pub type B1dpeR = crate::BitReader<B1dpe>;
impl B1dpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B1dpe {
        match self.bits {
            false => B1dpe::_0,
            true => B1dpe::_1,
        }
    }
    #[doc = "Do not prefetch in response to data references."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1dpe::_0
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1dpe::_1
    }
}
#[doc = "Field `B1DPE` writer - Bank 1 Data Prefetch Enable"]
pub type B1dpeW<'a, REG> = crate::BitWriter<'a, REG, B1dpe>;
impl<'a, REG> B1dpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not prefetch in response to data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B1dpe::_0)
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B1dpe::_1)
    }
}
#[doc = "Bank 1 Instruction Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1ice {
    #[doc = "0: Do not cache instruction fetches."]
    _0 = 0,
    #[doc = "1: Cache instruction fetches."]
    _1 = 1,
}
impl From<B1ice> for bool {
    #[inline(always)]
    fn from(variant: B1ice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B1ICE` reader - Bank 1 Instruction Cache Enable"]
pub type B1iceR = crate::BitReader<B1ice>;
impl B1iceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B1ice {
        match self.bits {
            false => B1ice::_0,
            true => B1ice::_1,
        }
    }
    #[doc = "Do not cache instruction fetches."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1ice::_0
    }
    #[doc = "Cache instruction fetches."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1ice::_1
    }
}
#[doc = "Field `B1ICE` writer - Bank 1 Instruction Cache Enable"]
pub type B1iceW<'a, REG> = crate::BitWriter<'a, REG, B1ice>;
impl<'a, REG> B1iceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not cache instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B1ice::_0)
    }
    #[doc = "Cache instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B1ice::_1)
    }
}
#[doc = "Bank 1 Data Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1dce {
    #[doc = "0: Do not cache data references."]
    _0 = 0,
    #[doc = "1: Cache data references."]
    _1 = 1,
}
impl From<B1dce> for bool {
    #[inline(always)]
    fn from(variant: B1dce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B1DCE` reader - Bank 1 Data Cache Enable"]
pub type B1dceR = crate::BitReader<B1dce>;
impl B1dceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B1dce {
        match self.bits {
            false => B1dce::_0,
            true => B1dce::_1,
        }
    }
    #[doc = "Do not cache data references."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1dce::_0
    }
    #[doc = "Cache data references."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1dce::_1
    }
}
#[doc = "Field `B1DCE` writer - Bank 1 Data Cache Enable"]
pub type B1dceW<'a, REG> = crate::BitWriter<'a, REG, B1dce>;
impl<'a, REG> B1dceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not cache data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B1dce::_0)
    }
    #[doc = "Cache data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B1dce::_1)
    }
}
#[doc = "Bank 1 Memory Width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum B1mw {
    #[doc = "0: 32 bits"]
    _00 = 0,
    #[doc = "1: 64 bits"]
    _01 = 1,
}
impl From<B1mw> for u8 {
    #[inline(always)]
    fn from(variant: B1mw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for B1mw {
    type Ux = u8;
}
impl crate::IsEnum for B1mw {}
#[doc = "Field `B1MW` reader - Bank 1 Memory Width"]
pub type B1mwR = crate::FieldReader<B1mw>;
impl B1mwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<B1mw> {
        match self.bits {
            0 => Some(B1mw::_00),
            1 => Some(B1mw::_01),
            _ => None,
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == B1mw::_00
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == B1mw::_01
    }
}
#[doc = "Field `B1RWSC` reader - Bank 1 Read Wait State Control"]
pub type B1rwscR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Bank 1 Single Entry Buffer Enable"]
    #[inline(always)]
    pub fn b1sebe(&self) -> B1sebeR {
        B1sebeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 Instruction Prefetch Enable"]
    #[inline(always)]
    pub fn b1ipe(&self) -> B1ipeR {
        B1ipeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 Data Prefetch Enable"]
    #[inline(always)]
    pub fn b1dpe(&self) -> B1dpeR {
        B1dpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 Instruction Cache Enable"]
    #[inline(always)]
    pub fn b1ice(&self) -> B1iceR {
        B1iceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bank 1 Data Cache Enable"]
    #[inline(always)]
    pub fn b1dce(&self) -> B1dceR {
        B1dceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Bank 1 Memory Width"]
    #[inline(always)]
    pub fn b1mw(&self) -> B1mwR {
        B1mwR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Bank 1 Read Wait State Control"]
    #[inline(always)]
    pub fn b1rwsc(&self) -> B1rwscR {
        B1rwscR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 Single Entry Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1sebe(&mut self) -> B1sebeW<Pfb1crSpec> {
        B1sebeW::new(self, 0)
    }
    #[doc = "Bit 1 - Bank 1 Instruction Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1ipe(&mut self) -> B1ipeW<Pfb1crSpec> {
        B1ipeW::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 1 Data Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1dpe(&mut self) -> B1dpeW<Pfb1crSpec> {
        B1dpeW::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 1 Instruction Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1ice(&mut self) -> B1iceW<Pfb1crSpec> {
        B1iceW::new(self, 3)
    }
    #[doc = "Bit 4 - Bank 1 Data Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1dce(&mut self) -> B1dceW<Pfb1crSpec> {
        B1dceW::new(self, 4)
    }
}
#[doc = "Flash Bank 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfb1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfb1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfb1crSpec;
impl crate::RegisterSpec for Pfb1crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfb1cr::R`](R) reader structure"]
impl crate::Readable for Pfb1crSpec {}
#[doc = "`write(|w| ..)` method takes [`pfb1cr::W`](W) writer structure"]
impl crate::Writable for Pfb1crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFB1CR to value 0x3002_001f"]
impl crate::Resettable for Pfb1crSpec {
    const RESET_VALUE: u32 = 0x3002_001f;
}
