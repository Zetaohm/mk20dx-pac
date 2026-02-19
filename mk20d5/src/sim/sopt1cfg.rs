#[doc = "Register `SOPT1CFG` reader"]
pub type R = crate::R<Sopt1cfgSpec>;
#[doc = "Register `SOPT1CFG` writer"]
pub type W = crate::W<Sopt1cfgSpec>;
#[doc = "USB voltage regulator enable write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Urwe {
    #[doc = "0: SOPT1 USBREGEN cannot be written."]
    _0 = 0,
    #[doc = "1: SOPT1 USBREGEN can be written."]
    _1 = 1,
}
impl From<Urwe> for bool {
    #[inline(always)]
    fn from(variant: Urwe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `URWE` reader - USB voltage regulator enable write enable"]
pub type UrweR = crate::BitReader<Urwe>;
impl UrweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Urwe {
        match self.bits {
            false => Urwe::_0,
            true => Urwe::_1,
        }
    }
    #[doc = "SOPT1 USBREGEN cannot be written."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Urwe::_0
    }
    #[doc = "SOPT1 USBREGEN can be written."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Urwe::_1
    }
}
#[doc = "Field `URWE` writer - USB voltage regulator enable write enable"]
pub type UrweW<'a, REG> = crate::BitWriter<'a, REG, Urwe>;
impl<'a, REG> UrweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOPT1 USBREGEN cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Urwe::_0)
    }
    #[doc = "SOPT1 USBREGEN can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Urwe::_1)
    }
}
#[doc = "USB voltage regulator VLP standby write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uvswe {
    #[doc = "0: SOPT1 USBVSTB cannot be written."]
    _0 = 0,
    #[doc = "1: SOPT1 USBVSTB can be written."]
    _1 = 1,
}
impl From<Uvswe> for bool {
    #[inline(always)]
    fn from(variant: Uvswe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVSWE` reader - USB voltage regulator VLP standby write enable"]
pub type UvsweR = crate::BitReader<Uvswe>;
impl UvsweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uvswe {
        match self.bits {
            false => Uvswe::_0,
            true => Uvswe::_1,
        }
    }
    #[doc = "SOPT1 USBVSTB cannot be written."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uvswe::_0
    }
    #[doc = "SOPT1 USBVSTB can be written."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uvswe::_1
    }
}
#[doc = "Field `UVSWE` writer - USB voltage regulator VLP standby write enable"]
pub type UvsweW<'a, REG> = crate::BitWriter<'a, REG, Uvswe>;
impl<'a, REG> UvsweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOPT1 USBVSTB cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uvswe::_0)
    }
    #[doc = "SOPT1 USBVSTB can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uvswe::_1)
    }
}
#[doc = "USB voltage regulator stop standby write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usswe {
    #[doc = "0: SOPT1 USBSSTB cannot be written."]
    _0 = 0,
    #[doc = "1: SOPT1 USBSSTB can be written."]
    _1 = 1,
}
impl From<Usswe> for bool {
    #[inline(always)]
    fn from(variant: Usswe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USSWE` reader - USB voltage regulator stop standby write enable"]
pub type UssweR = crate::BitReader<Usswe>;
impl UssweR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usswe {
        match self.bits {
            false => Usswe::_0,
            true => Usswe::_1,
        }
    }
    #[doc = "SOPT1 USBSSTB cannot be written."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usswe::_0
    }
    #[doc = "SOPT1 USBSSTB can be written."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usswe::_1
    }
}
#[doc = "Field `USSWE` writer - USB voltage regulator stop standby write enable"]
pub type UssweW<'a, REG> = crate::BitWriter<'a, REG, Usswe>;
impl<'a, REG> UssweW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOPT1 USBSSTB cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usswe::_0)
    }
    #[doc = "SOPT1 USBSSTB can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usswe::_1)
    }
}
impl R {
    #[doc = "Bit 24 - USB voltage regulator enable write enable"]
    #[inline(always)]
    pub fn urwe(&self) -> UrweR {
        UrweR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USB voltage regulator VLP standby write enable"]
    #[inline(always)]
    pub fn uvswe(&self) -> UvsweR {
        UvsweR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB voltage regulator stop standby write enable"]
    #[inline(always)]
    pub fn usswe(&self) -> UssweR {
        UssweR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USB voltage regulator enable write enable"]
    #[inline(always)]
    #[must_use]
    pub fn urwe(&mut self) -> UrweW<Sopt1cfgSpec> {
        UrweW::new(self, 24)
    }
    #[doc = "Bit 25 - USB voltage regulator VLP standby write enable"]
    #[inline(always)]
    #[must_use]
    pub fn uvswe(&mut self) -> UvsweW<Sopt1cfgSpec> {
        UvsweW::new(self, 25)
    }
    #[doc = "Bit 26 - USB voltage regulator stop standby write enable"]
    #[inline(always)]
    #[must_use]
    pub fn usswe(&mut self) -> UssweW<Sopt1cfgSpec> {
        UssweW::new(self, 26)
    }
}
#[doc = "SOPT1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt1cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt1cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt1cfgSpec;
impl crate::RegisterSpec for Sopt1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt1cfg::R`](R) reader structure"]
impl crate::Readable for Sopt1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sopt1cfg::W`](W) writer structure"]
impl crate::Writable for Sopt1cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT1CFG to value 0"]
impl crate::Resettable for Sopt1cfgSpec {
    const RESET_VALUE: u32 = 0;
}
