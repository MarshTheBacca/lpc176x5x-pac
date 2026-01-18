#[doc = "Register `CON` reader"]
pub type R = crate::R<ConSpec>;
#[doc = "Stops/starts timer channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConRun0Enum {
    #[doc = "0: Stop."]
    Stop_ = 0,
    #[doc = "1: Run."]
    Run_ = 1,
}
impl From<McpwmConRun0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConRun0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN0` reader - Stops/starts timer channel 0."]
pub type Run0R = crate::BitReader<McpwmConRun0Enum>;
impl Run0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConRun0Enum {
        match self.bits {
            false => McpwmConRun0Enum::Stop_,
            true => McpwmConRun0Enum::Run_,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == McpwmConRun0Enum::Stop_
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == McpwmConRun0Enum::Run_
    }
}
#[doc = "Edge/center aligned operation for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConCenter0Enum {
    #[doc = "0: Edge-aligned."]
    EdgeAligned_ = 0,
    #[doc = "1: Center-aligned."]
    CenterAligned_ = 1,
}
impl From<McpwmConCenter0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConCenter0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER0` reader - Edge/center aligned operation for channel 0."]
pub type Center0R = crate::BitReader<McpwmConCenter0Enum>;
impl Center0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConCenter0Enum {
        match self.bits {
            false => McpwmConCenter0Enum::EdgeAligned_,
            true => McpwmConCenter0Enum::CenterAligned_,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == McpwmConCenter0Enum::EdgeAligned_
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == McpwmConCenter0Enum::CenterAligned_
    }
}
#[doc = "Selects polarity of the MCOA0 and MCOB0 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConPola0Enum {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PassiveStateIsLow = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PassiveStateIsHig = 1,
}
impl From<McpwmConPola0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConPola0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA0` reader - Selects polarity of the MCOA0 and MCOB0 pins."]
pub type Pola0R = crate::BitReader<McpwmConPola0Enum>;
impl Pola0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConPola0Enum {
        match self.bits {
            false => McpwmConPola0Enum::PassiveStateIsLow,
            true => McpwmConPola0Enum::PassiveStateIsHig,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == McpwmConPola0Enum::PassiveStateIsLow
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == McpwmConPola0Enum::PassiveStateIsHig
    }
}
#[doc = "Controls the dead-time feature for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConDte0Enum {
    #[doc = "0: Dead-time disabled."]
    DeadTimeDisabled_ = 0,
    #[doc = "1: Dead-time enabled."]
    DeadTimeEnabled_ = 1,
}
impl From<McpwmConDte0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConDte0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE0` reader - Controls the dead-time feature for channel 0."]
pub type Dte0R = crate::BitReader<McpwmConDte0Enum>;
impl Dte0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConDte0Enum {
        match self.bits {
            false => McpwmConDte0Enum::DeadTimeDisabled_,
            true => McpwmConDte0Enum::DeadTimeEnabled_,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == McpwmConDte0Enum::DeadTimeDisabled_
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == McpwmConDte0Enum::DeadTimeEnabled_
    }
}
#[doc = "Enable/disable updates of functional registers for channel 0 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConDisup0Enum {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    Update = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    Noupdate = 1,
}
impl From<McpwmConDisup0Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConDisup0Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP0` reader - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
pub type Disup0R = crate::BitReader<McpwmConDisup0Enum>;
impl Disup0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConDisup0Enum {
        match self.bits {
            false => McpwmConDisup0Enum::Update,
            true => McpwmConDisup0Enum::Noupdate,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == McpwmConDisup0Enum::Update
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == McpwmConDisup0Enum::Noupdate
    }
}
#[doc = "Stops/starts timer channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConRun1Enum {
    #[doc = "0: Stop."]
    Stop_ = 0,
    #[doc = "1: Run."]
    Run_ = 1,
}
impl From<McpwmConRun1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConRun1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN1` reader - Stops/starts timer channel 1."]
pub type Run1R = crate::BitReader<McpwmConRun1Enum>;
impl Run1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConRun1Enum {
        match self.bits {
            false => McpwmConRun1Enum::Stop_,
            true => McpwmConRun1Enum::Run_,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == McpwmConRun1Enum::Stop_
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == McpwmConRun1Enum::Run_
    }
}
#[doc = "Edge/center aligned operation for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConCenter1Enum {
    #[doc = "0: Edge-aligned."]
    EdgeAligned_ = 0,
    #[doc = "1: Center-aligned."]
    CenterAligned_ = 1,
}
impl From<McpwmConCenter1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConCenter1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER1` reader - Edge/center aligned operation for channel 1."]
pub type Center1R = crate::BitReader<McpwmConCenter1Enum>;
impl Center1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConCenter1Enum {
        match self.bits {
            false => McpwmConCenter1Enum::EdgeAligned_,
            true => McpwmConCenter1Enum::CenterAligned_,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == McpwmConCenter1Enum::EdgeAligned_
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == McpwmConCenter1Enum::CenterAligned_
    }
}
#[doc = "Selects polarity of the MCOA1 and MCOB1 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConPola1Enum {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PassiveStateIsLow = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PassiveStateIsHig = 1,
}
impl From<McpwmConPola1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConPola1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA1` reader - Selects polarity of the MCOA1 and MCOB1 pins."]
pub type Pola1R = crate::BitReader<McpwmConPola1Enum>;
impl Pola1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConPola1Enum {
        match self.bits {
            false => McpwmConPola1Enum::PassiveStateIsLow,
            true => McpwmConPola1Enum::PassiveStateIsHig,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == McpwmConPola1Enum::PassiveStateIsLow
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == McpwmConPola1Enum::PassiveStateIsHig
    }
}
#[doc = "Controls the dead-time feature for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConDte1Enum {
    #[doc = "0: Dead-time disabled."]
    DeadTimeDisabled_ = 0,
    #[doc = "1: Dead-time enabled."]
    DeadTimeEnabled_ = 1,
}
impl From<McpwmConDte1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConDte1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE1` reader - Controls the dead-time feature for channel 1."]
pub type Dte1R = crate::BitReader<McpwmConDte1Enum>;
impl Dte1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConDte1Enum {
        match self.bits {
            false => McpwmConDte1Enum::DeadTimeDisabled_,
            true => McpwmConDte1Enum::DeadTimeEnabled_,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == McpwmConDte1Enum::DeadTimeDisabled_
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == McpwmConDte1Enum::DeadTimeEnabled_
    }
}
#[doc = "Enable/disable updates of functional registers for channel 1 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConDisup1Enum {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    Update = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    Noupdate = 1,
}
impl From<McpwmConDisup1Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConDisup1Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP1` reader - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
pub type Disup1R = crate::BitReader<McpwmConDisup1Enum>;
impl Disup1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConDisup1Enum {
        match self.bits {
            false => McpwmConDisup1Enum::Update,
            true => McpwmConDisup1Enum::Noupdate,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == McpwmConDisup1Enum::Update
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == McpwmConDisup1Enum::Noupdate
    }
}
#[doc = "Stops/starts timer channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConRun2Enum {
    #[doc = "0: Stop."]
    Stop_ = 0,
    #[doc = "1: Run."]
    Run_ = 1,
}
impl From<McpwmConRun2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConRun2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN2` reader - Stops/starts timer channel 2."]
pub type Run2R = crate::BitReader<McpwmConRun2Enum>;
impl Run2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConRun2Enum {
        match self.bits {
            false => McpwmConRun2Enum::Stop_,
            true => McpwmConRun2Enum::Run_,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == McpwmConRun2Enum::Stop_
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == McpwmConRun2Enum::Run_
    }
}
#[doc = "Edge/center aligned operation for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConCenter2Enum {
    #[doc = "0: Edge-aligned."]
    EdgeAligned_ = 0,
    #[doc = "1: Center-aligned."]
    CenterAligned_ = 1,
}
impl From<McpwmConCenter2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConCenter2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER2` reader - Edge/center aligned operation for channel 2."]
pub type Center2R = crate::BitReader<McpwmConCenter2Enum>;
impl Center2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConCenter2Enum {
        match self.bits {
            false => McpwmConCenter2Enum::EdgeAligned_,
            true => McpwmConCenter2Enum::CenterAligned_,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == McpwmConCenter2Enum::EdgeAligned_
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == McpwmConCenter2Enum::CenterAligned_
    }
}
#[doc = "Selects polarity of the MCOA2 and MCOB2 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConPola2Enum {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PassiveStateIsLow = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PassiveStateIsHig = 1,
}
impl From<McpwmConPola2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConPola2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA2` reader - Selects polarity of the MCOA2 and MCOB2 pins."]
pub type Pola2R = crate::BitReader<McpwmConPola2Enum>;
impl Pola2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConPola2Enum {
        match self.bits {
            false => McpwmConPola2Enum::PassiveStateIsLow,
            true => McpwmConPola2Enum::PassiveStateIsHig,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == McpwmConPola2Enum::PassiveStateIsLow
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == McpwmConPola2Enum::PassiveStateIsHig
    }
}
#[doc = "Controls the dead-time feature for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConDte2Enum {
    #[doc = "0: Dead-time disabled."]
    DeadTimeDisabled_ = 0,
    #[doc = "1: Dead-time enabled."]
    DeadTimeEnabled_ = 1,
}
impl From<McpwmConDte2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConDte2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE2` reader - Controls the dead-time feature for channel 1."]
pub type Dte2R = crate::BitReader<McpwmConDte2Enum>;
impl Dte2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConDte2Enum {
        match self.bits {
            false => McpwmConDte2Enum::DeadTimeDisabled_,
            true => McpwmConDte2Enum::DeadTimeEnabled_,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == McpwmConDte2Enum::DeadTimeDisabled_
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == McpwmConDte2Enum::DeadTimeEnabled_
    }
}
#[doc = "Enable/disable updates of functional registers for channel 2 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConDisup2Enum {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    Update = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    Noupdate = 1,
}
impl From<McpwmConDisup2Enum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConDisup2Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP2` reader - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
pub type Disup2R = crate::BitReader<McpwmConDisup2Enum>;
impl Disup2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConDisup2Enum {
        match self.bits {
            false => McpwmConDisup2Enum::Update,
            true => McpwmConDisup2Enum::Noupdate,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == McpwmConDisup2Enum::Update
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == McpwmConDisup2Enum::Noupdate
    }
}
#[doc = "Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConInvbdcEnum {
    #[doc = "0: The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    Opposite = 0,
    #[doc = "1: The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    Same = 1,
}
impl From<McpwmConInvbdcEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConInvbdcEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVBDC` reader - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
pub type InvbdcR = crate::BitReader<McpwmConInvbdcEnum>;
impl InvbdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConInvbdcEnum {
        match self.bits {
            false => McpwmConInvbdcEnum::Opposite,
            true => McpwmConInvbdcEnum::Same,
        }
    }
    #[doc = "The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    #[inline(always)]
    pub fn is_opposite(&self) -> bool {
        *self == McpwmConInvbdcEnum::Opposite
    }
    #[doc = "The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == McpwmConInvbdcEnum::Same
    }
}
#[doc = "3-phase AC mode select (see Section 24.8.7).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConAcmodeEnum {
    #[doc = "0: 3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    _3PhaseAcModeOff = 0,
    #[doc = "1: 3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    _3PhaseAcModeOn_ = 1,
}
impl From<McpwmConAcmodeEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConAcmodeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMODE` reader - 3-phase AC mode select (see Section 24.8.7)."]
pub type AcmodeR = crate::BitReader<McpwmConAcmodeEnum>;
impl AcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConAcmodeEnum {
        match self.bits {
            false => McpwmConAcmodeEnum::_3PhaseAcModeOff,
            true => McpwmConAcmodeEnum::_3PhaseAcModeOn_,
        }
    }
    #[doc = "3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    #[inline(always)]
    pub fn is_3_phase_ac_mode_off(&self) -> bool {
        *self == McpwmConAcmodeEnum::_3PhaseAcModeOff
    }
    #[doc = "3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    #[inline(always)]
    pub fn is_3_phase_ac_mode_on_(&self) -> bool {
        *self == McpwmConAcmodeEnum::_3PhaseAcModeOn_
    }
}
#[doc = "3-phase DC mode select (see Section 24.8.6).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpwmConDcmodeEnum {
    #[doc = "0: 3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    _3PhaseDcModeOff = 0,
    #[doc = "1: 3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    _3PhaseDcModeOn_ = 1,
}
impl From<McpwmConDcmodeEnum> for bool {
    #[inline(always)]
    fn from(variant: McpwmConDcmodeEnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMODE` reader - 3-phase DC mode select (see Section 24.8.6)."]
pub type DcmodeR = crate::BitReader<McpwmConDcmodeEnum>;
impl DcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McpwmConDcmodeEnum {
        match self.bits {
            false => McpwmConDcmodeEnum::_3PhaseDcModeOff,
            true => McpwmConDcmodeEnum::_3PhaseDcModeOn_,
        }
    }
    #[doc = "3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    #[inline(always)]
    pub fn is_3_phase_dc_mode_off(&self) -> bool {
        *self == McpwmConDcmodeEnum::_3PhaseDcModeOff
    }
    #[doc = "3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    #[inline(always)]
    pub fn is_3_phase_dc_mode_on_(&self) -> bool {
        *self == McpwmConDcmodeEnum::_3PhaseDcModeOn_
    }
}
impl R {
    #[doc = "Bit 0 - Stops/starts timer channel 0."]
    #[inline(always)]
    pub fn run0(&self) -> Run0R {
        Run0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge/center aligned operation for channel 0."]
    #[inline(always)]
    pub fn center0(&self) -> Center0R {
        Center0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects polarity of the MCOA0 and MCOB0 pins."]
    #[inline(always)]
    pub fn pola0(&self) -> Pola0R {
        Pola0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the dead-time feature for channel 0."]
    #[inline(always)]
    pub fn dte0(&self) -> Dte0R {
        Dte0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup0(&self) -> Disup0R {
        Disup0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Stops/starts timer channel 1."]
    #[inline(always)]
    pub fn run1(&self) -> Run1R {
        Run1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Edge/center aligned operation for channel 1."]
    #[inline(always)]
    pub fn center1(&self) -> Center1R {
        Center1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects polarity of the MCOA1 and MCOB1 pins."]
    #[inline(always)]
    pub fn pola1(&self) -> Pola1R {
        Pola1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte1(&self) -> Dte1R {
        Dte1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup1(&self) -> Disup1R {
        Disup1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Stops/starts timer channel 2."]
    #[inline(always)]
    pub fn run2(&self) -> Run2R {
        Run2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Edge/center aligned operation for channel 2."]
    #[inline(always)]
    pub fn center2(&self) -> Center2R {
        Center2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Selects polarity of the MCOA2 and MCOB2 pins."]
    #[inline(always)]
    pub fn pola2(&self) -> Pola2R {
        Pola2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte2(&self) -> Dte2R {
        Dte2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup2(&self) -> Disup2R {
        Disup2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
    #[inline(always)]
    pub fn invbdc(&self) -> InvbdcR {
        InvbdcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 3-phase AC mode select (see Section 24.8.7)."]
    #[inline(always)]
    pub fn acmode(&self) -> AcmodeR {
        AcmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 3-phase DC mode select (see Section 24.8.6)."]
    #[inline(always)]
    pub fn dcmode(&self) -> DcmodeR {
        DcmodeR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "PWM Control read address\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConSpec;
impl crate::RegisterSpec for ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`con::R`](R) reader structure"]
impl crate::Readable for ConSpec {}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for ConSpec {}
