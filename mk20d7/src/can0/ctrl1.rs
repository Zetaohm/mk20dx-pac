#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `PROPSEG` reader - Propagation Segment"]
pub type PropsegR = crate::FieldReader;
#[doc = "Field `PROPSEG` writer - Propagation Segment"]
pub type PropsegW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Listen-Only Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lom {
    #[doc = "0: Listen-Only Mode is deactivated."]
    _0 = 0,
    #[doc = "1: FlexCAN module operates in Listen-Only Mode."]
    _1 = 1,
}
impl From<Lom> for bool {
    #[inline(always)]
    fn from(variant: Lom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOM` reader - Listen-Only Mode"]
pub type LomR = crate::BitReader<Lom>;
impl LomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lom {
        match self.bits {
            false => Lom::_0,
            true => Lom::_1,
        }
    }
    #[doc = "Listen-Only Mode is deactivated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lom::_0
    }
    #[doc = "FlexCAN module operates in Listen-Only Mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lom::_1
    }
}
#[doc = "Field `LOM` writer - Listen-Only Mode"]
pub type LomW<'a, REG> = crate::BitWriter<'a, REG, Lom>;
impl<'a, REG> LomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Listen-Only Mode is deactivated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lom::_0)
    }
    #[doc = "FlexCAN module operates in Listen-Only Mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lom::_1)
    }
}
#[doc = "Lowest Buffer Transmitted First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbuf {
    #[doc = "0: Buffer with highest priority is transmitted first."]
    _0 = 0,
    #[doc = "1: Lowest number buffer is transmitted first."]
    _1 = 1,
}
impl From<Lbuf> for bool {
    #[inline(always)]
    fn from(variant: Lbuf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBUF` reader - Lowest Buffer Transmitted First"]
pub type LbufR = crate::BitReader<Lbuf>;
impl LbufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbuf {
        match self.bits {
            false => Lbuf::_0,
            true => Lbuf::_1,
        }
    }
    #[doc = "Buffer with highest priority is transmitted first."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lbuf::_0
    }
    #[doc = "Lowest number buffer is transmitted first."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lbuf::_1
    }
}
#[doc = "Field `LBUF` writer - Lowest Buffer Transmitted First"]
pub type LbufW<'a, REG> = crate::BitWriter<'a, REG, Lbuf>;
impl<'a, REG> LbufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Buffer with highest priority is transmitted first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbuf::_0)
    }
    #[doc = "Lowest number buffer is transmitted first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbuf::_1)
    }
}
#[doc = "Timer Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsyn {
    #[doc = "0: Timer Sync feature disabled"]
    _0 = 0,
    #[doc = "1: Timer Sync feature enabled"]
    _1 = 1,
}
impl From<Tsyn> for bool {
    #[inline(always)]
    fn from(variant: Tsyn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSYN` reader - Timer Sync"]
pub type TsynR = crate::BitReader<Tsyn>;
impl TsynR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsyn {
        match self.bits {
            false => Tsyn::_0,
            true => Tsyn::_1,
        }
    }
    #[doc = "Timer Sync feature disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsyn::_0
    }
    #[doc = "Timer Sync feature enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsyn::_1
    }
}
#[doc = "Field `TSYN` writer - Timer Sync"]
pub type TsynW<'a, REG> = crate::BitWriter<'a, REG, Tsyn>;
impl<'a, REG> TsynW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Sync feature disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsyn::_0)
    }
    #[doc = "Timer Sync feature enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsyn::_1)
    }
}
#[doc = "Bus Off Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boffrec {
    #[doc = "0: Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
    _0 = 0,
    #[doc = "1: Automatic recovering from Bus Off state disabled"]
    _1 = 1,
}
impl From<Boffrec> for bool {
    #[inline(always)]
    fn from(variant: Boffrec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFREC` reader - Bus Off Recovery"]
pub type BoffrecR = crate::BitReader<Boffrec>;
impl BoffrecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boffrec {
        match self.bits {
            false => Boffrec::_0,
            true => Boffrec::_1,
        }
    }
    #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Boffrec::_0
    }
    #[doc = "Automatic recovering from Bus Off state disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Boffrec::_1
    }
}
#[doc = "Field `BOFFREC` writer - Bus Off Recovery"]
pub type BoffrecW<'a, REG> = crate::BitWriter<'a, REG, Boffrec>;
impl<'a, REG> BoffrecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Boffrec::_0)
    }
    #[doc = "Automatic recovering from Bus Off state disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Boffrec::_1)
    }
}
#[doc = "CAN Bit Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smp {
    #[doc = "0: Just one sample is used to determine the bit value."]
    _0 = 0,
    #[doc = "1: Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples; a majority rule is used."]
    _1 = 1,
}
impl From<Smp> for bool {
    #[inline(always)]
    fn from(variant: Smp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMP` reader - CAN Bit Sampling"]
pub type SmpR = crate::BitReader<Smp>;
impl SmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smp {
        match self.bits {
            false => Smp::_0,
            true => Smp::_1,
        }
    }
    #[doc = "Just one sample is used to determine the bit value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Smp::_0
    }
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples; a majority rule is used."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Smp::_1
    }
}
#[doc = "Field `SMP` writer - CAN Bit Sampling"]
pub type SmpW<'a, REG> = crate::BitWriter<'a, REG, Smp>;
impl<'a, REG> SmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Just one sample is used to determine the bit value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Smp::_0)
    }
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples; a majority rule is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Smp::_1)
    }
}
#[doc = "Rx Warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwrnmsk {
    #[doc = "0: Rx Warning Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Rx Warning Interrupt enabled"]
    _1 = 1,
}
impl From<Rwrnmsk> for bool {
    #[inline(always)]
    fn from(variant: Rwrnmsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWRNMSK` reader - Rx Warning Interrupt Mask"]
pub type RwrnmskR = crate::BitReader<Rwrnmsk>;
impl RwrnmskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwrnmsk {
        match self.bits {
            false => Rwrnmsk::_0,
            true => Rwrnmsk::_1,
        }
    }
    #[doc = "Rx Warning Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rwrnmsk::_0
    }
    #[doc = "Rx Warning Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rwrnmsk::_1
    }
}
#[doc = "Field `RWRNMSK` writer - Rx Warning Interrupt Mask"]
pub type RwrnmskW<'a, REG> = crate::BitWriter<'a, REG, Rwrnmsk>;
impl<'a, REG> RwrnmskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx Warning Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwrnmsk::_0)
    }
    #[doc = "Rx Warning Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwrnmsk::_1)
    }
}
#[doc = "Tx Warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twrnmsk {
    #[doc = "0: Tx Warning Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Tx Warning Interrupt enabled"]
    _1 = 1,
}
impl From<Twrnmsk> for bool {
    #[inline(always)]
    fn from(variant: Twrnmsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWRNMSK` reader - Tx Warning Interrupt Mask"]
pub type TwrnmskR = crate::BitReader<Twrnmsk>;
impl TwrnmskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Twrnmsk {
        match self.bits {
            false => Twrnmsk::_0,
            true => Twrnmsk::_1,
        }
    }
    #[doc = "Tx Warning Interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Twrnmsk::_0
    }
    #[doc = "Tx Warning Interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Twrnmsk::_1
    }
}
#[doc = "Field `TWRNMSK` writer - Tx Warning Interrupt Mask"]
pub type TwrnmskW<'a, REG> = crate::BitWriter<'a, REG, Twrnmsk>;
impl<'a, REG> TwrnmskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx Warning Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Twrnmsk::_0)
    }
    #[doc = "Tx Warning Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Twrnmsk::_1)
    }
}
#[doc = "Loop Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpb {
    #[doc = "0: Loop Back disabled"]
    _0 = 0,
    #[doc = "1: Loop Back enabled"]
    _1 = 1,
}
impl From<Lpb> for bool {
    #[inline(always)]
    fn from(variant: Lpb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPB` reader - Loop Back Mode"]
pub type LpbR = crate::BitReader<Lpb>;
impl LpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpb {
        match self.bits {
            false => Lpb::_0,
            true => Lpb::_1,
        }
    }
    #[doc = "Loop Back disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lpb::_0
    }
    #[doc = "Loop Back enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lpb::_1
    }
}
#[doc = "Field `LPB` writer - Loop Back Mode"]
pub type LpbW<'a, REG> = crate::BitWriter<'a, REG, Lpb>;
impl<'a, REG> LpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop Back disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpb::_0)
    }
    #[doc = "Loop Back enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpb::_1)
    }
}
#[doc = "CAN Engine Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksrc {
    #[doc = "0: The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    _0 = 0,
    #[doc = "1: The CAN engine clock source is the peripheral clock."]
    _1 = 1,
}
impl From<Clksrc> for bool {
    #[inline(always)]
    fn from(variant: Clksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - CAN Engine Clock Source"]
pub type ClksrcR = crate::BitReader<Clksrc>;
impl ClksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksrc {
        match self.bits {
            false => Clksrc::_0,
            true => Clksrc::_1,
        }
    }
    #[doc = "The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clksrc::_0
    }
    #[doc = "The CAN engine clock source is the peripheral clock."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clksrc::_1
    }
}
#[doc = "Field `CLKSRC` writer - CAN Engine Clock Source"]
pub type ClksrcW<'a, REG> = crate::BitWriter<'a, REG, Clksrc>;
impl<'a, REG> ClksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::_0)
    }
    #[doc = "The CAN engine clock source is the peripheral clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::_1)
    }
}
#[doc = "Error Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errmsk {
    #[doc = "0: Error interrupt disabled"]
    _0 = 0,
    #[doc = "1: Error interrupt enabled"]
    _1 = 1,
}
impl From<Errmsk> for bool {
    #[inline(always)]
    fn from(variant: Errmsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRMSK` reader - Error Mask"]
pub type ErrmskR = crate::BitReader<Errmsk>;
impl ErrmskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errmsk {
        match self.bits {
            false => Errmsk::_0,
            true => Errmsk::_1,
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Errmsk::_0
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Errmsk::_1
    }
}
#[doc = "Field `ERRMSK` writer - Error Mask"]
pub type ErrmskW<'a, REG> = crate::BitWriter<'a, REG, Errmsk>;
impl<'a, REG> ErrmskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Errmsk::_0)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Errmsk::_1)
    }
}
#[doc = "Bus Off Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boffmsk {
    #[doc = "0: Bus Off interrupt disabled"]
    _0 = 0,
    #[doc = "1: Bus Off interrupt enabled"]
    _1 = 1,
}
impl From<Boffmsk> for bool {
    #[inline(always)]
    fn from(variant: Boffmsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFMSK` reader - Bus Off Mask"]
pub type BoffmskR = crate::BitReader<Boffmsk>;
impl BoffmskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boffmsk {
        match self.bits {
            false => Boffmsk::_0,
            true => Boffmsk::_1,
        }
    }
    #[doc = "Bus Off interrupt disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Boffmsk::_0
    }
    #[doc = "Bus Off interrupt enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Boffmsk::_1
    }
}
#[doc = "Field `BOFFMSK` writer - Bus Off Mask"]
pub type BoffmskW<'a, REG> = crate::BitWriter<'a, REG, Boffmsk>;
impl<'a, REG> BoffmskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus Off interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Boffmsk::_0)
    }
    #[doc = "Bus Off interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Boffmsk::_1)
    }
}
#[doc = "Field `PSEG2` reader - Phase Segment 2"]
pub type Pseg2R = crate::FieldReader;
#[doc = "Field `PSEG2` writer - Phase Segment 2"]
pub type Pseg2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PSEG1` reader - Phase Segment 1"]
pub type Pseg1R = crate::FieldReader;
#[doc = "Field `PSEG1` writer - Phase Segment 1"]
pub type Pseg1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RJW` reader - Resync Jump Width"]
pub type RjwR = crate::FieldReader;
#[doc = "Field `RJW` writer - Resync Jump Width"]
pub type RjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESDIV` reader - Prescaler Division Factor"]
pub type PresdivR = crate::FieldReader;
#[doc = "Field `PRESDIV` writer - Prescaler Division Factor"]
pub type PresdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Propagation Segment"]
    #[inline(always)]
    pub fn propseg(&self) -> PropsegR {
        PropsegR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Listen-Only Mode"]
    #[inline(always)]
    pub fn lom(&self) -> LomR {
        LomR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lowest Buffer Transmitted First"]
    #[inline(always)]
    pub fn lbuf(&self) -> LbufR {
        LbufR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Sync"]
    #[inline(always)]
    pub fn tsyn(&self) -> TsynR {
        TsynR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bus Off Recovery"]
    #[inline(always)]
    pub fn boffrec(&self) -> BoffrecR {
        BoffrecR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN Bit Sampling"]
    #[inline(always)]
    pub fn smp(&self) -> SmpR {
        SmpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Rx Warning Interrupt Mask"]
    #[inline(always)]
    pub fn rwrnmsk(&self) -> RwrnmskR {
        RwrnmskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Warning Interrupt Mask"]
    #[inline(always)]
    pub fn twrnmsk(&self) -> TwrnmskR {
        TwrnmskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loop Back Mode"]
    #[inline(always)]
    pub fn lpb(&self) -> LpbR {
        LpbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CAN Engine Clock Source"]
    #[inline(always)]
    pub fn clksrc(&self) -> ClksrcR {
        ClksrcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Mask"]
    #[inline(always)]
    pub fn errmsk(&self) -> ErrmskR {
        ErrmskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus Off Mask"]
    #[inline(always)]
    pub fn boffmsk(&self) -> BoffmskR {
        BoffmskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Phase Segment 2"]
    #[inline(always)]
    pub fn pseg2(&self) -> Pseg2R {
        Pseg2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Phase Segment 1"]
    #[inline(always)]
    pub fn pseg1(&self) -> Pseg1R {
        Pseg1R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Resync Jump Width"]
    #[inline(always)]
    pub fn rjw(&self) -> RjwR {
        RjwR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Prescaler Division Factor"]
    #[inline(always)]
    pub fn presdiv(&self) -> PresdivR {
        PresdivR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Propagation Segment"]
    #[inline(always)]
    #[must_use]
    pub fn propseg(&mut self) -> PropsegW<Ctrl1Spec> {
        PropsegW::new(self, 0)
    }
    #[doc = "Bit 3 - Listen-Only Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lom(&mut self) -> LomW<Ctrl1Spec> {
        LomW::new(self, 3)
    }
    #[doc = "Bit 4 - Lowest Buffer Transmitted First"]
    #[inline(always)]
    #[must_use]
    pub fn lbuf(&mut self) -> LbufW<Ctrl1Spec> {
        LbufW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer Sync"]
    #[inline(always)]
    #[must_use]
    pub fn tsyn(&mut self) -> TsynW<Ctrl1Spec> {
        TsynW::new(self, 5)
    }
    #[doc = "Bit 6 - Bus Off Recovery"]
    #[inline(always)]
    #[must_use]
    pub fn boffrec(&mut self) -> BoffrecW<Ctrl1Spec> {
        BoffrecW::new(self, 6)
    }
    #[doc = "Bit 7 - CAN Bit Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SmpW<Ctrl1Spec> {
        SmpW::new(self, 7)
    }
    #[doc = "Bit 10 - Rx Warning Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwrnmsk(&mut self) -> RwrnmskW<Ctrl1Spec> {
        RwrnmskW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx Warning Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twrnmsk(&mut self) -> TwrnmskW<Ctrl1Spec> {
        TwrnmskW::new(self, 11)
    }
    #[doc = "Bit 12 - Loop Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpb(&mut self) -> LpbW<Ctrl1Spec> {
        LpbW::new(self, 12)
    }
    #[doc = "Bit 13 - CAN Engine Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> ClksrcW<Ctrl1Spec> {
        ClksrcW::new(self, 13)
    }
    #[doc = "Bit 14 - Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn errmsk(&mut self) -> ErrmskW<Ctrl1Spec> {
        ErrmskW::new(self, 14)
    }
    #[doc = "Bit 15 - Bus Off Mask"]
    #[inline(always)]
    #[must_use]
    pub fn boffmsk(&mut self) -> BoffmskW<Ctrl1Spec> {
        BoffmskW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Phase Segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn pseg2(&mut self) -> Pseg2W<Ctrl1Spec> {
        Pseg2W::new(self, 16)
    }
    #[doc = "Bits 19:21 - Phase Segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn pseg1(&mut self) -> Pseg1W<Ctrl1Spec> {
        Pseg1W::new(self, 19)
    }
    #[doc = "Bits 22:23 - Resync Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn rjw(&mut self) -> RjwW<Ctrl1Spec> {
        RjwW::new(self, 22)
    }
    #[doc = "Bits 24:31 - Prescaler Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn presdiv(&mut self) -> PresdivW<Ctrl1Spec> {
        PresdivW::new(self, 24)
    }
}
#[doc = "Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
