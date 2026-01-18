#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Interrupt on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr0iEnum {
    #[doc = "1: Interrupt is generated when MR0 matches the value in the TC."]
    InterruptIsGenerat = 1,
    #[doc = "0: Interrupt is disabled"]
    InterruptIsDisable = 0,
}
impl From<Timer0McrMr0iEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr0iEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0I` reader - Interrupt on MR0"]
pub type Mr0iR = crate::BitReader<Timer0McrMr0iEnum>;
impl Mr0iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr0iEnum {
        match self.bits {
            true => Timer0McrMr0iEnum::InterruptIsGenerat,
            false => Timer0McrMr0iEnum::InterruptIsDisable,
        }
    }
    #[doc = "Interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == Timer0McrMr0iEnum::InterruptIsGenerat
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == Timer0McrMr0iEnum::InterruptIsDisable
    }
}
#[doc = "Field `MR0I` writer - Interrupt on MR0"]
pub type Mr0iW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr0iEnum>;
impl<'a, REG> Mr0iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr0iEnum::InterruptIsGenerat)
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr0iEnum::InterruptIsDisable)
    }
}
#[doc = "Reset on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr0rEnum {
    #[doc = "1: TC will be reset if MR0 matches it."]
    TcWillBeResetIf_ = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Timer0McrMr0rEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr0rEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0R` reader - Reset on MR0"]
pub type Mr0rR = crate::BitReader<Timer0McrMr0rEnum>;
impl Mr0rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr0rEnum {
        match self.bits {
            true => Timer0McrMr0rEnum::TcWillBeResetIf_,
            false => Timer0McrMr0rEnum::FeatureDisabled_,
        }
    }
    #[doc = "TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == Timer0McrMr0rEnum::TcWillBeResetIf_
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Timer0McrMr0rEnum::FeatureDisabled_
    }
}
#[doc = "Field `MR0R` writer - Reset on MR0"]
pub type Mr0rW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr0rEnum>;
impl<'a, REG> Mr0rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr0rEnum::TcWillBeResetIf_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr0rEnum::FeatureDisabled_)
    }
}
#[doc = "Stop on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr0sEnum {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    TcAndPcWillBeSt = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Timer0McrMr0sEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr0sEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0S` reader - Stop on MR0"]
pub type Mr0sR = crate::BitReader<Timer0McrMr0sEnum>;
impl Mr0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr0sEnum {
        match self.bits {
            true => Timer0McrMr0sEnum::TcAndPcWillBeSt,
            false => Timer0McrMr0sEnum::FeatureDisabled_,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == Timer0McrMr0sEnum::TcAndPcWillBeSt
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Timer0McrMr0sEnum::FeatureDisabled_
    }
}
#[doc = "Field `MR0S` writer - Stop on MR0"]
pub type Mr0sW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr0sEnum>;
impl<'a, REG> Mr0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr0sEnum::TcAndPcWillBeSt)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr0sEnum::FeatureDisabled_)
    }
}
#[doc = "Interrupt on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr1iEnum {
    #[doc = "1: Interrupt is generated when MR1 matches the value in the TC."]
    InterruptIsGenerat = 1,
    #[doc = "0: Interrupt is disabled."]
    InterruptIsDisable = 0,
}
impl From<Timer0McrMr1iEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr1iEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1I` reader - Interrupt on MR1"]
pub type Mr1iR = crate::BitReader<Timer0McrMr1iEnum>;
impl Mr1iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr1iEnum {
        match self.bits {
            true => Timer0McrMr1iEnum::InterruptIsGenerat,
            false => Timer0McrMr1iEnum::InterruptIsDisable,
        }
    }
    #[doc = "Interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == Timer0McrMr1iEnum::InterruptIsGenerat
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == Timer0McrMr1iEnum::InterruptIsDisable
    }
}
#[doc = "Field `MR1I` writer - Interrupt on MR1"]
pub type Mr1iW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr1iEnum>;
impl<'a, REG> Mr1iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr1iEnum::InterruptIsGenerat)
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr1iEnum::InterruptIsDisable)
    }
}
#[doc = "Reset on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr1rEnum {
    #[doc = "1: TC will be reset if MR1 matches it."]
    TcWillBeResetIf_ = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Timer0McrMr1rEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr1rEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1R` reader - Reset on MR1"]
pub type Mr1rR = crate::BitReader<Timer0McrMr1rEnum>;
impl Mr1rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr1rEnum {
        match self.bits {
            true => Timer0McrMr1rEnum::TcWillBeResetIf_,
            false => Timer0McrMr1rEnum::FeatureDisabled_,
        }
    }
    #[doc = "TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == Timer0McrMr1rEnum::TcWillBeResetIf_
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Timer0McrMr1rEnum::FeatureDisabled_
    }
}
#[doc = "Field `MR1R` writer - Reset on MR1"]
pub type Mr1rW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr1rEnum>;
impl<'a, REG> Mr1rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr1rEnum::TcWillBeResetIf_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr1rEnum::FeatureDisabled_)
    }
}
#[doc = "Stop on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr1sEnum {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    TcAndPcWillBeSt = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Timer0McrMr1sEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr1sEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1S` reader - Stop on MR1"]
pub type Mr1sR = crate::BitReader<Timer0McrMr1sEnum>;
impl Mr1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr1sEnum {
        match self.bits {
            true => Timer0McrMr1sEnum::TcAndPcWillBeSt,
            false => Timer0McrMr1sEnum::FeatureDisabled_,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == Timer0McrMr1sEnum::TcAndPcWillBeSt
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Timer0McrMr1sEnum::FeatureDisabled_
    }
}
#[doc = "Field `MR1S` writer - Stop on MR1"]
pub type Mr1sW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr1sEnum>;
impl<'a, REG> Mr1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr1sEnum::TcAndPcWillBeSt)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr1sEnum::FeatureDisabled_)
    }
}
#[doc = "Interrupt on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr2iEnum {
    #[doc = "1: Interrupt is generated when MR2 matches the value in the TC."]
    InterruptIsGenerat = 1,
    #[doc = "0: Interrupt is disabled"]
    InterruptIsDisable = 0,
}
impl From<Timer0McrMr2iEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr2iEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2I` reader - Interrupt on MR2"]
pub type Mr2iR = crate::BitReader<Timer0McrMr2iEnum>;
impl Mr2iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr2iEnum {
        match self.bits {
            true => Timer0McrMr2iEnum::InterruptIsGenerat,
            false => Timer0McrMr2iEnum::InterruptIsDisable,
        }
    }
    #[doc = "Interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == Timer0McrMr2iEnum::InterruptIsGenerat
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == Timer0McrMr2iEnum::InterruptIsDisable
    }
}
#[doc = "Field `MR2I` writer - Interrupt on MR2"]
pub type Mr2iW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr2iEnum>;
impl<'a, REG> Mr2iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr2iEnum::InterruptIsGenerat)
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr2iEnum::InterruptIsDisable)
    }
}
#[doc = "Reset on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr2rEnum {
    #[doc = "1: TC will be reset if MR2 matches it."]
    TcWillBeResetIf_ = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Timer0McrMr2rEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr2rEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2R` reader - Reset on MR2"]
pub type Mr2rR = crate::BitReader<Timer0McrMr2rEnum>;
impl Mr2rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr2rEnum {
        match self.bits {
            true => Timer0McrMr2rEnum::TcWillBeResetIf_,
            false => Timer0McrMr2rEnum::FeatureDisabled_,
        }
    }
    #[doc = "TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == Timer0McrMr2rEnum::TcWillBeResetIf_
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Timer0McrMr2rEnum::FeatureDisabled_
    }
}
#[doc = "Field `MR2R` writer - Reset on MR2"]
pub type Mr2rW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr2rEnum>;
impl<'a, REG> Mr2rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr2rEnum::TcWillBeResetIf_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr2rEnum::FeatureDisabled_)
    }
}
#[doc = "Stop on MR2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr2sEnum {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC"]
    TcAndPcWillBeSt = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Timer0McrMr2sEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr2sEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2S` reader - Stop on MR2."]
pub type Mr2sR = crate::BitReader<Timer0McrMr2sEnum>;
impl Mr2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr2sEnum {
        match self.bits {
            true => Timer0McrMr2sEnum::TcAndPcWillBeSt,
            false => Timer0McrMr2sEnum::FeatureDisabled_,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC"]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == Timer0McrMr2sEnum::TcAndPcWillBeSt
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Timer0McrMr2sEnum::FeatureDisabled_
    }
}
#[doc = "Field `MR2S` writer - Stop on MR2."]
pub type Mr2sW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr2sEnum>;
impl<'a, REG> Mr2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC"]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr2sEnum::TcAndPcWillBeSt)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr2sEnum::FeatureDisabled_)
    }
}
#[doc = "Interrupt on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr3iEnum {
    #[doc = "1: Interrupt is generated when MR3 matches the value in the TC."]
    InterruptIsGenerat = 1,
    #[doc = "0: This interrupt is disabled"]
    ThisInterruptIsDi = 0,
}
impl From<Timer0McrMr3iEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr3iEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3I` reader - Interrupt on MR3"]
pub type Mr3iR = crate::BitReader<Timer0McrMr3iEnum>;
impl Mr3iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr3iEnum {
        match self.bits {
            true => Timer0McrMr3iEnum::InterruptIsGenerat,
            false => Timer0McrMr3iEnum::ThisInterruptIsDi,
        }
    }
    #[doc = "Interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == Timer0McrMr3iEnum::InterruptIsGenerat
    }
    #[doc = "This interrupt is disabled"]
    #[inline(always)]
    pub fn is_this_interrupt_is_di(&self) -> bool {
        *self == Timer0McrMr3iEnum::ThisInterruptIsDi
    }
}
#[doc = "Field `MR3I` writer - Interrupt on MR3"]
pub type Mr3iW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr3iEnum>;
impl<'a, REG> Mr3iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr3iEnum::InterruptIsGenerat)
    }
    #[doc = "This interrupt is disabled"]
    #[inline(always)]
    pub fn this_interrupt_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr3iEnum::ThisInterruptIsDi)
    }
}
#[doc = "Reset on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr3rEnum {
    #[doc = "1: TC will be reset if MR3 matches it."]
    TcWillBeResetIf_ = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Timer0McrMr3rEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr3rEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3R` reader - Reset on MR3"]
pub type Mr3rR = crate::BitReader<Timer0McrMr3rEnum>;
impl Mr3rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr3rEnum {
        match self.bits {
            true => Timer0McrMr3rEnum::TcWillBeResetIf_,
            false => Timer0McrMr3rEnum::FeatureDisabled_,
        }
    }
    #[doc = "TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == Timer0McrMr3rEnum::TcWillBeResetIf_
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Timer0McrMr3rEnum::FeatureDisabled_
    }
}
#[doc = "Field `MR3R` writer - Reset on MR3"]
pub type Mr3rW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr3rEnum>;
impl<'a, REG> Mr3rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr3rEnum::TcWillBeResetIf_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr3rEnum::FeatureDisabled_)
    }
}
#[doc = "Stop on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer0McrMr3sEnum {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    TcAndPcWillBeSt = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Timer0McrMr3sEnum> for bool {
    #[inline(always)]
    fn from(variant: Timer0McrMr3sEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3S` reader - Stop on MR3"]
pub type Mr3sR = crate::BitReader<Timer0McrMr3sEnum>;
impl Mr3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer0McrMr3sEnum {
        match self.bits {
            true => Timer0McrMr3sEnum::TcAndPcWillBeSt,
            false => Timer0McrMr3sEnum::FeatureDisabled_,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == Timer0McrMr3sEnum::TcAndPcWillBeSt
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Timer0McrMr3sEnum::FeatureDisabled_
    }
}
#[doc = "Field `MR3S` writer - Stop on MR3"]
pub type Mr3sW<'a, REG> = crate::BitWriter<'a, REG, Timer0McrMr3sEnum>;
impl<'a, REG> Mr3sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr3sEnum::TcAndPcWillBeSt)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Timer0McrMr3sEnum::FeatureDisabled_)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    pub fn mr0i(&self) -> Mr0iR {
        Mr0iR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    pub fn mr0r(&self) -> Mr0rR {
        Mr0rR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    pub fn mr0s(&self) -> Mr0sR {
        Mr0sR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    pub fn mr1i(&self) -> Mr1iR {
        Mr1iR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    pub fn mr1r(&self) -> Mr1rR {
        Mr1rR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    pub fn mr1s(&self) -> Mr1sR {
        Mr1sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    pub fn mr2i(&self) -> Mr2iR {
        Mr2iR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    pub fn mr2r(&self) -> Mr2rR {
        Mr2rR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline(always)]
    pub fn mr2s(&self) -> Mr2sR {
        Mr2sR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    pub fn mr3i(&self) -> Mr3iR {
        Mr3iR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    pub fn mr3r(&self) -> Mr3rR {
        Mr3rR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    pub fn mr3s(&self) -> Mr3sR {
        Mr3sR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    pub fn mr0i(&mut self) -> Mr0iW<'_, McrSpec> {
        Mr0iW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    pub fn mr0r(&mut self) -> Mr0rW<'_, McrSpec> {
        Mr0rW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    pub fn mr0s(&mut self) -> Mr0sW<'_, McrSpec> {
        Mr0sW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    pub fn mr1i(&mut self) -> Mr1iW<'_, McrSpec> {
        Mr1iW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    pub fn mr1r(&mut self) -> Mr1rW<'_, McrSpec> {
        Mr1rW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    pub fn mr1s(&mut self) -> Mr1sW<'_, McrSpec> {
        Mr1sW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    pub fn mr2i(&mut self) -> Mr2iW<'_, McrSpec> {
        Mr2iW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    pub fn mr2r(&mut self) -> Mr2rW<'_, McrSpec> {
        Mr2rW::new(self, 7)
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline(always)]
    pub fn mr2s(&mut self) -> Mr2sW<'_, McrSpec> {
        Mr2sW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    pub fn mr3i(&mut self) -> Mr3iW<'_, McrSpec> {
        Mr3iW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    pub fn mr3r(&mut self) -> Mr3rW<'_, McrSpec> {
        Mr3rW::new(self, 10)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    pub fn mr3s(&mut self) -> Mr3sW<'_, McrSpec> {
        Mr3sW::new(self, 11)
    }
}
#[doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {}
