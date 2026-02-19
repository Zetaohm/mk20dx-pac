#[doc = "Register `BITER_ELINKYES` reader"]
pub type R = crate::R<BiterElinkyesSpec>;
#[doc = "Register `BITER_ELINKYES` writer"]
pub type W = crate::W<BiterElinkyesSpec>;
#[doc = "Field `BITER` reader - Starting Major Iteration Count"]
pub type BiterR = crate::FieldReader<u16>;
#[doc = "Field `BITER` writer - Starting Major Iteration Count"]
pub type BiterW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `LINKCH` reader - Link Channel Number"]
pub type LinkchR = crate::FieldReader;
#[doc = "Field `LINKCH` writer - Link Channel Number"]
pub type LinkchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 0:8 - Starting Major Iteration Count"]
    #[inline(always)]
    pub fn biter(&self) -> BiterR {
        BiterR::new(self.bits & 0x01ff)
    }
    #[doc = "Bits 9:12 - Link Channel Number"]
    #[inline(always)]
    pub fn linkch(&self) -> LinkchR {
        LinkchR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub fn elink(&self) -> ElinkR {
        ElinkR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Starting Major Iteration Count"]
    #[inline(always)]
    #[must_use]
    pub fn biter(&mut self) -> BiterW<BiterElinkyesSpec> {
        BiterW::new(self, 0)
    }
    #[doc = "Bits 9:12 - Link Channel Number"]
    #[inline(always)]
    #[must_use]
    pub fn linkch(&mut self) -> LinkchW<BiterElinkyesSpec> {
        LinkchW::new(self, 9)
    }
    #[doc = "Bit 15 - Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    #[must_use]
    pub fn elink(&mut self) -> ElinkW<BiterElinkyesSpec> {
        ElinkW::new(self, 15)
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`biter_elinkyes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biter_elinkyes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BiterElinkyesSpec;
impl crate::RegisterSpec for BiterElinkyesSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`biter_elinkyes::R`](R) reader structure"]
impl crate::Readable for BiterElinkyesSpec {}
#[doc = "`write(|w| ..)` method takes [`biter_elinkyes::W`](W) writer structure"]
impl crate::Writable for BiterElinkyesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BITER_ELINKYES to value 0"]
impl crate::Resettable for BiterElinkyesSpec {
    const RESET_VALUE: u16 = 0;
}
