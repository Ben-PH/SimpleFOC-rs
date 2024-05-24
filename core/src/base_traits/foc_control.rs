use core::{mem::MaybeUninit, ops::Sub};
use counters::{Counter, TimeCount};
use fixed::types::I16F16;
use foc::park_clarke::MovingReferenceFrame;
use pid::Pid;

use crate::common::helpers::DutyCycle;

use super::{bldc_driver::MotorHiPins, pos_sensor::PosSensor};

pub struct ForceVoltage(pub f32);
pub struct DCCurrent(f32);
pub struct FOCCurrent(f32);

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

pub struct QCurrentPID(pub Pid<f32>);
pub struct DCurrentPID(pub Pid<f32>);
pub struct VelocityPID(pub Pid<f32>);
pub struct VoltagePID(pub Pid<f32>);
pub struct PositionPID(pub Pid<f32>);

pub struct Amps(I16F16);

/// Note: P::Output and Instant<C> relies on popping the stack to release memory.
// TODO: setup BufSize so that its length is typed. Len 1: position, 2: vel, and so on
//       ...with arbitrary buffer-len, we can use math-magic like taylor siries and
//       other cool things to get nice analysis
pub struct MotionTracker<C: TimeCount, P: Counter, const BufSize: usize> {
    clock_source: C,
    pos_source: P,
    mvmnt_buffer: [MaybeUninit<(P::RawData, C::RawData)>; BufSize],
    head: u8,
    entry_count: u8,
}
// + NonZero + typenum::IsLess<typenum::U255, Output = typenum::True>
impl<T: TimeCount, P: Counter, const BUF_SIZE: usize> MotionTracker<T, P, BUF_SIZE> {
    pub fn init(clock_source: T, instant: T::RawData, pos_source: P, pos: P::RawData) -> Self {
        let mut mvmnt_buffer: [MaybeUninit<(P::RawData, T::RawData)>; BUF_SIZE] =
            unsafe { MaybeUninit::uninit().assume_init() };
        mvmnt_buffer[0] = MaybeUninit::new((pos, instant));
        Self {
            clock_source,
            pos_source,
            mvmnt_buffer,
            head: 1,
            entry_count: 1,
        }
    }
    fn push_update(&mut self) {
        let pos = self.pos_source.try_read_raw();
        let instant = self.clock_source.try_now_raw();
        let (Ok(pos), Ok(instant)) = (pos, instant) else {
            panic!("todo. just read the code and weep...");
        };
        self.head += 1;
        self.head %= BUF_SIZE as u8;
        self.mvmnt_buffer[self.head as usize] = MaybeUninit::new((pos, instant));
        if self.entry_count != (BUF_SIZE as u8) {
            self.entry_count += 1;
        }
    }
    fn latest_pos(&self) -> P::RawData {
        let head = self.mvmnt_buffer[self.head as usize];
        unsafe { head.assume_init() }.0
    }
    fn latest_vel(&self) -> (P::CountMeasure, T::TickMeasure)
    where
        T::RawData: num::CheckedSub,
        P::RawData: Sub<P::RawData, Output = P::RawData>,
    {
        assert!(self.entry_count > 1, "todo: getting latest velocity should by type-constrained to when it's tracked two position/instant pairs");
        let pta = self.mvmnt_buffer[self.head as usize];
        let prev = if self.head == 0 {
            BUF_SIZE - 1
        } else {
            self.head as usize - 1
        };
        let pt_before = self.mvmnt_buffer[self.head as usize];
        let pt_latest = self.mvmnt_buffer[prev];
        let (pt_before, pt_latest) = unsafe { (pt_before.assume_init(), pt_latest.assume_init()) };
        let raw_diff: T::RawData = num::CheckedSub::checked_sub(&pt_latest.1, &pt_before.1)
            .unwrap()
            .into();
        let time_diff = raw_diff.into();
        (P::raw_to_measure(pt_latest.0 - pt_before.0), time_diff)
    }
}

pub trait MotionControlMode: MotionControl {
    type MotionParams;
    fn do_motion_impl(&mut self, motion: Self::MotionParams);
}

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
    type PosSource: Counter;
    fn set_displacement(&mut self, disp: Displacement<f32>);
    // fn do_motion_impl<M: MotionControlMode>(&mut self, motion: M);
}

pub struct DefaultMotionCtrl<T: TimeCount, P: Counter> {
    motion_down_sample: Option<(u32, u32)>,
    motion_tracker: MotionTracker<T, P, 4>,
}

impl<T: TimeCount, P: Counter> DefaultMotionCtrl<T, P> {
    pub fn new(
        motion_down_sample: Option<(u32, u32)>,
        motion_tracker: MotionTracker<T, P, 4>,
    ) -> Self {
        Self {
            motion_down_sample,
            motion_tracker,
        }
    }
}

impl<T: TimeCount, P: Counter> MotionControl for DefaultMotionCtrl<T, P>
where
    T::Error: core::fmt::Display + core::fmt::Debug,
    T::RawData: num::CheckedSub,
{
    type PosSource = P;

    // fn do_motion_impl(&mut self, motion: Displacement<f32>) {
    //     todo!()
    // }
    fn set_displacement(&mut self, motion: Displacement<f32>) {
        // optional downsample early-return/update
        if let Some(&mut (ref mut count, sample)) = self.motion_down_sample.as_mut() {
            *count += 1;
            if *count < sample {
                return;
            } else {
                *count = 0;
            }
        }

        self.motion_tracker.push_update();

        // self.do_motion_impl(motion);
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
