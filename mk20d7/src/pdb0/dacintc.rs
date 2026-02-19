#[doc = "Register `DACINTC` reader"]
pub type R = crate::R<DacintcSpec>;
#[doc = "Register `DACINTC` writer"]
pub type W = crate::W<DacintcSpec>;
#[doc = "DAC Interval Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toe {
    #[doc = "0: DAC interval trigger disabled."]
    _0 = 0,
    #[doc = "1: DAC interval trigger enabled."]
    _1 = 1,
}
impl From<Toe> for bool {
    #[inline(always)]
    fn from(variant: Toe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOE` reader - DAC Interval Trigger Enable"]
pub type ToeR = crate::BitReader<Toe>;
impl ToeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toe {
        match self.bits {
            false => Toe::_0,
            true => Toe::_1,
        }
    }
    #[doc = "DAC interval trigger disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toe::_0
    }
    #[doc = "DAC interval trigger enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toe::_1
    }
}
#[doc = "Field `TOE` writer - DAC Interval Trigger Enable"]
pub type ToeW<'a, REG> = crate::BitWriter<'a, REG, Toe>;
impl<'a, REG> ToeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC interval trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toe::_0)
    }
    #[doc = "DAC interval trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toe::_1)
    }
}
#[doc = "DAC External Trigger Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ext {
    #[doc = "0: DAC external trigger input disabled. DAC interval counter is reset and started counting when a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: DAC external trigger input enabled. DAC interval counter is bypassed and DAC external trigger input triggers the DAC interval trigger."]
    _1 = 1,
}
impl From<Ext> for bool {
    #[inline(always)]
    fn from(variant: Ext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT` reader - DAC External Trigger Input Enable"]
pub type ExtR = crate::BitReader<Ext>;
impl ExtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ext {
        match self.bits {
            false => Ext::_0,
            true => Ext::_1,
        }
    }
    #[doc = "DAC external trigger input disabled. DAC interval counter is reset and started counting when a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ext::_0
    }
    #[doc = "DAC external trigger input enabled. DAC interval counter is bypassed and DAC external trigger input triggers the DAC interval trigger."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ext::_1
    }
}
#[doc = "Field `EXT` writer - DAC External Trigger Input Enable"]
pub type ExtW<'a, REG> = crate::BitWriter<'a, REG, Ext>;
impl<'a, REG> ExtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC external trigger input disabled. DAC interval counter is reset and started counting when a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ext::_0)
    }
    #[doc = "DAC external trigger input enabled. DAC interval counter is bypassed and DAC external trigger input triggers the DAC interval trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ext::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC Interval Trigger Enable"]
    #[inline(always)]
    pub fn toe(&self) -> ToeR {
        ToeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC External Trigger Input Enable"]
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Interval Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toe(&mut self) -> ToeW<DacintcSpec> {
        ToeW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC External Trigger Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> ExtW<DacintcSpec> {
        ExtW::new(self, 1)
    }
}
#[doc = "DAC Interval Trigger n Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacintc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacintc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacintcSpec;
impl crate::RegisterSpec for DacintcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacintc::R`](R) reader structure"]
impl crate::Readable for DacintcSpec {}
#[doc = "`write(|w| ..)` method takes [`dacintc::W`](W) writer structure"]
impl crate::Writable for DacintcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACINTC to value 0"]
impl crate::Resettable for DacintcSpec {
    const RESET_VALUE: u32 = 0;
}
