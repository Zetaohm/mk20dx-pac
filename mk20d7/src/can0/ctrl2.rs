#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Entire Frame Arbitration Field Comparison Enable for Rx Mailboxes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eacen {
    #[doc = "0: Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    _0 = 0,
    #[doc = "1: Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    _1 = 1,
}
impl From<Eacen> for bool {
    #[inline(always)]
    fn from(variant: Eacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EACEN` reader - Entire Frame Arbitration Field Comparison Enable for Rx Mailboxes"]
pub type EacenR = crate::BitReader<Eacen>;
impl EacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eacen {
        match self.bits {
            false => Eacen::_0,
            true => Eacen::_1,
        }
    }
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eacen::_0
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eacen::_1
    }
}
#[doc = "Field `EACEN` writer - Entire Frame Arbitration Field Comparison Enable for Rx Mailboxes"]
pub type EacenW<'a, REG> = crate::BitWriter<'a, REG, Eacen>;
impl<'a, REG> EacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eacen::_0)
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eacen::_1)
    }
}
#[doc = "Remote Request Storing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrs {
    #[doc = "0: Remote Response Frame is generated."]
    _0 = 0,
    #[doc = "1: Remote Request Frame is stored."]
    _1 = 1,
}
impl From<Rrs> for bool {
    #[inline(always)]
    fn from(variant: Rrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRS` reader - Remote Request Storing"]
pub type RrsR = crate::BitReader<Rrs>;
impl RrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrs {
        match self.bits {
            false => Rrs::_0,
            true => Rrs::_1,
        }
    }
    #[doc = "Remote Response Frame is generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rrs::_0
    }
    #[doc = "Remote Request Frame is stored."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rrs::_1
    }
}
#[doc = "Field `RRS` writer - Remote Request Storing"]
pub type RrsW<'a, REG> = crate::BitWriter<'a, REG, Rrs>;
impl<'a, REG> RrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Remote Response Frame is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::_0)
    }
    #[doc = "Remote Request Frame is stored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::_1)
    }
}
#[doc = "Mailboxes Reception Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrp {
    #[doc = "0: Matching starts from Rx FIFO and continues on Mailboxes."]
    _0 = 0,
    #[doc = "1: Matching starts from Mailboxes and continues on Rx FIFO."]
    _1 = 1,
}
impl From<Mrp> for bool {
    #[inline(always)]
    fn from(variant: Mrp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRP` reader - Mailboxes Reception Priority"]
pub type MrpR = crate::BitReader<Mrp>;
impl MrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrp {
        match self.bits {
            false => Mrp::_0,
            true => Mrp::_1,
        }
    }
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mrp::_0
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mrp::_1
    }
}
#[doc = "Field `MRP` writer - Mailboxes Reception Priority"]
pub type MrpW<'a, REG> = crate::BitWriter<'a, REG, Mrp>;
impl<'a, REG> MrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mrp::_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mrp::_1)
    }
}
#[doc = "Field `TASD` reader - Tx Arbitration Start Delay"]
pub type TasdR = crate::FieldReader;
#[doc = "Field `TASD` writer - Tx Arbitration Start Delay"]
pub type TasdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RFFN` reader - Number of Rx FIFO Filters"]
pub type RffnR = crate::FieldReader;
#[doc = "Field `RFFN` writer - Number of Rx FIFO Filters"]
pub type RffnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Write-Access to Memory in Freeze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrmfrz {
    #[doc = "0: Maintain the write access restrictions."]
    _0 = 0,
    #[doc = "1: Enable unrestricted write access to FlexCAN memory."]
    _1 = 1,
}
impl From<Wrmfrz> for bool {
    #[inline(always)]
    fn from(variant: Wrmfrz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRMFRZ` reader - Write-Access to Memory in Freeze mode"]
pub type WrmfrzR = crate::BitReader<Wrmfrz>;
impl WrmfrzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrmfrz {
        match self.bits {
            false => Wrmfrz::_0,
            true => Wrmfrz::_1,
        }
    }
    #[doc = "Maintain the write access restrictions."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wrmfrz::_0
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wrmfrz::_1
    }
}
#[doc = "Field `WRMFRZ` writer - Write-Access to Memory in Freeze mode"]
pub type WrmfrzW<'a, REG> = crate::BitWriter<'a, REG, Wrmfrz>;
impl<'a, REG> WrmfrzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Maintain the write access restrictions."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrmfrz::_0)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrmfrz::_1)
    }
}
impl R {
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable for Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&self) -> EacenR {
        EacenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&self) -> RrsR {
        RrsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&self) -> MrpR {
        MrpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&self) -> TasdR {
        TasdR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Number of Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&self) -> RffnR {
        RffnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Write-Access to Memory in Freeze mode"]
    #[inline(always)]
    pub fn wrmfrz(&self) -> WrmfrzR {
        WrmfrzR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable for Rx Mailboxes"]
    #[inline(always)]
    #[must_use]
    pub fn eacen(&mut self) -> EacenW<Ctrl2Spec> {
        EacenW::new(self, 16)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RrsW<Ctrl2Spec> {
        RrsW::new(self, 17)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    #[must_use]
    pub fn mrp(&mut self) -> MrpW<Ctrl2Spec> {
        MrpW::new(self, 18)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tasd(&mut self) -> TasdW<Ctrl2Spec> {
        TasdW::new(self, 19)
    }
    #[doc = "Bits 24:27 - Number of Rx FIFO Filters"]
    #[inline(always)]
    #[must_use]
    pub fn rffn(&mut self) -> RffnW<Ctrl2Spec> {
        RffnW::new(self, 24)
    }
    #[doc = "Bit 28 - Write-Access to Memory in Freeze mode"]
    #[inline(always)]
    #[must_use]
    pub fn wrmfrz(&mut self) -> WrmfrzW<Ctrl2Spec> {
        WrmfrzW::new(self, 28)
    }
}
#[doc = "Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x00b0_0000"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0x00b0_0000;
}
