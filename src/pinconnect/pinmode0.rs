#[doc = "Register `PINMODE0` reader"]
pub type R = crate::R<Pinmode0Spec>;
#[doc = "Register `PINMODE0` writer"]
pub type W = crate::W<Pinmode0Spec>;
#[doc = "Port 0 pin 0 on-chip pull-up/down resistor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_00modeEnum {
    #[doc = "0: Pull-up. P0.0 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.0 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.0 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.0 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_00modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_00modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_00modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_00modeEnum {}
#[doc = "Field `P0_00MODE` reader - Port 0 pin 0 on-chip pull-up/down resistor control."]
pub type P0_00modeR = crate::FieldReader<PinconnectPinmode0P0_00modeEnum>;
impl P0_00modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_00modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_00modeEnum::PullUp,
            1 => PinconnectPinmode0P0_00modeEnum::Repeater,
            2 => PinconnectPinmode0P0_00modeEnum::Disabled,
            3 => PinconnectPinmode0P0_00modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_00modeEnum::PullUp
    }
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_00modeEnum::Repeater
    }
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_00modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_00modeEnum::PullDown
    }
}
#[doc = "Field `P0_00MODE` writer - Port 0 pin 0 on-chip pull-up/down resistor control."]
pub type P0_00modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_00modeEnum, crate::Safe>;
impl<'a, REG> P0_00modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_00modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_00modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_00modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_00modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_01modeEnum {
    #[doc = "0: Pull-up. P0.1 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.1 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.1 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.1 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_01modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_01modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_01modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_01modeEnum {}
#[doc = "Field `P0_01MODE` reader - Port 0 pin 1 control."]
pub type P0_01modeR = crate::FieldReader<PinconnectPinmode0P0_01modeEnum>;
impl P0_01modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_01modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_01modeEnum::PullUp,
            1 => PinconnectPinmode0P0_01modeEnum::Repeater,
            2 => PinconnectPinmode0P0_01modeEnum::Disabled,
            3 => PinconnectPinmode0P0_01modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_01modeEnum::PullUp
    }
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_01modeEnum::Repeater
    }
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_01modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_01modeEnum::PullDown
    }
}
#[doc = "Field `P0_01MODE` writer - Port 0 pin 1 control."]
pub type P0_01modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_01modeEnum, crate::Safe>;
impl<'a, REG> P0_01modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_01modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_01modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_01modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_01modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 2 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_02modeEnum {
    #[doc = "0: Pull-up. P0.2 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.2 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.2 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.2 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_02modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_02modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_02modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_02modeEnum {}
#[doc = "Field `P0_02MODE` reader - Port 0 pin 2 control."]
pub type P0_02modeR = crate::FieldReader<PinconnectPinmode0P0_02modeEnum>;
impl P0_02modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_02modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_02modeEnum::PullUp,
            1 => PinconnectPinmode0P0_02modeEnum::Repeater,
            2 => PinconnectPinmode0P0_02modeEnum::Disabled,
            3 => PinconnectPinmode0P0_02modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_02modeEnum::PullUp
    }
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_02modeEnum::Repeater
    }
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_02modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_02modeEnum::PullDown
    }
}
#[doc = "Field `P0_02MODE` writer - Port 0 pin 2 control."]
pub type P0_02modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_02modeEnum, crate::Safe>;
impl<'a, REG> P0_02modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_02modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_02modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_02modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_02modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 3 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_03modeEnum {
    #[doc = "0: Pull-up. P0.3 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.3 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.3 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.3 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_03modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_03modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_03modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_03modeEnum {}
#[doc = "Field `P0_03MODE` reader - Port 0 pin 3 control."]
pub type P0_03modeR = crate::FieldReader<PinconnectPinmode0P0_03modeEnum>;
impl P0_03modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_03modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_03modeEnum::PullUp,
            1 => PinconnectPinmode0P0_03modeEnum::Repeater,
            2 => PinconnectPinmode0P0_03modeEnum::Disabled,
            3 => PinconnectPinmode0P0_03modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_03modeEnum::PullUp
    }
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_03modeEnum::Repeater
    }
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_03modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_03modeEnum::PullDown
    }
}
#[doc = "Field `P0_03MODE` writer - Port 0 pin 3 control."]
pub type P0_03modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_03modeEnum, crate::Safe>;
impl<'a, REG> P0_03modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_03modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_03modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_03modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_03modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_04modeEnum {
    #[doc = "0: Pull-up. P0.4 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.4 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.4 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.4 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_04modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_04modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_04modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_04modeEnum {}
#[doc = "Field `P0_04MODE` reader - Port 0 pin 4 control."]
pub type P0_04modeR = crate::FieldReader<PinconnectPinmode0P0_04modeEnum>;
impl P0_04modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_04modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_04modeEnum::PullUp,
            1 => PinconnectPinmode0P0_04modeEnum::Repeater,
            2 => PinconnectPinmode0P0_04modeEnum::Disabled,
            3 => PinconnectPinmode0P0_04modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_04modeEnum::PullUp
    }
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_04modeEnum::Repeater
    }
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_04modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_04modeEnum::PullDown
    }
}
#[doc = "Field `P0_04MODE` writer - Port 0 pin 4 control."]
pub type P0_04modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_04modeEnum, crate::Safe>;
impl<'a, REG> P0_04modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_04modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_04modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_04modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_04modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 5 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_05modeEnum {
    #[doc = "0: Pull-up. P0.5 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.5 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.5 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.5 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_05modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_05modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_05modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_05modeEnum {}
#[doc = "Field `P0_05MODE` reader - Port 0 pin 5 control."]
pub type P0_05modeR = crate::FieldReader<PinconnectPinmode0P0_05modeEnum>;
impl P0_05modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_05modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_05modeEnum::PullUp,
            1 => PinconnectPinmode0P0_05modeEnum::Repeater,
            2 => PinconnectPinmode0P0_05modeEnum::Disabled,
            3 => PinconnectPinmode0P0_05modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_05modeEnum::PullUp
    }
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_05modeEnum::Repeater
    }
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_05modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_05modeEnum::PullDown
    }
}
#[doc = "Field `P0_05MODE` writer - Port 0 pin 5 control."]
pub type P0_05modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_05modeEnum, crate::Safe>;
impl<'a, REG> P0_05modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_05modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_05modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_05modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_05modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 6 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_06mode {
    #[doc = "0: Pull-up enabled"]
    PullUp = 0,
    #[doc = "1: Repeater mode enabled"]
    Repeater = 1,
    #[doc = "2: Neither pull-up nor pull-down"]
    None = 2,
    #[doc = "3: Pull-down enabled"]
    PullDown = 3,
}
impl From<P0_06mode> for u8 {
    #[inline(always)]
    fn from(variant: P0_06mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_06mode {
    type Ux = u8;
}
impl crate::IsEnum for P0_06mode {}
#[doc = "Field `P0_06MODE` reader - Port 0 pin 6 control."]
pub type P0_06modeR = crate::FieldReader<P0_06mode>;
impl P0_06modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_06mode {
        match self.bits {
            0 => P0_06mode::PullUp,
            1 => P0_06mode::Repeater,
            2 => P0_06mode::None,
            3 => P0_06mode::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up enabled"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_06mode::PullUp
    }
    #[doc = "Repeater mode enabled"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_06mode::Repeater
    }
    #[doc = "Neither pull-up nor pull-down"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P0_06mode::None
    }
    #[doc = "Pull-down enabled"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_06mode::PullDown
    }
}
#[doc = "Field `P0_06MODE` writer - Port 0 pin 6 control."]
pub type P0_06modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, P0_06mode, crate::Safe>;
impl<'a, REG> P0_06modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up enabled"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06mode::PullUp)
    }
    #[doc = "Repeater mode enabled"]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06mode::Repeater)
    }
    #[doc = "Neither pull-up nor pull-down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06mode::None)
    }
    #[doc = "Pull-down enabled"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06mode::PullDown)
    }
}
#[doc = "Port 0 pin 7 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_07modeEnum {
    #[doc = "0: Pull-up. P0.7 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.7 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.7 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.7 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_07modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_07modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_07modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_07modeEnum {}
#[doc = "Field `P0_07MODE` reader - Port 0 pin 7 control."]
pub type P0_07modeR = crate::FieldReader<PinconnectPinmode0P0_07modeEnum>;
impl P0_07modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_07modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_07modeEnum::PullUp,
            1 => PinconnectPinmode0P0_07modeEnum::Repeater,
            2 => PinconnectPinmode0P0_07modeEnum::Disabled,
            3 => PinconnectPinmode0P0_07modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_07modeEnum::PullUp
    }
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_07modeEnum::Repeater
    }
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_07modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_07modeEnum::PullDown
    }
}
#[doc = "Field `P0_07MODE` writer - Port 0 pin 7 control."]
pub type P0_07modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_07modeEnum, crate::Safe>;
impl<'a, REG> P0_07modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_07modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_07modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_07modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_07modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_08modeEnum {
    #[doc = "0: Pull-up. P0.8 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.8 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.8 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.8 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_08modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_08modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_08modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_08modeEnum {}
#[doc = "Field `P0_08MODE` reader - Port 0 pin 8 control."]
pub type P0_08modeR = crate::FieldReader<PinconnectPinmode0P0_08modeEnum>;
impl P0_08modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_08modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_08modeEnum::PullUp,
            1 => PinconnectPinmode0P0_08modeEnum::Repeater,
            2 => PinconnectPinmode0P0_08modeEnum::Disabled,
            3 => PinconnectPinmode0P0_08modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_08modeEnum::PullUp
    }
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_08modeEnum::Repeater
    }
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_08modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_08modeEnum::PullDown
    }
}
#[doc = "Field `P0_08MODE` writer - Port 0 pin 8 control."]
pub type P0_08modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_08modeEnum, crate::Safe>;
impl<'a, REG> P0_08modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_08modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_08modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_08modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_08modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_09modeEnum {
    #[doc = "0: Pull-up. P0.9 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.9 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.9 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.9 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_09modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_09modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_09modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_09modeEnum {}
#[doc = "Field `P0_09MODE` reader - Port 0 pin 9 control."]
pub type P0_09modeR = crate::FieldReader<PinconnectPinmode0P0_09modeEnum>;
impl P0_09modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_09modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_09modeEnum::PullUp,
            1 => PinconnectPinmode0P0_09modeEnum::Repeater,
            2 => PinconnectPinmode0P0_09modeEnum::Disabled,
            3 => PinconnectPinmode0P0_09modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_09modeEnum::PullUp
    }
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_09modeEnum::Repeater
    }
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_09modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_09modeEnum::PullDown
    }
}
#[doc = "Field `P0_09MODE` writer - Port 0 pin 9 control."]
pub type P0_09modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_09modeEnum, crate::Safe>;
impl<'a, REG> P0_09modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_09modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_09modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_09modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_09modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_10modeEnum {
    #[doc = "0: Pull-up. P0.10 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.10 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.10 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.10 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_10modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_10modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_10modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_10modeEnum {}
#[doc = "Field `P0_10MODE` reader - Port 0 pin 10 control."]
pub type P0_10modeR = crate::FieldReader<PinconnectPinmode0P0_10modeEnum>;
impl P0_10modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_10modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_10modeEnum::PullUp,
            1 => PinconnectPinmode0P0_10modeEnum::Repeater,
            2 => PinconnectPinmode0P0_10modeEnum::Disabled,
            3 => PinconnectPinmode0P0_10modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_10modeEnum::PullUp
    }
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_10modeEnum::Repeater
    }
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_10modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_10modeEnum::PullDown
    }
}
#[doc = "Field `P0_10MODE` writer - Port 0 pin 10 control."]
pub type P0_10modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_10modeEnum, crate::Safe>;
impl<'a, REG> P0_10modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_10modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_10modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_10modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_10modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 11 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_11modeEnum {
    #[doc = "0: Pull-up. P0.11 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.11 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.11 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.11 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_11modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_11modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_11modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_11modeEnum {}
#[doc = "Field `P0_11MODE` reader - Port 0 pin 11 control."]
pub type P0_11modeR = crate::FieldReader<PinconnectPinmode0P0_11modeEnum>;
impl P0_11modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_11modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_11modeEnum::PullUp,
            1 => PinconnectPinmode0P0_11modeEnum::Repeater,
            2 => PinconnectPinmode0P0_11modeEnum::Disabled,
            3 => PinconnectPinmode0P0_11modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_11modeEnum::PullUp
    }
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_11modeEnum::Repeater
    }
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_11modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_11modeEnum::PullDown
    }
}
#[doc = "Field `P0_11MODE` writer - Port 0 pin 11 control."]
pub type P0_11modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_11modeEnum, crate::Safe>;
impl<'a, REG> P0_11modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_11modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_11modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_11modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_11modeEnum::PullDown)
    }
}
#[doc = "Port 0 pin 15 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode0P0_15modeEnum {
    #[doc = "0: Pull-up. P0.15 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P0.15 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P0.15 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P0.15 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode0P0_15modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode0P0_15modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode0P0_15modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode0P0_15modeEnum {}
#[doc = "Field `P0_15MODE` reader - Port 0 pin 15 control."]
pub type P0_15modeR = crate::FieldReader<PinconnectPinmode0P0_15modeEnum>;
impl P0_15modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode0P0_15modeEnum {
        match self.bits {
            0 => PinconnectPinmode0P0_15modeEnum::PullUp,
            1 => PinconnectPinmode0P0_15modeEnum::Repeater,
            2 => PinconnectPinmode0P0_15modeEnum::Disabled,
            3 => PinconnectPinmode0P0_15modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode0P0_15modeEnum::PullUp
    }
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode0P0_15modeEnum::Repeater
    }
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode0P0_15modeEnum::Disabled
    }
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode0P0_15modeEnum::PullDown
    }
}
#[doc = "Field `P0_15MODE` writer - Port 0 pin 15 control."]
pub type P0_15modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode0P0_15modeEnum, crate::Safe>;
impl<'a, REG> P0_15modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_15modeEnum::PullUp)
    }
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_15modeEnum::Repeater)
    }
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_15modeEnum::Disabled)
    }
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode0P0_15modeEnum::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    pub fn p0_00mode(&self) -> P0_00modeR {
        P0_00modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    pub fn p0_01mode(&self) -> P0_01modeR {
        P0_01modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    pub fn p0_02mode(&self) -> P0_02modeR {
        P0_02modeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    pub fn p0_03mode(&self) -> P0_03modeR {
        P0_03modeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    pub fn p0_04mode(&self) -> P0_04modeR {
        P0_04modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    pub fn p0_05mode(&self) -> P0_05modeR {
        P0_05modeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    pub fn p0_06mode(&self) -> P0_06modeR {
        P0_06modeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    pub fn p0_07mode(&self) -> P0_07modeR {
        P0_07modeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    pub fn p0_08mode(&self) -> P0_08modeR {
        P0_08modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    pub fn p0_09mode(&self) -> P0_09modeR {
        P0_09modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    pub fn p0_10mode(&self) -> P0_10modeR {
        P0_10modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    pub fn p0_11mode(&self) -> P0_11modeR {
        P0_11modeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    pub fn p0_15mode(&self) -> P0_15modeR {
        P0_15modeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    pub fn p0_00mode(&mut self) -> P0_00modeW<'_, Pinmode0Spec> {
        P0_00modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    pub fn p0_01mode(&mut self) -> P0_01modeW<'_, Pinmode0Spec> {
        P0_01modeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    pub fn p0_02mode(&mut self) -> P0_02modeW<'_, Pinmode0Spec> {
        P0_02modeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    pub fn p0_03mode(&mut self) -> P0_03modeW<'_, Pinmode0Spec> {
        P0_03modeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    pub fn p0_04mode(&mut self) -> P0_04modeW<'_, Pinmode0Spec> {
        P0_04modeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    pub fn p0_05mode(&mut self) -> P0_05modeW<'_, Pinmode0Spec> {
        P0_05modeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    pub fn p0_06mode(&mut self) -> P0_06modeW<'_, Pinmode0Spec> {
        P0_06modeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    pub fn p0_07mode(&mut self) -> P0_07modeW<'_, Pinmode0Spec> {
        P0_07modeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    pub fn p0_08mode(&mut self) -> P0_08modeW<'_, Pinmode0Spec> {
        P0_08modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    pub fn p0_09mode(&mut self) -> P0_09modeW<'_, Pinmode0Spec> {
        P0_09modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    pub fn p0_10mode(&mut self) -> P0_10modeW<'_, Pinmode0Spec> {
        P0_10modeW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    pub fn p0_11mode(&mut self) -> P0_11modeW<'_, Pinmode0Spec> {
        P0_11modeW::new(self, 22)
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    pub fn p0_15mode(&mut self) -> P0_15modeW<'_, Pinmode0Spec> {
        P0_15modeW::new(self, 30)
    }
}
#[doc = "Pin mode select register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode0Spec;
impl crate::RegisterSpec for Pinmode0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode0::R`](R) reader structure"]
impl crate::Readable for Pinmode0Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode0::W`](W) writer structure"]
impl crate::Writable for Pinmode0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE0 to value 0"]
impl crate::Resettable for Pinmode0Spec {}
