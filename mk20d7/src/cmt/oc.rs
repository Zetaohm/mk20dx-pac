#[doc = "Register `OC` reader"]
pub type R = crate::R<OcSpec>;
#[doc = "Register `OC` writer"]
pub type W = crate::W<OcSpec>;
#[doc = "IRO Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iropen {
    #[doc = "0: CMT_IRO signal disabled"]
    _0 = 0,
    #[doc = "1: CMT_IRO signal enabled as output"]
    _1 = 1,
}
impl From<Iropen> for bool {
    #[inline(always)]
    fn from(variant: Iropen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IROPEN` reader - IRO Pin Enable"]
pub type IropenR = crate::BitReader<Iropen>;
impl IropenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iropen {
        match self.bits {
            false => Iropen::_0,
            true => Iropen::_1,
        }
    }
    #[doc = "CMT_IRO signal disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iropen::_0
    }
    #[doc = "CMT_IRO signal enabled as output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iropen::_1
    }
}
#[doc = "Field `IROPEN` writer - IRO Pin Enable"]
pub type IropenW<'a, REG> = crate::BitWriter<'a, REG, Iropen>;
impl<'a, REG> IropenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMT_IRO signal disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iropen::_0)
    }
    #[doc = "CMT_IRO signal enabled as output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iropen::_1)
    }
}
#[doc = "CMT Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmtpol {
    #[doc = "0: CMT_IRO signal is active low"]
    _0 = 0,
    #[doc = "1: CMT_IRO signal is active high"]
    _1 = 1,
}
impl From<Cmtpol> for bool {
    #[inline(always)]
    fn from(variant: Cmtpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTPOL` reader - CMT Output Polarity"]
pub type CmtpolR = crate::BitReader<Cmtpol>;
impl CmtpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmtpol {
        match self.bits {
            false => Cmtpol::_0,
            true => Cmtpol::_1,
        }
    }
    #[doc = "CMT_IRO signal is active low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmtpol::_0
    }
    #[doc = "CMT_IRO signal is active high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmtpol::_1
    }
}
#[doc = "Field `CMTPOL` writer - CMT Output Polarity"]
pub type CmtpolW<'a, REG> = crate::BitWriter<'a, REG, Cmtpol>;
impl<'a, REG> CmtpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMT_IRO signal is active low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmtpol::_0)
    }
    #[doc = "CMT_IRO signal is active high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmtpol::_1)
    }
}
#[doc = "Field `IROL` reader - IRO Latch Control"]
pub type IrolR = crate::BitReader;
#[doc = "Field `IROL` writer - IRO Latch Control"]
pub type IrolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - IRO Pin Enable"]
    #[inline(always)]
    pub fn iropen(&self) -> IropenR {
        IropenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMT Output Polarity"]
    #[inline(always)]
    pub fn cmtpol(&self) -> CmtpolR {
        CmtpolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRO Latch Control"]
    #[inline(always)]
    pub fn irol(&self) -> IrolR {
        IrolR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - IRO Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iropen(&mut self) -> IropenW<OcSpec> {
        IropenW::new(self, 5)
    }
    #[doc = "Bit 6 - CMT Output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmtpol(&mut self) -> CmtpolW<OcSpec> {
        CmtpolW::new(self, 6)
    }
    #[doc = "Bit 7 - IRO Latch Control"]
    #[inline(always)]
    #[must_use]
    pub fn irol(&mut self) -> IrolW<OcSpec> {
        IrolW::new(self, 7)
    }
}
#[doc = "CMT Output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`oc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcSpec;
impl crate::RegisterSpec for OcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oc::R`](R) reader structure"]
impl crate::Readable for OcSpec {}
#[doc = "`write(|w| ..)` method takes [`oc::W`](W) writer structure"]
impl crate::Writable for OcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OC to value 0"]
impl crate::Resettable for OcSpec {
    const RESET_VALUE: u8 = 0;
}
