#[doc = "Register `SOPT5` reader"]
pub type R = crate::R<Sopt5Spec>;
#[doc = "Register `SOPT5` writer"]
pub type W = crate::W<Sopt5Spec>;
#[doc = "UART 0 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart0txsrc {
    #[doc = "0: UART0_TX pin"]
    _0 = 0,
    #[doc = "1: UART0_TX pin modulated with FTM1 channel 0 output"]
    _1 = 1,
}
impl From<Uart0txsrc> for u8 {
    #[inline(always)]
    fn from(variant: Uart0txsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart0txsrc {
    type Ux = u8;
}
impl crate::IsEnum for Uart0txsrc {}
#[doc = "Field `UART0TXSRC` reader - UART 0 transmit data source select"]
pub type Uart0txsrcR = crate::FieldReader<Uart0txsrc>;
impl Uart0txsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uart0txsrc> {
        match self.bits {
            0 => Some(Uart0txsrc::_0),
            1 => Some(Uart0txsrc::_1),
            _ => None,
        }
    }
    #[doc = "UART0_TX pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uart0txsrc::_0
    }
    #[doc = "UART0_TX pin modulated with FTM1 channel 0 output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uart0txsrc::_1
    }
}
#[doc = "Field `UART0TXSRC` writer - UART 0 transmit data source select"]
pub type Uart0txsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart0txsrc>;
impl<'a, REG> Uart0txsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART0_TX pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0txsrc::_0)
    }
    #[doc = "UART0_TX pin modulated with FTM1 channel 0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0txsrc::_1)
    }
}
#[doc = "UART 0 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart0rxsrc {
    #[doc = "0: UART0_RX pin"]
    _00 = 0,
    #[doc = "1: CMP0"]
    _01 = 1,
    #[doc = "2: CMP1"]
    _10 = 2,
}
impl From<Uart0rxsrc> for u8 {
    #[inline(always)]
    fn from(variant: Uart0rxsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart0rxsrc {
    type Ux = u8;
}
impl crate::IsEnum for Uart0rxsrc {}
#[doc = "Field `UART0RXSRC` reader - UART 0 receive data source select"]
pub type Uart0rxsrcR = crate::FieldReader<Uart0rxsrc>;
impl Uart0rxsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uart0rxsrc> {
        match self.bits {
            0 => Some(Uart0rxsrc::_00),
            1 => Some(Uart0rxsrc::_01),
            2 => Some(Uart0rxsrc::_10),
            _ => None,
        }
    }
    #[doc = "UART0_RX pin"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Uart0rxsrc::_00
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Uart0rxsrc::_01
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Uart0rxsrc::_10
    }
}
#[doc = "Field `UART0RXSRC` writer - UART 0 receive data source select"]
pub type Uart0rxsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart0rxsrc>;
impl<'a, REG> Uart0rxsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART0_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0rxsrc::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0rxsrc::_01)
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0rxsrc::_10)
    }
}
#[doc = "UART 1 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart1txsrc {
    #[doc = "0: UART1_TX pin"]
    _0 = 0,
    #[doc = "1: UART1_TX pin modulated with FTM1 channel 0 output"]
    _1 = 1,
}
impl From<Uart1txsrc> for u8 {
    #[inline(always)]
    fn from(variant: Uart1txsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart1txsrc {
    type Ux = u8;
}
impl crate::IsEnum for Uart1txsrc {}
#[doc = "Field `UART1TXSRC` reader - UART 1 transmit data source select"]
pub type Uart1txsrcR = crate::FieldReader<Uart1txsrc>;
impl Uart1txsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uart1txsrc> {
        match self.bits {
            0 => Some(Uart1txsrc::_0),
            1 => Some(Uart1txsrc::_1),
            _ => None,
        }
    }
    #[doc = "UART1_TX pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uart1txsrc::_0
    }
    #[doc = "UART1_TX pin modulated with FTM1 channel 0 output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uart1txsrc::_1
    }
}
#[doc = "Field `UART1TXSRC` writer - UART 1 transmit data source select"]
pub type Uart1txsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart1txsrc>;
impl<'a, REG> Uart1txsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART1_TX pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1txsrc::_0)
    }
    #[doc = "UART1_TX pin modulated with FTM1 channel 0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1txsrc::_1)
    }
}
#[doc = "UART 1 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart1rxsrc {
    #[doc = "0: UART1_RX pin"]
    _00 = 0,
    #[doc = "1: CMP0"]
    _01 = 1,
    #[doc = "2: CMP1"]
    _10 = 2,
}
impl From<Uart1rxsrc> for u8 {
    #[inline(always)]
    fn from(variant: Uart1rxsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart1rxsrc {
    type Ux = u8;
}
impl crate::IsEnum for Uart1rxsrc {}
#[doc = "Field `UART1RXSRC` reader - UART 1 receive data source select"]
pub type Uart1rxsrcR = crate::FieldReader<Uart1rxsrc>;
impl Uart1rxsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uart1rxsrc> {
        match self.bits {
            0 => Some(Uart1rxsrc::_00),
            1 => Some(Uart1rxsrc::_01),
            2 => Some(Uart1rxsrc::_10),
            _ => None,
        }
    }
    #[doc = "UART1_RX pin"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Uart1rxsrc::_00
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Uart1rxsrc::_01
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Uart1rxsrc::_10
    }
}
#[doc = "Field `UART1RXSRC` writer - UART 1 receive data source select"]
pub type Uart1rxsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart1rxsrc>;
impl<'a, REG> Uart1rxsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART1_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1rxsrc::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1rxsrc::_01)
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1rxsrc::_10)
    }
}
impl R {
    #[doc = "Bits 0:1 - UART 0 transmit data source select"]
    #[inline(always)]
    pub fn uart0txsrc(&self) -> Uart0txsrcR {
        Uart0txsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - UART 0 receive data source select"]
    #[inline(always)]
    pub fn uart0rxsrc(&self) -> Uart0rxsrcR {
        Uart0rxsrcR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - UART 1 transmit data source select"]
    #[inline(always)]
    pub fn uart1txsrc(&self) -> Uart1txsrcR {
        Uart1txsrcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - UART 1 receive data source select"]
    #[inline(always)]
    pub fn uart1rxsrc(&self) -> Uart1rxsrcR {
        Uart1rxsrcR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART 0 transmit data source select"]
    #[inline(always)]
    #[must_use]
    pub fn uart0txsrc(&mut self) -> Uart0txsrcW<Sopt5Spec> {
        Uart0txsrcW::new(self, 0)
    }
    #[doc = "Bits 2:3 - UART 0 receive data source select"]
    #[inline(always)]
    #[must_use]
    pub fn uart0rxsrc(&mut self) -> Uart0rxsrcW<Sopt5Spec> {
        Uart0rxsrcW::new(self, 2)
    }
    #[doc = "Bits 4:5 - UART 1 transmit data source select"]
    #[inline(always)]
    #[must_use]
    pub fn uart1txsrc(&mut self) -> Uart1txsrcW<Sopt5Spec> {
        Uart1txsrcW::new(self, 4)
    }
    #[doc = "Bits 6:7 - UART 1 receive data source select"]
    #[inline(always)]
    #[must_use]
    pub fn uart1rxsrc(&mut self) -> Uart1rxsrcW<Sopt5Spec> {
        Uart1rxsrcW::new(self, 6)
    }
}
#[doc = "System Options Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt5Spec;
impl crate::RegisterSpec for Sopt5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt5::R`](R) reader structure"]
impl crate::Readable for Sopt5Spec {}
#[doc = "`write(|w| ..)` method takes [`sopt5::W`](W) writer structure"]
impl crate::Writable for Sopt5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT5 to value 0"]
impl crate::Resettable for Sopt5Spec {
    const RESET_VALUE: u32 = 0;
}
