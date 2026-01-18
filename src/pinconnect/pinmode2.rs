#[doc = "Register `PINMODE2` reader"]
pub type R = crate::R<Pinmode2Spec>;
#[doc = "Register `PINMODE2` writer"]
pub type W = crate::W<Pinmode2Spec>;
#[doc = "Port 1 pin 0 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode2P1_00modeEnum {
    #[doc = "0: Pull-up. P1.0 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.0 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.0 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.0 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode2P1_00modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode2P1_00modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode2P1_00modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode2P1_00modeEnum {}
#[doc = "Field `P1_00MODE` reader - Port 1 pin 0 control."]
pub type P1_00modeR = crate::FieldReader<PinconnectPinmode2P1_00modeEnum>;
impl P1_00modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode2P1_00modeEnum {
        match self.bits {
            0 => PinconnectPinmode2P1_00modeEnum::PullUp,
            1 => PinconnectPinmode2P1_00modeEnum::Repeater,
            2 => PinconnectPinmode2P1_00modeEnum::Disabled,
            3 => PinconnectPinmode2P1_00modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode2P1_00modeEnum::PullUp
    }
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode2P1_00modeEnum::Repeater
    }
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode2P1_00modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode2P1_00modeEnum::PullDown
    }
}
#[doc = "Field `P1_00MODE` writer - Port 1 pin 0 control."]
pub type P1_00modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode2P1_00modeEnum, crate::Safe>;
impl<'a, REG> P1_00modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_00modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_00modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_00modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_00modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode2P1_01modeEnum {
    #[doc = "0: Pull-up. P1.1 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.1 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.1 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.1 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode2P1_01modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode2P1_01modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode2P1_01modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode2P1_01modeEnum {}
#[doc = "Field `P1_01MODE` reader - Port 1 pin 1 control."]
pub type P1_01modeR = crate::FieldReader<PinconnectPinmode2P1_01modeEnum>;
impl P1_01modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode2P1_01modeEnum {
        match self.bits {
            0 => PinconnectPinmode2P1_01modeEnum::PullUp,
            1 => PinconnectPinmode2P1_01modeEnum::Repeater,
            2 => PinconnectPinmode2P1_01modeEnum::Disabled,
            3 => PinconnectPinmode2P1_01modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode2P1_01modeEnum::PullUp
    }
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode2P1_01modeEnum::Repeater
    }
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode2P1_01modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode2P1_01modeEnum::PullDown
    }
}
#[doc = "Field `P1_01MODE` writer - Port 1 pin 1 control."]
pub type P1_01modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode2P1_01modeEnum, crate::Safe>;
impl<'a, REG> P1_01modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_01modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_01modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_01modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_01modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode2P1_04modeEnum {
    #[doc = "0: Pull-up. P1.4 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.4 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.4 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.4 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode2P1_04modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode2P1_04modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode2P1_04modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode2P1_04modeEnum {}
#[doc = "Field `P1_04MODE` reader - Port 1 pin 4 control."]
pub type P1_04modeR = crate::FieldReader<PinconnectPinmode2P1_04modeEnum>;
impl P1_04modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode2P1_04modeEnum {
        match self.bits {
            0 => PinconnectPinmode2P1_04modeEnum::PullUp,
            1 => PinconnectPinmode2P1_04modeEnum::Repeater,
            2 => PinconnectPinmode2P1_04modeEnum::Disabled,
            3 => PinconnectPinmode2P1_04modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode2P1_04modeEnum::PullUp
    }
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode2P1_04modeEnum::Repeater
    }
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode2P1_04modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode2P1_04modeEnum::PullDown
    }
}
#[doc = "Field `P1_04MODE` writer - Port 1 pin 4 control."]
pub type P1_04modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode2P1_04modeEnum, crate::Safe>;
impl<'a, REG> P1_04modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_04modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_04modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_04modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_04modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode2P1_08modeEnum {
    #[doc = "0: Pull-up. P1.8 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.8 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.8 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.8 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode2P1_08modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode2P1_08modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode2P1_08modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode2P1_08modeEnum {}
#[doc = "Field `P1_08MODE` reader - Port 1 pin 8 control."]
pub type P1_08modeR = crate::FieldReader<PinconnectPinmode2P1_08modeEnum>;
impl P1_08modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode2P1_08modeEnum {
        match self.bits {
            0 => PinconnectPinmode2P1_08modeEnum::PullUp,
            1 => PinconnectPinmode2P1_08modeEnum::Repeater,
            2 => PinconnectPinmode2P1_08modeEnum::Disabled,
            3 => PinconnectPinmode2P1_08modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode2P1_08modeEnum::PullUp
    }
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode2P1_08modeEnum::Repeater
    }
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode2P1_08modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode2P1_08modeEnum::PullDown
    }
}
#[doc = "Field `P1_08MODE` writer - Port 1 pin 8 control."]
pub type P1_08modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode2P1_08modeEnum, crate::Safe>;
impl<'a, REG> P1_08modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_08modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_08modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_08modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_08modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode2P1_09modeEnum {
    #[doc = "0: Pull-up. P1.9 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.9 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.9 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.9 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode2P1_09modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode2P1_09modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode2P1_09modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode2P1_09modeEnum {}
#[doc = "Field `P1_09MODE` reader - Port 1 pin 9 control."]
pub type P1_09modeR = crate::FieldReader<PinconnectPinmode2P1_09modeEnum>;
impl P1_09modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode2P1_09modeEnum {
        match self.bits {
            0 => PinconnectPinmode2P1_09modeEnum::PullUp,
            1 => PinconnectPinmode2P1_09modeEnum::Repeater,
            2 => PinconnectPinmode2P1_09modeEnum::Disabled,
            3 => PinconnectPinmode2P1_09modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode2P1_09modeEnum::PullUp
    }
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode2P1_09modeEnum::Repeater
    }
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode2P1_09modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode2P1_09modeEnum::PullDown
    }
}
#[doc = "Field `P1_09MODE` writer - Port 1 pin 9 control."]
pub type P1_09modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode2P1_09modeEnum, crate::Safe>;
impl<'a, REG> P1_09modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_09modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_09modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_09modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_09modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode2P1_10modeEnum {
    #[doc = "0: Pull-up. P1.10 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.10 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.10 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.10 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode2P1_10modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode2P1_10modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode2P1_10modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode2P1_10modeEnum {}
#[doc = "Field `P1_10MODE` reader - Port 1 pin 10 control."]
pub type P1_10modeR = crate::FieldReader<PinconnectPinmode2P1_10modeEnum>;
impl P1_10modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode2P1_10modeEnum {
        match self.bits {
            0 => PinconnectPinmode2P1_10modeEnum::PullUp,
            1 => PinconnectPinmode2P1_10modeEnum::Repeater,
            2 => PinconnectPinmode2P1_10modeEnum::Disabled,
            3 => PinconnectPinmode2P1_10modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode2P1_10modeEnum::PullUp
    }
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode2P1_10modeEnum::Repeater
    }
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode2P1_10modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode2P1_10modeEnum::PullDown
    }
}
#[doc = "Field `P1_10MODE` writer - Port 1 pin 10 control."]
pub type P1_10modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode2P1_10modeEnum, crate::Safe>;
impl<'a, REG> P1_10modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_10modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_10modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_10modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_10modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 14 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode2P1_14modeEnum {
    #[doc = "0: Pull-up. P1.14 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.14 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.14 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.14 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode2P1_14modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode2P1_14modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode2P1_14modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode2P1_14modeEnum {}
#[doc = "Field `P1_14MODE` reader - Port 1 pin 14 control."]
pub type P1_14modeR = crate::FieldReader<PinconnectPinmode2P1_14modeEnum>;
impl P1_14modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode2P1_14modeEnum {
        match self.bits {
            0 => PinconnectPinmode2P1_14modeEnum::PullUp,
            1 => PinconnectPinmode2P1_14modeEnum::Repeater,
            2 => PinconnectPinmode2P1_14modeEnum::Disabled,
            3 => PinconnectPinmode2P1_14modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode2P1_14modeEnum::PullUp
    }
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode2P1_14modeEnum::Repeater
    }
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode2P1_14modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode2P1_14modeEnum::PullDown
    }
}
#[doc = "Field `P1_14MODE` writer - Port 1 pin 14 control."]
pub type P1_14modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode2P1_14modeEnum, crate::Safe>;
impl<'a, REG> P1_14modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_14modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_14modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_14modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_14modeEnum::PullDown)
    }
}
#[doc = "Port 1 pin 15 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode2P1_15modeEnum {
    #[doc = "0: Pull-up. P1.15 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P1.15 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P1.15 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P1.15 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode2P1_15modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode2P1_15modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode2P1_15modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode2P1_15modeEnum {}
#[doc = "Field `P1_15MODE` reader - Port 1 pin 15 control."]
pub type P1_15modeR = crate::FieldReader<PinconnectPinmode2P1_15modeEnum>;
impl P1_15modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode2P1_15modeEnum {
        match self.bits {
            0 => PinconnectPinmode2P1_15modeEnum::PullUp,
            1 => PinconnectPinmode2P1_15modeEnum::Repeater,
            2 => PinconnectPinmode2P1_15modeEnum::Disabled,
            3 => PinconnectPinmode2P1_15modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode2P1_15modeEnum::PullUp
    }
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode2P1_15modeEnum::Repeater
    }
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode2P1_15modeEnum::Disabled
    }
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode2P1_15modeEnum::PullDown
    }
}
#[doc = "Field `P1_15MODE` writer - Port 1 pin 15 control."]
pub type P1_15modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode2P1_15modeEnum, crate::Safe>;
impl<'a, REG> P1_15modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_15modeEnum::PullUp)
    }
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_15modeEnum::Repeater)
    }
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_15modeEnum::Disabled)
    }
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode2P1_15modeEnum::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    pub fn p1_00mode(&self) -> P1_00modeR {
        P1_00modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    pub fn p1_01mode(&self) -> P1_01modeR {
        P1_01modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    pub fn p1_04mode(&self) -> P1_04modeR {
        P1_04modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    pub fn p1_08mode(&self) -> P1_08modeR {
        P1_08modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    pub fn p1_09mode(&self) -> P1_09modeR {
        P1_09modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    pub fn p1_10mode(&self) -> P1_10modeR {
        P1_10modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    pub fn p1_14mode(&self) -> P1_14modeR {
        P1_14modeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    pub fn p1_15mode(&self) -> P1_15modeR {
        P1_15modeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    pub fn p1_00mode(&mut self) -> P1_00modeW<'_, Pinmode2Spec> {
        P1_00modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    pub fn p1_01mode(&mut self) -> P1_01modeW<'_, Pinmode2Spec> {
        P1_01modeW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    pub fn p1_04mode(&mut self) -> P1_04modeW<'_, Pinmode2Spec> {
        P1_04modeW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    pub fn p1_08mode(&mut self) -> P1_08modeW<'_, Pinmode2Spec> {
        P1_08modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    pub fn p1_09mode(&mut self) -> P1_09modeW<'_, Pinmode2Spec> {
        P1_09modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    pub fn p1_10mode(&mut self) -> P1_10modeW<'_, Pinmode2Spec> {
        P1_10modeW::new(self, 20)
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    pub fn p1_14mode(&mut self) -> P1_14modeW<'_, Pinmode2Spec> {
        P1_14modeW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    pub fn p1_15mode(&mut self) -> P1_15modeW<'_, Pinmode2Spec> {
        P1_15modeW::new(self, 30)
    }
}
#[doc = "Pin mode select register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode2Spec;
impl crate::RegisterSpec for Pinmode2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode2::R`](R) reader structure"]
impl crate::Readable for Pinmode2Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode2::W`](W) writer structure"]
impl crate::Writable for Pinmode2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE2 to value 0"]
impl crate::Resettable for Pinmode2Spec {}
