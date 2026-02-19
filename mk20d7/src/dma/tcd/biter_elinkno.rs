#[doc = "Register `BITER_ELINKNO` reader"]
pub type R = crate::R<BiterElinknoSpec>;
#[doc = "Register `BITER_ELINKNO` writer"]
pub type W = crate::W<BiterElinknoSpec>;
#[doc = "Field `BITER` reader - Starting Major Iteration Count"]
pub type BiterR = crate::FieldReader<u16>;
#[doc = "Field `BITER` writer - Starting Major Iteration Count"]
pub type BiterW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Enables channel-to-channel linking on minor loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elink {
    #[doc = "0: The channel-to-channel linking is disabled"]
    _0 = 0,
    #[doc = "1: The channel-to-channel linking is enabled"]
    _1 = 1,
}
impl From<Elink> for bool {
    #[inline(always)]
    fn from(variant: Elink) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELINK` reader - Enables channel-to-channel linking on minor loop complete"]
pub type ElinkR = crate::BitReader<Elink>;
impl ElinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Elink {
        match self.bits {
            false => Elink::_0,
            true => Elink::_1,
        }
    }
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Elink::_0
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Elink::_1
    }
}
#[doc = "Field `ELINK` writer - Enables channel-to-channel linking on minor loop complete"]
pub type ElinkW<'a, REG> = crate::BitWriter<'a, REG, Elink>;
impl<'a, REG> ElinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Elink::_0)
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Elink::_1)
    }
}
impl R {
    #[doc = "Bits 0:14 - Starting Major Iteration Count"]
    #[inline(always)]
    pub fn biter(&self) -> BiterR {
        BiterR::new(self.bits & 0x7fff)
    }
    #[doc = "Bit 15 - Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub fn elink(&self) -> ElinkR {
        ElinkR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Starting Major Iteration Count"]
    #[inline(always)]
    #[must_use]
    pub fn biter(&mut self) -> BiterW<BiterElinknoSpec> {
        BiterW::new(self, 0)
    }
    #[doc = "Bit 15 - Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    #[must_use]
    pub fn elink(&mut self) -> ElinkW<BiterElinknoSpec> {
        ElinkW::new(self, 15)
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`biter_elinkno::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biter_elinkno::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BiterElinknoSpec;
impl crate::RegisterSpec for BiterElinknoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`biter_elinkno::R`](R) reader structure"]
impl crate::Readable for BiterElinknoSpec {}
#[doc = "`write(|w| ..)` method takes [`biter_elinkno::W`](W) writer structure"]
impl crate::Writable for BiterElinknoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BITER_ELINKNO to value 0"]
impl crate::Resettable for BiterElinknoSpec {
    const RESET_VALUE: u16 = 0;
}
