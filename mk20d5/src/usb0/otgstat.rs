#[doc = "Register `OTGSTAT` reader"]
pub type R = crate::R<OtgstatSpec>;
#[doc = "Register `OTGSTAT` writer"]
pub type W = crate::W<OtgstatSpec>;
#[doc = "A VBUS Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avbusvld {
    #[doc = "0: The VBUS voltage is below the A VBUS Valid threshold."]
    _0 = 0,
    #[doc = "1: The VBUS voltage is above the A VBUS Valid threshold."]
    _1 = 1,
}
impl From<Avbusvld> for bool {
    #[inline(always)]
    fn from(variant: Avbusvld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVBUSVLD` reader - A VBUS Valid"]
pub type AvbusvldR = crate::BitReader<Avbusvld>;
impl AvbusvldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avbusvld {
        match self.bits {
            false => Avbusvld::_0,
            true => Avbusvld::_1,
        }
    }
    #[doc = "The VBUS voltage is below the A VBUS Valid threshold."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Avbusvld::_0
    }
    #[doc = "The VBUS voltage is above the A VBUS Valid threshold."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Avbusvld::_1
    }
}
#[doc = "Field `AVBUSVLD` writer - A VBUS Valid"]
pub type AvbusvldW<'a, REG> = crate::BitWriter<'a, REG, Avbusvld>;
impl<'a, REG> AvbusvldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The VBUS voltage is below the A VBUS Valid threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Avbusvld::_0)
    }
    #[doc = "The VBUS voltage is above the A VBUS Valid threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Avbusvld::_1)
    }
}
#[doc = "B Session END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsessend {
    #[doc = "0: The VBUS voltage is above the B session End threshold."]
    _0 = 0,
    #[doc = "1: The VBUS voltage is below the B session End threshold."]
    _1 = 1,
}
impl From<Bsessend> for bool {
    #[inline(always)]
    fn from(variant: Bsessend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSESSEND` reader - B Session END"]
pub type BsessendR = crate::BitReader<Bsessend>;
impl BsessendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsessend {
        match self.bits {
            false => Bsessend::_0,
            true => Bsessend::_1,
        }
    }
    #[doc = "The VBUS voltage is above the B session End threshold."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bsessend::_0
    }
    #[doc = "The VBUS voltage is below the B session End threshold."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bsessend::_1
    }
}
#[doc = "Field `BSESSEND` writer - B Session END"]
pub type BsessendW<'a, REG> = crate::BitWriter<'a, REG, Bsessend>;
impl<'a, REG> BsessendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The VBUS voltage is above the B session End threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bsessend::_0)
    }
    #[doc = "The VBUS voltage is below the B session End threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bsessend::_1)
    }
}
#[doc = "Session valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessVld {
    #[doc = "0: The VBUS voltage is below the B session Valid threshold"]
    _0 = 0,
    #[doc = "1: The VBUS voltage is above the B session Valid threshold."]
    _1 = 1,
}
impl From<SessVld> for bool {
    #[inline(always)]
    fn from(variant: SessVld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SESS_VLD` reader - Session valid"]
pub type SessVldR = crate::BitReader<SessVld>;
impl SessVldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SessVld {
        match self.bits {
            false => SessVld::_0,
            true => SessVld::_1,
        }
    }
    #[doc = "The VBUS voltage is below the B session Valid threshold"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SessVld::_0
    }
    #[doc = "The VBUS voltage is above the B session Valid threshold."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SessVld::_1
    }
}
#[doc = "Field `SESS_VLD` writer - Session valid"]
pub type SessVldW<'a, REG> = crate::BitWriter<'a, REG, SessVld>;
impl<'a, REG> SessVldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The VBUS voltage is below the B session Valid threshold"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SessVld::_0)
    }
    #[doc = "The VBUS voltage is above the B session Valid threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SessVld::_1)
    }
}
#[doc = "This bit indicates that the internal signals that control the LINE_STATE_CHG bit (bit 5) of the OTGISTAT register have been stable for at least 1 millisecond\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linestatestable {
    #[doc = "0: The LINE_STAT_CHG bit is not yet stable."]
    _0 = 0,
    #[doc = "1: The LINE_STAT_CHG bit has been debounced and is stable."]
    _1 = 1,
}
impl From<Linestatestable> for bool {
    #[inline(always)]
    fn from(variant: Linestatestable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINESTATESTABLE` reader - This bit indicates that the internal signals that control the LINE_STATE_CHG bit (bit 5) of the OTGISTAT register have been stable for at least 1 millisecond"]
pub type LinestatestableR = crate::BitReader<Linestatestable>;
impl LinestatestableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linestatestable {
        match self.bits {
            false => Linestatestable::_0,
            true => Linestatestable::_1,
        }
    }
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Linestatestable::_0
    }
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Linestatestable::_1
    }
}
#[doc = "Field `LINESTATESTABLE` writer - This bit indicates that the internal signals that control the LINE_STATE_CHG bit (bit 5) of the OTGISTAT register have been stable for at least 1 millisecond"]
pub type LinestatestableW<'a, REG> = crate::BitWriter<'a, REG, Linestatestable>;
impl<'a, REG> LinestatestableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Linestatestable::_0)
    }
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Linestatestable::_1)
    }
}
#[doc = "Field `ONEMSECEN` reader - This bit is reserved for the 1msec count, but it is not useful to software."]
pub type OnemsecenR = crate::BitReader;
#[doc = "Field `ONEMSECEN` writer - This bit is reserved for the 1msec count, but it is not useful to software."]
pub type OnemsecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Indicates the current state of the ID pin on the USB connector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Id {
    #[doc = "0: Indicates a Type A cable has been plugged into the USB connector"]
    _0 = 0,
    #[doc = "1: Indicates no cable is attached or a Type B cable has been plugged into the USB connector"]
    _1 = 1,
}
impl From<Id> for bool {
    #[inline(always)]
    fn from(variant: Id) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ID` reader - Indicates the current state of the ID pin on the USB connector"]
pub type IdR = crate::BitReader<Id>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id {
        match self.bits {
            false => Id::_0,
            true => Id::_1,
        }
    }
    #[doc = "Indicates a Type A cable has been plugged into the USB connector"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Id::_0
    }
    #[doc = "Indicates no cable is attached or a Type B cable has been plugged into the USB connector"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Id::_1
    }
}
#[doc = "Field `ID` writer - Indicates the current state of the ID pin on the USB connector"]
pub type IdW<'a, REG> = crate::BitWriter<'a, REG, Id>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates a Type A cable has been plugged into the USB connector"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_0)
    }
    #[doc = "Indicates no cable is attached or a Type B cable has been plugged into the USB connector"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Id::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A VBUS Valid"]
    #[inline(always)]
    pub fn avbusvld(&self) -> AvbusvldR {
        AvbusvldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - B Session END"]
    #[inline(always)]
    pub fn bsessend(&self) -> BsessendR {
        BsessendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Session valid"]
    #[inline(always)]
    pub fn sess_vld(&self) -> SessVldR {
        SessVldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that the internal signals that control the LINE_STATE_CHG bit (bit 5) of the OTGISTAT register have been stable for at least 1 millisecond"]
    #[inline(always)]
    pub fn linestatestable(&self) -> LinestatestableR {
        LinestatestableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1msec count, but it is not useful to software."]
    #[inline(always)]
    pub fn onemsecen(&self) -> OnemsecenR {
        OnemsecenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the current state of the ID pin on the USB connector"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A VBUS Valid"]
    #[inline(always)]
    #[must_use]
    pub fn avbusvld(&mut self) -> AvbusvldW<OtgstatSpec> {
        AvbusvldW::new(self, 0)
    }
    #[doc = "Bit 2 - B Session END"]
    #[inline(always)]
    #[must_use]
    pub fn bsessend(&mut self) -> BsessendW<OtgstatSpec> {
        BsessendW::new(self, 2)
    }
    #[doc = "Bit 3 - Session valid"]
    #[inline(always)]
    #[must_use]
    pub fn sess_vld(&mut self) -> SessVldW<OtgstatSpec> {
        SessVldW::new(self, 3)
    }
    #[doc = "Bit 5 - This bit indicates that the internal signals that control the LINE_STATE_CHG bit (bit 5) of the OTGISTAT register have been stable for at least 1 millisecond"]
    #[inline(always)]
    #[must_use]
    pub fn linestatestable(&mut self) -> LinestatestableW<OtgstatSpec> {
        LinestatestableW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1msec count, but it is not useful to software."]
    #[inline(always)]
    #[must_use]
    pub fn onemsecen(&mut self) -> OnemsecenW<OtgstatSpec> {
        OnemsecenW::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates the current state of the ID pin on the USB connector"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<OtgstatSpec> {
        IdW::new(self, 7)
    }
}
#[doc = "OTG Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otgstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgstatSpec;
impl crate::RegisterSpec for OtgstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otgstat::R`](R) reader structure"]
impl crate::Readable for OtgstatSpec {}
#[doc = "`write(|w| ..)` method takes [`otgstat::W`](W) writer structure"]
impl crate::Writable for OtgstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTGSTAT to value 0"]
impl crate::Resettable for OtgstatSpec {
    const RESET_VALUE: u8 = 0;
}
