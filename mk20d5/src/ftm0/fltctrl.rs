#[doc = "Register `FLTCTRL` reader"]
pub type R = crate::R<FltctrlSpec>;
#[doc = "Register `FLTCTRL` writer"]
pub type W = crate::W<FltctrlSpec>;
#[doc = "Fault Input 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault0en {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<Fault0en> for bool {
    #[inline(always)]
    fn from(variant: Fault0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT0EN` reader - Fault Input 0 Enable"]
pub type Fault0enR = crate::BitReader<Fault0en>;
impl Fault0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fault0en {
        match self.bits {
            false => Fault0en::_0,
            true => Fault0en::_1,
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fault0en::_0
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fault0en::_1
    }
}
#[doc = "Field `FAULT0EN` writer - Fault Input 0 Enable"]
pub type Fault0enW<'a, REG> = crate::BitWriter<'a, REG, Fault0en>;
impl<'a, REG> Fault0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fault0en::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fault0en::_1)
    }
}
#[doc = "Fault Input 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault1en {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<Fault1en> for bool {
    #[inline(always)]
    fn from(variant: Fault1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT1EN` reader - Fault Input 1 Enable"]
pub type Fault1enR = crate::BitReader<Fault1en>;
impl Fault1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fault1en {
        match self.bits {
            false => Fault1en::_0,
            true => Fault1en::_1,
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fault1en::_0
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fault1en::_1
    }
}
#[doc = "Field `FAULT1EN` writer - Fault Input 1 Enable"]
pub type Fault1enW<'a, REG> = crate::BitWriter<'a, REG, Fault1en>;
impl<'a, REG> Fault1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fault1en::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fault1en::_1)
    }
}
#[doc = "Fault Input 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault2en {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<Fault2en> for bool {
    #[inline(always)]
    fn from(variant: Fault2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT2EN` reader - Fault Input 2 Enable"]
pub type Fault2enR = crate::BitReader<Fault2en>;
impl Fault2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fault2en {
        match self.bits {
            false => Fault2en::_0,
            true => Fault2en::_1,
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fault2en::_0
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fault2en::_1
    }
}
#[doc = "Field `FAULT2EN` writer - Fault Input 2 Enable"]
pub type Fault2enW<'a, REG> = crate::BitWriter<'a, REG, Fault2en>;
impl<'a, REG> Fault2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fault2en::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fault2en::_1)
    }
}
#[doc = "Fault Input 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault3en {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<Fault3en> for bool {
    #[inline(always)]
    fn from(variant: Fault3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT3EN` reader - Fault Input 3 Enable"]
pub type Fault3enR = crate::BitReader<Fault3en>;
impl Fault3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fault3en {
        match self.bits {
            false => Fault3en::_0,
            true => Fault3en::_1,
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fault3en::_0
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fault3en::_1
    }
}
#[doc = "Field `FAULT3EN` writer - Fault Input 3 Enable"]
pub type Fault3enW<'a, REG> = crate::BitWriter<'a, REG, Fault3en>;
impl<'a, REG> Fault3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fault3en::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fault3en::_1)
    }
}
#[doc = "Fault Input 0 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffltr0en {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<Ffltr0en> for bool {
    #[inline(always)]
    fn from(variant: Ffltr0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLTR0EN` reader - Fault Input 0 Filter Enable"]
pub type Ffltr0enR = crate::BitReader<Ffltr0en>;
impl Ffltr0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffltr0en {
        match self.bits {
            false => Ffltr0en::_0,
            true => Ffltr0en::_1,
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ffltr0en::_0
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ffltr0en::_1
    }
}
#[doc = "Field `FFLTR0EN` writer - Fault Input 0 Filter Enable"]
pub type Ffltr0enW<'a, REG> = crate::BitWriter<'a, REG, Ffltr0en>;
impl<'a, REG> Ffltr0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ffltr0en::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ffltr0en::_1)
    }
}
#[doc = "Fault Input 1 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffltr1en {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<Ffltr1en> for bool {
    #[inline(always)]
    fn from(variant: Ffltr1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLTR1EN` reader - Fault Input 1 Filter Enable"]
pub type Ffltr1enR = crate::BitReader<Ffltr1en>;
impl Ffltr1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffltr1en {
        match self.bits {
            false => Ffltr1en::_0,
            true => Ffltr1en::_1,
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ffltr1en::_0
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ffltr1en::_1
    }
}
#[doc = "Field `FFLTR1EN` writer - Fault Input 1 Filter Enable"]
pub type Ffltr1enW<'a, REG> = crate::BitWriter<'a, REG, Ffltr1en>;
impl<'a, REG> Ffltr1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ffltr1en::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ffltr1en::_1)
    }
}
#[doc = "Fault Input 2 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffltr2en {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<Ffltr2en> for bool {
    #[inline(always)]
    fn from(variant: Ffltr2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLTR2EN` reader - Fault Input 2 Filter Enable"]
pub type Ffltr2enR = crate::BitReader<Ffltr2en>;
impl Ffltr2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffltr2en {
        match self.bits {
            false => Ffltr2en::_0,
            true => Ffltr2en::_1,
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ffltr2en::_0
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ffltr2en::_1
    }
}
#[doc = "Field `FFLTR2EN` writer - Fault Input 2 Filter Enable"]
pub type Ffltr2enW<'a, REG> = crate::BitWriter<'a, REG, Ffltr2en>;
impl<'a, REG> Ffltr2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ffltr2en::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ffltr2en::_1)
    }
}
#[doc = "Fault Input 3 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffltr3en {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<Ffltr3en> for bool {
    #[inline(always)]
    fn from(variant: Ffltr3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLTR3EN` reader - Fault Input 3 Filter Enable"]
pub type Ffltr3enR = crate::BitReader<Ffltr3en>;
impl Ffltr3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffltr3en {
        match self.bits {
            false => Ffltr3en::_0,
            true => Ffltr3en::_1,
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ffltr3en::_0
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ffltr3en::_1
    }
}
#[doc = "Field `FFLTR3EN` writer - Fault Input 3 Filter Enable"]
pub type Ffltr3enW<'a, REG> = crate::BitWriter<'a, REG, Ffltr3en>;
impl<'a, REG> Ffltr3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ffltr3en::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ffltr3en::_1)
    }
}
#[doc = "Field `FFVAL` reader - Fault Input Filter"]
pub type FfvalR = crate::FieldReader;
#[doc = "Field `FFVAL` writer - Fault Input Filter"]
pub type FfvalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline(always)]
    pub fn fault0en(&self) -> Fault0enR {
        Fault0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline(always)]
    pub fn fault1en(&self) -> Fault1enR {
        Fault1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline(always)]
    pub fn fault2en(&self) -> Fault2enR {
        Fault2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline(always)]
    pub fn fault3en(&self) -> Fault3enR {
        Fault3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline(always)]
    pub fn ffltr0en(&self) -> Ffltr0enR {
        Ffltr0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline(always)]
    pub fn ffltr1en(&self) -> Ffltr1enR {
        Ffltr1enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline(always)]
    pub fn ffltr2en(&self) -> Ffltr2enR {
        Ffltr2enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline(always)]
    pub fn ffltr3en(&self) -> Ffltr3enR {
        Ffltr3enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline(always)]
    pub fn ffval(&self) -> FfvalR {
        FfvalR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault0en(&mut self) -> Fault0enW<FltctrlSpec> {
        Fault0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault1en(&mut self) -> Fault1enW<FltctrlSpec> {
        Fault1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault2en(&mut self) -> Fault2enW<FltctrlSpec> {
        Fault2enW::new(self, 2)
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault3en(&mut self) -> Fault3enW<FltctrlSpec> {
        Fault3enW::new(self, 3)
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffltr0en(&mut self) -> Ffltr0enW<FltctrlSpec> {
        Ffltr0enW::new(self, 4)
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffltr1en(&mut self) -> Ffltr1enW<FltctrlSpec> {
        Ffltr1enW::new(self, 5)
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffltr2en(&mut self) -> Ffltr2enW<FltctrlSpec> {
        Ffltr2enW::new(self, 6)
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffltr3en(&mut self) -> Ffltr3enW<FltctrlSpec> {
        Ffltr3enW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ffval(&mut self) -> FfvalW<FltctrlSpec> {
        FfvalW::new(self, 8)
    }
}
#[doc = "Fault Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fltctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltctrlSpec;
impl crate::RegisterSpec for FltctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltctrl::R`](R) reader structure"]
impl crate::Readable for FltctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fltctrl::W`](W) writer structure"]
impl crate::Writable for FltctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTCTRL to value 0"]
impl crate::Resettable for FltctrlSpec {
    const RESET_VALUE: u32 = 0;
}
