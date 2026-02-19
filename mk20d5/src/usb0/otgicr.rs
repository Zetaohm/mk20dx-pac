#[doc = "Register `OTGICR` reader"]
pub type R = crate::R<OtgicrSpec>;
#[doc = "Register `OTGICR` writer"]
pub type W = crate::W<OtgicrSpec>;
#[doc = "A VBUS Valid interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avbusen {
    #[doc = "0: The AVBUSCHG interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The AVBUSCHG interrupt is enabled"]
    _1 = 1,
}
impl From<Avbusen> for bool {
    #[inline(always)]
    fn from(variant: Avbusen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVBUSEN` reader - A VBUS Valid interrupt enable"]
pub type AvbusenR = crate::BitReader<Avbusen>;
impl AvbusenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avbusen {
        match self.bits {
            false => Avbusen::_0,
            true => Avbusen::_1,
        }
    }
    #[doc = "The AVBUSCHG interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Avbusen::_0
    }
    #[doc = "The AVBUSCHG interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Avbusen::_1
    }
}
#[doc = "Field `AVBUSEN` writer - A VBUS Valid interrupt enable"]
pub type AvbusenW<'a, REG> = crate::BitWriter<'a, REG, Avbusen>;
impl<'a, REG> AvbusenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The AVBUSCHG interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Avbusen::_0)
    }
    #[doc = "The AVBUSCHG interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Avbusen::_1)
    }
}
#[doc = "B Session END interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsessen {
    #[doc = "0: The B_SESS_CHG interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The B_SESS_CHG interrupt is enabled"]
    _1 = 1,
}
impl From<Bsessen> for bool {
    #[inline(always)]
    fn from(variant: Bsessen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSESSEN` reader - B Session END interrupt enable"]
pub type BsessenR = crate::BitReader<Bsessen>;
impl BsessenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsessen {
        match self.bits {
            false => Bsessen::_0,
            true => Bsessen::_1,
        }
    }
    #[doc = "The B_SESS_CHG interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsessen::_0
    }
    #[doc = "The B_SESS_CHG interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsessen::_1
    }
}
#[doc = "Field `BSESSEN` writer - B Session END interrupt enable"]
pub type BsessenW<'a, REG> = crate::BitWriter<'a, REG, Bsessen>;
impl<'a, REG> BsessenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The B_SESS_CHG interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bsessen::_0)
    }
    #[doc = "The B_SESS_CHG interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bsessen::_1)
    }
}
#[doc = "Session valid interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sessvlden {
    #[doc = "0: The SESSVLDCHG interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The SESSVLDCHG interrupt is enabled."]
    _1 = 1,
}
impl From<Sessvlden> for bool {
    #[inline(always)]
    fn from(variant: Sessvlden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SESSVLDEN` reader - Session valid interrupt enable"]
pub type SessvldenR = crate::BitReader<Sessvlden>;
impl SessvldenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sessvlden {
        match self.bits {
            false => Sessvlden::_0,
            true => Sessvlden::_1,
        }
    }
    #[doc = "The SESSVLDCHG interrupt is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sessvlden::_0
    }
    #[doc = "The SESSVLDCHG interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sessvlden::_1
    }
}
#[doc = "Field `SESSVLDEN` writer - Session valid interrupt enable"]
pub type SessvldenW<'a, REG> = crate::BitWriter<'a, REG, Sessvlden>;
impl<'a, REG> SessvldenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SESSVLDCHG interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sessvlden::_0)
    }
    #[doc = "The SESSVLDCHG interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sessvlden::_1)
    }
}
#[doc = "Line State change interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linestateen {
    #[doc = "0: The LINE_STAT_CHG interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The LINE_STAT_CHG interrupt is enabled"]
    _1 = 1,
}
impl From<Linestateen> for bool {
    #[inline(always)]
    fn from(variant: Linestateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINESTATEEN` reader - Line State change interrupt enable"]
pub type LinestateenR = crate::BitReader<Linestateen>;
impl LinestateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linestateen {
        match self.bits {
            false => Linestateen::_0,
            true => Linestateen::_1,
        }
    }
    #[doc = "The LINE_STAT_CHG interrupt is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Linestateen::_0
    }
    #[doc = "The LINE_STAT_CHG interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Linestateen::_1
    }
}
#[doc = "Field `LINESTATEEN` writer - Line State change interrupt enable"]
pub type LinestateenW<'a, REG> = crate::BitWriter<'a, REG, Linestateen>;
impl<'a, REG> LinestateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The LINE_STAT_CHG interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Linestateen::_0)
    }
    #[doc = "The LINE_STAT_CHG interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Linestateen::_1)
    }
}
#[doc = "1 millisecond interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onemsecen {
    #[doc = "0: The 1msec timer interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The 1msec timer interrupt is enabled."]
    _1 = 1,
}
impl From<Onemsecen> for bool {
    #[inline(always)]
    fn from(variant: Onemsecen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONEMSECEN` reader - 1 millisecond interrupt enable"]
pub type OnemsecenR = crate::BitReader<Onemsecen>;
impl OnemsecenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onemsecen {
        match self.bits {
            false => Onemsecen::_0,
            true => Onemsecen::_1,
        }
    }
    #[doc = "The 1msec timer interrupt is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Onemsecen::_0
    }
    #[doc = "The 1msec timer interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Onemsecen::_1
    }
}
#[doc = "Field `ONEMSECEN` writer - 1 millisecond interrupt enable"]
pub type OnemsecenW<'a, REG> = crate::BitWriter<'a, REG, Onemsecen>;
impl<'a, REG> OnemsecenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 1msec timer interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Onemsecen::_0)
    }
    #[doc = "The 1msec timer interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Onemsecen::_1)
    }
}
#[doc = "ID interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iden {
    #[doc = "0: The ID interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The ID interrupt is enabled"]
    _1 = 1,
}
impl From<Iden> for bool {
    #[inline(always)]
    fn from(variant: Iden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDEN` reader - ID interrupt enable"]
pub type IdenR = crate::BitReader<Iden>;
impl IdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iden {
        match self.bits {
            false => Iden::_0,
            true => Iden::_1,
        }
    }
    #[doc = "The ID interrupt is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iden::_0
    }
    #[doc = "The ID interrupt is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iden::_1
    }
}
#[doc = "Field `IDEN` writer - ID interrupt enable"]
pub type IdenW<'a, REG> = crate::BitWriter<'a, REG, Iden>;
impl<'a, REG> IdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The ID interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iden::_0)
    }
    #[doc = "The ID interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iden::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A VBUS Valid interrupt enable"]
    #[inline(always)]
    pub fn avbusen(&self) -> AvbusenR {
        AvbusenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - B Session END interrupt enable"]
    #[inline(always)]
    pub fn bsessen(&self) -> BsessenR {
        BsessenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Session valid interrupt enable"]
    #[inline(always)]
    pub fn sessvlden(&self) -> SessvldenR {
        SessvldenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Line State change interrupt enable"]
    #[inline(always)]
    pub fn linestateen(&self) -> LinestateenR {
        LinestateenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 millisecond interrupt enable"]
    #[inline(always)]
    pub fn onemsecen(&self) -> OnemsecenR {
        OnemsecenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ID interrupt enable"]
    #[inline(always)]
    pub fn iden(&self) -> IdenR {
        IdenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A VBUS Valid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn avbusen(&mut self) -> AvbusenW<OtgicrSpec> {
        AvbusenW::new(self, 0)
    }
    #[doc = "Bit 2 - B Session END interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsessen(&mut self) -> BsessenW<OtgicrSpec> {
        BsessenW::new(self, 2)
    }
    #[doc = "Bit 3 - Session valid interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sessvlden(&mut self) -> SessvldenW<OtgicrSpec> {
        SessvldenW::new(self, 3)
    }
    #[doc = "Bit 5 - Line State change interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn linestateen(&mut self) -> LinestateenW<OtgicrSpec> {
        LinestateenW::new(self, 5)
    }
    #[doc = "Bit 6 - 1 millisecond interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn onemsecen(&mut self) -> OnemsecenW<OtgicrSpec> {
        OnemsecenW::new(self, 6)
    }
    #[doc = "Bit 7 - ID interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iden(&mut self) -> IdenW<OtgicrSpec> {
        IdenW::new(self, 7)
    }
}
#[doc = "OTG Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otgicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgicrSpec;
impl crate::RegisterSpec for OtgicrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otgicr::R`](R) reader structure"]
impl crate::Readable for OtgicrSpec {}
#[doc = "`write(|w| ..)` method takes [`otgicr::W`](W) writer structure"]
impl crate::Writable for OtgicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTGICR to value 0"]
impl crate::Resettable for OtgicrSpec {
    const RESET_VALUE: u8 = 0;
}
