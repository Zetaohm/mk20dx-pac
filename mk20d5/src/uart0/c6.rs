#[doc = "Register `C6` reader"]
pub type R = crate::R<C6Spec>;
#[doc = "Register `C6` writer"]
pub type W = crate::W<C6Spec>;
#[doc = "Collision Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cp {
    #[doc = "0: Collision signal is active low."]
    _0 = 0,
    #[doc = "1: Collision signal is active high."]
    _1 = 1,
}
impl From<Cp> for bool {
    #[inline(always)]
    fn from(variant: Cp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CP` reader - Collision Signal Polarity"]
pub type CpR = crate::BitReader<Cp>;
impl CpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cp {
        match self.bits {
            false => Cp::_0,
            true => Cp::_1,
        }
    }
    #[doc = "Collision signal is active low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cp::_0
    }
    #[doc = "Collision signal is active high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cp::_1
    }
}
#[doc = "Field `CP` writer - Collision Signal Polarity"]
pub type CpW<'a, REG> = crate::BitWriter<'a, REG, Cp>;
impl<'a, REG> CpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Collision signal is active low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cp::_0)
    }
    #[doc = "Collision signal is active high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cp::_1)
    }
}
#[doc = "Collision Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ce {
    #[doc = "0: Collision detect feature is disabled."]
    _0 = 0,
    #[doc = "1: Collision detect feature is enabled."]
    _1 = 1,
}
impl From<Ce> for bool {
    #[inline(always)]
    fn from(variant: Ce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE` reader - Collision Enable"]
pub type CeR = crate::BitReader<Ce>;
impl CeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ce {
        match self.bits {
            false => Ce::_0,
            true => Ce::_1,
        }
    }
    #[doc = "Collision detect feature is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ce::_0
    }
    #[doc = "Collision detect feature is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ce::_1
    }
}
#[doc = "Field `CE` writer - Collision Enable"]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG, Ce>;
impl<'a, REG> CeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Collision detect feature is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ce::_0)
    }
    #[doc = "Collision detect feature is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ce::_1)
    }
}
#[doc = "CEA709.1-B Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx709 {
    #[doc = "0: CEA709.1-B transmitter is disabled."]
    _0 = 0,
    #[doc = "1: CEA709.1-B transmitter is enabled."]
    _1 = 1,
}
impl From<Tx709> for bool {
    #[inline(always)]
    fn from(variant: Tx709) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX709` reader - CEA709.1-B Transmit Enable"]
pub type Tx709R = crate::BitReader<Tx709>;
impl Tx709R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx709 {
        match self.bits {
            false => Tx709::_0,
            true => Tx709::_1,
        }
    }
    #[doc = "CEA709.1-B transmitter is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tx709::_0
    }
    #[doc = "CEA709.1-B transmitter is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tx709::_1
    }
}
#[doc = "Field `TX709` writer - CEA709.1-B Transmit Enable"]
pub type Tx709W<'a, REG> = crate::BitWriter<'a, REG, Tx709>;
impl<'a, REG> Tx709W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CEA709.1-B transmitter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tx709::_0)
    }
    #[doc = "CEA709.1-B transmitter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tx709::_1)
    }
}
#[doc = "EN709\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En709 {
    #[doc = "0: CEA709.1-B is disabled."]
    _0 = 0,
    #[doc = "1: CEA709.1-B is enabled"]
    _1 = 1,
}
impl From<En709> for bool {
    #[inline(always)]
    fn from(variant: En709) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN709` reader - EN709"]
pub type En709R = crate::BitReader<En709>;
impl En709R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En709 {
        match self.bits {
            false => En709::_0,
            true => En709::_1,
        }
    }
    #[doc = "CEA709.1-B is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En709::_0
    }
    #[doc = "CEA709.1-B is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En709::_1
    }
}
#[doc = "Field `EN709` writer - EN709"]
pub type En709W<'a, REG> = crate::BitWriter<'a, REG, En709>;
impl<'a, REG> En709W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CEA709.1-B is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En709::_0)
    }
    #[doc = "CEA709.1-B is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En709::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Collision Signal Polarity"]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Collision Enable"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CEA709.1-B Transmit Enable"]
    #[inline(always)]
    pub fn tx709(&self) -> Tx709R {
        Tx709R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EN709"]
    #[inline(always)]
    pub fn en709(&self) -> En709R {
        En709R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Collision Signal Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cp(&mut self) -> CpW<C6Spec> {
        CpW::new(self, 4)
    }
    #[doc = "Bit 5 - Collision Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CeW<C6Spec> {
        CeW::new(self, 5)
    }
    #[doc = "Bit 6 - CEA709.1-B Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx709(&mut self) -> Tx709W<C6Spec> {
        Tx709W::new(self, 6)
    }
    #[doc = "Bit 7 - EN709"]
    #[inline(always)]
    #[must_use]
    pub fn en709(&mut self) -> En709W<C6Spec> {
        En709W::new(self, 7)
    }
}
#[doc = "UART CEA709.1-B Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`c6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6Spec;
impl crate::RegisterSpec for C6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c6::R`](R) reader structure"]
impl crate::Readable for C6Spec {}
#[doc = "`write(|w| ..)` method takes [`c6::W`](W) writer structure"]
impl crate::Writable for C6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C6 to value 0"]
impl crate::Resettable for C6Spec {
    const RESET_VALUE: u8 = 0;
}
