#[doc = "Register `C5` reader"]
pub type R = crate::R<C5Spec>;
#[doc = "Register `C5` writer"]
pub type W = crate::W<C5Spec>;
#[doc = "Field `PRDIV0` reader - PLL External Reference Divider"]
pub type Prdiv0R = crate::FieldReader;
#[doc = "Field `PRDIV0` writer - PLL External Reference Divider"]
pub type Prdiv0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "PLL Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllsten0 {
    #[doc = "0: MCGPLLCLK is disabled in any of the Stop modes."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK is enabled if system is in Normal Stop mode."]
    _1 = 1,
}
impl From<Pllsten0> for bool {
    #[inline(always)]
    fn from(variant: Pllsten0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTEN0` reader - PLL Stop Enable"]
pub type Pllsten0R = crate::BitReader<Pllsten0>;
impl Pllsten0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsten0 {
        match self.bits {
            false => Pllsten0::_0,
            true => Pllsten0::_1,
        }
    }
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pllsten0::_0
    }
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pllsten0::_1
    }
}
#[doc = "Field `PLLSTEN0` writer - PLL Stop Enable"]
pub type Pllsten0W<'a, REG> = crate::BitWriter<'a, REG, Pllsten0>;
impl<'a, REG> Pllsten0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsten0::_0)
    }
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsten0::_1)
    }
}
#[doc = "PLL Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllclken0 {
    #[doc = "0: MCGPLLCLK is inactive."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK is active."]
    _1 = 1,
}
impl From<Pllclken0> for bool {
    #[inline(always)]
    fn from(variant: Pllclken0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLCLKEN0` reader - PLL Clock Enable"]
pub type Pllclken0R = crate::BitReader<Pllclken0>;
impl Pllclken0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllclken0 {
        match self.bits {
            false => Pllclken0::_0,
            true => Pllclken0::_1,
        }
    }
    #[doc = "MCGPLLCLK is inactive."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pllclken0::_0
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pllclken0::_1
    }
}
#[doc = "Field `PLLCLKEN0` writer - PLL Clock Enable"]
pub type Pllclken0W<'a, REG> = crate::BitWriter<'a, REG, Pllclken0>;
impl<'a, REG> Pllclken0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCGPLLCLK is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllclken0::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllclken0::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv0(&self) -> Prdiv0R {
        Prdiv0R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten0(&self) -> Pllsten0R {
        Pllsten0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken0(&self) -> Pllclken0R {
        Pllclken0R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline(always)]
    #[must_use]
    pub fn prdiv0(&mut self) -> Prdiv0W<C5Spec> {
        Prdiv0W::new(self, 0)
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsten0(&mut self) -> Pllsten0W<C5Spec> {
        Pllsten0W::new(self, 5)
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllclken0(&mut self) -> Pllclken0W<C5Spec> {
        Pllclken0W::new(self, 6)
    }
}
#[doc = "MCG Control 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5Spec;
impl crate::RegisterSpec for C5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c5::R`](R) reader structure"]
impl crate::Readable for C5Spec {}
#[doc = "`write(|w| ..)` method takes [`c5::W`](W) writer structure"]
impl crate::Writable for C5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C5 to value 0"]
impl crate::Resettable for C5Spec {
    const RESET_VALUE: u8 = 0;
}
