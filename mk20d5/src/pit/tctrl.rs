#[doc = "Register `TCTRL%s` reader"]
pub type R = crate::R<TctrlSpec>;
#[doc = "Register `TCTRL%s` writer"]
pub type W = crate::W<TctrlSpec>;
#[doc = "Timer Enable Bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten {
    #[doc = "0: Timer n is disabled."]
    _0 = 0,
    #[doc = "1: Timer n is active."]
    _1 = 1,
}
impl From<Ten> for bool {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Timer Enable Bit."]
pub type TenR = crate::BitReader<Ten>;
impl TenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ten {
        match self.bits {
            false => Ten::_0,
            true => Ten::_1,
        }
    }
    #[doc = "Timer n is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ten::_0
    }
    #[doc = "Timer n is active."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ten::_1
    }
}
#[doc = "Field `TEN` writer - Timer Enable Bit."]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG, Ten>;
impl<'a, REG> TenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer n is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::_0)
    }
    #[doc = "Timer n is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::_1)
    }
}
#[doc = "Timer Interrupt Enable Bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: Interrupt requests from Timer n are disabled."]
    _0 = 0,
    #[doc = "1: Interrupt will be requested whenever TIF is set."]
    _1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable Bit."]
pub type TieR = crate::BitReader<Tie>;
impl TieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tie {
        match self.bits {
            false => Tie::_0,
            true => Tie::_1,
        }
    }
    #[doc = "Interrupt requests from Timer n are disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tie::_0
    }
    #[doc = "Interrupt will be requested whenever TIF is set."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tie::_1
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable Bit."]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt requests from Timer n are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_0)
    }
    #[doc = "Interrupt will be requested whenever TIF is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Enable Bit."]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Interrupt Enable Bit."]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable Bit."]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TenW<TctrlSpec> {
        TenW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Interrupt Enable Bit."]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<TctrlSpec> {
        TieW::new(self, 1)
    }
}
#[doc = "Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TctrlSpec;
impl crate::RegisterSpec for TctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tctrl::R`](R) reader structure"]
impl crate::Readable for TctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tctrl::W`](W) writer structure"]
impl crate::Writable for TctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCTRL%s to value 0"]
impl crate::Resettable for TctrlSpec {
    const RESET_VALUE: u32 = 0;
}
