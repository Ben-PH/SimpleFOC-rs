use core::marker::PhantomData;

use embedded_time::{Clock, Instant, duration};
use fixed::types::I16F16;
use foc::park_clarke::MovingReferenceFrame;
use generic_array::{ArrayLength, GenericArray};
use pid::Pid;
use typenum::NonZero;
use core::mem::MaybeUninit;

use crate::common::helpers::DutyCycle;

use self::force_paradigms::{Newtons, Voltage};

use super::{bldc_driver::MotorHiPins, pos_sensor::PosSensor};

/// The describes the position of an inductor in the pitch of the permenant magnetic field, in
/// units of tau.
/// A linear motor with a 20mm pitch, 10mm from reference zero, the value would be 0.5
/// A rotary motor with a pitch of 36 degrees, and 9 inductors would have 12 degrees in
/// the physical rotation of the motor for each phase-angle rotation.

/// These module scopes are effectively a no-op when combined with `use $MODULE_NAME::*`,
/// It's just handy to compartmentalise.
mod force_paradigms {
    pub struct Voltage(pub f32);
    pub struct DCCurrent(f32);
    pub struct FOCCurrent(f32);

    pub trait ForceControlType {}
    impl ForceControlType for Voltage {}
    impl ForceControlType for DCCurrent {}
    impl ForceControlType for FOCCurrent {}
    pub struct Newtons<FType: ForceControlType>(FType);
}
mod positioning_paradigms {
    use fixed::types::I16F16;

    pub struct PhaseAngle(pub I16F16);
    /// Distance from 0-reference to denote position.
    /// T could be encoder pulses, Millimeters<i32>, etc
    pub struct Displacement<T>(pub T);
    /// Used in the derivitives of Displacement
    pub struct TimeDelta<T>(pub T);
    pub struct Velocity<Dd, Dt> {
        d_disp: Displacement<Dd>,
        d_time: TimeDelta<Dt>,
    }
}
pub use positioning_paradigms::PhaseAngle;
mod pid_types {
    use pid::Pid;

    pub struct QCurrentPID(pub Pid<f32>);
    pub struct DCurrentPID(pub Pid<f32>);
    pub struct VelocityPID(pub Pid<f32>);
    pub struct VoltagePID(pub Pid<f32>);
    pub struct PositionPID(pub Pid<f32>);
}
use positioning_paradigms::*;

pub struct Amps(I16F16);

/// Note: P::Output and Instant<C> relies on popping the stack to release memory.
// TODO: setup BufSize so that its length is typed. Len 1: position, 2: vel, and so on
//       ...with arbitrary buffer-len, we can use math-magic like taylor siries and 
//       other cool things to get nice analysis
pub struct MotionTracker<P: PosSensor, C: Clock, const BufSize: usize> {
    clock_source: C,
    pos_source: P,
    mvmnt_buffer: [MaybeUninit<(P::Output, Instant<C>)>; BufSize],
    head: u8,
    entry_count: u8,
}
// + NonZero + typenum::IsLess<typenum::U255, Output = typenum::True>
impl<P: PosSensor<Output = u32>, C: Clock, const BufSize: usize> MotionTracker<P, C, BufSize> {
    fn init(clock_source: C, pos_source: P, pos: P::Output, instant: Instant<C>) -> Self {
        let mut mvmnt_buffer: [MaybeUninit<(P::Output, Instant<C>)>; BufSize] = unsafe { MaybeUninit::uninit().assume_init() };
        mvmnt_buffer[0] = MaybeUninit::new((pos, instant));
        Self {
            clock_source,
            pos_source,
            mvmnt_buffer,
            head: 1,
            entry_count: 1,
        }
    }
    fn push(&mut self, pos: P::Output, instant: Instant<C>) {
        self.head += 1;
        self.head %= BufSize as u8;
        self.mvmnt_buffer[self.head as usize] = MaybeUninit::new((pos, instant));
        if self.entry_count != (BufSize as u8) {
            self.entry_count += 1;
        }
    }
    fn latest_pos(&self) -> P::Output {
        let head = self.mvmnt_buffer[self.head as usize];
        unsafe {
            head.assume_init()
        }.0
    }
    fn latest_vel(&self) -> (P::Output, duration::Generic<C::T>) {
        assert!(self.entry_count > 1, "todo: getting latest velocity should by type-constrained to when it's tracked two position/instant pairs");
        let pta = self.mvmnt_buffer[self.head as usize];
        let prev = if self.head == 0 {
            BufSize - 1
        } else {
            self.head as usize - 1
        };
        let pt_before = self.mvmnt_buffer[self.head as usize];
        let pt_latest = self.mvmnt_buffer[prev];
        let (pt_before, pt_latest) = unsafe {(
                pt_before.assume_init(),
                pt_latest.assume_init()
        )};
        (pt_latest.0 - pt_before.0, pt_latest.1.checked_duration_since(&pt_before.1).unwrap())
    }
}

mod control_modes {
    use super::{
        force_paradigms::{ForceControlType, Newtons},
        positioning_paradigms::{Displacement, Velocity},
    };

    pub trait MotionControlMode {}
    impl<FType: ForceControlType> MotionControlMode for Newtons<FType> {}
    // todo: use fixed point. stuck with f32 in the meantime due to pid crate
    impl MotionControlMode for Velocity<f32, f32> {}
    impl MotionControlMode for Displacement<f32> {}
    // todo: open loop velocity and displacement
}
use control_modes::*;

// TODO: these varients determine behavior, and deserve to be encapsulated using type-state
// patterns
pub enum FOCMotorStatus {
    Uninit,
    Initting,
    Uncalibrated,
    Callibrating,
    Ready,
    RecoverableError,
    CalbrationFail,
    InitFail,
}
#[derive(Default)]
pub enum FOCModulationType {
    #[default]
    SinePWM,
    SpaceVectorPWM,
    Trapezoid120,
    Trapezoid150,
}

pub enum PidSetpoints<D, T> {
    Displacement(Displacement<D>),
    Velocity(Velocity<D, T>),
    Current(Amps),
}

pub trait MotionControl: Sized {
    type Mode: MotionControlMode;
    type PosSource: PosSensor;
    fn set_motion(&mut self, motion: Self::Mode);
}

struct DefaultMotionCtrl<T, P: PosSensor, C: Clock> {
    motion_down_sample: Option<(u32, u32)>,
    motion_tracker: MotionTracker<P, C, 4>,
    clock_src: C,
    pos_sensor: P,
    m_type: PhantomData<T>,
}

impl<P: PosSensor, C: Clock> DefaultMotionCtrl<Newtons<Voltage>, P, C> {
    fn do_motion_impl(&mut self, motion: Newtons<Voltage>) {
    }
}
impl<P: PosSensor, C: Clock> DefaultMotionCtrl<Displacement<f32>, P, C> {
    fn do_motion_impl(&mut self, motion: Displacement<f32>) {
    }
}

impl<T: MotionControlMode, P: PosSensor<Output = u32>, C: Clock> MotionControl for DefaultMotionCtrl<T, P, C> {
    type Mode = T;
    type PosSource = P;

    fn set_motion(&mut self, motion: Self::Mode) {
        // optional downsample early-return/update
        if let Some(&mut (ref mut count, sample)) = self.motion_down_sample.as_mut() {
            *count += 1;
            if *count < sample {
                return;
            } else {
                *count = 0;
            }
        }

        self.motion_tracker.push(
            self.pos_sensor.get_position_um(),
            self.clock_src.try_now().unwrap(),
        );

        self.do_motion_impl(motion);
    }
}


// temporarily hacked to be for a 3pwm bldc motor
pub trait FOController: Sized + MotorHiPins {
    // fn enable(&mut self);
    // fn disable(&mut self);
    // fn link_sensor/current_sensor(....
    // todo: fn init_foc_algo(&mut self) -> u32; // why the u32?
    // todo: fn foc_loop(&mut self) -> !;
    // todo: fn move_command(motion: MotionCtrl);
    fn set_phase_voltage(&mut self, voltages_q_d: MovingReferenceFrame, phase_angle: PhaseAngle) {
        let (sin_angle, cos_angle) = cordic::sin_cos(phase_angle.0);
        let orth_v = foc::park_clarke::inverse_park(cos_angle, sin_angle, voltages_q_d);
        let [phase_a, phase_b, phase_c] = foc::pwm::spwm(orth_v);
        self.set_pwms(DutyCycle(phase_a), DutyCycle(phase_b), DutyCycle(phase_c))
    }
}
