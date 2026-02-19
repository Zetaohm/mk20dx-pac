#[doc = "Register `CS15` reader"]
pub type R = crate::R<Cs15Spec>;
#[doc = "Register `CS15` writer"]
pub type W = crate::W<Cs15Spec>;
#[doc = "Field `TIME_STAMP` reader - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
pub type TimeStampR = crate::FieldReader<u16>;
#[doc = "Field `TIME_STAMP` writer - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
pub type TimeStampW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DLC` reader - Length of the data to be stored/transmitted."]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Length of the data to be stored/transmitted."]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTR` reader - Remote Transmission Request. One/zero for remote/data frame."]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - Remote Transmission Request. One/zero for remote/data frame."]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDE` reader - ID Extended. One/zero for extended/standard format frame."]
pub type IdeR = crate::BitReader;
#[doc = "Field `IDE` writer - ID Extended. One/zero for extended/standard format frame."]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRR` reader - Substitute Remote Request. Contains a fixed recessive bit."]
pub type SrrR = crate::BitReader;
#[doc = "Field `SRR` writer - Substitute Remote Request. Contains a fixed recessive bit."]
pub type SrrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODE` reader - Reserved"]
pub type CodeR = crate::FieldReader;
#[doc = "Field `CODE` writer - Reserved"]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub fn time_stamp(&self) -> TimeStampR {
        TimeStampR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub fn srr(&self) -> SrrR {
        SrrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Reserved"]
    #[inline(always)]
    pub fn code(&self) -> CodeR {
        CodeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp(&mut self) -> TimeStampW<Cs15Spec> {
        TimeStampW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Length of the data to be stored/transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DlcW<Cs15Spec> {
        DlcW::new(self, 16)
    }
    #[doc = "Bit 20 - Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RtrW<Cs15Spec> {
        RtrW::new(self, 20)
    }
    #[doc = "Bit 21 - ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IdeW<Cs15Spec> {
        IdeW::new(self, 21)
    }
    #[doc = "Bit 22 - Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SrrW<Cs15Spec> {
        SrrW::new(self, 22)
    }
    #[doc = "Bits 24:27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CodeW<Cs15Spec> {
        CodeW::new(self, 24)
    }
}
#[doc = "Message Buffer 15 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cs15Spec;
impl crate::RegisterSpec for Cs15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs15::R`](R) reader structure"]
impl crate::Readable for Cs15Spec {}
#[doc = "`write(|w| ..)` method takes [`cs15::W`](W) writer structure"]
impl crate::Writable for Cs15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS15 to value 0"]
impl crate::Resettable for Cs15Spec {
    const RESET_VALUE: u32 = 0;
}
