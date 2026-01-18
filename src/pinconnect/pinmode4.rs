#[doc = "Register `PINMODE4` reader"]
pub type R = crate::R<Pinmode4Spec>;
#[doc = "Register `PINMODE4` writer"]
pub type W = crate::W<Pinmode4Spec>;
#[doc = "Port 2 pin 0 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_00modeEnum {
    #[doc = "0: Pull-up. P2.0 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.0 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.0 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.0 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_00modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_00modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_00modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_00modeEnum {}
#[doc = "Field `P2_00MODE` reader - Port 2 pin 0 control."]
pub type P2_00modeR = crate::FieldReader<PinconnectPinmode4P2_00modeEnum>;
impl P2_00modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_00modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_00modeEnum::PullUp,
            1 => PinconnectPinmode4P2_00modeEnum::Repeater,
            2 => PinconnectPinmode4P2_00modeEnum::Disabled,
            3 => PinconnectPinmode4P2_00modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_00modeEnum::PullUp
    }
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_00modeEnum::Repeater
    }
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_00modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_00modeEnum::PullDown
    }
}
#[doc = "Field `P2_00MODE` writer - Port 2 pin 0 control."]
pub type P2_00modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_00modeEnum, crate::Safe>;
impl<'a, REG> P2_00modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_00modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_00modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_00modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_00modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_01modeEnum {
    #[doc = "0: Pull-up. P2.1 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.1 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.1 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.1 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_01modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_01modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_01modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_01modeEnum {}
#[doc = "Field `P2_01MODE` reader - Port 2 pin 1 control."]
pub type P2_01modeR = crate::FieldReader<PinconnectPinmode4P2_01modeEnum>;
impl P2_01modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_01modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_01modeEnum::PullUp,
            1 => PinconnectPinmode4P2_01modeEnum::Repeater,
            2 => PinconnectPinmode4P2_01modeEnum::Disabled,
            3 => PinconnectPinmode4P2_01modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_01modeEnum::PullUp
    }
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_01modeEnum::Repeater
    }
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_01modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_01modeEnum::PullDown
    }
}
#[doc = "Field `P2_01MODE` writer - Port 2 pin 1 control."]
pub type P2_01modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_01modeEnum, crate::Safe>;
impl<'a, REG> P2_01modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_01modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_01modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_01modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_01modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 2 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_02modeEnum {
    #[doc = "0: Pull-up. P2.2 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.2 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.2 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.2 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_02modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_02modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_02modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_02modeEnum {}
#[doc = "Field `P2_02MODE` reader - Port 2 pin 2 control."]
pub type P2_02modeR = crate::FieldReader<PinconnectPinmode4P2_02modeEnum>;
impl P2_02modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_02modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_02modeEnum::PullUp,
            1 => PinconnectPinmode4P2_02modeEnum::Repeater,
            2 => PinconnectPinmode4P2_02modeEnum::Disabled,
            3 => PinconnectPinmode4P2_02modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_02modeEnum::PullUp
    }
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_02modeEnum::Repeater
    }
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_02modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_02modeEnum::PullDown
    }
}
#[doc = "Field `P2_02MODE` writer - Port 2 pin 2 control."]
pub type P2_02modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_02modeEnum, crate::Safe>;
impl<'a, REG> P2_02modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_02modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_02modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_02modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_02modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 3 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_03modeEnum {
    #[doc = "0: Pull-up. P2.3 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.3 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.3 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.3 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_03modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_03modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_03modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_03modeEnum {}
#[doc = "Field `P2_03MODE` reader - Port 2 pin 3 control."]
pub type P2_03modeR = crate::FieldReader<PinconnectPinmode4P2_03modeEnum>;
impl P2_03modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_03modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_03modeEnum::PullUp,
            1 => PinconnectPinmode4P2_03modeEnum::Repeater,
            2 => PinconnectPinmode4P2_03modeEnum::Disabled,
            3 => PinconnectPinmode4P2_03modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_03modeEnum::PullUp
    }
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_03modeEnum::Repeater
    }
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_03modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_03modeEnum::PullDown
    }
}
#[doc = "Field `P2_03MODE` writer - Port 2 pin 3 control."]
pub type P2_03modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_03modeEnum, crate::Safe>;
impl<'a, REG> P2_03modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_03modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_03modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_03modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_03modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_04modeEnum {
    #[doc = "0: Pull-up. P2.4 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.4 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.4 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.4 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_04modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_04modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_04modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_04modeEnum {}
#[doc = "Field `P2_04MODE` reader - Port 2 pin 4 control."]
pub type P2_04modeR = crate::FieldReader<PinconnectPinmode4P2_04modeEnum>;
impl P2_04modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_04modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_04modeEnum::PullUp,
            1 => PinconnectPinmode4P2_04modeEnum::Repeater,
            2 => PinconnectPinmode4P2_04modeEnum::Disabled,
            3 => PinconnectPinmode4P2_04modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_04modeEnum::PullUp
    }
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_04modeEnum::Repeater
    }
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_04modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_04modeEnum::PullDown
    }
}
#[doc = "Field `P2_04MODE` writer - Port 2 pin 4 control."]
pub type P2_04modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_04modeEnum, crate::Safe>;
impl<'a, REG> P2_04modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_04modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_04modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_04modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_04modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 5 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_05modeEnum {
    #[doc = "0: Pull-up. P2.5 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.5 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.5 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.5 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_05modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_05modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_05modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_05modeEnum {}
#[doc = "Field `P2_05MODE` reader - Port 2 pin 5 control."]
pub type P2_05modeR = crate::FieldReader<PinconnectPinmode4P2_05modeEnum>;
impl P2_05modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_05modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_05modeEnum::PullUp,
            1 => PinconnectPinmode4P2_05modeEnum::Repeater,
            2 => PinconnectPinmode4P2_05modeEnum::Disabled,
            3 => PinconnectPinmode4P2_05modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_05modeEnum::PullUp
    }
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_05modeEnum::Repeater
    }
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_05modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_05modeEnum::PullDown
    }
}
#[doc = "Field `P2_05MODE` writer - Port 2 pin 5 control."]
pub type P2_05modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_05modeEnum, crate::Safe>;
impl<'a, REG> P2_05modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_05modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_05modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_05modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_05modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 6 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_06modeEnum {
    #[doc = "0: Pull-up. P2.6 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.6 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.6 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.6 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_06modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_06modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_06modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_06modeEnum {}
#[doc = "Field `P2_06MODE` reader - Port 2 pin 6 control."]
pub type P2_06modeR = crate::FieldReader<PinconnectPinmode4P2_06modeEnum>;
impl P2_06modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_06modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_06modeEnum::PullUp,
            1 => PinconnectPinmode4P2_06modeEnum::Repeater,
            2 => PinconnectPinmode4P2_06modeEnum::Disabled,
            3 => PinconnectPinmode4P2_06modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_06modeEnum::PullUp
    }
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_06modeEnum::Repeater
    }
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_06modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_06modeEnum::PullDown
    }
}
#[doc = "Field `P2_06MODE` writer - Port 2 pin 6 control."]
pub type P2_06modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_06modeEnum, crate::Safe>;
impl<'a, REG> P2_06modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_06modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_06modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_06modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_06modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 7 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_07modeEnum {
    #[doc = "0: Pull-up. P2.7 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.7 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.7 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.7 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_07modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_07modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_07modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_07modeEnum {}
#[doc = "Field `P2_07MODE` reader - Port 2 pin 7 control."]
pub type P2_07modeR = crate::FieldReader<PinconnectPinmode4P2_07modeEnum>;
impl P2_07modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_07modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_07modeEnum::PullUp,
            1 => PinconnectPinmode4P2_07modeEnum::Repeater,
            2 => PinconnectPinmode4P2_07modeEnum::Disabled,
            3 => PinconnectPinmode4P2_07modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_07modeEnum::PullUp
    }
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_07modeEnum::Repeater
    }
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_07modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_07modeEnum::PullDown
    }
}
#[doc = "Field `P2_07MODE` writer - Port 2 pin 7 control."]
pub type P2_07modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_07modeEnum, crate::Safe>;
impl<'a, REG> P2_07modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_07modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_07modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_07modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_07modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_08modeEnum {
    #[doc = "0: Pull-up. P2.8 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.8 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.8 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.8 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_08modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_08modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_08modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_08modeEnum {}
#[doc = "Field `P2_08MODE` reader - Port 2 pin 8 control."]
pub type P2_08modeR = crate::FieldReader<PinconnectPinmode4P2_08modeEnum>;
impl P2_08modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_08modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_08modeEnum::PullUp,
            1 => PinconnectPinmode4P2_08modeEnum::Repeater,
            2 => PinconnectPinmode4P2_08modeEnum::Disabled,
            3 => PinconnectPinmode4P2_08modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_08modeEnum::PullUp
    }
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_08modeEnum::Repeater
    }
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_08modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_08modeEnum::PullDown
    }
}
#[doc = "Field `P2_08MODE` writer - Port 2 pin 8 control."]
pub type P2_08modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_08modeEnum, crate::Safe>;
impl<'a, REG> P2_08modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_08modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_08modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_08modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_08modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_09modeEnum {
    #[doc = "0: Pull-up. P2.9 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.9 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.9 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.9 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_09modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_09modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_09modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_09modeEnum {}
#[doc = "Field `P2_09MODE` reader - Port 2 pin 9 control."]
pub type P2_09modeR = crate::FieldReader<PinconnectPinmode4P2_09modeEnum>;
impl P2_09modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_09modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_09modeEnum::PullUp,
            1 => PinconnectPinmode4P2_09modeEnum::Repeater,
            2 => PinconnectPinmode4P2_09modeEnum::Disabled,
            3 => PinconnectPinmode4P2_09modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_09modeEnum::PullUp
    }
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_09modeEnum::Repeater
    }
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_09modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_09modeEnum::PullDown
    }
}
#[doc = "Field `P2_09MODE` writer - Port 2 pin 9 control."]
pub type P2_09modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_09modeEnum, crate::Safe>;
impl<'a, REG> P2_09modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_09modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_09modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_09modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_09modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_10modeEnum {
    #[doc = "0: Pull-up. P2.10 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.10 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.10 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.10 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_10modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_10modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_10modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_10modeEnum {}
#[doc = "Field `P2_10MODE` reader - Port 2 pin 10 control."]
pub type P2_10modeR = crate::FieldReader<PinconnectPinmode4P2_10modeEnum>;
impl P2_10modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_10modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_10modeEnum::PullUp,
            1 => PinconnectPinmode4P2_10modeEnum::Repeater,
            2 => PinconnectPinmode4P2_10modeEnum::Disabled,
            3 => PinconnectPinmode4P2_10modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_10modeEnum::PullUp
    }
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_10modeEnum::Repeater
    }
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_10modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_10modeEnum::PullDown
    }
}
#[doc = "Field `P2_10MODE` writer - Port 2 pin 10 control."]
pub type P2_10modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_10modeEnum, crate::Safe>;
impl<'a, REG> P2_10modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_10modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_10modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_10modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_10modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 11 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_11modeEnum {
    #[doc = "0: Pull-up. P2.11 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.11 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.11 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.11 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_11modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_11modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_11modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_11modeEnum {}
#[doc = "Field `P2_11MODE` reader - Port 2 pin 11 control."]
pub type P2_11modeR = crate::FieldReader<PinconnectPinmode4P2_11modeEnum>;
impl P2_11modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_11modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_11modeEnum::PullUp,
            1 => PinconnectPinmode4P2_11modeEnum::Repeater,
            2 => PinconnectPinmode4P2_11modeEnum::Disabled,
            3 => PinconnectPinmode4P2_11modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_11modeEnum::PullUp
    }
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_11modeEnum::Repeater
    }
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_11modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_11modeEnum::PullDown
    }
}
#[doc = "Field `P2_11MODE` writer - Port 2 pin 11 control."]
pub type P2_11modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_11modeEnum, crate::Safe>;
impl<'a, REG> P2_11modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_11modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_11modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_11modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_11modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 12 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_12modeEnum {
    #[doc = "0: Pull-up. P2.12 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.12 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.12 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.12 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_12modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_12modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_12modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_12modeEnum {}
#[doc = "Field `P2_12MODE` reader - Port 2 pin 12 control."]
pub type P2_12modeR = crate::FieldReader<PinconnectPinmode4P2_12modeEnum>;
impl P2_12modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_12modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_12modeEnum::PullUp,
            1 => PinconnectPinmode4P2_12modeEnum::Repeater,
            2 => PinconnectPinmode4P2_12modeEnum::Disabled,
            3 => PinconnectPinmode4P2_12modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_12modeEnum::PullUp
    }
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_12modeEnum::Repeater
    }
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_12modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_12modeEnum::PullDown
    }
}
#[doc = "Field `P2_12MODE` writer - Port 2 pin 12 control."]
pub type P2_12modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_12modeEnum, crate::Safe>;
impl<'a, REG> P2_12modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_12modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_12modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_12modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_12modeEnum::PullDown)
    }
}
#[doc = "Port 2 pin 13 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinconnectPinmode4P2_13modeEnum {
    #[doc = "0: Pull-up. P2.13 pin has a pull-up resistor enabled."]
    PullUp = 0,
    #[doc = "1: Repeater. P2.13 pin has repeater mode enabled."]
    Repeater = 1,
    #[doc = "2: Disabled. P2.13 pin has neither pull-up nor pull-down."]
    Disabled = 2,
    #[doc = "3: Pull-down. P2.13 has a pull-down resistor enabled."]
    PullDown = 3,
}
impl From<PinconnectPinmode4P2_13modeEnum> for u8 {
    #[inline(always)]
    fn from(variant: PinconnectPinmode4P2_13modeEnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PinconnectPinmode4P2_13modeEnum {
    type Ux = u8;
}
impl crate::IsEnum for PinconnectPinmode4P2_13modeEnum {}
#[doc = "Field `P2_13MODE` reader - Port 2 pin 13 control."]
pub type P2_13modeR = crate::FieldReader<PinconnectPinmode4P2_13modeEnum>;
impl P2_13modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinconnectPinmode4P2_13modeEnum {
        match self.bits {
            0 => PinconnectPinmode4P2_13modeEnum::PullUp,
            1 => PinconnectPinmode4P2_13modeEnum::Repeater,
            2 => PinconnectPinmode4P2_13modeEnum::Disabled,
            3 => PinconnectPinmode4P2_13modeEnum::PullDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PinconnectPinmode4P2_13modeEnum::PullUp
    }
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PinconnectPinmode4P2_13modeEnum::Repeater
    }
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PinconnectPinmode4P2_13modeEnum::Disabled
    }
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PinconnectPinmode4P2_13modeEnum::PullDown
    }
}
#[doc = "Field `P2_13MODE` writer - Port 2 pin 13 control."]
pub type P2_13modeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, PinconnectPinmode4P2_13modeEnum, crate::Safe>;
impl<'a, REG> P2_13modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_13modeEnum::PullUp)
    }
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_13modeEnum::Repeater)
    }
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_13modeEnum::Disabled)
    }
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PinconnectPinmode4P2_13modeEnum::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    pub fn p2_00mode(&self) -> P2_00modeR {
        P2_00modeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    pub fn p2_01mode(&self) -> P2_01modeR {
        P2_01modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    pub fn p2_02mode(&self) -> P2_02modeR {
        P2_02modeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    pub fn p2_03mode(&self) -> P2_03modeR {
        P2_03modeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    pub fn p2_04mode(&self) -> P2_04modeR {
        P2_04modeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    pub fn p2_05mode(&self) -> P2_05modeR {
        P2_05modeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    pub fn p2_06mode(&self) -> P2_06modeR {
        P2_06modeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    pub fn p2_07mode(&self) -> P2_07modeR {
        P2_07modeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    pub fn p2_08mode(&self) -> P2_08modeR {
        P2_08modeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    pub fn p2_09mode(&self) -> P2_09modeR {
        P2_09modeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    pub fn p2_10mode(&self) -> P2_10modeR {
        P2_10modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    pub fn p2_11mode(&self) -> P2_11modeR {
        P2_11modeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    pub fn p2_12mode(&self) -> P2_12modeR {
        P2_12modeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    pub fn p2_13mode(&self) -> P2_13modeR {
        P2_13modeR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    pub fn p2_00mode(&mut self) -> P2_00modeW<'_, Pinmode4Spec> {
        P2_00modeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    pub fn p2_01mode(&mut self) -> P2_01modeW<'_, Pinmode4Spec> {
        P2_01modeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    pub fn p2_02mode(&mut self) -> P2_02modeW<'_, Pinmode4Spec> {
        P2_02modeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    pub fn p2_03mode(&mut self) -> P2_03modeW<'_, Pinmode4Spec> {
        P2_03modeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    pub fn p2_04mode(&mut self) -> P2_04modeW<'_, Pinmode4Spec> {
        P2_04modeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    pub fn p2_05mode(&mut self) -> P2_05modeW<'_, Pinmode4Spec> {
        P2_05modeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    pub fn p2_06mode(&mut self) -> P2_06modeW<'_, Pinmode4Spec> {
        P2_06modeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    pub fn p2_07mode(&mut self) -> P2_07modeW<'_, Pinmode4Spec> {
        P2_07modeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    pub fn p2_08mode(&mut self) -> P2_08modeW<'_, Pinmode4Spec> {
        P2_08modeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    pub fn p2_09mode(&mut self) -> P2_09modeW<'_, Pinmode4Spec> {
        P2_09modeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    pub fn p2_10mode(&mut self) -> P2_10modeW<'_, Pinmode4Spec> {
        P2_10modeW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    pub fn p2_11mode(&mut self) -> P2_11modeW<'_, Pinmode4Spec> {
        P2_11modeW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    pub fn p2_12mode(&mut self) -> P2_12modeW<'_, Pinmode4Spec> {
        P2_12modeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    pub fn p2_13mode(&mut self) -> P2_13modeW<'_, Pinmode4Spec> {
        P2_13modeW::new(self, 26)
    }
}
#[doc = "Pin mode select register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmode4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmode4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pinmode4Spec;
impl crate::RegisterSpec for Pinmode4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode4::R`](R) reader structure"]
impl crate::Readable for Pinmode4Spec {}
#[doc = "`write(|w| ..)` method takes [`pinmode4::W`](W) writer structure"]
impl crate::Writable for Pinmode4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMODE4 to value 0"]
impl crate::Resettable for Pinmode4Spec {}
