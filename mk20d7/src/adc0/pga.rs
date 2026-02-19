#[doc = "Register `PGA` reader"]
pub type R = crate::R<PgaSpec>;
#[doc = "Register `PGA` writer"]
pub type W = crate::W<PgaSpec>;
#[doc = "PGA gain setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pgag {
    #[doc = "0: 1"]
    _0000 = 0,
    #[doc = "1: 2"]
    _0001 = 1,
    #[doc = "2: 4"]
    _0010 = 2,
    #[doc = "3: 8"]
    _0011 = 3,
    #[doc = "4: 16"]
    _0100 = 4,
    #[doc = "5: 32"]
    _0101 = 5,
    #[doc = "6: 64"]
    _0110 = 6,
}
impl From<Pgag> for u8 {
    #[inline(always)]
    fn from(variant: Pgag) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pgag {
    type Ux = u8;
}
impl crate::IsEnum for Pgag {}
#[doc = "Field `PGAG` reader - PGA gain setting"]
pub type PgagR = crate::FieldReader<Pgag>;
impl PgagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pgag> {
        match self.bits {
            0 => Some(Pgag::_0000),
            1 => Some(Pgag::_0001),
            2 => Some(Pgag::_0010),
            3 => Some(Pgag::_0011),
            4 => Some(Pgag::_0100),
            5 => Some(Pgag::_0101),
            6 => Some(Pgag::_0110),
            _ => None,
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Pgag::_0000
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Pgag::_0001
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Pgag::_0010
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Pgag::_0011
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Pgag::_0100
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Pgag::_0101
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Pgag::_0110
    }
}
#[doc = "Field `PGAG` writer - PGA gain setting"]
pub type PgagW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pgag>;
impl<'a, REG> PgagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Pgag::_0000)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Pgag::_0001)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Pgag::_0010)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Pgag::_0011)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Pgag::_0100)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Pgag::_0101)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Pgag::_0110)
    }
}
#[doc = "PGA low-power mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pgalpb {
    #[doc = "0: PGA runs in low power mode."]
    _0 = 0,
    #[doc = "1: PGA runs in normal power mode."]
    _1 = 1,
}
impl From<Pgalpb> for bool {
    #[inline(always)]
    fn from(variant: Pgalpb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGALPb` reader - PGA low-power mode control"]
pub type PgalpbR = crate::BitReader<Pgalpb>;
impl PgalpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgalpb {
        match self.bits {
            false => Pgalpb::_0,
            true => Pgalpb::_1,
        }
    }
    #[doc = "PGA runs in low power mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pgalpb::_0
    }
    #[doc = "PGA runs in normal power mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pgalpb::_1
    }
}
#[doc = "Field `PGALPb` writer - PGA low-power mode control"]
pub type PgalpbW<'a, REG> = crate::BitWriter<'a, REG, Pgalpb>;
impl<'a, REG> PgalpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PGA runs in low power mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgalpb::_0)
    }
    #[doc = "PGA runs in normal power mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgalpb::_1)
    }
}
#[doc = "PGA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pgaen {
    #[doc = "0: PGA disabled."]
    _0 = 0,
    #[doc = "1: PGA enabled."]
    _1 = 1,
}
impl From<Pgaen> for bool {
    #[inline(always)]
    fn from(variant: Pgaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAEN` reader - PGA enable"]
pub type PgaenR = crate::BitReader<Pgaen>;
impl PgaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgaen {
        match self.bits {
            false => Pgaen::_0,
            true => Pgaen::_1,
        }
    }
    #[doc = "PGA disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pgaen::_0
    }
    #[doc = "PGA enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pgaen::_1
    }
}
#[doc = "Field `PGAEN` writer - PGA enable"]
pub type PgaenW<'a, REG> = crate::BitWriter<'a, REG, Pgaen>;
impl<'a, REG> PgaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PGA disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pgaen::_0)
    }
    #[doc = "PGA enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pgaen::_1)
    }
}
impl R {
    #[doc = "Bits 16:19 - PGA gain setting"]
    #[inline(always)]
    pub fn pgag(&self) -> PgagR {
        PgagR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - PGA low-power mode control"]
    #[inline(always)]
    pub fn pgalpb(&self) -> PgalpbR {
        PgalpbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - PGA enable"]
    #[inline(always)]
    pub fn pgaen(&self) -> PgaenR {
        PgaenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - PGA gain setting"]
    #[inline(always)]
    #[must_use]
    pub fn pgag(&mut self) -> PgagW<PgaSpec> {
        PgagW::new(self, 16)
    }
    #[doc = "Bit 20 - PGA low-power mode control"]
    #[inline(always)]
    #[must_use]
    pub fn pgalpb(&mut self) -> PgalpbW<PgaSpec> {
        PgalpbW::new(self, 20)
    }
    #[doc = "Bit 23 - PGA enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgaen(&mut self) -> PgaenW<PgaSpec> {
        PgaenW::new(self, 23)
    }
}
#[doc = "ADC PGA register\n\nYou can [`read`](crate::Reg::read) this register and get [`pga::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pga::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgaSpec;
impl crate::RegisterSpec for PgaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pga::R`](R) reader structure"]
impl crate::Readable for PgaSpec {}
#[doc = "`write(|w| ..)` method takes [`pga::W`](W) writer structure"]
impl crate::Writable for PgaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PGA to value 0"]
impl crate::Resettable for PgaSpec {
    const RESET_VALUE: u32 = 0;
}
