#[doc = "Register `LVDSC1` reader"]
pub type R = crate::R<Lvdsc1Spec>;
#[doc = "Register `LVDSC1` writer"]
pub type W = crate::W<Lvdsc1Spec>;
#[doc = "Low-Voltage Detect Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvdv {
    #[doc = "0: Low trip point selected (V LVD = V LVDL )"]
    _00 = 0,
    #[doc = "1: High trip point selected (V LVD = V LVDH )"]
    _01 = 1,
}
impl From<Lvdv> for u8 {
    #[inline(always)]
    fn from(variant: Lvdv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvdv {
    type Ux = u8;
}
impl crate::IsEnum for Lvdv {}
#[doc = "Field `LVDV` reader - Low-Voltage Detect Voltage Select"]
pub type LvdvR = crate::FieldReader<Lvdv>;
impl LvdvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lvdv> {
        match self.bits {
            0 => Some(Lvdv::_00),
            1 => Some(Lvdv::_01),
            _ => None,
        }
    }
    #[doc = "Low trip point selected (V LVD = V LVDL )"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Lvdv::_00
    }
    #[doc = "High trip point selected (V LVD = V LVDH )"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Lvdv::_01
    }
}
#[doc = "Field `LVDV` writer - Low-Voltage Detect Voltage Select"]
pub type LvdvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lvdv>;
impl<'a, REG> LvdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low trip point selected (V LVD = V LVDL )"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdv::_00)
    }
    #[doc = "High trip point selected (V LVD = V LVDH )"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdv::_01)
    }
}
#[doc = "Low-Voltage Detect Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvdre {
    #[doc = "0: LVDF does not generate hardware resets"]
    _0 = 0,
    #[doc = "1: Force an MCU reset when LVDF = 1"]
    _1 = 1,
}
impl From<Lvdre> for bool {
    #[inline(always)]
    fn from(variant: Lvdre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDRE` reader - Low-Voltage Detect Reset Enable"]
pub type LvdreR = crate::BitReader<Lvdre>;
impl LvdreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdre {
        match self.bits {
            false => Lvdre::_0,
            true => Lvdre::_1,
        }
    }
    #[doc = "LVDF does not generate hardware resets"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvdre::_0
    }
    #[doc = "Force an MCU reset when LVDF = 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvdre::_1
    }
}
#[doc = "Field `LVDRE` writer - Low-Voltage Detect Reset Enable"]
pub type LvdreW<'a, REG> = crate::BitWriter<'a, REG, Lvdre>;
impl<'a, REG> LvdreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LVDF does not generate hardware resets"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdre::_0)
    }
    #[doc = "Force an MCU reset when LVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdre::_1)
    }
}
#[doc = "Low-Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvdie {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0 = 0,
    #[doc = "1: Request a hardware interrupt when LVDF = 1."]
    _1 = 1,
}
impl From<Lvdie> for bool {
    #[inline(always)]
    fn from(variant: Lvdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDIE` reader - Low-Voltage Detect Interrupt Enable"]
pub type LvdieR = crate::BitReader<Lvdie>;
impl LvdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdie {
        match self.bits {
            false => Lvdie::_0,
            true => Lvdie::_1,
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvdie::_0
    }
    #[doc = "Request a hardware interrupt when LVDF = 1."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvdie::_1
    }
}
#[doc = "Field `LVDIE` writer - Low-Voltage Detect Interrupt Enable"]
pub type LvdieW<'a, REG> = crate::BitWriter<'a, REG, Lvdie>;
impl<'a, REG> LvdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdie::_0)
    }
    #[doc = "Request a hardware interrupt when LVDF = 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdie::_1)
    }
}
#[doc = "Field `LVDACK` writer - Low-Voltage Detect Acknowledge"]
pub type LvdackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low-Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvdf {
    #[doc = "0: Low-voltage event not detected"]
    _0 = 0,
    #[doc = "1: Low-voltage event detected"]
    _1 = 1,
}
impl From<Lvdf> for bool {
    #[inline(always)]
    fn from(variant: Lvdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDF` reader - Low-Voltage Detect Flag"]
pub type LvdfR = crate::BitReader<Lvdf>;
impl LvdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdf {
        match self.bits {
            false => Lvdf::_0,
            true => Lvdf::_1,
        }
    }
    #[doc = "Low-voltage event not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvdf::_0
    }
    #[doc = "Low-voltage event detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvdf::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-Voltage Detect Voltage Select"]
    #[inline(always)]
    pub fn lvdv(&self) -> LvdvR {
        LvdvR::new(self.bits & 3)
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&self) -> LvdreR {
        LvdreR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lvdie(&self) -> LvdieR {
        LvdieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Detect Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LvdfR {
        LvdfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-Voltage Detect Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn lvdv(&mut self) -> LvdvW<Lvdsc1Spec> {
        LvdvW::new(self, 0)
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvdre(&mut self) -> LvdreW<Lvdsc1Spec> {
        LvdreW::new(self, 4)
    }
    #[doc = "Bit 5 - Low-Voltage Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvdie(&mut self) -> LvdieW<Lvdsc1Spec> {
        LvdieW::new(self, 5)
    }
    #[doc = "Bit 6 - Low-Voltage Detect Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn lvdack(&mut self) -> LvdackW<Lvdsc1Spec> {
        LvdackW::new(self, 6)
    }
}
#[doc = "Low Voltage Detect Status and Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdsc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lvdsc1Spec;
impl crate::RegisterSpec for Lvdsc1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdsc1::R`](R) reader structure"]
impl crate::Readable for Lvdsc1Spec {}
#[doc = "`write(|w| ..)` method takes [`lvdsc1::W`](W) writer structure"]
impl crate::Writable for Lvdsc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LVDSC1 to value 0x10"]
impl crate::Resettable for Lvdsc1Spec {
    const RESET_VALUE: u8 = 0x10;
}
