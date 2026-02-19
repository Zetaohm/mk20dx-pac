#[doc = "Register `USBTRC0` reader"]
pub type R = crate::R<Usbtrc0Spec>;
#[doc = "Register `USBTRC0` writer"]
pub type W = crate::W<Usbtrc0Spec>;
#[doc = "USB Asynchronous Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbResumeInt {
    #[doc = "0: No interrupt was generated."]
    _0 = 0,
    #[doc = "1: Interrupt was generated because of the USB asynchronous interrupt."]
    _1 = 1,
}
impl From<UsbResumeInt> for bool {
    #[inline(always)]
    fn from(variant: UsbResumeInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_RESUME_INT` reader - USB Asynchronous Interrupt"]
pub type UsbResumeIntR = crate::BitReader<UsbResumeInt>;
impl UsbResumeIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbResumeInt {
        match self.bits {
            false => UsbResumeInt::_0,
            true => UsbResumeInt::_1,
        }
    }
    #[doc = "No interrupt was generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UsbResumeInt::_0
    }
    #[doc = "Interrupt was generated because of the USB asynchronous interrupt."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UsbResumeInt::_1
    }
}
#[doc = "Synchronous USB Interrupt Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncDet {
    #[doc = "0: Synchronous interrupt has not been detected."]
    _0 = 0,
    #[doc = "1: Synchronous interrupt has been detected."]
    _1 = 1,
}
impl From<SyncDet> for bool {
    #[inline(always)]
    fn from(variant: SyncDet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC_DET` reader - Synchronous USB Interrupt Detect"]
pub type SyncDetR = crate::BitReader<SyncDet>;
impl SyncDetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SyncDet {
        match self.bits {
            false => SyncDet::_0,
            true => SyncDet::_1,
        }
    }
    #[doc = "Synchronous interrupt has not been detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SyncDet::_0
    }
    #[doc = "Synchronous interrupt has been detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SyncDet::_1
    }
}
#[doc = "Asynchronous Resume Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbresmen {
    #[doc = "0: USB asynchronous wakeup from suspend mode disabled."]
    _0 = 0,
    #[doc = "1: USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interupt should only be enabled when the Transceiver is suspended."]
    _1 = 1,
}
impl From<Usbresmen> for bool {
    #[inline(always)]
    fn from(variant: Usbresmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRESMEN` reader - Asynchronous Resume Interrupt Enable"]
pub type UsbresmenR = crate::BitReader<Usbresmen>;
impl UsbresmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbresmen {
        match self.bits {
            false => Usbresmen::_0,
            true => Usbresmen::_1,
        }
    }
    #[doc = "USB asynchronous wakeup from suspend mode disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbresmen::_0
    }
    #[doc = "USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interupt should only be enabled when the Transceiver is suspended."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbresmen::_1
    }
}
#[doc = "Field `USBRESMEN` writer - Asynchronous Resume Interrupt Enable"]
pub type UsbresmenW<'a, REG> = crate::BitWriter<'a, REG, Usbresmen>;
impl<'a, REG> UsbresmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB asynchronous wakeup from suspend mode disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbresmen::_0)
    }
    #[doc = "USB asynchronous wakeup from suspend mode enabled. The asynchronous resume interrupt differs from the synchronous resume interrupt in that it asynchronously detects K-state using the unfiltered state of the D+ and D- pins. This interupt should only be enabled when the Transceiver is suspended."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbresmen::_1)
    }
}
#[doc = "USB reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbreset {
    #[doc = "0: Normal USB module operation."]
    _0 = 0,
    #[doc = "1: Returns the USB module to its reset state."]
    _1 = 1,
}
impl From<Usbreset> for bool {
    #[inline(always)]
    fn from(variant: Usbreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRESET` writer - USB reset"]
pub type UsbresetW<'a, REG> = crate::BitWriter<'a, REG, Usbreset>;
impl<'a, REG> UsbresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal USB module operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbreset::_0)
    }
    #[doc = "Returns the USB module to its reset state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbreset::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Asynchronous Interrupt"]
    #[inline(always)]
    pub fn usb_resume_int(&self) -> UsbResumeIntR {
        UsbResumeIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronous USB Interrupt Detect"]
    #[inline(always)]
    pub fn sync_det(&self) -> SyncDetR {
        SyncDetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn usbresmen(&self) -> UsbresmenR {
        UsbresmenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbresmen(&mut self) -> UsbresmenW<Usbtrc0Spec> {
        UsbresmenW::new(self, 5)
    }
    #[doc = "Bit 7 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbreset(&mut self) -> UsbresetW<Usbtrc0Spec> {
        UsbresetW::new(self, 7)
    }
}
#[doc = "USB Transceiver Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`usbtrc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbtrc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usbtrc0Spec;
impl crate::RegisterSpec for Usbtrc0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbtrc0::R`](R) reader structure"]
impl crate::Readable for Usbtrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`usbtrc0::W`](W) writer structure"]
impl crate::Writable for Usbtrc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USBTRC0 to value 0"]
impl crate::Resettable for Usbtrc0Spec {
    const RESET_VALUE: u8 = 0;
}
