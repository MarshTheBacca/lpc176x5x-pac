#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Limit interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenIlim0Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenIlim0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenIlim0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM0` reader - Limit interrupt for channel 0."]
pub type Ilim0R = crate::BitReader<McpwmIntenIlim0Enum>;
impl Ilim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenIlim0Enum {
        match self.bits {
            false => McpwmIntenIlim0Enum::InterruptDisabled_,
            true => McpwmIntenIlim0Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenIlim0Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenIlim0Enum::InterruptEnabled_
    }
}
#[doc = "Match interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenImat0Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenImat0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenImat0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT0` reader - Match interrupt for channel 0."]
pub type Imat0R = crate::BitReader<McpwmIntenImat0Enum>;
impl Imat0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenImat0Enum {
        match self.bits {
            false => McpwmIntenImat0Enum::InterruptDisabled_,
            true => McpwmIntenImat0Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenImat0Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenImat0Enum::InterruptEnabled_
    }
}
#[doc = "Capture interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenIcap0Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenIcap0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenIcap0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP0` reader - Capture interrupt for channel 0."]
pub type Icap0R = crate::BitReader<McpwmIntenIcap0Enum>;
impl Icap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenIcap0Enum {
        match self.bits {
            false => McpwmIntenIcap0Enum::InterruptDisabled_,
            true => McpwmIntenIcap0Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenIcap0Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenIcap0Enum::InterruptEnabled_
    }
}
#[doc = "Limit interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenIlim1Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenIlim1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenIlim1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM1` reader - Limit interrupt for channel 1."]
pub type Ilim1R = crate::BitReader<McpwmIntenIlim1Enum>;
impl Ilim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenIlim1Enum {
        match self.bits {
            false => McpwmIntenIlim1Enum::InterruptDisabled_,
            true => McpwmIntenIlim1Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenIlim1Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenIlim1Enum::InterruptEnabled_
    }
}
#[doc = "Match interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenImat1Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenImat1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenImat1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT1` reader - Match interrupt for channel 1."]
pub type Imat1R = crate::BitReader<McpwmIntenImat1Enum>;
impl Imat1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenImat1Enum {
        match self.bits {
            false => McpwmIntenImat1Enum::InterruptDisabled_,
            true => McpwmIntenImat1Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenImat1Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenImat1Enum::InterruptEnabled_
    }
}
#[doc = "Capture interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenIcap1Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenIcap1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenIcap1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP1` reader - Capture interrupt for channel 1."]
pub type Icap1R = crate::BitReader<McpwmIntenIcap1Enum>;
impl Icap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenIcap1Enum {
        match self.bits {
            false => McpwmIntenIcap1Enum::InterruptDisabled_,
            true => McpwmIntenIcap1Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenIcap1Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenIcap1Enum::InterruptEnabled_
    }
}
#[doc = "Limit interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenIlim2Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenIlim2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenIlim2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM2` reader - Limit interrupt for channel 2."]
pub type Ilim2R = crate::BitReader<McpwmIntenIlim2Enum>;
impl Ilim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenIlim2Enum {
        match self.bits {
            false => McpwmIntenIlim2Enum::InterruptDisabled_,
            true => McpwmIntenIlim2Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenIlim2Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenIlim2Enum::InterruptEnabled_
    }
}
#[doc = "Match interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenImat2Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenImat2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenImat2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT2` reader - Match interrupt for channel 2."]
pub type Imat2R = crate::BitReader<McpwmIntenImat2Enum>;
impl Imat2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenImat2Enum {
        match self.bits {
            false => McpwmIntenImat2Enum::InterruptDisabled_,
            true => McpwmIntenImat2Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenImat2Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenImat2Enum::InterruptEnabled_
    }
}
#[doc = "Capture interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenIcap2Enum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenIcap2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenIcap2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP2` reader - Capture interrupt for channel 2."]
pub type Icap2R = crate::BitReader<McpwmIntenIcap2Enum>;
impl Icap2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenIcap2Enum {
        match self.bits {
            false => McpwmIntenIcap2Enum::InterruptDisabled_,
            true => McpwmIntenIcap2Enum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenIcap2Enum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenIcap2Enum::InterruptEnabled_
    }
}
#[doc = "Fast abort interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntenAbortEnum {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<McpwmIntenAbortEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntenAbortEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT` reader - Fast abort interrupt."]
pub type AbortR = crate::BitReader<McpwmIntenAbortEnum>;
impl AbortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntenAbortEnum {
        match self.bits {
            false => McpwmIntenAbortEnum::InterruptDisabled_,
            true => McpwmIntenAbortEnum::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == McpwmIntenAbortEnum::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == McpwmIntenAbortEnum::InterruptEnabled_
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt for channel 0."]
    #[inline(always)]
    pub fn ilim0(&self) -> Ilim0R {
        Ilim0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match interrupt for channel 0."]
    #[inline(always)]
    pub fn imat0(&self) -> Imat0R {
        Imat0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt for channel 0."]
    #[inline(always)]
    pub fn icap0(&self) -> Icap0R {
        Icap0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt for channel 1."]
    #[inline(always)]
    pub fn ilim1(&self) -> Ilim1R {
        Ilim1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match interrupt for channel 1."]
    #[inline(always)]
    pub fn imat1(&self) -> Imat1R {
        Imat1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt for channel 1."]
    #[inline(always)]
    pub fn icap1(&self) -> Icap1R {
        Icap1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt for channel 2."]
    #[inline(always)]
    pub fn ilim2(&self) -> Ilim2R {
        Ilim2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match interrupt for channel 2."]
    #[inline(always)]
    pub fn imat2(&self) -> Imat2R {
        Imat2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt for channel 2."]
    #[inline(always)]
    pub fn icap2(&self) -> Icap2R {
        Icap2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt Enable read address\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
