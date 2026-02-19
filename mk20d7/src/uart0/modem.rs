#[doc = "Register `MODEM` reader"]
pub type R = crate::R<ModemSpec>;
#[doc = "Register `MODEM` writer"]
pub type W = crate::W<ModemSpec>;
#[doc = "Transmitter clear-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txctse {
    #[doc = "0: CTS has no effect on the transmitter."]
    _0 = 0,
    #[doc = "1: Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    _1 = 1,
}
impl From<Txctse> for bool {
    #[inline(always)]
    fn from(variant: Txctse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCTSE` reader - Transmitter clear-to-send enable"]
pub type TxctseR = crate::BitReader<Txctse>;
impl TxctseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txctse {
        match self.bits {
            false => Txctse::_0,
            true => Txctse::_1,
        }
    }
    #[doc = "CTS has no effect on the transmitter."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txctse::_0
    }
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txctse::_1
    }
}
#[doc = "Field `TXCTSE` writer - Transmitter clear-to-send enable"]
pub type TxctseW<'a, REG> = crate::BitWriter<'a, REG, Txctse>;
impl<'a, REG> TxctseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS has no effect on the transmitter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txctse::_0)
    }
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txctse::_1)
    }
}
#[doc = "Transmitter request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txrtse {
    #[doc = "0: The transmitter has no effect on RTS."]
    _0 = 0,
    #[doc = "1: When a character is placed into an empty transmitter data buffer(FIFO), RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer(FIFO) and shift register are completely sent, including the last stop bit."]
    _1 = 1,
}
impl From<Txrtse> for bool {
    #[inline(always)]
    fn from(variant: Txrtse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRTSE` reader - Transmitter request-to-send enable"]
pub type TxrtseR = crate::BitReader<Txrtse>;
impl TxrtseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txrtse {
        match self.bits {
            false => Txrtse::_0,
            true => Txrtse::_1,
        }
    }
    #[doc = "The transmitter has no effect on RTS."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txrtse::_0
    }
    #[doc = "When a character is placed into an empty transmitter data buffer(FIFO), RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer(FIFO) and shift register are completely sent, including the last stop bit."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txrtse::_1
    }
}
#[doc = "Field `TXRTSE` writer - Transmitter request-to-send enable"]
pub type TxrtseW<'a, REG> = crate::BitWriter<'a, REG, Txrtse>;
impl<'a, REG> TxrtseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmitter has no effect on RTS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txrtse::_0)
    }
    #[doc = "When a character is placed into an empty transmitter data buffer(FIFO), RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer(FIFO) and shift register are completely sent, including the last stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txrtse::_1)
    }
}
#[doc = "Transmitter request-to-send polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txrtspol {
    #[doc = "0: Transmitter RTS is active low."]
    _0 = 0,
    #[doc = "1: Transmitter RTS is active high."]
    _1 = 1,
}
impl From<Txrtspol> for bool {
    #[inline(always)]
    fn from(variant: Txrtspol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRTSPOL` reader - Transmitter request-to-send polarity"]
pub type TxrtspolR = crate::BitReader<Txrtspol>;
impl TxrtspolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txrtspol {
        match self.bits {
            false => Txrtspol::_0,
            true => Txrtspol::_1,
        }
    }
    #[doc = "Transmitter RTS is active low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txrtspol::_0
    }
    #[doc = "Transmitter RTS is active high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txrtspol::_1
    }
}
#[doc = "Field `TXRTSPOL` writer - Transmitter request-to-send polarity"]
pub type TxrtspolW<'a, REG> = crate::BitWriter<'a, REG, Txrtspol>;
impl<'a, REG> TxrtspolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter RTS is active low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txrtspol::_0)
    }
    #[doc = "Transmitter RTS is active high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txrtspol::_1)
    }
}
#[doc = "Receiver request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxrtse {
    #[doc = "0: The receiver has no effect on RTS."]
    _0 = 0,
    #[doc = "1: RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]."]
    _1 = 1,
}
impl From<Rxrtse> for bool {
    #[inline(always)]
    fn from(variant: Rxrtse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRTSE` reader - Receiver request-to-send enable"]
pub type RxrtseR = crate::BitReader<Rxrtse>;
impl RxrtseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxrtse {
        match self.bits {
            false => Rxrtse::_0,
            true => Rxrtse::_1,
        }
    }
    #[doc = "The receiver has no effect on RTS."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxrtse::_0
    }
    #[doc = "RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxrtse::_1
    }
}
#[doc = "Field `RXRTSE` writer - Receiver request-to-send enable"]
pub type RxrtseW<'a, REG> = crate::BitWriter<'a, REG, Rxrtse>;
impl<'a, REG> RxrtseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receiver has no effect on RTS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxrtse::_0)
    }
    #[doc = "RTS is deasserted if the number of characters in the receiver data register (FIFO) is equal to or greater than RWFIFO\\[RXWATER\\]. RTS is asserted when the number of characters in the receiver data register (FIFO) is less than RWFIFO\\[RXWATER\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxrtse::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    pub fn txctse(&self) -> TxctseR {
        TxctseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    pub fn txrtse(&self) -> TxrtseR {
        TxrtseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    pub fn txrtspol(&self) -> TxrtspolR {
        TxrtspolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    pub fn rxrtse(&self) -> RxrtseR {
        RxrtseR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn txctse(&mut self) -> TxctseW<ModemSpec> {
        TxctseW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrtse(&mut self) -> TxrtseW<ModemSpec> {
        TxrtseW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    #[must_use]
    pub fn txrtspol(&mut self) -> TxrtspolW<ModemSpec> {
        TxrtspolW::new(self, 2)
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrtse(&mut self) -> RxrtseW<ModemSpec> {
        RxrtseW::new(self, 3)
    }
}
#[doc = "UART Modem Register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModemSpec;
impl crate::RegisterSpec for ModemSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`modem::R`](R) reader structure"]
impl crate::Readable for ModemSpec {}
#[doc = "`write(|w| ..)` method takes [`modem::W`](W) writer structure"]
impl crate::Writable for ModemSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MODEM to value 0"]
impl crate::Resettable for ModemSpec {
    const RESET_VALUE: u8 = 0;
}
