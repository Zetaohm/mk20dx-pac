#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PsrSpec>;
#[doc = "Prescaler Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcs {
    #[doc = "0: Prescaler/glitch filter clock 0 selected"]
    _00 = 0,
    #[doc = "1: Prescaler/glitch filter clock 1 selected"]
    _01 = 1,
    #[doc = "2: Prescaler/glitch filter clock 2 selected"]
    _10 = 2,
    #[doc = "3: Prescaler/glitch filter clock 3 selected"]
    _11 = 3,
}
impl From<Pcs> for u8 {
    #[inline(always)]
    fn from(variant: Pcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcs {
    type Ux = u8;
}
impl crate::IsEnum for Pcs {}
#[doc = "Field `PCS` reader - Prescaler Clock Select"]
pub type PcsR = crate::FieldReader<Pcs>;
impl PcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcs {
        match self.bits {
            0 => Pcs::_00,
            1 => Pcs::_01,
            2 => Pcs::_10,
            3 => Pcs::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler/glitch filter clock 0 selected"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Pcs::_00
    }
    #[doc = "Prescaler/glitch filter clock 1 selected"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Pcs::_01
    }
    #[doc = "Prescaler/glitch filter clock 2 selected"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Pcs::_10
    }
    #[doc = "Prescaler/glitch filter clock 3 selected"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Pcs::_11
    }
}
#[doc = "Field `PCS` writer - Prescaler Clock Select"]
pub type PcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pcs, crate::Safe>;
impl<'a, REG> PcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler/glitch filter clock 0 selected"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::_00)
    }
    #[doc = "Prescaler/glitch filter clock 1 selected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::_01)
    }
    #[doc = "Prescaler/glitch filter clock 2 selected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::_10)
    }
    #[doc = "Prescaler/glitch filter clock 3 selected"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::_11)
    }
}
#[doc = "Prescaler Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbyp {
    #[doc = "0: Prescaler/Glitch Filter is enabled."]
    _0 = 0,
    #[doc = "1: Prescaler/Glitch Filter is bypassed."]
    _1 = 1,
}
impl From<Pbyp> for bool {
    #[inline(always)]
    fn from(variant: Pbyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBYP` reader - Prescaler Bypass"]
pub type PbypR = crate::BitReader<Pbyp>;
impl PbypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbyp {
        match self.bits {
            false => Pbyp::_0,
            true => Pbyp::_1,
        }
    }
    #[doc = "Prescaler/Glitch Filter is enabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pbyp::_0
    }
    #[doc = "Prescaler/Glitch Filter is bypassed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pbyp::_1
    }
}
#[doc = "Field `PBYP` writer - Prescaler Bypass"]
pub type PbypW<'a, REG> = crate::BitWriter<'a, REG, Pbyp>;
impl<'a, REG> PbypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaler/Glitch Filter is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pbyp::_0)
    }
    #[doc = "Prescaler/Glitch Filter is bypassed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pbyp::_1)
    }
}
#[doc = "Prescale Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescale {
    #[doc = "0: Prescaler divides the prescaler clock by 2; Glitch Filter does not support this configuration."]
    _0000 = 0,
    #[doc = "1: Prescaler divides the prescaler clock by 4; Glitch Filter recognizes change on input pin after 2 rising clock edges."]
    _0001 = 1,
    #[doc = "2: Prescaler divides the prescaler clock by 8; Glitch Filter recognizes change on input pin after 4 rising clock edges."]
    _0010 = 2,
    #[doc = "3: Prescaler divides the prescaler clock by 16; Glitch Filter recognizes change on input pin after 8 rising clock edges."]
    _0011 = 3,
    #[doc = "4: Prescaler divides the prescaler clock by 32; Glitch Filter recognizes change on input pin after 16 rising clock edges."]
    _0100 = 4,
    #[doc = "5: Prescaler divides the prescaler clock by 64; Glitch Filter recognizes change on input pin after 32 rising clock edges."]
    _0101 = 5,
    #[doc = "6: Prescaler divides the prescaler clock by 128; Glitch Filter recognizes change on input pin after 64 rising clock edges."]
    _0110 = 6,
    #[doc = "7: Prescaler divides the prescaler clock by 256; Glitch Filter recognizes change on input pin after 128 rising clock edges."]
    _0111 = 7,
    #[doc = "8: Prescaler divides the prescaler clock by 512; Glitch Filter recognizes change on input pin after 256 rising clock edges."]
    _1000 = 8,
    #[doc = "9: Prescaler divides the prescaler clock by 1024; Glitch Filter recognizes change on input pin after 512 rising clock edges."]
    _1001 = 9,
    #[doc = "10: Prescaler divides the prescaler clock by 2048; Glitch Filter recognizes change on input pin after 1024 rising clock edges."]
    _1010 = 10,
    #[doc = "11: Prescaler divides the prescaler clock by 4096; Glitch Filter recognizes change on input pin after 2048 rising clock edges."]
    _1011 = 11,
    #[doc = "12: Prescaler divides the prescaler clock by 8192; Glitch Filter recognizes change on input pin after 4096 rising clock edges."]
    _1100 = 12,
    #[doc = "13: Prescaler divides the prescaler clock by 16384; Glitch Filter recognizes change on input pin after 8192 rising clock edges."]
    _1101 = 13,
    #[doc = "14: Prescaler divides the prescaler clock by 32768; Glitch Filter recognizes change on input pin after 16384 rising clock edges."]
    _1110 = 14,
    #[doc = "15: Prescaler divides the prescaler clock by 65536; Glitch Filter recognizes change on input pin after 32768 rising clock edges."]
    _1111 = 15,
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(variant: Prescale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescale {
    type Ux = u8;
}
impl crate::IsEnum for Prescale {}
#[doc = "Field `PRESCALE` reader - Prescale Value"]
pub type PrescaleR = crate::FieldReader<Prescale>;
impl PrescaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescale {
        match self.bits {
            0 => Prescale::_0000,
            1 => Prescale::_0001,
            2 => Prescale::_0010,
            3 => Prescale::_0011,
            4 => Prescale::_0100,
            5 => Prescale::_0101,
            6 => Prescale::_0110,
            7 => Prescale::_0111,
            8 => Prescale::_1000,
            9 => Prescale::_1001,
            10 => Prescale::_1010,
            11 => Prescale::_1011,
            12 => Prescale::_1100,
            13 => Prescale::_1101,
            14 => Prescale::_1110,
            15 => Prescale::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler divides the prescaler clock by 2; Glitch Filter does not support this configuration."]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Prescale::_0000
    }
    #[doc = "Prescaler divides the prescaler clock by 4; Glitch Filter recognizes change on input pin after 2 rising clock edges."]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Prescale::_0001
    }
    #[doc = "Prescaler divides the prescaler clock by 8; Glitch Filter recognizes change on input pin after 4 rising clock edges."]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Prescale::_0010
    }
    #[doc = "Prescaler divides the prescaler clock by 16; Glitch Filter recognizes change on input pin after 8 rising clock edges."]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Prescale::_0011
    }
    #[doc = "Prescaler divides the prescaler clock by 32; Glitch Filter recognizes change on input pin after 16 rising clock edges."]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Prescale::_0100
    }
    #[doc = "Prescaler divides the prescaler clock by 64; Glitch Filter recognizes change on input pin after 32 rising clock edges."]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Prescale::_0101
    }
    #[doc = "Prescaler divides the prescaler clock by 128; Glitch Filter recognizes change on input pin after 64 rising clock edges."]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Prescale::_0110
    }
    #[doc = "Prescaler divides the prescaler clock by 256; Glitch Filter recognizes change on input pin after 128 rising clock edges."]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Prescale::_0111
    }
    #[doc = "Prescaler divides the prescaler clock by 512; Glitch Filter recognizes change on input pin after 256 rising clock edges."]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Prescale::_1000
    }
    #[doc = "Prescaler divides the prescaler clock by 1024; Glitch Filter recognizes change on input pin after 512 rising clock edges."]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Prescale::_1001
    }
    #[doc = "Prescaler divides the prescaler clock by 2048; Glitch Filter recognizes change on input pin after 1024 rising clock edges."]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Prescale::_1010
    }
    #[doc = "Prescaler divides the prescaler clock by 4096; Glitch Filter recognizes change on input pin after 2048 rising clock edges."]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Prescale::_1011
    }
    #[doc = "Prescaler divides the prescaler clock by 8192; Glitch Filter recognizes change on input pin after 4096 rising clock edges."]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Prescale::_1100
    }
    #[doc = "Prescaler divides the prescaler clock by 16384; Glitch Filter recognizes change on input pin after 8192 rising clock edges."]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == Prescale::_1101
    }
    #[doc = "Prescaler divides the prescaler clock by 32768; Glitch Filter recognizes change on input pin after 16384 rising clock edges."]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == Prescale::_1110
    }
    #[doc = "Prescaler divides the prescaler clock by 65536; Glitch Filter recognizes change on input pin after 32768 rising clock edges."]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == Prescale::_1111
    }
}
#[doc = "Field `PRESCALE` writer - Prescale Value"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 4, Prescale, crate::Safe>;
impl<'a, REG> PrescaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler divides the prescaler clock by 2; Glitch Filter does not support this configuration."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_0000)
    }
    #[doc = "Prescaler divides the prescaler clock by 4; Glitch Filter recognizes change on input pin after 2 rising clock edges."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_0001)
    }
    #[doc = "Prescaler divides the prescaler clock by 8; Glitch Filter recognizes change on input pin after 4 rising clock edges."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_0010)
    }
    #[doc = "Prescaler divides the prescaler clock by 16; Glitch Filter recognizes change on input pin after 8 rising clock edges."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_0011)
    }
    #[doc = "Prescaler divides the prescaler clock by 32; Glitch Filter recognizes change on input pin after 16 rising clock edges."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_0100)
    }
    #[doc = "Prescaler divides the prescaler clock by 64; Glitch Filter recognizes change on input pin after 32 rising clock edges."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_0101)
    }
    #[doc = "Prescaler divides the prescaler clock by 128; Glitch Filter recognizes change on input pin after 64 rising clock edges."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_0110)
    }
    #[doc = "Prescaler divides the prescaler clock by 256; Glitch Filter recognizes change on input pin after 128 rising clock edges."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_0111)
    }
    #[doc = "Prescaler divides the prescaler clock by 512; Glitch Filter recognizes change on input pin after 256 rising clock edges."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1000)
    }
    #[doc = "Prescaler divides the prescaler clock by 1024; Glitch Filter recognizes change on input pin after 512 rising clock edges."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1001)
    }
    #[doc = "Prescaler divides the prescaler clock by 2048; Glitch Filter recognizes change on input pin after 1024 rising clock edges."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1010)
    }
    #[doc = "Prescaler divides the prescaler clock by 4096; Glitch Filter recognizes change on input pin after 2048 rising clock edges."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1011)
    }
    #[doc = "Prescaler divides the prescaler clock by 8192; Glitch Filter recognizes change on input pin after 4096 rising clock edges."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1100)
    }
    #[doc = "Prescaler divides the prescaler clock by 16384; Glitch Filter recognizes change on input pin after 8192 rising clock edges."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1101)
    }
    #[doc = "Prescaler divides the prescaler clock by 32768; Glitch Filter recognizes change on input pin after 16384 rising clock edges."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1110)
    }
    #[doc = "Prescaler divides the prescaler clock by 65536; Glitch Filter recognizes change on input pin after 32768 rising clock edges."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1111)
    }
}
impl R {
    #[doc = "Bits 0:1 - Prescaler Clock Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PcsR {
        PcsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Prescaler Bypass"]
    #[inline(always)]
    pub fn pbyp(&self) -> PbypR {
        PbypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Prescale Value"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaler Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PcsW<PsrSpec> {
        PcsW::new(self, 0)
    }
    #[doc = "Bit 2 - Prescaler Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn pbyp(&mut self) -> PbypW<PsrSpec> {
        PbypW::new(self, 2)
    }
    #[doc = "Bits 3:6 - Prescale Value"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PrescaleW<PsrSpec> {
        PrescaleW::new(self, 3)
    }
}
#[doc = "Low Power Timer Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0;
}
