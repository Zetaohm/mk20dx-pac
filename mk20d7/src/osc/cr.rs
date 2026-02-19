#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Oscillator 16 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc16p {
    #[doc = "0: Disable the selection."]
    _0 = 0,
    #[doc = "1: Add 16 pF capacitor to the oscillator load."]
    _1 = 1,
}
impl From<Sc16p> for bool {
    #[inline(always)]
    fn from(variant: Sc16p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC16P` reader - Oscillator 16 pF Capacitor Load Configure"]
pub type Sc16pR = crate::BitReader<Sc16p>;
impl Sc16pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc16p {
        match self.bits {
            false => Sc16p::_0,
            true => Sc16p::_1,
        }
    }
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sc16p::_0
    }
    #[doc = "Add 16 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sc16p::_1
    }
}
#[doc = "Field `SC16P` writer - Oscillator 16 pF Capacitor Load Configure"]
pub type Sc16pW<'a, REG> = crate::BitWriter<'a, REG, Sc16p>;
impl<'a, REG> Sc16pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sc16p::_0)
    }
    #[doc = "Add 16 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sc16p::_1)
    }
}
#[doc = "Oscillator 8 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc8p {
    #[doc = "0: Disable the selection."]
    _0 = 0,
    #[doc = "1: Add 8 pF capacitor to the oscillator load."]
    _1 = 1,
}
impl From<Sc8p> for bool {
    #[inline(always)]
    fn from(variant: Sc8p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC8P` reader - Oscillator 8 pF Capacitor Load Configure"]
pub type Sc8pR = crate::BitReader<Sc8p>;
impl Sc8pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc8p {
        match self.bits {
            false => Sc8p::_0,
            true => Sc8p::_1,
        }
    }
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sc8p::_0
    }
    #[doc = "Add 8 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sc8p::_1
    }
}
#[doc = "Field `SC8P` writer - Oscillator 8 pF Capacitor Load Configure"]
pub type Sc8pW<'a, REG> = crate::BitWriter<'a, REG, Sc8p>;
impl<'a, REG> Sc8pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sc8p::_0)
    }
    #[doc = "Add 8 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sc8p::_1)
    }
}
#[doc = "Oscillator 4 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc4p {
    #[doc = "0: Disable the selection."]
    _0 = 0,
    #[doc = "1: Add 4 pF capacitor to the oscillator load."]
    _1 = 1,
}
impl From<Sc4p> for bool {
    #[inline(always)]
    fn from(variant: Sc4p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC4P` reader - Oscillator 4 pF Capacitor Load Configure"]
pub type Sc4pR = crate::BitReader<Sc4p>;
impl Sc4pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc4p {
        match self.bits {
            false => Sc4p::_0,
            true => Sc4p::_1,
        }
    }
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sc4p::_0
    }
    #[doc = "Add 4 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sc4p::_1
    }
}
#[doc = "Field `SC4P` writer - Oscillator 4 pF Capacitor Load Configure"]
pub type Sc4pW<'a, REG> = crate::BitWriter<'a, REG, Sc4p>;
impl<'a, REG> Sc4pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sc4p::_0)
    }
    #[doc = "Add 4 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sc4p::_1)
    }
}
#[doc = "Oscillator 2 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc2p {
    #[doc = "0: Disable the selection."]
    _0 = 0,
    #[doc = "1: Add 2 pF capacitor to the oscillator load."]
    _1 = 1,
}
impl From<Sc2p> for bool {
    #[inline(always)]
    fn from(variant: Sc2p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC2P` reader - Oscillator 2 pF Capacitor Load Configure"]
pub type Sc2pR = crate::BitReader<Sc2p>;
impl Sc2pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc2p {
        match self.bits {
            false => Sc2p::_0,
            true => Sc2p::_1,
        }
    }
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sc2p::_0
    }
    #[doc = "Add 2 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sc2p::_1
    }
}
#[doc = "Field `SC2P` writer - Oscillator 2 pF Capacitor Load Configure"]
pub type Sc2pW<'a, REG> = crate::BitWriter<'a, REG, Sc2p>;
impl<'a, REG> Sc2pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sc2p::_0)
    }
    #[doc = "Add 2 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sc2p::_1)
    }
}
#[doc = "External Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erefsten {
    #[doc = "0: External reference clock is disabled in Stop mode."]
    _0 = 0,
    #[doc = "1: External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    _1 = 1,
}
impl From<Erefsten> for bool {
    #[inline(always)]
    fn from(variant: Erefsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFSTEN` reader - External Reference Stop Enable"]
pub type ErefstenR = crate::BitReader<Erefsten>;
impl ErefstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erefsten {
        match self.bits {
            false => Erefsten::_0,
            true => Erefsten::_1,
        }
    }
    #[doc = "External reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Erefsten::_0
    }
    #[doc = "External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Erefsten::_1
    }
}
#[doc = "Field `EREFSTEN` writer - External Reference Stop Enable"]
pub type ErefstenW<'a, REG> = crate::BitWriter<'a, REG, Erefsten>;
impl<'a, REG> ErefstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Erefsten::_0)
    }
    #[doc = "External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Erefsten::_1)
    }
}
#[doc = "External Reference Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erclken {
    #[doc = "0: External reference clock is inactive."]
    _0 = 0,
    #[doc = "1: External reference clock is enabled."]
    _1 = 1,
}
impl From<Erclken> for bool {
    #[inline(always)]
    fn from(variant: Erclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERCLKEN` reader - External Reference Enable"]
pub type ErclkenR = crate::BitReader<Erclken>;
impl ErclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erclken {
        match self.bits {
            false => Erclken::_0,
            true => Erclken::_1,
        }
    }
    #[doc = "External reference clock is inactive."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Erclken::_0
    }
    #[doc = "External reference clock is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Erclken::_1
    }
}
#[doc = "Field `ERCLKEN` writer - External Reference Enable"]
pub type ErclkenW<'a, REG> = crate::BitWriter<'a, REG, Erclken>;
impl<'a, REG> ErclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External reference clock is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Erclken::_0)
    }
    #[doc = "External reference clock is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Erclken::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Oscillator 16 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc16p(&self) -> Sc16pR {
        Sc16pR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oscillator 8 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc8p(&self) -> Sc8pR {
        Sc8pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Oscillator 4 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc4p(&self) -> Sc4pR {
        Sc4pR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Oscillator 2 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc2p(&self) -> Sc2pR {
        Sc2pR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - External Reference Stop Enable"]
    #[inline(always)]
    pub fn erefsten(&self) -> ErefstenR {
        ErefstenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - External Reference Enable"]
    #[inline(always)]
    pub fn erclken(&self) -> ErclkenR {
        ErclkenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator 16 pF Capacitor Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc16p(&mut self) -> Sc16pW<CrSpec> {
        Sc16pW::new(self, 0)
    }
    #[doc = "Bit 1 - Oscillator 8 pF Capacitor Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc8p(&mut self) -> Sc8pW<CrSpec> {
        Sc8pW::new(self, 1)
    }
    #[doc = "Bit 2 - Oscillator 4 pF Capacitor Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc4p(&mut self) -> Sc4pW<CrSpec> {
        Sc4pW::new(self, 2)
    }
    #[doc = "Bit 3 - Oscillator 2 pF Capacitor Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc2p(&mut self) -> Sc2pW<CrSpec> {
        Sc2pW::new(self, 3)
    }
    #[doc = "Bit 5 - External Reference Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erefsten(&mut self) -> ErefstenW<CrSpec> {
        ErefstenW::new(self, 5)
    }
    #[doc = "Bit 7 - External Reference Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erclken(&mut self) -> ErclkenW<CrSpec> {
        ErclkenW::new(self, 7)
    }
}
#[doc = "OSC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u8 = 0;
}
