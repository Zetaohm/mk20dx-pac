#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "USB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbensofen {
    #[doc = "0: The USB Module is disabled."]
    _0 = 0,
    #[doc = "1: The USB Module is enabled."]
    _1 = 1,
}
impl From<Usbensofen> for bool {
    #[inline(always)]
    fn from(variant: Usbensofen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBENSOFEN` reader - USB Enable"]
pub type UsbensofenR = crate::BitReader<Usbensofen>;
impl UsbensofenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbensofen {
        match self.bits {
            false => Usbensofen::_0,
            true => Usbensofen::_1,
        }
    }
    #[doc = "The USB Module is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbensofen::_0
    }
    #[doc = "The USB Module is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbensofen::_1
    }
}
#[doc = "Field `USBENSOFEN` writer - USB Enable"]
pub type UsbensofenW<'a, REG> = crate::BitWriter<'a, REG, Usbensofen>;
impl<'a, REG> UsbensofenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The USB Module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbensofen::_0)
    }
    #[doc = "The USB Module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbensofen::_1)
    }
}
#[doc = "Field `ODDRST` reader - Setting this bit to 1 resets all the BDT ODD ping/pong bits to 0, which then specifies the EVEN BDT bank"]
pub type OddrstR = crate::BitReader;
#[doc = "Field `ODDRST` writer - Setting this bit to 1 resets all the BDT ODD ping/pong bits to 0, which then specifies the EVEN BDT bank"]
pub type OddrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - When set to 1 this bit enables the USB Module to execute resume signaling"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - When set to 1 this bit enables the USB Module to execute resume signaling"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTMODEEN` reader - When set to 1, this bit enables the USB Module to operate in Host mode"]
pub type HostmodeenR = crate::BitReader;
#[doc = "Field `HOSTMODEEN` writer - When set to 1, this bit enables the USB Module to operate in Host mode"]
pub type HostmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Setting this bit enables the USB Module to generate USB reset signaling"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Setting this bit enables the USB Module to generate USB reset signaling"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSUSPENDTOKENBUSY` reader - When the USB Module is in Host mode TOKEN_BUSY is set when the USB Module is busy executing a USB token and no more token commands should be written to the Token Register"]
pub type TxsuspendtokenbusyR = crate::BitReader;
#[doc = "Field `TXSUSPENDTOKENBUSY` writer - When the USB Module is in Host mode TOKEN_BUSY is set when the USB Module is busy executing a USB token and no more token commands should be written to the Token Register"]
pub type TxsuspendtokenbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE0` reader - Live USB Single Ended Zero signal"]
pub type Se0R = crate::BitReader;
#[doc = "Field `SE0` writer - Live USB Single Ended Zero signal"]
pub type Se0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSTATE` reader - Live USB differential receiver JSTATE signal"]
pub type JstateR = crate::BitReader;
#[doc = "Field `JSTATE` writer - Live USB differential receiver JSTATE signal"]
pub type JstateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    pub fn usbensofen(&self) -> UsbensofenR {
        UsbensofenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong bits to 0, which then specifies the EVEN BDT bank"]
    #[inline(always)]
    pub fn oddrst(&self) -> OddrstR {
        OddrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline(always)]
    pub fn hostmodeen(&self) -> HostmodeenR {
        HostmodeenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When the USB Module is in Host mode TOKEN_BUSY is set when the USB Module is busy executing a USB token and no more token commands should be written to the Token Register"]
    #[inline(always)]
    pub fn txsuspendtokenbusy(&self) -> TxsuspendtokenbusyR {
        TxsuspendtokenbusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Live USB Single Ended Zero signal"]
    #[inline(always)]
    pub fn se0(&self) -> Se0R {
        Se0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
    #[inline(always)]
    pub fn jstate(&self) -> JstateR {
        JstateR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbensofen(&mut self) -> UsbensofenW<CtlSpec> {
        UsbensofenW::new(self, 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong bits to 0, which then specifies the EVEN BDT bank"]
    #[inline(always)]
    #[must_use]
    pub fn oddrst(&mut self) -> OddrstW<CtlSpec> {
        OddrstW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> ResumeW<CtlSpec> {
        ResumeW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline(always)]
    #[must_use]
    pub fn hostmodeen(&mut self) -> HostmodeenW<CtlSpec> {
        HostmodeenW::new(self, 3)
    }
    #[doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CtlSpec> {
        ResetW::new(self, 4)
    }
    #[doc = "Bit 5 - When the USB Module is in Host mode TOKEN_BUSY is set when the USB Module is busy executing a USB token and no more token commands should be written to the Token Register"]
    #[inline(always)]
    #[must_use]
    pub fn txsuspendtokenbusy(&mut self) -> TxsuspendtokenbusyW<CtlSpec> {
        TxsuspendtokenbusyW::new(self, 5)
    }
    #[doc = "Bit 6 - Live USB Single Ended Zero signal"]
    #[inline(always)]
    #[must_use]
    pub fn se0(&mut self) -> Se0W<CtlSpec> {
        Se0W::new(self, 6)
    }
    #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
    #[inline(always)]
    #[must_use]
    pub fn jstate(&mut self) -> JstateW<CtlSpec> {
        JstateW::new(self, 7)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u8 = 0;
}
