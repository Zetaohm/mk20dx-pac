#[doc = "Register `SCGC4` reader"]
pub type R = crate::R<Scgc4Spec>;
#[doc = "Register `SCGC4` writer"]
pub type W = crate::W<Scgc4Spec>;
#[doc = "EWM Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewm {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Ewm> for bool {
    #[inline(always)]
    fn from(variant: Ewm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWM` reader - EWM Clock Gate Control"]
pub type EwmR = crate::BitReader<Ewm>;
impl EwmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewm {
        match self.bits {
            false => Ewm::_0,
            true => Ewm::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ewm::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ewm::_1
    }
}
#[doc = "Field `EWM` writer - EWM Clock Gate Control"]
pub type EwmW<'a, REG> = crate::BitWriter<'a, REG, Ewm>;
impl<'a, REG> EwmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ewm::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ewm::_1)
    }
}
#[doc = "CMT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmt {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Cmt> for bool {
    #[inline(always)]
    fn from(variant: Cmt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMT` reader - CMT Clock Gate Control"]
pub type CmtR = crate::BitReader<Cmt>;
impl CmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmt {
        match self.bits {
            false => Cmt::_0,
            true => Cmt::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmt::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmt::_1
    }
}
#[doc = "Field `CMT` writer - CMT Clock Gate Control"]
pub type CmtW<'a, REG> = crate::BitWriter<'a, REG, Cmt>;
impl<'a, REG> CmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmt::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmt::_1)
    }
}
#[doc = "I2C0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0 {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<I2c0> for bool {
    #[inline(always)]
    fn from(variant: I2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` reader - I2C0 Clock Gate Control"]
pub type I2c0R = crate::BitReader<I2c0>;
impl I2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c0 {
        match self.bits {
            false => I2c0::_0,
            true => I2c0::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2c0::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2c0::_1
    }
}
#[doc = "Field `I2C0` writer - I2C0 Clock Gate Control"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG, I2c0>;
impl<'a, REG> I2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::_1)
    }
}
#[doc = "UART0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart0 {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Uart0> for bool {
    #[inline(always)]
    fn from(variant: Uart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` reader - UART0 Clock Gate Control"]
pub type Uart0R = crate::BitReader<Uart0>;
impl Uart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart0 {
        match self.bits {
            false => Uart0::_0,
            true => Uart0::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uart0::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uart0::_1
    }
}
#[doc = "Field `UART0` writer - UART0 Clock Gate Control"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG, Uart0>;
impl<'a, REG> Uart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0::_1)
    }
}
#[doc = "UART1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1 {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Uart1> for bool {
    #[inline(always)]
    fn from(variant: Uart1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1` reader - UART1 Clock Gate Control"]
pub type Uart1R = crate::BitReader<Uart1>;
impl Uart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1 {
        match self.bits {
            false => Uart1::_0,
            true => Uart1::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uart1::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uart1::_1
    }
}
#[doc = "Field `UART1` writer - UART1 Clock Gate Control"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG, Uart1>;
impl<'a, REG> Uart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1::_1)
    }
}
#[doc = "UART2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart2 {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Uart2> for bool {
    #[inline(always)]
    fn from(variant: Uart2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2` reader - UART2 Clock Gate Control"]
pub type Uart2R = crate::BitReader<Uart2>;
impl Uart2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart2 {
        match self.bits {
            false => Uart2::_0,
            true => Uart2::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uart2::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uart2::_1
    }
}
#[doc = "Field `UART2` writer - UART2 Clock Gate Control"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG, Uart2>;
impl<'a, REG> Uart2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2::_1)
    }
}
#[doc = "USB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbotg {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Usbotg> for bool {
    #[inline(always)]
    fn from(variant: Usbotg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTG` reader - USB Clock Gate Control"]
pub type UsbotgR = crate::BitReader<Usbotg>;
impl UsbotgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbotg {
        match self.bits {
            false => Usbotg::_0,
            true => Usbotg::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbotg::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbotg::_1
    }
}
#[doc = "Field `USBOTG` writer - USB Clock Gate Control"]
pub type UsbotgW<'a, REG> = crate::BitWriter<'a, REG, Usbotg>;
impl<'a, REG> UsbotgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbotg::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbotg::_1)
    }
}
#[doc = "Comparator Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Cmp> for bool {
    #[inline(always)]
    fn from(variant: Cmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP` reader - Comparator Clock Gate Control"]
pub type CmpR = crate::BitReader<Cmp>;
impl CmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp {
        match self.bits {
            false => Cmp::_0,
            true => Cmp::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmp::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmp::_1
    }
}
#[doc = "Field `CMP` writer - Comparator Clock Gate Control"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG, Cmp>;
impl<'a, REG> CmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp::_1)
    }
}
#[doc = "VREF Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vref {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<Vref> for bool {
    #[inline(always)]
    fn from(variant: Vref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREF` reader - VREF Clock Gate Control"]
pub type VrefR = crate::BitReader<Vref>;
impl VrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vref {
        match self.bits {
            false => Vref::_0,
            true => Vref::_1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vref::_0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vref::_1
    }
}
#[doc = "Field `VREF` writer - VREF Clock Gate Control"]
pub type VrefW<'a, REG> = crate::BitWriter<'a, REG, Vref>;
impl<'a, REG> VrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vref::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vref::_1)
    }
}
impl R {
    #[doc = "Bit 1 - EWM Clock Gate Control"]
    #[inline(always)]
    pub fn ewm(&self) -> EwmR {
        EwmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMT Clock Gate Control"]
    #[inline(always)]
    pub fn cmt(&self) -> CmtR {
        CmtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C0 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - UART0 Clock Gate Control"]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART1 Clock Gate Control"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART2 Clock Gate Control"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Clock Gate Control"]
    #[inline(always)]
    pub fn usbotg(&self) -> UsbotgR {
        UsbotgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Comparator Clock Gate Control"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF Clock Gate Control"]
    #[inline(always)]
    pub fn vref(&self) -> VrefR {
        VrefR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - EWM Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ewm(&mut self) -> EwmW<Scgc4Spec> {
        EwmW::new(self, 1)
    }
    #[doc = "Bit 2 - CMT Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn cmt(&mut self) -> CmtW<Scgc4Spec> {
        CmtW::new(self, 2)
    }
    #[doc = "Bit 6 - I2C0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2c0W<Scgc4Spec> {
        I2c0W::new(self, 6)
    }
    #[doc = "Bit 10 - UART0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> Uart0W<Scgc4Spec> {
        Uart0W::new(self, 10)
    }
    #[doc = "Bit 11 - UART1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> Uart1W<Scgc4Spec> {
        Uart1W::new(self, 11)
    }
    #[doc = "Bit 12 - UART2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> Uart2W<Scgc4Spec> {
        Uart2W::new(self, 12)
    }
    #[doc = "Bit 18 - USB Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbotg(&mut self) -> UsbotgW<Scgc4Spec> {
        UsbotgW::new(self, 18)
    }
    #[doc = "Bit 19 - Comparator Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CmpW<Scgc4Spec> {
        CmpW::new(self, 19)
    }
    #[doc = "Bit 20 - VREF Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn vref(&mut self) -> VrefW<Scgc4Spec> {
        VrefW::new(self, 20)
    }
}
#[doc = "System Clock Gating Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc4Spec;
impl crate::RegisterSpec for Scgc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc4::R`](R) reader structure"]
impl crate::Readable for Scgc4Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc4::W`](W) writer structure"]
impl crate::Writable for Scgc4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGC4 to value 0xf010_0030"]
impl crate::Resettable for Scgc4Spec {
    const RESET_VALUE: u32 = 0xf010_0030;
}
