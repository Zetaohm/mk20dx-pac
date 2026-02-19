#[doc = "Register `PFB0CR` reader"]
pub type R = crate::R<Pfb0crSpec>;
#[doc = "Register `PFB0CR` writer"]
pub type W = crate::W<Pfb0crSpec>;
#[doc = "Bank 0 Single Entry Buffer Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0sebe {
    #[doc = "0: Single entry buffer is disabled."]
    _0 = 0,
    #[doc = "1: Single entry buffer is enabled."]
    _1 = 1,
}
impl From<B0sebe> for bool {
    #[inline(always)]
    fn from(variant: B0sebe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B0SEBE` reader - Bank 0 Single Entry Buffer Enable"]
pub type B0sebeR = crate::BitReader<B0sebe>;
impl B0sebeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B0sebe {
        match self.bits {
            false => B0sebe::_0,
            true => B0sebe::_1,
        }
    }
    #[doc = "Single entry buffer is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0sebe::_0
    }
    #[doc = "Single entry buffer is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0sebe::_1
    }
}
#[doc = "Field `B0SEBE` writer - Bank 0 Single Entry Buffer Enable"]
pub type B0sebeW<'a, REG> = crate::BitWriter<'a, REG, B0sebe>;
impl<'a, REG> B0sebeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single entry buffer is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B0sebe::_0)
    }
    #[doc = "Single entry buffer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B0sebe::_1)
    }
}
#[doc = "Bank 0 Instruction Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0ipe {
    #[doc = "0: Do not prefetch in response to instruction fetches."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to instruction fetches."]
    _1 = 1,
}
impl From<B0ipe> for bool {
    #[inline(always)]
    fn from(variant: B0ipe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B0IPE` reader - Bank 0 Instruction Prefetch Enable"]
pub type B0ipeR = crate::BitReader<B0ipe>;
impl B0ipeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B0ipe {
        match self.bits {
            false => B0ipe::_0,
            true => B0ipe::_1,
        }
    }
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0ipe::_0
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0ipe::_1
    }
}
#[doc = "Field `B0IPE` writer - Bank 0 Instruction Prefetch Enable"]
pub type B0ipeW<'a, REG> = crate::BitWriter<'a, REG, B0ipe>;
impl<'a, REG> B0ipeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B0ipe::_0)
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B0ipe::_1)
    }
}
#[doc = "Bank 0 Data Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0dpe {
    #[doc = "0: Do not prefetch in response to data references."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to data references."]
    _1 = 1,
}
impl From<B0dpe> for bool {
    #[inline(always)]
    fn from(variant: B0dpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B0DPE` reader - Bank 0 Data Prefetch Enable"]
pub type B0dpeR = crate::BitReader<B0dpe>;
impl B0dpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B0dpe {
        match self.bits {
            false => B0dpe::_0,
            true => B0dpe::_1,
        }
    }
    #[doc = "Do not prefetch in response to data references."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0dpe::_0
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0dpe::_1
    }
}
#[doc = "Field `B0DPE` writer - Bank 0 Data Prefetch Enable"]
pub type B0dpeW<'a, REG> = crate::BitWriter<'a, REG, B0dpe>;
impl<'a, REG> B0dpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not prefetch in response to data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B0dpe::_0)
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B0dpe::_1)
    }
}
#[doc = "Bank 0 Instruction Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0ice {
    #[doc = "0: Do not cache instruction fetches."]
    _0 = 0,
    #[doc = "1: Cache instruction fetches."]
    _1 = 1,
}
impl From<B0ice> for bool {
    #[inline(always)]
    fn from(variant: B0ice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B0ICE` reader - Bank 0 Instruction Cache Enable"]
pub type B0iceR = crate::BitReader<B0ice>;
impl B0iceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B0ice {
        match self.bits {
            false => B0ice::_0,
            true => B0ice::_1,
        }
    }
    #[doc = "Do not cache instruction fetches."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0ice::_0
    }
    #[doc = "Cache instruction fetches."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0ice::_1
    }
}
#[doc = "Field `B0ICE` writer - Bank 0 Instruction Cache Enable"]
pub type B0iceW<'a, REG> = crate::BitWriter<'a, REG, B0ice>;
impl<'a, REG> B0iceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not cache instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B0ice::_0)
    }
    #[doc = "Cache instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B0ice::_1)
    }
}
#[doc = "Bank 0 Data Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0dce {
    #[doc = "0: Do not cache data references."]
    _0 = 0,
    #[doc = "1: Cache data references."]
    _1 = 1,
}
impl From<B0dce> for bool {
    #[inline(always)]
    fn from(variant: B0dce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B0DCE` reader - Bank 0 Data Cache Enable"]
pub type B0dceR = crate::BitReader<B0dce>;
impl B0dceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B0dce {
        match self.bits {
            false => B0dce::_0,
            true => B0dce::_1,
        }
    }
    #[doc = "Do not cache data references."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0dce::_0
    }
    #[doc = "Cache data references."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0dce::_1
    }
}
#[doc = "Field `B0DCE` writer - Bank 0 Data Cache Enable"]
pub type B0dceW<'a, REG> = crate::BitWriter<'a, REG, B0dce>;
impl<'a, REG> B0dceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not cache data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(B0dce::_0)
    }
    #[doc = "Cache data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(B0dce::_1)
    }
}
#[doc = "Cache Replacement Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crc {
    #[doc = "0: LRU replacement algorithm per set across all four ways"]
    _000 = 0,
    #[doc = "2: Independent LRU with ways \\[0-1\\]
for ifetches, \\[2-3\\]
for data"]
    _010 = 2,
    #[doc = "3: Independent LRU with ways \\[0-2\\]
for ifetches, \\[3\\]
for data"]
    _011 = 3,
}
impl From<Crc> for u8 {
    #[inline(always)]
    fn from(variant: Crc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crc {
    type Ux = u8;
}
impl crate::IsEnum for Crc {}
#[doc = "Field `CRC` reader - Cache Replacement Control"]
pub type CrcR = crate::FieldReader<Crc>;
impl CrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Crc> {
        match self.bits {
            0 => Some(Crc::_000),
            2 => Some(Crc::_010),
            3 => Some(Crc::_011),
            _ => None,
        }
    }
    #[doc = "LRU replacement algorithm per set across all four ways"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Crc::_000
    }
    #[doc = "Independent LRU with ways \\[0-1\\]
for ifetches, \\[2-3\\]
for data"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Crc::_010
    }
    #[doc = "Independent LRU with ways \\[0-2\\]
for ifetches, \\[3\\]
for data"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Crc::_011
    }
}
#[doc = "Field `CRC` writer - Cache Replacement Control"]
pub type CrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Crc>;
impl<'a, REG> CrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LRU replacement algorithm per set across all four ways"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Crc::_000)
    }
    #[doc = "Independent LRU with ways \\[0-1\\]
for ifetches, \\[2-3\\]
for data"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Crc::_010)
    }
    #[doc = "Independent LRU with ways \\[0-2\\]
for ifetches, \\[3\\]
for data"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Crc::_011)
    }
}
#[doc = "Bank 0 Memory Width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum B0mw {
    #[doc = "0: 32 bits"]
    _00 = 0,
    #[doc = "1: 64 bits"]
    _01 = 1,
}
impl From<B0mw> for u8 {
    #[inline(always)]
    fn from(variant: B0mw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for B0mw {
    type Ux = u8;
}
impl crate::IsEnum for B0mw {}
#[doc = "Field `B0MW` reader - Bank 0 Memory Width"]
pub type B0mwR = crate::FieldReader<B0mw>;
impl B0mwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<B0mw> {
        match self.bits {
            0 => Some(B0mw::_00),
            1 => Some(B0mw::_01),
            _ => None,
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == B0mw::_00
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == B0mw::_01
    }
}
#[doc = "Invalidate Prefetch Speculation Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBInv {
    #[doc = "0: Speculation buffer and single entry buffer are not affected."]
    _0 = 0,
    #[doc = "1: Invalidate (clear) speculation buffer and single entry buffer."]
    _1 = 1,
}
impl From<SBInv> for bool {
    #[inline(always)]
    fn from(variant: SBInv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_B_INV` writer - Invalidate Prefetch Speculation Buffer"]
pub type SBInvW<'a, REG> = crate::BitWriter<'a, REG, SBInv>;
impl<'a, REG> SBInvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Speculation buffer and single entry buffer are not affected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SBInv::_0)
    }
    #[doc = "Invalidate (clear) speculation buffer and single entry buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SBInv::_1)
    }
}
#[doc = "Cache Invalidate Way x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CinvWay {
    #[doc = "0: No cache way invalidation for the corresponding cache"]
    _0 = 0,
    #[doc = "1: Invalidate cache way for the corresponding cache: clear the tag, data, and vld bits of ways selected"]
    _1 = 1,
}
impl From<CinvWay> for u8 {
    #[inline(always)]
    fn from(variant: CinvWay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CinvWay {
    type Ux = u8;
}
impl crate::IsEnum for CinvWay {}
#[doc = "Field `CINV_WAY` writer - Cache Invalidate Way x"]
pub type CinvWayW<'a, REG> = crate::FieldWriter<'a, REG, 4, CinvWay>;
impl<'a, REG> CinvWayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No cache way invalidation for the corresponding cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CinvWay::_0)
    }
    #[doc = "Invalidate cache way for the corresponding cache: clear the tag, data, and vld bits of ways selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CinvWay::_1)
    }
}
#[doc = "Cache Lock Way x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClckWay {
    #[doc = "0: Cache way is unlocked and may be displaced"]
    _0 = 0,
    #[doc = "1: Cache way is locked and its contents are not displaced"]
    _1 = 1,
}
impl From<ClckWay> for u8 {
    #[inline(always)]
    fn from(variant: ClckWay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClckWay {
    type Ux = u8;
}
impl crate::IsEnum for ClckWay {}
#[doc = "Field `CLCK_WAY` reader - Cache Lock Way x"]
pub type ClckWayR = crate::FieldReader<ClckWay>;
impl ClckWayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClckWay> {
        match self.bits {
            0 => Some(ClckWay::_0),
            1 => Some(ClckWay::_1),
            _ => None,
        }
    }
    #[doc = "Cache way is unlocked and may be displaced"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ClckWay::_0
    }
    #[doc = "Cache way is locked and its contents are not displaced"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ClckWay::_1
    }
}
#[doc = "Field `CLCK_WAY` writer - Cache Lock Way x"]
pub type ClckWayW<'a, REG> = crate::FieldWriter<'a, REG, 4, ClckWay>;
impl<'a, REG> ClckWayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cache way is unlocked and may be displaced"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ClckWay::_0)
    }
    #[doc = "Cache way is locked and its contents are not displaced"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ClckWay::_1)
    }
}
#[doc = "Field `B0RWSC` reader - Bank 0 Read Wait State Control"]
pub type B0rwscR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Bank 0 Single Entry Buffer Enable"]
    #[inline(always)]
    pub fn b0sebe(&self) -> B0sebeR {
        B0sebeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 0 Instruction Prefetch Enable"]
    #[inline(always)]
    pub fn b0ipe(&self) -> B0ipeR {
        B0ipeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 0 Data Prefetch Enable"]
    #[inline(always)]
    pub fn b0dpe(&self) -> B0dpeR {
        B0dpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 0 Instruction Cache Enable"]
    #[inline(always)]
    pub fn b0ice(&self) -> B0iceR {
        B0iceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bank 0 Data Cache Enable"]
    #[inline(always)]
    pub fn b0dce(&self) -> B0dceR {
        B0dceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Cache Replacement Control"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Bank 0 Memory Width"]
    #[inline(always)]
    pub fn b0mw(&self) -> B0mwR {
        B0mwR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Cache Lock Way x"]
    #[inline(always)]
    pub fn clck_way(&self) -> ClckWayR {
        ClckWayR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Bank 0 Read Wait State Control"]
    #[inline(always)]
    pub fn b0rwsc(&self) -> B0rwscR {
        B0rwscR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 0 Single Entry Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0sebe(&mut self) -> B0sebeW<Pfb0crSpec> {
        B0sebeW::new(self, 0)
    }
    #[doc = "Bit 1 - Bank 0 Instruction Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0ipe(&mut self) -> B0ipeW<Pfb0crSpec> {
        B0ipeW::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 0 Data Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0dpe(&mut self) -> B0dpeW<Pfb0crSpec> {
        B0dpeW::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 0 Instruction Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0ice(&mut self) -> B0iceW<Pfb0crSpec> {
        B0iceW::new(self, 3)
    }
    #[doc = "Bit 4 - Bank 0 Data Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0dce(&mut self) -> B0dceW<Pfb0crSpec> {
        B0dceW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Cache Replacement Control"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<Pfb0crSpec> {
        CrcW::new(self, 5)
    }
    #[doc = "Bit 19 - Invalidate Prefetch Speculation Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn s_b_inv(&mut self) -> SBInvW<Pfb0crSpec> {
        SBInvW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Cache Invalidate Way x"]
    #[inline(always)]
    #[must_use]
    pub fn cinv_way(&mut self) -> CinvWayW<Pfb0crSpec> {
        CinvWayW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Cache Lock Way x"]
    #[inline(always)]
    #[must_use]
    pub fn clck_way(&mut self) -> ClckWayW<Pfb0crSpec> {
        ClckWayW::new(self, 24)
    }
}
#[doc = "Flash Bank 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfb0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfb0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfb0crSpec;
impl crate::RegisterSpec for Pfb0crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfb0cr::R`](R) reader structure"]
impl crate::Readable for Pfb0crSpec {}
#[doc = "`write(|w| ..)` method takes [`pfb0cr::W`](W) writer structure"]
impl crate::Writable for Pfb0crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFB0CR to value 0x3002_001f"]
impl crate::Resettable for Pfb0crSpec {
    const RESET_VALUE: u32 = 0x3002_001f;
}
