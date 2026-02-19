#[doc = "Register `RXMGMASK` reader"]
pub type R = crate::R<RxmgmaskSpec>;
#[doc = "Register `RXMGMASK` writer"]
pub type W = crate::W<RxmgmaskSpec>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Mg {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<Mg> for u32 {
    #[inline(always)]
    fn from(variant: Mg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mg {
    type Ux = u32;
}
impl crate::IsEnum for Mg {}
#[doc = "Field `MG` reader - Rx Mailboxes Global Mask Bits"]
pub type MgR = crate::FieldReader<Mg>;
impl MgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mg> {
        match self.bits {
            0 => Some(Mg::_0),
            1 => Some(Mg::_1),
            _ => None,
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mg::_0
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mg::_1
    }
}
#[doc = "Field `MG` writer - Rx Mailboxes Global Mask Bits"]
pub type MgW<'a, REG> = crate::FieldWriter<'a, REG, 32, Mg>;
impl<'a, REG> MgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mg::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mg::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg(&self) -> MgR {
        MgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg(&mut self) -> MgW<RxmgmaskSpec> {
        MgW::new(self, 0)
    }
}
#[doc = "Rx Mailboxes Global Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmgmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmgmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmgmaskSpec;
impl crate::RegisterSpec for RxmgmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmgmask::R`](R) reader structure"]
impl crate::Readable for RxmgmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`rxmgmask::W`](W) writer structure"]
impl crate::Writable for RxmgmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXMGMASK to value 0xffff_ffff"]
impl crate::Resettable for RxmgmaskSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
