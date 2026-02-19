#[doc = "Register `CH%sC1` reader"]
pub type R = crate::R<Chc1Spec>;
#[doc = "Register `CH%sC1` writer"]
pub type W = crate::W<Chc1Spec>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum En {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for En {
    type Ux = u8;
}
impl crate::IsEnum for En {}
#[doc = "Field `EN` reader - PDB Channel Pre-Trigger Enable"]
pub type EnR = crate::FieldReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<En> {
        match self.bits {
            0 => Some(En::_0),
            1 => Some(En::_1),
            _ => None,
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En::_0
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En::_1
    }
}
#[doc = "Field `EN` writer - PDB Channel Pre-Trigger Enable"]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 8, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En::_1)
    }
}
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tos {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register plus one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<Tos> for u8 {
    #[inline(always)]
    fn from(variant: Tos) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tos {
    type Ux = u8;
}
impl crate::IsEnum for Tos {}
#[doc = "Field `TOS` reader - PDB Channel Pre-Trigger Output Select"]
pub type TosR = crate::FieldReader<Tos>;
impl TosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tos> {
        match self.bits {
            0 => Some(Tos::_0),
            1 => Some(Tos::_1),
            _ => None,
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tos::_0
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register plus one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tos::_1
    }
}
#[doc = "Field `TOS` writer - PDB Channel Pre-Trigger Output Select"]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 8, Tos>;
impl<'a, REG> TosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tos::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register plus one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tos::_1)
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bb {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<Bb> for u8 {
    #[inline(always)]
    fn from(variant: Bb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bb {
    type Ux = u8;
}
impl crate::IsEnum for Bb {}
#[doc = "Field `BB` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BbR = crate::FieldReader<Bb>;
impl BbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bb> {
        match self.bits {
            0 => Some(Bb::_0),
            1 => Some(Bb::_1),
            _ => None,
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bb::_0
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bb::_1
    }
}
#[doc = "Field `BB` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BbW<'a, REG> = crate::FieldWriter<'a, REG, 8, Bb>;
impl<'a, REG> BbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bb::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bb::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Chc1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TosW<Chc1Spec> {
        TosW::new(self, 8)
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb(&mut self) -> BbW<Chc1Spec> {
        BbW::new(self, 16)
    }
}
#[doc = "Channel n Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chc1Spec;
impl crate::RegisterSpec for Chc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chc1::R`](R) reader structure"]
impl crate::Readable for Chc1Spec {}
#[doc = "`write(|w| ..)` method takes [`chc1::W`](W) writer structure"]
impl crate::Writable for Chc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%sC1 to value 0"]
impl crate::Resettable for Chc1Spec {
    const RESET_VALUE: u32 = 0;
}
