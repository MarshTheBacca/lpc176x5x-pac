#[doc = "Register `PINMODE_OD1` reader"]
pub type R = crate::R<PinmodeOd1Spec>;
#[doc = "Register `PINMODE_OD1` writer"]
pub type W = crate::W<PinmodeOd1Spec>;
#[doc = "Port 1 pin 0 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_00odEnum {
    #[doc = "0: Normal. P1.0 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.0 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_00odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_00odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_00OD` reader - Port 1 pin 0 open drain mode control."]
pub type P1_00odR = crate::BitReader<PinconnectPinmodeOd1P1_00odEnum>;
impl P1_00odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_00odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_00odEnum::Normal,
            true => PinconnectPinmodeOd1P1_00odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_00odEnum::Normal
    }
    #[doc = "Open-drain. P1.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_00odEnum::OpenDrain
    }
}
#[doc = "Field `P1_00OD` writer - Port 1 pin 0 open drain mode control."]
pub type P1_00odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_00odEnum>;
impl<'a, REG> P1_00odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_00odEnum::Normal)
    }
    #[doc = "Open-drain. P1.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_00odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 1 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_01odEnum {
    #[doc = "0: Normal. P1.1 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.1 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_01odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_01odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_01OD` reader - Port 1 pin 1 open drain mode control, see P1.00OD"]
pub type P1_01odR = crate::BitReader<PinconnectPinmodeOd1P1_01odEnum>;
impl P1_01odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_01odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_01odEnum::Normal,
            true => PinconnectPinmodeOd1P1_01odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_01odEnum::Normal
    }
    #[doc = "Open-drain. P1.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_01odEnum::OpenDrain
    }
}
#[doc = "Field `P1_01OD` writer - Port 1 pin 1 open drain mode control, see P1.00OD"]
pub type P1_01odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_01odEnum>;
impl<'a, REG> P1_01odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_01odEnum::Normal)
    }
    #[doc = "Open-drain. P1.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_01odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 4 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_04odEnum {
    #[doc = "0: Normal. P1.4 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.4 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_04odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_04odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_04OD` reader - Port 1 pin 4 open drain mode control, see P1.00OD"]
pub type P1_04odR = crate::BitReader<PinconnectPinmodeOd1P1_04odEnum>;
impl P1_04odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_04odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_04odEnum::Normal,
            true => PinconnectPinmodeOd1P1_04odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_04odEnum::Normal
    }
    #[doc = "Open-drain. P1.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_04odEnum::OpenDrain
    }
}
#[doc = "Field `P1_04OD` writer - Port 1 pin 4 open drain mode control, see P1.00OD"]
pub type P1_04odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_04odEnum>;
impl<'a, REG> P1_04odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_04odEnum::Normal)
    }
    #[doc = "Open-drain. P1.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_04odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 8 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_08odEnum {
    #[doc = "0: Normal. P1.8 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.8 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_08odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_08odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_08OD` reader - Port 1 pin 8 open drain mode control, see P1.00OD"]
pub type P1_08odR = crate::BitReader<PinconnectPinmodeOd1P1_08odEnum>;
impl P1_08odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_08odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_08odEnum::Normal,
            true => PinconnectPinmodeOd1P1_08odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_08odEnum::Normal
    }
    #[doc = "Open-drain. P1.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_08odEnum::OpenDrain
    }
}
#[doc = "Field `P1_08OD` writer - Port 1 pin 8 open drain mode control, see P1.00OD"]
pub type P1_08odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_08odEnum>;
impl<'a, REG> P1_08odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_08odEnum::Normal)
    }
    #[doc = "Open-drain. P1.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_08odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 9 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_09odEnum {
    #[doc = "0: Normal. P1.9 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.9 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_09odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_09odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_09OD` reader - Port 1 pin 9 open drain mode control, see P1.00OD"]
pub type P1_09odR = crate::BitReader<PinconnectPinmodeOd1P1_09odEnum>;
impl P1_09odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_09odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_09odEnum::Normal,
            true => PinconnectPinmodeOd1P1_09odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_09odEnum::Normal
    }
    #[doc = "Open-drain. P1.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_09odEnum::OpenDrain
    }
}
#[doc = "Field `P1_09OD` writer - Port 1 pin 9 open drain mode control, see P1.00OD"]
pub type P1_09odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_09odEnum>;
impl<'a, REG> P1_09odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_09odEnum::Normal)
    }
    #[doc = "Open-drain. P1.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_09odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 10 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_10odEnum {
    #[doc = "0: Normal. P1.10 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.10 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_10odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_10odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_10OD` reader - Port 1 pin 10 open drain mode control, see P1.00OD"]
pub type P1_10odR = crate::BitReader<PinconnectPinmodeOd1P1_10odEnum>;
impl P1_10odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_10odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_10odEnum::Normal,
            true => PinconnectPinmodeOd1P1_10odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_10odEnum::Normal
    }
    #[doc = "Open-drain. P1.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_10odEnum::OpenDrain
    }
}
#[doc = "Field `P1_10OD` writer - Port 1 pin 10 open drain mode control, see P1.00OD"]
pub type P1_10odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_10odEnum>;
impl<'a, REG> P1_10odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_10odEnum::Normal)
    }
    #[doc = "Open-drain. P1.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_10odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 14 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_14odEnum {
    #[doc = "0: Normal. P1.14 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.14 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_14odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_14odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_14OD` reader - Port 1 pin 14 open drain mode control, see P1.00OD"]
pub type P1_14odR = crate::BitReader<PinconnectPinmodeOd1P1_14odEnum>;
impl P1_14odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_14odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_14odEnum::Normal,
            true => PinconnectPinmodeOd1P1_14odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.14 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_14odEnum::Normal
    }
    #[doc = "Open-drain. P1.14 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_14odEnum::OpenDrain
    }
}
#[doc = "Field `P1_14OD` writer - Port 1 pin 14 open drain mode control, see P1.00OD"]
pub type P1_14odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_14odEnum>;
impl<'a, REG> P1_14odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.14 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_14odEnum::Normal)
    }
    #[doc = "Open-drain. P1.14 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_14odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 15 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_15odEnum {
    #[doc = "0: Normal. P1.15 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.15 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_15odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_15odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_15OD` reader - Port 1 pin 15 open drain mode control, see P1.00OD"]
pub type P1_15odR = crate::BitReader<PinconnectPinmodeOd1P1_15odEnum>;
impl P1_15odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_15odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_15odEnum::Normal,
            true => PinconnectPinmodeOd1P1_15odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_15odEnum::Normal
    }
    #[doc = "Open-drain. P1.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_15odEnum::OpenDrain
    }
}
#[doc = "Field `P1_15OD` writer - Port 1 pin 15 open drain mode control, see P1.00OD"]
pub type P1_15odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_15odEnum>;
impl<'a, REG> P1_15odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_15odEnum::Normal)
    }
    #[doc = "Open-drain. P1.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_15odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 16 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_16odEnum {
    #[doc = "0: Normal. P1.16 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.16 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_16odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_16odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_16OD` reader - Port 1 pin 16 open drain mode control, see P1.00OD"]
pub type P1_16odR = crate::BitReader<PinconnectPinmodeOd1P1_16odEnum>;
impl P1_16odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_16odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_16odEnum::Normal,
            true => PinconnectPinmodeOd1P1_16odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_16odEnum::Normal
    }
    #[doc = "Open-drain. P1.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_16odEnum::OpenDrain
    }
}
#[doc = "Field `P1_16OD` writer - Port 1 pin 16 open drain mode control, see P1.00OD"]
pub type P1_16odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_16odEnum>;
impl<'a, REG> P1_16odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_16odEnum::Normal)
    }
    #[doc = "Open-drain. P1.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_16odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 17 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_17odEnum {
    #[doc = "0: Normal. P1.17 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.17 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_17odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_17odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_17OD` reader - Port 1 pin 17 open drain mode control, see P1.00OD"]
pub type P1_17odR = crate::BitReader<PinconnectPinmodeOd1P1_17odEnum>;
impl P1_17odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_17odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_17odEnum::Normal,
            true => PinconnectPinmodeOd1P1_17odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_17odEnum::Normal
    }
    #[doc = "Open-drain. P1.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_17odEnum::OpenDrain
    }
}
#[doc = "Field `P1_17OD` writer - Port 1 pin 17 open drain mode control, see P1.00OD"]
pub type P1_17odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_17odEnum>;
impl<'a, REG> P1_17odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_17odEnum::Normal)
    }
    #[doc = "Open-drain. P1.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_17odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 18 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_18odEnum {
    #[doc = "0: Normal. P1.18 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.18 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_18odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_18odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_18OD` reader - Port 1 pin 18 open drain mode control, see P1.00OD"]
pub type P1_18odR = crate::BitReader<PinconnectPinmodeOd1P1_18odEnum>;
impl P1_18odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_18odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_18odEnum::Normal,
            true => PinconnectPinmodeOd1P1_18odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_18odEnum::Normal
    }
    #[doc = "Open-drain. P1.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_18odEnum::OpenDrain
    }
}
#[doc = "Field `P1_18OD` writer - Port 1 pin 18 open drain mode control, see P1.00OD"]
pub type P1_18odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_18odEnum>;
impl<'a, REG> P1_18odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_18odEnum::Normal)
    }
    #[doc = "Open-drain. P1.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_18odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 19 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_19odEnum {
    #[doc = "0: Normal. P1.19 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.19 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_19odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_19odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_19OD` reader - Port 1 pin 19 open drain mode control, see P1.00OD"]
pub type P1_19odR = crate::BitReader<PinconnectPinmodeOd1P1_19odEnum>;
impl P1_19odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_19odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_19odEnum::Normal,
            true => PinconnectPinmodeOd1P1_19odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_19odEnum::Normal
    }
    #[doc = "Open-drain. P1.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_19odEnum::OpenDrain
    }
}
#[doc = "Field `P1_19OD` writer - Port 1 pin 19 open drain mode control, see P1.00OD"]
pub type P1_19odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_19odEnum>;
impl<'a, REG> P1_19odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_19odEnum::Normal)
    }
    #[doc = "Open-drain. P1.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_19odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 20open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_20odEnum {
    #[doc = "0: Normal. P1.20 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.20 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_20odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_20odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_20OD` reader - Port 1 pin 20open drain mode control, see P1.00OD"]
pub type P1_20odR = crate::BitReader<PinconnectPinmodeOd1P1_20odEnum>;
impl P1_20odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_20odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_20odEnum::Normal,
            true => PinconnectPinmodeOd1P1_20odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_20odEnum::Normal
    }
    #[doc = "Open-drain. P1.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_20odEnum::OpenDrain
    }
}
#[doc = "Field `P1_20OD` writer - Port 1 pin 20open drain mode control, see P1.00OD"]
pub type P1_20odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_20odEnum>;
impl<'a, REG> P1_20odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_20odEnum::Normal)
    }
    #[doc = "Open-drain. P1.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_20odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 21 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_21odEnum {
    #[doc = "0: Normal. P1.21 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.21 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_21odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_21odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_21OD` reader - Port 1 pin 21 open drain mode control, see P1.00OD"]
pub type P1_21odR = crate::BitReader<PinconnectPinmodeOd1P1_21odEnum>;
impl P1_21odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_21odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_21odEnum::Normal,
            true => PinconnectPinmodeOd1P1_21odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_21odEnum::Normal
    }
    #[doc = "Open-drain. P1.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_21odEnum::OpenDrain
    }
}
#[doc = "Field `P1_21OD` writer - Port 1 pin 21 open drain mode control, see P1.00OD"]
pub type P1_21odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_21odEnum>;
impl<'a, REG> P1_21odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_21odEnum::Normal)
    }
    #[doc = "Open-drain. P1.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_21odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 22 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_22odEnum {
    #[doc = "0: Normal. P1.22 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.22 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_22odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_22odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_22OD` reader - Port 1 pin 22 open drain mode control, see P1.00OD"]
pub type P1_22odR = crate::BitReader<PinconnectPinmodeOd1P1_22odEnum>;
impl P1_22odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_22odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_22odEnum::Normal,
            true => PinconnectPinmodeOd1P1_22odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_22odEnum::Normal
    }
    #[doc = "Open-drain. P1.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_22odEnum::OpenDrain
    }
}
#[doc = "Field `P1_22OD` writer - Port 1 pin 22 open drain mode control, see P1.00OD"]
pub type P1_22odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_22odEnum>;
impl<'a, REG> P1_22odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_22odEnum::Normal)
    }
    #[doc = "Open-drain. P1.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_22odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 23 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_23odEnum {
    #[doc = "0: Normal. P1.23 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.23 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_23odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_23odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_23OD` reader - Port 1 pin 23 open drain mode control, see P1.00OD"]
pub type P1_23odR = crate::BitReader<PinconnectPinmodeOd1P1_23odEnum>;
impl P1_23odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_23odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_23odEnum::Normal,
            true => PinconnectPinmodeOd1P1_23odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_23odEnum::Normal
    }
    #[doc = "Open-drain. P1.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_23odEnum::OpenDrain
    }
}
#[doc = "Field `P1_23OD` writer - Port 1 pin 23 open drain mode control, see P1.00OD"]
pub type P1_23odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_23odEnum>;
impl<'a, REG> P1_23odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_23odEnum::Normal)
    }
    #[doc = "Open-drain. P1.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_23odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 24open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_24odEnum {
    #[doc = "0: Normal. P1.24 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.24 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_24odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_24odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_24OD` reader - Port 1 pin 24open drain mode control, see P1.00OD"]
pub type P1_24odR = crate::BitReader<PinconnectPinmodeOd1P1_24odEnum>;
impl P1_24odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_24odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_24odEnum::Normal,
            true => PinconnectPinmodeOd1P1_24odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.24 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_24odEnum::Normal
    }
    #[doc = "Open-drain. P1.24 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_24odEnum::OpenDrain
    }
}
#[doc = "Field `P1_24OD` writer - Port 1 pin 24open drain mode control, see P1.00OD"]
pub type P1_24odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_24odEnum>;
impl<'a, REG> P1_24odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.24 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_24odEnum::Normal)
    }
    #[doc = "Open-drain. P1.24 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_24odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 25 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_25odEnum {
    #[doc = "0: Normal. P1.25 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.25 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_25odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_25odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_25OD` reader - Port 1 pin 25 open drain mode control, see P1.00OD"]
pub type P1_25odR = crate::BitReader<PinconnectPinmodeOd1P1_25odEnum>;
impl P1_25odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_25odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_25odEnum::Normal,
            true => PinconnectPinmodeOd1P1_25odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_25odEnum::Normal
    }
    #[doc = "Open-drain. P1.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_25odEnum::OpenDrain
    }
}
#[doc = "Field `P1_25OD` writer - Port 1 pin 25 open drain mode control, see P1.00OD"]
pub type P1_25odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_25odEnum>;
impl<'a, REG> P1_25odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_25odEnum::Normal)
    }
    #[doc = "Open-drain. P1.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_25odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 26 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_26odEnum {
    #[doc = "0: Normal. P1.26 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.26 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_26odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_26odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_26OD` reader - Port 1 pin 26 open drain mode control, see P1.00OD"]
pub type P1_26odR = crate::BitReader<PinconnectPinmodeOd1P1_26odEnum>;
impl P1_26odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_26odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_26odEnum::Normal,
            true => PinconnectPinmodeOd1P1_26odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_26odEnum::Normal
    }
    #[doc = "Open-drain. P1.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_26odEnum::OpenDrain
    }
}
#[doc = "Field `P1_26OD` writer - Port 1 pin 26 open drain mode control, see P1.00OD"]
pub type P1_26odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_26odEnum>;
impl<'a, REG> P1_26odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_26odEnum::Normal)
    }
    #[doc = "Open-drain. P1.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_26odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 27 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_27odEnum {
    #[doc = "0: Normal. P1.27 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.27 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_27odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_27odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_27OD` reader - Port 1 pin 27 open drain mode control, see P1.00OD"]
pub type P1_27odR = crate::BitReader<PinconnectPinmodeOd1P1_27odEnum>;
impl P1_27odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_27odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_27odEnum::Normal,
            true => PinconnectPinmodeOd1P1_27odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.27 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_27odEnum::Normal
    }
    #[doc = "Open-drain. P1.27 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_27odEnum::OpenDrain
    }
}
#[doc = "Field `P1_27OD` writer - Port 1 pin 27 open drain mode control, see P1.00OD"]
pub type P1_27odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_27odEnum>;
impl<'a, REG> P1_27odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.27 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_27odEnum::Normal)
    }
    #[doc = "Open-drain. P1.27 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_27odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 28 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_28odEnum {
    #[doc = "0: Normal. P1.28 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.28 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_28odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_28odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_28OD` reader - Port 1 pin 28 open drain mode control, see P1.00OD"]
pub type P1_28odR = crate::BitReader<PinconnectPinmodeOd1P1_28odEnum>;
impl P1_28odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_28odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_28odEnum::Normal,
            true => PinconnectPinmodeOd1P1_28odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_28odEnum::Normal
    }
    #[doc = "Open-drain. P1.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_28odEnum::OpenDrain
    }
}
#[doc = "Field `P1_28OD` writer - Port 1 pin 28 open drain mode control, see P1.00OD"]
pub type P1_28odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_28odEnum>;
impl<'a, REG> P1_28odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_28odEnum::Normal)
    }
    #[doc = "Open-drain. P1.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_28odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 29 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_29odEnum {
    #[doc = "0: Normal. P1.29 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.29 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_29odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_29odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_29OD` reader - Port 1 pin 29 open drain mode control, see P1.00OD"]
pub type P1_29odR = crate::BitReader<PinconnectPinmodeOd1P1_29odEnum>;
impl P1_29odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_29odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_29odEnum::Normal,
            true => PinconnectPinmodeOd1P1_29odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_29odEnum::Normal
    }
    #[doc = "Open-drain. P1.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_29odEnum::OpenDrain
    }
}
#[doc = "Field `P1_29OD` writer - Port 1 pin 29 open drain mode control, see P1.00OD"]
pub type P1_29odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_29odEnum>;
impl<'a, REG> P1_29odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_29odEnum::Normal)
    }
    #[doc = "Open-drain. P1.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_29odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 30 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_30odEnum {
    #[doc = "0: Normal. P1.30 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.30 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_30odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_30odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_30OD` reader - Port 1 pin 30 open drain mode control, see P1.00OD"]
pub type P1_30odR = crate::BitReader<PinconnectPinmodeOd1P1_30odEnum>;
impl P1_30odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_30odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_30odEnum::Normal,
            true => PinconnectPinmodeOd1P1_30odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_30odEnum::Normal
    }
    #[doc = "Open-drain. P1.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_30odEnum::OpenDrain
    }
}
#[doc = "Field `P1_30OD` writer - Port 1 pin 30 open drain mode control, see P1.00OD"]
pub type P1_30odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_30odEnum>;
impl<'a, REG> P1_30odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_30odEnum::Normal)
    }
    #[doc = "Open-drain. P1.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_30odEnum::OpenDrain)
    }
}
#[doc = "Port 1 pin 31 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd1P1_31odEnum {
    #[doc = "0: Normal. P1.31 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P1.31 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd1P1_31odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd1P1_31odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1_31OD` reader - Port 1 pin 31 open drain mode control."]
pub type P1_31odR = crate::BitReader<PinconnectPinmodeOd1P1_31odEnum>;
impl P1_31odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd1P1_31odEnum {
        match self.bits {
            false => PinconnectPinmodeOd1P1_31odEnum::Normal,
            true => PinconnectPinmodeOd1P1_31odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P1.31 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_31odEnum::Normal
    }
    #[doc = "Open-drain. P1.31 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd1P1_31odEnum::OpenDrain
    }
}
#[doc = "Field `P1_31OD` writer - Port 1 pin 31 open drain mode control."]
pub type P1_31odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd1P1_31odEnum>;
impl<'a, REG> P1_31odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P1.31 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_31odEnum::Normal)
    }
    #[doc = "Open-drain. P1.31 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd1P1_31odEnum::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 0 - Port 1 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p1_00od(&self) -> P1_00odR {
        P1_00odR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_01od(&self) -> P1_01odR {
        P1_01odR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_04od(&self) -> P1_04odR {
        P1_04odR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_08od(&self) -> P1_08odR {
        P1_08odR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_09od(&self) -> P1_09odR {
        P1_09odR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_10od(&self) -> P1_10odR {
        P1_10odR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_14od(&self) -> P1_14odR {
        P1_14odR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_15od(&self) -> P1_15odR {
        P1_15odR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_16od(&self) -> P1_16odR {
        P1_16odR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_17od(&self) -> P1_17odR {
        P1_17odR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_18od(&self) -> P1_18odR {
        P1_18odR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_19od(&self) -> P1_19odR {
        P1_19odR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_20od(&self) -> P1_20odR {
        P1_20odR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_21od(&self) -> P1_21odR {
        P1_21odR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_22od(&self) -> P1_22odR {
        P1_22odR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_23od(&self) -> P1_23odR {
        P1_23odR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_24od(&self) -> P1_24odR {
        P1_24odR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_25od(&self) -> P1_25odR {
        P1_25odR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_26od(&self) -> P1_26odR {
        P1_26odR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_27od(&self) -> P1_27odR {
        P1_27odR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_28od(&self) -> P1_28odR {
        P1_28odR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_29od(&self) -> P1_29odR {
        P1_29odR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_30od(&self) -> P1_30odR {
        P1_30odR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port 1 pin 31 open drain mode control."]
    #[inline(always)]
    pub fn p1_31od(&self) -> P1_31odR {
        P1_31odR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 1 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p1_00od(&mut self) -> P1_00odW<'_, PinmodeOd1Spec> {
        P1_00odW::new(self, 0)
    }
    #[doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_01od(&mut self) -> P1_01odW<'_, PinmodeOd1Spec> {
        P1_01odW::new(self, 1)
    }
    #[doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_04od(&mut self) -> P1_04odW<'_, PinmodeOd1Spec> {
        P1_04odW::new(self, 4)
    }
    #[doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_08od(&mut self) -> P1_08odW<'_, PinmodeOd1Spec> {
        P1_08odW::new(self, 8)
    }
    #[doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_09od(&mut self) -> P1_09odW<'_, PinmodeOd1Spec> {
        P1_09odW::new(self, 9)
    }
    #[doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_10od(&mut self) -> P1_10odW<'_, PinmodeOd1Spec> {
        P1_10odW::new(self, 10)
    }
    #[doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_14od(&mut self) -> P1_14odW<'_, PinmodeOd1Spec> {
        P1_14odW::new(self, 14)
    }
    #[doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_15od(&mut self) -> P1_15odW<'_, PinmodeOd1Spec> {
        P1_15odW::new(self, 15)
    }
    #[doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_16od(&mut self) -> P1_16odW<'_, PinmodeOd1Spec> {
        P1_16odW::new(self, 16)
    }
    #[doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_17od(&mut self) -> P1_17odW<'_, PinmodeOd1Spec> {
        P1_17odW::new(self, 17)
    }
    #[doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_18od(&mut self) -> P1_18odW<'_, PinmodeOd1Spec> {
        P1_18odW::new(self, 18)
    }
    #[doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_19od(&mut self) -> P1_19odW<'_, PinmodeOd1Spec> {
        P1_19odW::new(self, 19)
    }
    #[doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_20od(&mut self) -> P1_20odW<'_, PinmodeOd1Spec> {
        P1_20odW::new(self, 20)
    }
    #[doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_21od(&mut self) -> P1_21odW<'_, PinmodeOd1Spec> {
        P1_21odW::new(self, 21)
    }
    #[doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_22od(&mut self) -> P1_22odW<'_, PinmodeOd1Spec> {
        P1_22odW::new(self, 22)
    }
    #[doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_23od(&mut self) -> P1_23odW<'_, PinmodeOd1Spec> {
        P1_23odW::new(self, 23)
    }
    #[doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_24od(&mut self) -> P1_24odW<'_, PinmodeOd1Spec> {
        P1_24odW::new(self, 24)
    }
    #[doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_25od(&mut self) -> P1_25odW<'_, PinmodeOd1Spec> {
        P1_25odW::new(self, 25)
    }
    #[doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_26od(&mut self) -> P1_26odW<'_, PinmodeOd1Spec> {
        P1_26odW::new(self, 26)
    }
    #[doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_27od(&mut self) -> P1_27odW<'_, PinmodeOd1Spec> {
        P1_27odW::new(self, 27)
    }
    #[doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_28od(&mut self) -> P1_28odW<'_, PinmodeOd1Spec> {
        P1_28odW::new(self, 28)
    }
    #[doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_29od(&mut self) -> P1_29odW<'_, PinmodeOd1Spec> {
        P1_29odW::new(self, 29)
    }
    #[doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_30od(&mut self) -> P1_30odW<'_, PinmodeOd1Spec> {
        P1_30odW::new(self, 30)
    }
    #[doc = "Bit 31 - Port 1 pin 31 open drain mode control."]
    #[inline(always)]
    pub fn p1_31od(&mut self) -> P1_31odW<'_, PinmodeOd1Spec> {
        P1_31odW::new(self, 31)
    }
}
#[doc = "Open drain mode control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmodeOd1Spec;
impl crate::RegisterSpec for PinmodeOd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od1::R`](R) reader structure"]
impl crate::Readable for PinmodeOd1Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od1::W`](W) writer structure"]
impl crate::Writable for PinmodeOd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE_OD1 to value 0"]
impl crate::Resettable for PinmodeOd1Spec {}
