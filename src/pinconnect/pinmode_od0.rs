#[doc = "Register `PINMODE_OD0` reader"]
pub type R = crate::R<PinmodeOd0Spec>;
#[doc = "Register `PINMODE_OD0` writer"]
pub type W = crate::W<PinmodeOd0Spec>;
#[doc = "Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_00odEnum {
    #[doc = "0: Normal. P0.0 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.0 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_00odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_00odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_00OD` reader - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_00odR = crate::BitReader<PinconnectPinmodeOd0P0_00odEnum>;
impl P0_00odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_00odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_00odEnum::Normal,
            true => PinconnectPinmodeOd0P0_00odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_00odEnum::Normal
    }
    #[doc = "Open-drain. P0.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_00odEnum::OpenDrain
    }
}
#[doc = "Field `P0_00OD` writer - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_00odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_00odEnum>;
impl<'a, REG> P0_00odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_00odEnum::Normal)
    }
    #[doc = "Open-drain. P0.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_00odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_01odEnum {
    #[doc = "0: Normal. P0.1 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.1 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_01odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_01odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_01OD` reader - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_01odR = crate::BitReader<PinconnectPinmodeOd0P0_01odEnum>;
impl P0_01odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_01odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_01odEnum::Normal,
            true => PinconnectPinmodeOd0P0_01odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_01odEnum::Normal
    }
    #[doc = "Open-drain. P0.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_01odEnum::OpenDrain
    }
}
#[doc = "Field `P0_01OD` writer - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_01odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_01odEnum>;
impl<'a, REG> P0_01odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_01odEnum::Normal)
    }
    #[doc = "Open-drain. P0.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_01odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 2 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_02odEnum {
    #[doc = "0: Normal. P0.2 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.2 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_02odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_02odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_02OD` reader - Port 0 pin 2 open drain mode control"]
pub type P0_02odR = crate::BitReader<PinconnectPinmodeOd0P0_02odEnum>;
impl P0_02odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_02odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_02odEnum::Normal,
            true => PinconnectPinmodeOd0P0_02odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_02odEnum::Normal
    }
    #[doc = "Open-drain. P0.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_02odEnum::OpenDrain
    }
}
#[doc = "Field `P0_02OD` writer - Port 0 pin 2 open drain mode control"]
pub type P0_02odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_02odEnum>;
impl<'a, REG> P0_02odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_02odEnum::Normal)
    }
    #[doc = "Open-drain. P0.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_02odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 3 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_03odEnum {
    #[doc = "0: Normal. P0.3 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.3 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_03odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_03odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_03OD` reader - Port 0 pin 3 open drain mode control"]
pub type P0_03odR = crate::BitReader<PinconnectPinmodeOd0P0_03odEnum>;
impl P0_03odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_03odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_03odEnum::Normal,
            true => PinconnectPinmodeOd0P0_03odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_03odEnum::Normal
    }
    #[doc = "Open-drain. P0.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_03odEnum::OpenDrain
    }
}
#[doc = "Field `P0_03OD` writer - Port 0 pin 3 open drain mode control"]
pub type P0_03odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_03odEnum>;
impl<'a, REG> P0_03odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_03odEnum::Normal)
    }
    #[doc = "Open-drain. P0.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_03odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 4 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_04odEnum {
    #[doc = "0: Normal. P0.4 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.4 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_04odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_04odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_04OD` reader - Port 0 pin 4 open drain mode control"]
pub type P0_04odR = crate::BitReader<PinconnectPinmodeOd0P0_04odEnum>;
impl P0_04odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_04odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_04odEnum::Normal,
            true => PinconnectPinmodeOd0P0_04odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_04odEnum::Normal
    }
    #[doc = "Open-drain. P0.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_04odEnum::OpenDrain
    }
}
#[doc = "Field `P0_04OD` writer - Port 0 pin 4 open drain mode control"]
pub type P0_04odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_04odEnum>;
impl<'a, REG> P0_04odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_04odEnum::Normal)
    }
    #[doc = "Open-drain. P0.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_04odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 5 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_05odEnum {
    #[doc = "0: Normal. P0.5 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.5 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_05odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_05odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_05OD` reader - Port 0 pin 5 open drain mode control"]
pub type P0_05odR = crate::BitReader<PinconnectPinmodeOd0P0_05odEnum>;
impl P0_05odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_05odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_05odEnum::Normal,
            true => PinconnectPinmodeOd0P0_05odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_05odEnum::Normal
    }
    #[doc = "Open-drain. P0.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_05odEnum::OpenDrain
    }
}
#[doc = "Field `P0_05OD` writer - Port 0 pin 5 open drain mode control"]
pub type P0_05odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_05odEnum>;
impl<'a, REG> P0_05odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_05odEnum::Normal)
    }
    #[doc = "Open-drain. P0.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_05odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 6 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_06odEnum {
    #[doc = "0: Normal. P0.6 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.6 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_06odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_06odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_06OD` reader - Port 0 pin 6 open drain mode control"]
pub type P0_06odR = crate::BitReader<PinconnectPinmodeOd0P0_06odEnum>;
impl P0_06odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_06odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_06odEnum::Normal,
            true => PinconnectPinmodeOd0P0_06odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_06odEnum::Normal
    }
    #[doc = "Open-drain. P0.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_06odEnum::OpenDrain
    }
}
#[doc = "Field `P0_06OD` writer - Port 0 pin 6 open drain mode control"]
pub type P0_06odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_06odEnum>;
impl<'a, REG> P0_06odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_06odEnum::Normal)
    }
    #[doc = "Open-drain. P0.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_06odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 7 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_07odEnum {
    #[doc = "0: Normal. P0.7 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.7 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_07odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_07odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_07OD` reader - Port 0 pin 7 open drain mode control"]
pub type P0_07odR = crate::BitReader<PinconnectPinmodeOd0P0_07odEnum>;
impl P0_07odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_07odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_07odEnum::Normal,
            true => PinconnectPinmodeOd0P0_07odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_07odEnum::Normal
    }
    #[doc = "Open-drain. P0.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_07odEnum::OpenDrain
    }
}
#[doc = "Field `P0_07OD` writer - Port 0 pin 7 open drain mode control"]
pub type P0_07odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_07odEnum>;
impl<'a, REG> P0_07odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_07odEnum::Normal)
    }
    #[doc = "Open-drain. P0.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_07odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 8 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_08odEnum {
    #[doc = "0: Normal. P0.8 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.8 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_08odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_08odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_08OD` reader - Port 0 pin 8 open drain mode control"]
pub type P0_08odR = crate::BitReader<PinconnectPinmodeOd0P0_08odEnum>;
impl P0_08odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_08odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_08odEnum::Normal,
            true => PinconnectPinmodeOd0P0_08odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_08odEnum::Normal
    }
    #[doc = "Open-drain. P0.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_08odEnum::OpenDrain
    }
}
#[doc = "Field `P0_08OD` writer - Port 0 pin 8 open drain mode control"]
pub type P0_08odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_08odEnum>;
impl<'a, REG> P0_08odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_08odEnum::Normal)
    }
    #[doc = "Open-drain. P0.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_08odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 9 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_09odEnum {
    #[doc = "0: Normal. P0.9 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.9 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_09odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_09odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_09OD` reader - Port 0 pin 9 open drain mode control"]
pub type P0_09odR = crate::BitReader<PinconnectPinmodeOd0P0_09odEnum>;
impl P0_09odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_09odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_09odEnum::Normal,
            true => PinconnectPinmodeOd0P0_09odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_09odEnum::Normal
    }
    #[doc = "Open-drain. P0.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_09odEnum::OpenDrain
    }
}
#[doc = "Field `P0_09OD` writer - Port 0 pin 9 open drain mode control"]
pub type P0_09odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_09odEnum>;
impl<'a, REG> P0_09odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_09odEnum::Normal)
    }
    #[doc = "Open-drain. P0.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_09odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_10odEnum {
    #[doc = "0: Normal. P0.10 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.10 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_10odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_10odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_10OD` reader - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_10odR = crate::BitReader<PinconnectPinmodeOd0P0_10odEnum>;
impl P0_10odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_10odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_10odEnum::Normal,
            true => PinconnectPinmodeOd0P0_10odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_10odEnum::Normal
    }
    #[doc = "Open-drain. P0.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_10odEnum::OpenDrain
    }
}
#[doc = "Field `P0_10OD` writer - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_10odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_10odEnum>;
impl<'a, REG> P0_10odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_10odEnum::Normal)
    }
    #[doc = "Open-drain. P0.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_10odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_11odEnum {
    #[doc = "0: Normal. P0.11 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.11 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_11odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_11odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_11OD` reader - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_11odR = crate::BitReader<PinconnectPinmodeOd0P0_11odEnum>;
impl P0_11odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_11odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_11odEnum::Normal,
            true => PinconnectPinmodeOd0P0_11odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_11odEnum::Normal
    }
    #[doc = "Open-drain. P0.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_11odEnum::OpenDrain
    }
}
#[doc = "Field `P0_11OD` writer - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_11odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_11odEnum>;
impl<'a, REG> P0_11odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_11odEnum::Normal)
    }
    #[doc = "Open-drain. P0.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_11odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 15 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_15odEnum {
    #[doc = "0: Normal. P0.15 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.15 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_15odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_15odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_15OD` reader - Port 0 pin 15 open drain mode control"]
pub type P0_15odR = crate::BitReader<PinconnectPinmodeOd0P0_15odEnum>;
impl P0_15odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_15odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_15odEnum::Normal,
            true => PinconnectPinmodeOd0P0_15odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_15odEnum::Normal
    }
    #[doc = "Open-drain. P0.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_15odEnum::OpenDrain
    }
}
#[doc = "Field `P0_15OD` writer - Port 0 pin 15 open drain mode control"]
pub type P0_15odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_15odEnum>;
impl<'a, REG> P0_15odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_15odEnum::Normal)
    }
    #[doc = "Open-drain. P0.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_15odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 16 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_16odEnum {
    #[doc = "0: Normal. P0.16 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.16 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_16odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_16odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_16OD` reader - Port 0 pin 16 open drain mode control"]
pub type P0_16odR = crate::BitReader<PinconnectPinmodeOd0P0_16odEnum>;
impl P0_16odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_16odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_16odEnum::Normal,
            true => PinconnectPinmodeOd0P0_16odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_16odEnum::Normal
    }
    #[doc = "Open-drain. P0.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_16odEnum::OpenDrain
    }
}
#[doc = "Field `P0_16OD` writer - Port 0 pin 16 open drain mode control"]
pub type P0_16odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_16odEnum>;
impl<'a, REG> P0_16odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_16odEnum::Normal)
    }
    #[doc = "Open-drain. P0.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_16odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 17 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_17odEnum {
    #[doc = "0: Normal. P0.17 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.17 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_17odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_17odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_17OD` reader - Port 0 pin 17 open drain mode control"]
pub type P0_17odR = crate::BitReader<PinconnectPinmodeOd0P0_17odEnum>;
impl P0_17odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_17odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_17odEnum::Normal,
            true => PinconnectPinmodeOd0P0_17odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_17odEnum::Normal
    }
    #[doc = "Open-drain. P0.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_17odEnum::OpenDrain
    }
}
#[doc = "Field `P0_17OD` writer - Port 0 pin 17 open drain mode control"]
pub type P0_17odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_17odEnum>;
impl<'a, REG> P0_17odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_17odEnum::Normal)
    }
    #[doc = "Open-drain. P0.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_17odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 18 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_18odEnum {
    #[doc = "0: Normal. P0.18 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.18 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_18odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_18odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_18OD` reader - Port 0 pin 18 open drain mode control"]
pub type P0_18odR = crate::BitReader<PinconnectPinmodeOd0P0_18odEnum>;
impl P0_18odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_18odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_18odEnum::Normal,
            true => PinconnectPinmodeOd0P0_18odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_18odEnum::Normal
    }
    #[doc = "Open-drain. P0.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_18odEnum::OpenDrain
    }
}
#[doc = "Field `P0_18OD` writer - Port 0 pin 18 open drain mode control"]
pub type P0_18odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_18odEnum>;
impl<'a, REG> P0_18odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_18odEnum::Normal)
    }
    #[doc = "Open-drain. P0.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_18odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_19odEnum {
    #[doc = "0: Normal. P0.19 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.19 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_19odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_19odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_19OD` reader - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_19odR = crate::BitReader<PinconnectPinmodeOd0P0_19odEnum>;
impl P0_19odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_19odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_19odEnum::Normal,
            true => PinconnectPinmodeOd0P0_19odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_19odEnum::Normal
    }
    #[doc = "Open-drain. P0.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_19odEnum::OpenDrain
    }
}
#[doc = "Field `P0_19OD` writer - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_19odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_19odEnum>;
impl<'a, REG> P0_19odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_19odEnum::Normal)
    }
    #[doc = "Open-drain. P0.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_19odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_20odEnum {
    #[doc = "0: Normal. P0.20 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.20 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_20odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_20odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_20OD` reader - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_20odR = crate::BitReader<PinconnectPinmodeOd0P0_20odEnum>;
impl P0_20odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_20odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_20odEnum::Normal,
            true => PinconnectPinmodeOd0P0_20odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_20odEnum::Normal
    }
    #[doc = "Open-drain. P0.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_20odEnum::OpenDrain
    }
}
#[doc = "Field `P0_20OD` writer - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_20odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_20odEnum>;
impl<'a, REG> P0_20odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_20odEnum::Normal)
    }
    #[doc = "Open-drain. P0.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_20odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 21 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_21odEnum {
    #[doc = "0: Normal. P0.21 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.21 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_21odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_21odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_21OD` reader - Port 0 pin 21 open drain mode control"]
pub type P0_21odR = crate::BitReader<PinconnectPinmodeOd0P0_21odEnum>;
impl P0_21odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_21odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_21odEnum::Normal,
            true => PinconnectPinmodeOd0P0_21odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_21odEnum::Normal
    }
    #[doc = "Open-drain. P0.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_21odEnum::OpenDrain
    }
}
#[doc = "Field `P0_21OD` writer - Port 0 pin 21 open drain mode control"]
pub type P0_21odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_21odEnum>;
impl<'a, REG> P0_21odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_21odEnum::Normal)
    }
    #[doc = "Open-drain. P0.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_21odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 22 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_22odEnum {
    #[doc = "0: Normal. P0.22 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.22 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_22odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_22odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_22OD` reader - Port 0 pin 22 open drain mode control"]
pub type P0_22odR = crate::BitReader<PinconnectPinmodeOd0P0_22odEnum>;
impl P0_22odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_22odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_22odEnum::Normal,
            true => PinconnectPinmodeOd0P0_22odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_22odEnum::Normal
    }
    #[doc = "Open-drain. P0.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_22odEnum::OpenDrain
    }
}
#[doc = "Field `P0_22OD` writer - Port 0 pin 22 open drain mode control"]
pub type P0_22odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_22odEnum>;
impl<'a, REG> P0_22odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_22odEnum::Normal)
    }
    #[doc = "Open-drain. P0.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_22odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 23 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_23odEnum {
    #[doc = "0: Normal. P0.23 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.23 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_23odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_23odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_23OD` reader - Port 0 pin 23 open drain mode control"]
pub type P0_23odR = crate::BitReader<PinconnectPinmodeOd0P0_23odEnum>;
impl P0_23odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_23odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_23odEnum::Normal,
            true => PinconnectPinmodeOd0P0_23odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_23odEnum::Normal
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_23odEnum::OpenDrain
    }
}
#[doc = "Field `P0_23OD` writer - Port 0 pin 23 open drain mode control"]
pub type P0_23odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_23odEnum>;
impl<'a, REG> P0_23odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_23odEnum::Normal)
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_23odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 24open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_24odEnum {
    #[doc = "0: Normal. P0.23 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.23 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_24odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_24odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_24OD` reader - Port 0 pin 24open drain mode control"]
pub type P0_24odR = crate::BitReader<PinconnectPinmodeOd0P0_24odEnum>;
impl P0_24odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_24odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_24odEnum::Normal,
            true => PinconnectPinmodeOd0P0_24odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_24odEnum::Normal
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_24odEnum::OpenDrain
    }
}
#[doc = "Field `P0_24OD` writer - Port 0 pin 24open drain mode control"]
pub type P0_24odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_24odEnum>;
impl<'a, REG> P0_24odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_24odEnum::Normal)
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_24odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 25 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_25odEnum {
    #[doc = "0: Normal. P0.25 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.25 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_25odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_25odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_25OD` reader - Port 0 pin 25 open drain mode control"]
pub type P0_25odR = crate::BitReader<PinconnectPinmodeOd0P0_25odEnum>;
impl P0_25odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_25odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_25odEnum::Normal,
            true => PinconnectPinmodeOd0P0_25odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_25odEnum::Normal
    }
    #[doc = "Open-drain. P0.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_25odEnum::OpenDrain
    }
}
#[doc = "Field `P0_25OD` writer - Port 0 pin 25 open drain mode control"]
pub type P0_25odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_25odEnum>;
impl<'a, REG> P0_25odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_25odEnum::Normal)
    }
    #[doc = "Open-drain. P0.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_25odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 26 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_26odEnum {
    #[doc = "0: Normal. P0.26 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.26 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_26odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_26odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_26OD` reader - Port 0 pin 26 open drain mode control"]
pub type P0_26odR = crate::BitReader<PinconnectPinmodeOd0P0_26odEnum>;
impl P0_26odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_26odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_26odEnum::Normal,
            true => PinconnectPinmodeOd0P0_26odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_26odEnum::Normal
    }
    #[doc = "Open-drain. P0.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_26odEnum::OpenDrain
    }
}
#[doc = "Field `P0_26OD` writer - Port 0 pin 26 open drain mode control"]
pub type P0_26odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_26odEnum>;
impl<'a, REG> P0_26odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_26odEnum::Normal)
    }
    #[doc = "Open-drain. P0.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_26odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 29 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_29odEnum {
    #[doc = "0: Normal. P0.29 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.29 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_29odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_29odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_29OD` reader - Port 0 pin 29 open drain mode control"]
pub type P0_29odR = crate::BitReader<PinconnectPinmodeOd0P0_29odEnum>;
impl P0_29odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_29odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_29odEnum::Normal,
            true => PinconnectPinmodeOd0P0_29odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_29odEnum::Normal
    }
    #[doc = "Open-drain. P0.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_29odEnum::OpenDrain
    }
}
#[doc = "Field `P0_29OD` writer - Port 0 pin 29 open drain mode control"]
pub type P0_29odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_29odEnum>;
impl<'a, REG> P0_29odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_29odEnum::Normal)
    }
    #[doc = "Open-drain. P0.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_29odEnum::OpenDrain)
    }
}
#[doc = "Port 0 pin 30 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinconnectPinmodeOd0P0_30odEnum {
    #[doc = "0: Normal. P0.30 pin is in the normal (not open drain) mode."]
    Normal = 0,
    #[doc = "1: Open-drain. P0.30 pin is in the open drain mode."]
    OpenDrain = 1,
}
impl From<PinconnectPinmodeOd0P0_30odEnum> for bool {
    #[inline(always)]
    fn from(variant: PinconnectPinmodeOd0P0_30odEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0_30OD` reader - Port 0 pin 30 open drain mode control"]
pub type P0_30odR = crate::BitReader<PinconnectPinmodeOd0P0_30odEnum>;
impl P0_30odR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmodeOd0P0_30odEnum {
        match self.bits {
            false => PinconnectPinmodeOd0P0_30odEnum::Normal,
            true => PinconnectPinmodeOd0P0_30odEnum::OpenDrain,
        }
    }
    #[doc = "Normal. P0.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_30odEnum::Normal
    }
    #[doc = "Open-drain. P0.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PinconnectPinmodeOd0P0_30odEnum::OpenDrain
    }
}
#[doc = "Field `P0_30OD` writer - Port 0 pin 30 open drain mode control"]
pub type P0_30odW<'a, REG> = crate::BitWriter<'a, REG, PinconnectPinmodeOd0P0_30odEnum>;
impl<'a, REG> P0_30odW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. P0.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_30odEnum::Normal)
    }
    #[doc = "Open-drain. P0.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmodeOd0P0_30odEnum::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_00od(&self) -> P0_00odR {
        P0_00odR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_01od(&self) -> P0_01odR {
        P0_01odR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    pub fn p0_02od(&self) -> P0_02odR {
        P0_02odR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    pub fn p0_03od(&self) -> P0_03odR {
        P0_03odR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    pub fn p0_04od(&self) -> P0_04odR {
        P0_04odR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    pub fn p0_05od(&self) -> P0_05odR {
        P0_05odR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    pub fn p0_06od(&self) -> P0_06odR {
        P0_06odR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    pub fn p0_07od(&self) -> P0_07odR {
        P0_07odR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    pub fn p0_08od(&self) -> P0_08odR {
        P0_08odR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    pub fn p0_09od(&self) -> P0_09odR {
        P0_09odR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_10od(&self) -> P0_10odR {
        P0_10odR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_11od(&self) -> P0_11odR {
        P0_11odR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    pub fn p0_15od(&self) -> P0_15odR {
        P0_15odR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    pub fn p0_16od(&self) -> P0_16odR {
        P0_16odR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    pub fn p0_17od(&self) -> P0_17odR {
        P0_17odR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    pub fn p0_18od(&self) -> P0_18odR {
        P0_18odR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_19od(&self) -> P0_19odR {
        P0_19odR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_20od(&self) -> P0_20odR {
        P0_20odR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    pub fn p0_21od(&self) -> P0_21odR {
        P0_21odR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    pub fn p0_22od(&self) -> P0_22odR {
        P0_22odR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    pub fn p0_23od(&self) -> P0_23odR {
        P0_23odR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    pub fn p0_24od(&self) -> P0_24odR {
        P0_24odR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    pub fn p0_25od(&self) -> P0_25odR {
        P0_25odR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    pub fn p0_26od(&self) -> P0_26odR {
        P0_26odR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    pub fn p0_29od(&self) -> P0_29odR {
        P0_29odR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    pub fn p0_30od(&self) -> P0_30odR {
        P0_30odR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_00od(&mut self) -> P0_00odW<'_, PinmodeOd0Spec> {
        P0_00odW::new(self, 0)
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_01od(&mut self) -> P0_01odW<'_, PinmodeOd0Spec> {
        P0_01odW::new(self, 1)
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    pub fn p0_02od(&mut self) -> P0_02odW<'_, PinmodeOd0Spec> {
        P0_02odW::new(self, 2)
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    pub fn p0_03od(&mut self) -> P0_03odW<'_, PinmodeOd0Spec> {
        P0_03odW::new(self, 3)
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    pub fn p0_04od(&mut self) -> P0_04odW<'_, PinmodeOd0Spec> {
        P0_04odW::new(self, 4)
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    pub fn p0_05od(&mut self) -> P0_05odW<'_, PinmodeOd0Spec> {
        P0_05odW::new(self, 5)
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    pub fn p0_06od(&mut self) -> P0_06odW<'_, PinmodeOd0Spec> {
        P0_06odW::new(self, 6)
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    pub fn p0_07od(&mut self) -> P0_07odW<'_, PinmodeOd0Spec> {
        P0_07odW::new(self, 7)
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    pub fn p0_08od(&mut self) -> P0_08odW<'_, PinmodeOd0Spec> {
        P0_08odW::new(self, 8)
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    pub fn p0_09od(&mut self) -> P0_09odW<'_, PinmodeOd0Spec> {
        P0_09odW::new(self, 9)
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_10od(&mut self) -> P0_10odW<'_, PinmodeOd0Spec> {
        P0_10odW::new(self, 10)
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_11od(&mut self) -> P0_11odW<'_, PinmodeOd0Spec> {
        P0_11odW::new(self, 11)
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    pub fn p0_15od(&mut self) -> P0_15odW<'_, PinmodeOd0Spec> {
        P0_15odW::new(self, 15)
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    pub fn p0_16od(&mut self) -> P0_16odW<'_, PinmodeOd0Spec> {
        P0_16odW::new(self, 16)
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    pub fn p0_17od(&mut self) -> P0_17odW<'_, PinmodeOd0Spec> {
        P0_17odW::new(self, 17)
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    pub fn p0_18od(&mut self) -> P0_18odW<'_, PinmodeOd0Spec> {
        P0_18odW::new(self, 18)
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_19od(&mut self) -> P0_19odW<'_, PinmodeOd0Spec> {
        P0_19odW::new(self, 19)
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_20od(&mut self) -> P0_20odW<'_, PinmodeOd0Spec> {
        P0_20odW::new(self, 20)
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    pub fn p0_21od(&mut self) -> P0_21odW<'_, PinmodeOd0Spec> {
        P0_21odW::new(self, 21)
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    pub fn p0_22od(&mut self) -> P0_22odW<'_, PinmodeOd0Spec> {
        P0_22odW::new(self, 22)
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    pub fn p0_23od(&mut self) -> P0_23odW<'_, PinmodeOd0Spec> {
        P0_23odW::new(self, 23)
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    pub fn p0_24od(&mut self) -> P0_24odW<'_, PinmodeOd0Spec> {
        P0_24odW::new(self, 24)
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    pub fn p0_25od(&mut self) -> P0_25odW<'_, PinmodeOd0Spec> {
        P0_25odW::new(self, 25)
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    pub fn p0_26od(&mut self) -> P0_26odW<'_, PinmodeOd0Spec> {
        P0_26odW::new(self, 26)
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    pub fn p0_29od(&mut self) -> P0_29odW<'_, PinmodeOd0Spec> {
        P0_29odW::new(self, 29)
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    pub fn p0_30od(&mut self) -> P0_30odW<'_, PinmodeOd0Spec> {
        P0_30odW::new(self, 30)
    }
}
#[doc = "Open drain mode control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode_od0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode_od0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmodeOd0Spec;
impl crate::RegisterSpec for PinmodeOd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od0::R`](R) reader structure"]
impl crate::Readable for PinmodeOd0Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od0::W`](W) writer structure"]
impl crate::Writable for PinmodeOd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE_OD0 to value 0"]
impl crate::Resettable for PinmodeOd0Spec {}
