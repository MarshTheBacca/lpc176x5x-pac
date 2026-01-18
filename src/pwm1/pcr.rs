#[doc = "Register `PCR` reader"]
pub type R = crate::R<PcrSpec>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PcrSpec>;
#[doc = "PWM\\[2\\] output single/double edge mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmsel2Enum {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwm1PcrPwmsel2Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmsel2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL2` reader - PWM\\[2\\] output single/double edge mode control."]
pub type Pwmsel2R = crate::BitReader<Pwm1PcrPwmsel2Enum>;
impl Pwmsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmsel2Enum {
        match self.bits {
            false => Pwm1PcrPwmsel2Enum::SingleEdgeControll,
            true => Pwm1PcrPwmsel2Enum::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel2Enum::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel2Enum::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL2` writer - PWM\\[2\\] output single/double edge mode control."]
pub type Pwmsel2W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmsel2Enum>;
impl<'a, REG> Pwmsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel2Enum::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel2Enum::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[3\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmsel3Enum {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwm1PcrPwmsel3Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmsel3Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL3` reader - PWM\\[3\\] output edge control."]
pub type Pwmsel3R = crate::BitReader<Pwm1PcrPwmsel3Enum>;
impl Pwmsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmsel3Enum {
        match self.bits {
            false => Pwm1PcrPwmsel3Enum::SingleEdgeControll,
            true => Pwm1PcrPwmsel3Enum::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel3Enum::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel3Enum::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL3` writer - PWM\\[3\\] output edge control."]
pub type Pwmsel3W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmsel3Enum>;
impl<'a, REG> Pwmsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel3Enum::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel3Enum::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[4\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmsel4Enum {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwm1PcrPwmsel4Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmsel4Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL4` reader - PWM\\[4\\] output edge control."]
pub type Pwmsel4R = crate::BitReader<Pwm1PcrPwmsel4Enum>;
impl Pwmsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmsel4Enum {
        match self.bits {
            false => Pwm1PcrPwmsel4Enum::SingleEdgeControll,
            true => Pwm1PcrPwmsel4Enum::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel4Enum::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel4Enum::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL4` writer - PWM\\[4\\] output edge control."]
pub type Pwmsel4W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmsel4Enum>;
impl<'a, REG> Pwmsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel4Enum::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel4Enum::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[5\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmsel5Enum {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwm1PcrPwmsel5Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmsel5Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL5` reader - PWM\\[5\\] output edge control."]
pub type Pwmsel5R = crate::BitReader<Pwm1PcrPwmsel5Enum>;
impl Pwmsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmsel5Enum {
        match self.bits {
            false => Pwm1PcrPwmsel5Enum::SingleEdgeControll,
            true => Pwm1PcrPwmsel5Enum::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel5Enum::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel5Enum::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL5` writer - PWM\\[5\\] output edge control."]
pub type Pwmsel5W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmsel5Enum>;
impl<'a, REG> Pwmsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel5Enum::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel5Enum::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[6\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmsel6Enum {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwm1PcrPwmsel6Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmsel6Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL6` reader - PWM\\[6\\] output edge control."]
pub type Pwmsel6R = crate::BitReader<Pwm1PcrPwmsel6Enum>;
impl Pwmsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmsel6Enum {
        match self.bits {
            false => Pwm1PcrPwmsel6Enum::SingleEdgeControll,
            true => Pwm1PcrPwmsel6Enum::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel6Enum::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwm1PcrPwmsel6Enum::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL6` writer - PWM\\[6\\] output edge control."]
pub type Pwmsel6W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmsel6Enum>;
impl<'a, REG> Pwmsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel6Enum::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmsel6Enum::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[1\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmena1Enum {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwm1PcrPwmena1Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmena1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA1` reader - PWM\\[1\\] output enable control."]
pub type Pwmena1R = crate::BitReader<Pwm1PcrPwmena1Enum>;
impl Pwmena1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmena1Enum {
        match self.bits {
            false => Pwm1PcrPwmena1Enum::ThePwmOutputIsDi,
            true => Pwm1PcrPwmena1Enum::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwm1PcrPwmena1Enum::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwm1PcrPwmena1Enum::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA1` writer - PWM\\[1\\] output enable control."]
pub type Pwmena1W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmena1Enum>;
impl<'a, REG> Pwmena1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena1Enum::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena1Enum::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[2\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmena2Enum {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwm1PcrPwmena2Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmena2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA2` reader - PWM\\[2\\] output enable control."]
pub type Pwmena2R = crate::BitReader<Pwm1PcrPwmena2Enum>;
impl Pwmena2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmena2Enum {
        match self.bits {
            false => Pwm1PcrPwmena2Enum::ThePwmOutputIsDi,
            true => Pwm1PcrPwmena2Enum::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwm1PcrPwmena2Enum::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwm1PcrPwmena2Enum::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA2` writer - PWM\\[2\\] output enable control."]
pub type Pwmena2W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmena2Enum>;
impl<'a, REG> Pwmena2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena2Enum::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena2Enum::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[3\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmena3Enum {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwm1PcrPwmena3Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmena3Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA3` reader - PWM\\[3\\] output enable control."]
pub type Pwmena3R = crate::BitReader<Pwm1PcrPwmena3Enum>;
impl Pwmena3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmena3Enum {
        match self.bits {
            false => Pwm1PcrPwmena3Enum::ThePwmOutputIsDi,
            true => Pwm1PcrPwmena3Enum::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwm1PcrPwmena3Enum::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwm1PcrPwmena3Enum::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA3` writer - PWM\\[3\\] output enable control."]
pub type Pwmena3W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmena3Enum>;
impl<'a, REG> Pwmena3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena3Enum::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena3Enum::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[4\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmena4Enum {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwm1PcrPwmena4Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmena4Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA4` reader - PWM\\[4\\] output enable control."]
pub type Pwmena4R = crate::BitReader<Pwm1PcrPwmena4Enum>;
impl Pwmena4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmena4Enum {
        match self.bits {
            false => Pwm1PcrPwmena4Enum::ThePwmOutputIsDi,
            true => Pwm1PcrPwmena4Enum::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwm1PcrPwmena4Enum::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwm1PcrPwmena4Enum::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA4` writer - PWM\\[4\\] output enable control."]
pub type Pwmena4W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmena4Enum>;
impl<'a, REG> Pwmena4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena4Enum::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena4Enum::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[5\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmena5Enum {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwm1PcrPwmena5Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmena5Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA5` reader - PWM\\[5\\] output enable control."]
pub type Pwmena5R = crate::BitReader<Pwm1PcrPwmena5Enum>;
impl Pwmena5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmena5Enum {
        match self.bits {
            false => Pwm1PcrPwmena5Enum::ThePwmOutputIsDi,
            true => Pwm1PcrPwmena5Enum::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwm1PcrPwmena5Enum::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwm1PcrPwmena5Enum::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA5` writer - PWM\\[5\\] output enable control."]
pub type Pwmena5W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmena5Enum>;
impl<'a, REG> Pwmena5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena5Enum::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena5Enum::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[6\\] output enable control. See PWMENA1 for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwm1PcrPwmena6Enum {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwm1PcrPwmena6Enum> for bool {
    #[inline(always)]
    fn from(variant: Pwm1PcrPwmena6Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA6` reader - PWM\\[6\\] output enable control. See PWMENA1 for details."]
pub type Pwmena6R = crate::BitReader<Pwm1PcrPwmena6Enum>;
impl Pwmena6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwm1PcrPwmena6Enum {
        match self.bits {
            false => Pwm1PcrPwmena6Enum::ThePwmOutputIsDi,
            true => Pwm1PcrPwmena6Enum::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwm1PcrPwmena6Enum::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwm1PcrPwmena6Enum::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA6` writer - PWM\\[6\\] output enable control. See PWMENA1 for details."]
pub type Pwmena6W<'a, REG> = crate::BitWriter<'a, REG, Pwm1PcrPwmena6Enum>;
impl<'a, REG> Pwmena6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena6Enum::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwm1PcrPwmena6Enum::ThePwmOutputIsEn)
    }
}
impl R {
    #[doc = "Bit 2 - PWM\\[2\\] output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&self) -> Pwmsel2R {
        Pwmsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM\\[3\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&self) -> Pwmsel3R {
        Pwmsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM\\[4\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&self) -> Pwmsel4R {
        Pwmsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM\\[5\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&self) -> Pwmsel5R {
        Pwmsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM\\[6\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&self) -> Pwmsel6R {
        Pwmsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM\\[1\\] output enable control."]
    #[inline(always)]
    pub fn pwmena1(&self) -> Pwmena1R {
        Pwmena1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM\\[2\\] output enable control."]
    #[inline(always)]
    pub fn pwmena2(&self) -> Pwmena2R {
        Pwmena2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM\\[3\\] output enable control."]
    #[inline(always)]
    pub fn pwmena3(&self) -> Pwmena3R {
        Pwmena3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM\\[4\\] output enable control."]
    #[inline(always)]
    pub fn pwmena4(&self) -> Pwmena4R {
        Pwmena4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PWM\\[5\\] output enable control."]
    #[inline(always)]
    pub fn pwmena5(&self) -> Pwmena5R {
        Pwmena5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWM\\[6\\] output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&self) -> Pwmena6R {
        Pwmena6R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PWM\\[2\\] output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&mut self) -> Pwmsel2W<'_, PcrSpec> {
        Pwmsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - PWM\\[3\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&mut self) -> Pwmsel3W<'_, PcrSpec> {
        Pwmsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - PWM\\[4\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&mut self) -> Pwmsel4W<'_, PcrSpec> {
        Pwmsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - PWM\\[5\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&mut self) -> Pwmsel5W<'_, PcrSpec> {
        Pwmsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - PWM\\[6\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&mut self) -> Pwmsel6W<'_, PcrSpec> {
        Pwmsel6W::new(self, 6)
    }
    #[doc = "Bit 9 - PWM\\[1\\] output enable control."]
    #[inline(always)]
    pub fn pwmena1(&mut self) -> Pwmena1W<'_, PcrSpec> {
        Pwmena1W::new(self, 9)
    }
    #[doc = "Bit 10 - PWM\\[2\\] output enable control."]
    #[inline(always)]
    pub fn pwmena2(&mut self) -> Pwmena2W<'_, PcrSpec> {
        Pwmena2W::new(self, 10)
    }
    #[doc = "Bit 11 - PWM\\[3\\] output enable control."]
    #[inline(always)]
    pub fn pwmena3(&mut self) -> Pwmena3W<'_, PcrSpec> {
        Pwmena3W::new(self, 11)
    }
    #[doc = "Bit 12 - PWM\\[4\\] output enable control."]
    #[inline(always)]
    pub fn pwmena4(&mut self) -> Pwmena4W<'_, PcrSpec> {
        Pwmena4W::new(self, 12)
    }
    #[doc = "Bit 13 - PWM\\[5\\] output enable control."]
    #[inline(always)]
    pub fn pwmena5(&mut self) -> Pwmena5W<'_, PcrSpec> {
        Pwmena5W::new(self, 13)
    }
    #[doc = "Bit 14 - PWM\\[6\\] output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&mut self) -> Pwmena6W<'_, PcrSpec> {
        Pwmena6W::new(self, 14)
    }
}
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrSpec;
impl crate::RegisterSpec for PcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PcrSpec {}
