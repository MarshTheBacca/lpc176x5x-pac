#[doc = "Register `PINMODE_OD2` reader"]
pub type R = crate::R<PinmodeOd2Spec>;
#[doc = "Register `PINMODE_OD2` writer"]
pub type W = crate::W<PinmodeOd2Spec>;
#[doc = "Port 2 pin 0 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_00odEnum {
    #[doc = "0: Normal. P2.0 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.0 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_00odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_00odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_00OD` reader - Port 2 pin 0 open drain mode control."]
pub type P2_00odR = crate::BitReader<PinconnectPinmodeOd2P2_00odEnum>;
impl P2_00odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_00odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_00odEnum::Normal,
            true => PinconnectPinmodeOd2P2_00odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_00odEnum::Normal
    }
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_00odEnum::OpenDrain
    }
}
#[doc = "Field `P2_00OD` writer - Port 2 pin 0 open drain mode control."]
pub type P2_00odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_00odEnum>;
impl<'a, REG> P2_00odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_00odEnum::Normal)
    }
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_00odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 1 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_01odEnum {
    #[doc = "0: Normal. P2.1 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.1p in is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_01odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_01odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_01OD` reader - Port 2 pin 1 open drain mode control, see P2.00OD"]
pub type P2_01odR = crate::BitReader<PinconnectPinmodeOd2P2_01odEnum>;
impl P2_01odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_01odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_01odEnum::Normal,
            true => PinconnectPinmodeOd2P2_01odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_01odEnum::Normal
    }
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_01odEnum::OpenDrain
    }
}
#[doc = "Field `P2_01OD` writer - Port 2 pin 1 open drain mode control, see P2.00OD"]
pub type P2_01odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_01odEnum>;
impl<'a, REG> P2_01odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_01odEnum::Normal)
    }
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_01odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 2 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_02odEnum {
    #[doc = "0: Normal. P2.2 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.2 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_02odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_02odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_02OD` reader - Port 2 pin 2 open drain mode control, see P2.00OD"]
pub type P2_02odR = crate::BitReader<PinconnectPinmodeOd2P2_02odEnum>;
impl P2_02odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_02odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_02odEnum::Normal,
            true => PinconnectPinmodeOd2P2_02odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_02odEnum::Normal
    }
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_02odEnum::OpenDrain
    }
}
#[doc = "Field `P2_02OD` writer - Port 2 pin 2 open drain mode control, see P2.00OD"]
pub type P2_02odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_02odEnum>;
impl<'a, REG> P2_02odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_02odEnum::Normal)
    }
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_02odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 3 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_03odEnum {
    #[doc = "0: Normal. P2.3 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.3 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_03odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_03odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_03OD` reader - Port 2 pin 3 open drain mode control, see P2.00OD"]
pub type P2_03odR = crate::BitReader<PinconnectPinmodeOd2P2_03odEnum>;
impl P2_03odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_03odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_03odEnum::Normal,
            true => PinconnectPinmodeOd2P2_03odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_03odEnum::Normal
    }
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_03odEnum::OpenDrain
    }
}
#[doc = "Field `P2_03OD` writer - Port 2 pin 3 open drain mode control, see P2.00OD"]
pub type P2_03odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_03odEnum>;
impl<'a, REG> P2_03odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_03odEnum::Normal)
    }
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_03odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 4 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_04odEnum {
    #[doc = "0: Normal. P2.4 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.4 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_04odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_04odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_04OD` reader - Port 2 pin 4 open drain mode control, see P2.00OD"]
pub type P2_04odR = crate::BitReader<PinconnectPinmodeOd2P2_04odEnum>;
impl P2_04odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_04odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_04odEnum::Normal,
            true => PinconnectPinmodeOd2P2_04odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_04odEnum::Normal
    }
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_04odEnum::OpenDrain
    }
}
#[doc = "Field `P2_04OD` writer - Port 2 pin 4 open drain mode control, see P2.00OD"]
pub type P2_04odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_04odEnum>;
impl<'a, REG> P2_04odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_04odEnum::Normal)
    }
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_04odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 5 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_05odEnum {
    #[doc = "0: Normal. P2.5 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.5 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_05odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_05odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_05OD` reader - Port 2 pin 5 open drain mode control, see P2.00OD"]
pub type P2_05odR = crate::BitReader<PinconnectPinmodeOd2P2_05odEnum>;
impl P2_05odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_05odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_05odEnum::Normal,
            true => PinconnectPinmodeOd2P2_05odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_05odEnum::Normal
    }
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_05odEnum::OpenDrain
    }
}
#[doc = "Field `P2_05OD` writer - Port 2 pin 5 open drain mode control, see P2.00OD"]
pub type P2_05odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_05odEnum>;
impl<'a, REG> P2_05odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_05odEnum::Normal)
    }
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_05odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 6 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_06odEnum {
    #[doc = "0: Normal. P2.6 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.6 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_06odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_06odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_06OD` reader - Port 2 pin 6 open drain mode control, see P2.00OD"]
pub type P2_06odR = crate::BitReader<PinconnectPinmodeOd2P2_06odEnum>;
impl P2_06odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_06odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_06odEnum::Normal,
            true => PinconnectPinmodeOd2P2_06odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_06odEnum::Normal
    }
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_06odEnum::OpenDrain
    }
}
#[doc = "Field `P2_06OD` writer - Port 2 pin 6 open drain mode control, see P2.00OD"]
pub type P2_06odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_06odEnum>;
impl<'a, REG> P2_06odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_06odEnum::Normal)
    }
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_06odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 7 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_07odEnum {
    #[doc = "0: Normal. P2.7 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.7 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_07odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_07odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_07OD` reader - Port 2 pin 7 open drain mode control, see P2.00OD"]
pub type P2_07odR = crate::BitReader<PinconnectPinmodeOd2P2_07odEnum>;
impl P2_07odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_07odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_07odEnum::Normal,
            true => PinconnectPinmodeOd2P2_07odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_07odEnum::Normal
    }
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_07odEnum::OpenDrain
    }
}
#[doc = "Field `P2_07OD` writer - Port 2 pin 7 open drain mode control, see P2.00OD"]
pub type P2_07odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_07odEnum>;
impl<'a, REG> P2_07odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_07odEnum::Normal)
    }
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_07odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 8 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_08odEnum {
    #[doc = "0: Normal. P2.8 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.8 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_08odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_08odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_08OD` reader - Port 2 pin 8 open drain mode control, see P2.00OD"]
pub type P2_08odR = crate::BitReader<PinconnectPinmodeOd2P2_08odEnum>;
impl P2_08odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_08odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_08odEnum::Normal,
            true => PinconnectPinmodeOd2P2_08odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_08odEnum::Normal
    }
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_08odEnum::OpenDrain
    }
}
#[doc = "Field `P2_08OD` writer - Port 2 pin 8 open drain mode control, see P2.00OD"]
pub type P2_08odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_08odEnum>;
impl<'a, REG> P2_08odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_08odEnum::Normal)
    }
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_08odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 9 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_09odEnum {
    #[doc = "0: Normal. P2.9 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.9 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_09odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_09odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_09OD` reader - Port 2 pin 9 open drain mode control, see P2.00OD"]
pub type P2_09odR = crate::BitReader<PinconnectPinmodeOd2P2_09odEnum>;
impl P2_09odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_09odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_09odEnum::Normal,
            true => PinconnectPinmodeOd2P2_09odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_09odEnum::Normal
    }
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_09odEnum::OpenDrain
    }
}
#[doc = "Field `P2_09OD` writer - Port 2 pin 9 open drain mode control, see P2.00OD"]
pub type P2_09odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_09odEnum>;
impl<'a, REG> P2_09odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_09odEnum::Normal)
    }
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_09odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 10 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_10odEnum {
    #[doc = "0: Normal. P2.10 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.10 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_10odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_10odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_10OD` reader - Port 2 pin 10 open drain mode control, see P2.00OD"]
pub type P2_10odR = crate::BitReader<PinconnectPinmodeOd2P2_10odEnum>;
impl P2_10odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_10odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_10odEnum::Normal,
            true => PinconnectPinmodeOd2P2_10odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_10odEnum::Normal
    }
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_10odEnum::OpenDrain
    }
}
#[doc = "Field `P2_10OD` writer - Port 2 pin 10 open drain mode control, see P2.00OD"]
pub type P2_10odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_10odEnum>;
impl<'a, REG> P2_10odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_10odEnum::Normal)
    }
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_10odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 11 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_11odEnum {
    #[doc = "0: Normal. P2.11 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.11 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_11odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_11odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_11OD` reader - Port 2 pin 11 open drain mode control, see P2.00OD"]
pub type P2_11odR = crate::BitReader<PinconnectPinmodeOd2P2_11odEnum>;
impl P2_11odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_11odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_11odEnum::Normal,
            true => PinconnectPinmodeOd2P2_11odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_11odEnum::Normal
    }
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_11odEnum::OpenDrain
    }
}
#[doc = "Field `P2_11OD` writer - Port 2 pin 11 open drain mode control, see P2.00OD"]
pub type P2_11odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_11odEnum>;
impl<'a, REG> P2_11odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_11odEnum::Normal)
    }
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_11odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 12 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_12odEnum {
    #[doc = "0: Normal. P2.12 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.12 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_12odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_12odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_12OD` reader - Port 2 pin 12 open drain mode control, see P2.00OD"]
pub type P2_12odR = crate::BitReader<PinconnectPinmodeOd2P2_12odEnum>;
impl P2_12odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_12odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_12odEnum::Normal,
            true => PinconnectPinmodeOd2P2_12odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_12odEnum::Normal
    }
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_12odEnum::OpenDrain
    }
}
#[doc = "Field `P2_12OD` writer - Port 2 pin 12 open drain mode control, see P2.00OD"]
pub type P2_12odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_12odEnum>;
impl<'a, REG> P2_12odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_12odEnum::Normal)
    }
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_12odEnum::OpenDrain)
    }
}
#[doc = "Port 2 pin 13 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd2P2_13odEnum {
    #[doc = "0: Normal. P2.13 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P2.13 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd2P2_13odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd2P2_13odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2_13OD` reader - Port 2 pin 13 open drain mode control, see P2.00OD"]
pub type P2_13odR = crate::BitReader<PinconnectPinmodeOd2P2_13odEnum>;
impl P2_13odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd2P2_13odEnum {
        match self.bits {
            false => PinconnectPinmodeOd2P2_13odEnum::Normal,
            true => PinconnectPinmodeOd2P2_13odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_13odEnum::Normal
    }
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd2P2_13odEnum::OpenDrain
    }
}
#[doc = "Field `P2_13OD` writer - Port 2 pin 13 open drain mode control, see P2.00OD"]
pub type P2_13odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd2P2_13odEnum>;
impl<'a, REG> P2_13odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_13odEnum::Normal)
    }
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd2P2_13odEnum::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p2_00od(&self) -> P2_00odR {
        P2_00odR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_01od(&self) -> P2_01odR {
        P2_01odR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_02od(&self) -> P2_02odR {
        P2_02odR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_03od(&self) -> P2_03odR {
        P2_03odR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_04od(&self) -> P2_04odR {
        P2_04odR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_05od(&self) -> P2_05odR {
        P2_05odR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_06od(&self) -> P2_06odR {
        P2_06odR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_07od(&self) -> P2_07odR {
        P2_07odR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_08od(&self) -> P2_08odR {
        P2_08odR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_09od(&self) -> P2_09odR {
        P2_09odR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_10od(&self) -> P2_10odR {
        P2_10odR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_11od(&self) -> P2_11odR {
        P2_11odR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_12od(&self) -> P2_12odR {
        P2_12odR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_13od(&self) -> P2_13odR {
        P2_13odR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p2_00od(&mut self) -> P2_00odW<'_, PinmodeOd2Spec> {
        P2_00odW::new(self, 0)
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_01od(&mut self) -> P2_01odW<'_, PinmodeOd2Spec> {
        P2_01odW::new(self, 1)
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_02od(&mut self) -> P2_02odW<'_, PinmodeOd2Spec> {
        P2_02odW::new(self, 2)
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_03od(&mut self) -> P2_03odW<'_, PinmodeOd2Spec> {
        P2_03odW::new(self, 3)
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_04od(&mut self) -> P2_04odW<'_, PinmodeOd2Spec> {
        P2_04odW::new(self, 4)
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_05od(&mut self) -> P2_05odW<'_, PinmodeOd2Spec> {
        P2_05odW::new(self, 5)
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_06od(&mut self) -> P2_06odW<'_, PinmodeOd2Spec> {
        P2_06odW::new(self, 6)
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_07od(&mut self) -> P2_07odW<'_, PinmodeOd2Spec> {
        P2_07odW::new(self, 7)
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_08od(&mut self) -> P2_08odW<'_, PinmodeOd2Spec> {
        P2_08odW::new(self, 8)
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_09od(&mut self) -> P2_09odW<'_, PinmodeOd2Spec> {
        P2_09odW::new(self, 9)
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_10od(&mut self) -> P2_10odW<'_, PinmodeOd2Spec> {
        P2_10odW::new(self, 10)
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_11od(&mut self) -> P2_11odW<'_, PinmodeOd2Spec> {
        P2_11odW::new(self, 11)
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_12od(&mut self) -> P2_12odW<'_, PinmodeOd2Spec> {
        P2_12odW::new(self, 12)
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_13od(&mut self) -> P2_13odW<'_, PinmodeOd2Spec> {
        P2_13odW::new(self, 13)
    }
}
#[doc = "Open drain mode control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmodeOd2Spec;
impl crate::RegisterSpec for PinmodeOd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od2::R`](R) reader structure"]
impl crate::Readable for PinmodeOd2Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od2::W`](W) writer structure"]
impl crate::Writable for PinmodeOd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE_OD2 to value 0"]
impl crate::Resettable for PinmodeOd2Spec {}
