#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Limit interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfIlim0FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfIlim0FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfIlim0FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM0_F` reader - Limit interrupt flag for channel 0."]
pub type Ilim0FR = crate::BitReader<McpwmIntfIlim0FEnum>;
impl Ilim0FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfIlim0FEnum {
        match self.bits {
            false => McpwmIntfIlim0FEnum::ThisInterruptSourc,
            true => McpwmIntfIlim0FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfIlim0FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfIlim0FEnum::IfTheCorresponding
    }
}
#[doc = "Match interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfImat0FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfImat0FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfImat0FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT0_F` reader - Match interrupt flag for channel 0."]
pub type Imat0FR = crate::BitReader<McpwmIntfImat0FEnum>;
impl Imat0FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfImat0FEnum {
        match self.bits {
            false => McpwmIntfImat0FEnum::ThisInterruptSourc,
            true => McpwmIntfImat0FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfImat0FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfImat0FEnum::IfTheCorresponding
    }
}
#[doc = "Capture interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfIcap0FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfIcap0FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfIcap0FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP0_F` reader - Capture interrupt flag for channel 0."]
pub type Icap0FR = crate::BitReader<McpwmIntfIcap0FEnum>;
impl Icap0FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfIcap0FEnum {
        match self.bits {
            false => McpwmIntfIcap0FEnum::ThisInterruptSourc,
            true => McpwmIntfIcap0FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfIcap0FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfIcap0FEnum::IfTheCorresponding
    }
}
#[doc = "Limit interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfIlim1FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfIlim1FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfIlim1FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM1_F` reader - Limit interrupt flag for channel 1."]
pub type Ilim1FR = crate::BitReader<McpwmIntfIlim1FEnum>;
impl Ilim1FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfIlim1FEnum {
        match self.bits {
            false => McpwmIntfIlim1FEnum::ThisInterruptSourc,
            true => McpwmIntfIlim1FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfIlim1FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfIlim1FEnum::IfTheCorresponding
    }
}
#[doc = "Match interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfImat1FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfImat1FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfImat1FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT1_F` reader - Match interrupt flag for channel 1."]
pub type Imat1FR = crate::BitReader<McpwmIntfImat1FEnum>;
impl Imat1FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfImat1FEnum {
        match self.bits {
            false => McpwmIntfImat1FEnum::ThisInterruptSourc,
            true => McpwmIntfImat1FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfImat1FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfImat1FEnum::IfTheCorresponding
    }
}
#[doc = "Capture interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfIcap1FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfIcap1FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfIcap1FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP1_F` reader - Capture interrupt flag for channel 1."]
pub type Icap1FR = crate::BitReader<McpwmIntfIcap1FEnum>;
impl Icap1FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfIcap1FEnum {
        match self.bits {
            false => McpwmIntfIcap1FEnum::ThisInterruptSourc,
            true => McpwmIntfIcap1FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfIcap1FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfIcap1FEnum::IfTheCorresponding
    }
}
#[doc = "Limit interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfIlim2FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfIlim2FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfIlim2FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM2_F` reader - Limit interrupt flag for channel 2."]
pub type Ilim2FR = crate::BitReader<McpwmIntfIlim2FEnum>;
impl Ilim2FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfIlim2FEnum {
        match self.bits {
            false => McpwmIntfIlim2FEnum::ThisInterruptSourc,
            true => McpwmIntfIlim2FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfIlim2FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfIlim2FEnum::IfTheCorresponding
    }
}
#[doc = "Match interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfImat2FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfImat2FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfImat2FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT2_F` reader - Match interrupt flag for channel 2."]
pub type Imat2FR = crate::BitReader<McpwmIntfImat2FEnum>;
impl Imat2FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfImat2FEnum {
        match self.bits {
            false => McpwmIntfImat2FEnum::ThisInterruptSourc,
            true => McpwmIntfImat2FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfImat2FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfImat2FEnum::IfTheCorresponding
    }
}
#[doc = "Capture interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfIcap2FEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfIcap2FEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfIcap2FEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP2_F` reader - Capture interrupt flag for channel 2."]
pub type Icap2FR = crate::BitReader<McpwmIntfIcap2FEnum>;
impl Icap2FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfIcap2FEnum {
        match self.bits {
            false => McpwmIntfIcap2FEnum::ThisInterruptSourc,
            true => McpwmIntfIcap2FEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfIcap2FEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfIcap2FEnum::IfTheCorresponding
    }
}
#[doc = "Fast abort interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmIntfAbortFEnum {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<McpwmIntfAbortFEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmIntfAbortFEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT_F` reader - Fast abort interrupt flag."]
pub type AbortFR = crate::BitReader<McpwmIntfAbortFEnum>;
impl AbortFR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmIntfAbortFEnum {
        match self.bits {
            false => McpwmIntfAbortFEnum::ThisInterruptSourc,
            true => McpwmIntfAbortFEnum::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == McpwmIntfAbortFEnum::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == McpwmIntfAbortFEnum::IfTheCorresponding
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt flag for channel 0."]
    #[inline(always)]
    pub fn ilim0_f(&self) -> Ilim0FR {
        Ilim0FR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match interrupt flag for channel 0."]
    #[inline(always)]
    pub fn imat0_f(&self) -> Imat0FR {
        Imat0FR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt flag for channel 0."]
    #[inline(always)]
    pub fn icap0_f(&self) -> Icap0FR {
        Icap0FR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt flag for channel 1."]
    #[inline(always)]
    pub fn ilim1_f(&self) -> Ilim1FR {
        Ilim1FR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match interrupt flag for channel 1."]
    #[inline(always)]
    pub fn imat1_f(&self) -> Imat1FR {
        Imat1FR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt flag for channel 1."]
    #[inline(always)]
    pub fn icap1_f(&self) -> Icap1FR {
        Icap1FR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt flag for channel 2."]
    #[inline(always)]
    pub fn ilim2_f(&self) -> Ilim2FR {
        Ilim2FR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match interrupt flag for channel 2."]
    #[inline(always)]
    pub fn imat2_f(&self) -> Imat2FR {
        Imat2FR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt flag for channel 2."]
    #[inline(always)]
    pub fn icap2_f(&self) -> Icap2FR {
        Icap2FR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt flag."]
    #[inline(always)]
    pub fn abort_f(&self) -> AbortFR {
        AbortFR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt flags read address\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {}
