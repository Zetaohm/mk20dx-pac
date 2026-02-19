#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `MAXMB` reader - Number of the Last Message Buffer"]
pub type MaxmbR = crate::FieldReader;
#[doc = "Field `MAXMB` writer - Number of the Last Message Buffer"]
pub type MaxmbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "ID Acceptance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idam {
    #[doc = "0: Format A: One full ID (standard and extended) per ID Filter Table element."]
    _00 = 0,
    #[doc = "1: Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    _01 = 1,
    #[doc = "2: Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    _10 = 2,
    #[doc = "3: Format D: All frames rejected."]
    _11 = 3,
}
impl From<Idam> for u8 {
    #[inline(always)]
    fn from(variant: Idam) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idam {
    type Ux = u8;
}
impl crate::IsEnum for Idam {}
#[doc = "Field `IDAM` reader - ID Acceptance Mode"]
pub type IdamR = crate::FieldReader<Idam>;
impl IdamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idam {
        match self.bits {
            0 => Idam::_00,
            1 => Idam::_01,
            2 => Idam::_10,
            3 => Idam::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Idam::_00
    }
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Idam::_01
    }
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Idam::_10
    }
    #[doc = "Format D: All frames rejected."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Idam::_11
    }
}
#[doc = "Field `IDAM` writer - ID Acceptance Mode"]
pub type IdamW<'a, REG> = crate::FieldWriter<'a, REG, 2, Idam, crate::Safe>;
impl<'a, REG> IdamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Idam::_00)
    }
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Idam::_01)
    }
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Idam::_10)
    }
    #[doc = "Format D: All frames rejected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Idam::_11)
    }
}
#[doc = "Abort Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aen {
    #[doc = "0: Abort disabled"]
    _0 = 0,
    #[doc = "1: Abort enabled"]
    _1 = 1,
}
impl From<Aen> for bool {
    #[inline(always)]
    fn from(variant: Aen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEN` reader - Abort Enable"]
pub type AenR = crate::BitReader<Aen>;
impl AenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aen {
        match self.bits {
            false => Aen::_0,
            true => Aen::_1,
        }
    }
    #[doc = "Abort disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aen::_0
    }
    #[doc = "Abort enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aen::_1
    }
}
#[doc = "Field `AEN` writer - Abort Enable"]
pub type AenW<'a, REG> = crate::BitWriter<'a, REG, Aen>;
impl<'a, REG> AenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Abort disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aen::_0)
    }
    #[doc = "Abort enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aen::_1)
    }
}
#[doc = "Local Priority Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lprioen {
    #[doc = "0: Local Priority disabled"]
    _0 = 0,
    #[doc = "1: Local Priority enabled"]
    _1 = 1,
}
impl From<Lprioen> for bool {
    #[inline(always)]
    fn from(variant: Lprioen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPRIOEN` reader - Local Priority Enable"]
pub type LprioenR = crate::BitReader<Lprioen>;
impl LprioenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lprioen {
        match self.bits {
            false => Lprioen::_0,
            true => Lprioen::_1,
        }
    }
    #[doc = "Local Priority disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lprioen::_0
    }
    #[doc = "Local Priority enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lprioen::_1
    }
}
#[doc = "Field `LPRIOEN` writer - Local Priority Enable"]
pub type LprioenW<'a, REG> = crate::BitWriter<'a, REG, Lprioen>;
impl<'a, REG> LprioenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Local Priority disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lprioen::_0)
    }
    #[doc = "Local Priority enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lprioen::_1)
    }
}
#[doc = "Individual Rx Masking and Queue Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irmq {
    #[doc = "0: Individual Rx masking and queue feature are disabled. For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    _0 = 0,
    #[doc = "1: Individual Rx masking and queue feature are enabled."]
    _1 = 1,
}
impl From<Irmq> for bool {
    #[inline(always)]
    fn from(variant: Irmq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRMQ` reader - Individual Rx Masking and Queue Enable"]
pub type IrmqR = crate::BitReader<Irmq>;
impl IrmqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irmq {
        match self.bits {
            false => Irmq::_0,
            true => Irmq::_1,
        }
    }
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irmq::_0
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irmq::_1
    }
}
#[doc = "Field `IRMQ` writer - Individual Rx Masking and Queue Enable"]
pub type IrmqW<'a, REG> = crate::BitWriter<'a, REG, Irmq>;
impl<'a, REG> IrmqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irmq::_0)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irmq::_1)
    }
}
#[doc = "Self Reception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srxdis {
    #[doc = "0: Self reception enabled"]
    _0 = 0,
    #[doc = "1: Self reception disabled"]
    _1 = 1,
}
impl From<Srxdis> for bool {
    #[inline(always)]
    fn from(variant: Srxdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRXDIS` reader - Self Reception Disable"]
pub type SrxdisR = crate::BitReader<Srxdis>;
impl SrxdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srxdis {
        match self.bits {
            false => Srxdis::_0,
            true => Srxdis::_1,
        }
    }
    #[doc = "Self reception enabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Srxdis::_0
    }
    #[doc = "Self reception disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Srxdis::_1
    }
}
#[doc = "Field `SRXDIS` writer - Self Reception Disable"]
pub type SrxdisW<'a, REG> = crate::BitWriter<'a, REG, Srxdis>;
impl<'a, REG> SrxdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Self reception enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Srxdis::_0)
    }
    #[doc = "Self reception disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Srxdis::_1)
    }
}
#[doc = "Low Power Mode Acknowledge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpmack {
    #[doc = "0: FlexCAN is not in a low power mode."]
    _0 = 0,
    #[doc = "1: FlexCAN is in a low power mode."]
    _1 = 1,
}
impl From<Lpmack> for bool {
    #[inline(always)]
    fn from(variant: Lpmack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMACK` reader - Low Power Mode Acknowledge"]
pub type LpmackR = crate::BitReader<Lpmack>;
impl LpmackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpmack {
        match self.bits {
            false => Lpmack::_0,
            true => Lpmack::_1,
        }
    }
    #[doc = "FlexCAN is not in a low power mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lpmack::_0
    }
    #[doc = "FlexCAN is in a low power mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lpmack::_1
    }
}
#[doc = "Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrnen {
    #[doc = "0: TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    _0 = 0,
    #[doc = "1: TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    _1 = 1,
}
impl From<Wrnen> for bool {
    #[inline(always)]
    fn from(variant: Wrnen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRNEN` reader - Warning Interrupt Enable"]
pub type WrnenR = crate::BitReader<Wrnen>;
impl WrnenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrnen {
        match self.bits {
            false => Wrnen::_0,
            true => Wrnen::_1,
        }
    }
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wrnen::_0
    }
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wrnen::_1
    }
}
#[doc = "Field `WRNEN` writer - Warning Interrupt Enable"]
pub type WrnenW<'a, REG> = crate::BitWriter<'a, REG, Wrnen>;
impl<'a, REG> WrnenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wrnen::_0)
    }
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wrnen::_1)
    }
}
#[doc = "Self Wake Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slfwak {
    #[doc = "0: FlexCAN Self Wake Up feature is disabled."]
    _0 = 0,
    #[doc = "1: FlexCAN Self Wake Up feature is enabled."]
    _1 = 1,
}
impl From<Slfwak> for bool {
    #[inline(always)]
    fn from(variant: Slfwak) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLFWAK` reader - Self Wake Up"]
pub type SlfwakR = crate::BitReader<Slfwak>;
impl SlfwakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slfwak {
        match self.bits {
            false => Slfwak::_0,
            true => Slfwak::_1,
        }
    }
    #[doc = "FlexCAN Self Wake Up feature is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slfwak::_0
    }
    #[doc = "FlexCAN Self Wake Up feature is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slfwak::_1
    }
}
#[doc = "Field `SLFWAK` writer - Self Wake Up"]
pub type SlfwakW<'a, REG> = crate::BitWriter<'a, REG, Slfwak>;
impl<'a, REG> SlfwakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FlexCAN Self Wake Up feature is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Slfwak::_0)
    }
    #[doc = "FlexCAN Self Wake Up feature is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Slfwak::_1)
    }
}
#[doc = "Supervisor Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Supv {
    #[doc = "0: FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses."]
    _0 = 0,
    #[doc = "1: FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location."]
    _1 = 1,
}
impl From<Supv> for bool {
    #[inline(always)]
    fn from(variant: Supv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUPV` reader - Supervisor Mode"]
pub type SupvR = crate::BitReader<Supv>;
impl SupvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Supv {
        match self.bits {
            false => Supv::_0,
            true => Supv::_1,
        }
    }
    #[doc = "FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Supv::_0
    }
    #[doc = "FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Supv::_1
    }
}
#[doc = "Field `SUPV` writer - Supervisor Mode"]
pub type SupvW<'a, REG> = crate::BitWriter<'a, REG, Supv>;
impl<'a, REG> SupvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Supv::_0)
    }
    #[doc = "FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Supv::_1)
    }
}
#[doc = "Freeze Mode Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frzack {
    #[doc = "0: FlexCAN not in Freeze Mode, prescaler running"]
    _0 = 0,
    #[doc = "1: FlexCAN in Freeze Mode, prescaler stopped"]
    _1 = 1,
}
impl From<Frzack> for bool {
    #[inline(always)]
    fn from(variant: Frzack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZACK` reader - Freeze Mode Acknowledge"]
pub type FrzackR = crate::BitReader<Frzack>;
impl FrzackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frzack {
        match self.bits {
            false => Frzack::_0,
            true => Frzack::_1,
        }
    }
    #[doc = "FlexCAN not in Freeze Mode, prescaler running"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Frzack::_0
    }
    #[doc = "FlexCAN in Freeze Mode, prescaler stopped"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Frzack::_1
    }
}
#[doc = "Soft Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softrst {
    #[doc = "0: No reset request"]
    _0 = 0,
    #[doc = "1: Resets the registers affected by soft reset."]
    _1 = 1,
}
impl From<Softrst> for bool {
    #[inline(always)]
    fn from(variant: Softrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTRST` reader - Soft Reset"]
pub type SoftrstR = crate::BitReader<Softrst>;
impl SoftrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softrst {
        match self.bits {
            false => Softrst::_0,
            true => Softrst::_1,
        }
    }
    #[doc = "No reset request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Softrst::_0
    }
    #[doc = "Resets the registers affected by soft reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Softrst::_1
    }
}
#[doc = "Field `SOFTRST` writer - Soft Reset"]
pub type SoftrstW<'a, REG> = crate::BitWriter<'a, REG, Softrst>;
impl<'a, REG> SoftrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Softrst::_0)
    }
    #[doc = "Resets the registers affected by soft reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Softrst::_1)
    }
}
#[doc = "Wake Up Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakmsk {
    #[doc = "0: Wake Up Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Wake Up Interrupt is enabled."]
    _1 = 1,
}
impl From<Wakmsk> for bool {
    #[inline(always)]
    fn from(variant: Wakmsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKMSK` reader - Wake Up Interrupt Mask"]
pub type WakmskR = crate::BitReader<Wakmsk>;
impl WakmskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakmsk {
        match self.bits {
            false => Wakmsk::_0,
            true => Wakmsk::_1,
        }
    }
    #[doc = "Wake Up Interrupt is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wakmsk::_0
    }
    #[doc = "Wake Up Interrupt is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wakmsk::_1
    }
}
#[doc = "Field `WAKMSK` writer - Wake Up Interrupt Mask"]
pub type WakmskW<'a, REG> = crate::BitWriter<'a, REG, Wakmsk>;
impl<'a, REG> WakmskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake Up Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wakmsk::_0)
    }
    #[doc = "Wake Up Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wakmsk::_1)
    }
}
#[doc = "FlexCAN Not Ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notrdy {
    #[doc = "0: FlexCAN module is either in Normal Mode, Listen-Only Mode or Loop-Back Mode."]
    _0 = 0,
    #[doc = "1: FlexCAN module is either in Disable Mode, Stop Mode or Freeze Mode."]
    _1 = 1,
}
impl From<Notrdy> for bool {
    #[inline(always)]
    fn from(variant: Notrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRDY` reader - FlexCAN Not Ready"]
pub type NotrdyR = crate::BitReader<Notrdy>;
impl NotrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Notrdy {
        match self.bits {
            false => Notrdy::_0,
            true => Notrdy::_1,
        }
    }
    #[doc = "FlexCAN module is either in Normal Mode, Listen-Only Mode or Loop-Back Mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Notrdy::_0
    }
    #[doc = "FlexCAN module is either in Disable Mode, Stop Mode or Freeze Mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Notrdy::_1
    }
}
#[doc = "Halt FlexCAN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halt {
    #[doc = "0: No Freeze Mode request."]
    _0 = 0,
    #[doc = "1: Enters Freeze Mode if the FRZ bit is asserted."]
    _1 = 1,
}
impl From<Halt> for bool {
    #[inline(always)]
    fn from(variant: Halt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halt FlexCAN"]
pub type HaltR = crate::BitReader<Halt>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halt {
        match self.bits {
            false => Halt::_0,
            true => Halt::_1,
        }
    }
    #[doc = "No Freeze Mode request."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Halt::_0
    }
    #[doc = "Enters Freeze Mode if the FRZ bit is asserted."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Halt::_1
    }
}
#[doc = "Field `HALT` writer - Halt FlexCAN"]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG, Halt>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Freeze Mode request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::_0)
    }
    #[doc = "Enters Freeze Mode if the FRZ bit is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::_1)
    }
}
#[doc = "Rx FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfen {
    #[doc = "0: Rx FIFO not enabled"]
    _0 = 0,
    #[doc = "1: Rx FIFO enabled"]
    _1 = 1,
}
impl From<Rfen> for bool {
    #[inline(always)]
    fn from(variant: Rfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEN` reader - Rx FIFO Enable"]
pub type RfenR = crate::BitReader<Rfen>;
impl RfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfen {
        match self.bits {
            false => Rfen::_0,
            true => Rfen::_1,
        }
    }
    #[doc = "Rx FIFO not enabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rfen::_0
    }
    #[doc = "Rx FIFO enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rfen::_1
    }
}
#[doc = "Field `RFEN` writer - Rx FIFO Enable"]
pub type RfenW<'a, REG> = crate::BitWriter<'a, REG, Rfen>;
impl<'a, REG> RfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx FIFO not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfen::_0)
    }
    #[doc = "Rx FIFO enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfen::_1)
    }
}
#[doc = "Freeze Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frz {
    #[doc = "0: Not enabled to enter Freeze Mode"]
    _0 = 0,
    #[doc = "1: Enabled to enter Freeze Mode"]
    _1 = 1,
}
impl From<Frz> for bool {
    #[inline(always)]
    fn from(variant: Frz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZ` reader - Freeze Enable"]
pub type FrzR = crate::BitReader<Frz>;
impl FrzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frz {
        match self.bits {
            false => Frz::_0,
            true => Frz::_1,
        }
    }
    #[doc = "Not enabled to enter Freeze Mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Frz::_0
    }
    #[doc = "Enabled to enter Freeze Mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Frz::_1
    }
}
#[doc = "Field `FRZ` writer - Freeze Enable"]
pub type FrzW<'a, REG> = crate::BitWriter<'a, REG, Frz>;
impl<'a, REG> FrzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not enabled to enter Freeze Mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Frz::_0)
    }
    #[doc = "Enabled to enter Freeze Mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Frz::_1)
    }
}
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdis {
    #[doc = "0: Enable the FlexCAN module."]
    _0 = 0,
    #[doc = "1: Disable the FlexCAN module."]
    _1 = 1,
}
impl From<Mdis> for bool {
    #[inline(always)]
    fn from(variant: Mdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub type MdisR = crate::BitReader<Mdis>;
impl MdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdis {
        match self.bits {
            false => Mdis::_0,
            true => Mdis::_1,
        }
    }
    #[doc = "Enable the FlexCAN module."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mdis::_0
    }
    #[doc = "Disable the FlexCAN module."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mdis::_1
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub type MdisW<'a, REG> = crate::BitWriter<'a, REG, Mdis>;
impl<'a, REG> MdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the FlexCAN module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mdis::_0)
    }
    #[doc = "Disable the FlexCAN module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mdis::_1)
    }
}
impl R {
    #[doc = "Bits 0:6 - Number of the Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&self) -> MaxmbR {
        MaxmbR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&self) -> IdamR {
        IdamR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AenR {
        AenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&self) -> LprioenR {
        LprioenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Individual Rx Masking and Queue Enable"]
    #[inline(always)]
    pub fn irmq(&self) -> IrmqR {
        IrmqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&self) -> SrxdisR {
        SrxdisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Low Power Mode Acknowledge"]
    #[inline(always)]
    pub fn lpmack(&self) -> LpmackR {
        LpmackR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&self) -> WrnenR {
        WrnenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Self Wake Up"]
    #[inline(always)]
    pub fn slfwak(&self) -> SlfwakR {
        SlfwakR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&self) -> SupvR {
        SupvR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Freeze Mode Acknowledge"]
    #[inline(always)]
    pub fn frzack(&self) -> FrzackR {
        FrzackR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&self) -> SoftrstR {
        SoftrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake Up Interrupt Mask"]
    #[inline(always)]
    pub fn wakmsk(&self) -> WakmskR {
        WakmskR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FlexCAN Not Ready"]
    #[inline(always)]
    pub fn notrdy(&self) -> NotrdyR {
        NotrdyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RfenR {
        RfenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&self) -> FrzR {
        FrzR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MdisR {
        MdisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number of the Last Message Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn maxmb(&mut self) -> MaxmbW<McrSpec> {
        MaxmbW::new(self, 0)
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    #[must_use]
    pub fn idam(&mut self) -> IdamW<McrSpec> {
        IdamW::new(self, 8)
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aen(&mut self) -> AenW<McrSpec> {
        AenW::new(self, 12)
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lprioen(&mut self) -> LprioenW<McrSpec> {
        LprioenW::new(self, 13)
    }
    #[doc = "Bit 16 - Individual Rx Masking and Queue Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irmq(&mut self) -> IrmqW<McrSpec> {
        IrmqW::new(self, 16)
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    #[must_use]
    pub fn srxdis(&mut self) -> SrxdisW<McrSpec> {
        SrxdisW::new(self, 17)
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrnen(&mut self) -> WrnenW<McrSpec> {
        WrnenW::new(self, 21)
    }
    #[doc = "Bit 22 - Self Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn slfwak(&mut self) -> SlfwakW<McrSpec> {
        SlfwakW::new(self, 22)
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn supv(&mut self) -> SupvW<McrSpec> {
        SupvW::new(self, 23)
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn softrst(&mut self) -> SoftrstW<McrSpec> {
        SoftrstW::new(self, 25)
    }
    #[doc = "Bit 26 - Wake Up Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wakmsk(&mut self) -> WakmskW<McrSpec> {
        WakmskW::new(self, 26)
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<McrSpec> {
        HaltW::new(self, 28)
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RfenW<McrSpec> {
        RfenW::new(self, 29)
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frz(&mut self) -> FrzW<McrSpec> {
        FrzW::new(self, 30)
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MdisW<McrSpec> {
        MdisW::new(self, 31)
    }
}
#[doc = "Module Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0xd890_000f"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0xd890_000f;
}
