use core::mem::MaybeUninit;
use discrete_count::{re_exports::fixed::types::I16F16, CountRaw, CountReader, Counter};
use foc::park_clarke::MovingReferenceFrame;
use pid::Pid;

use crate::common::helpers::DutyCycle;

use super::bldc_driver::MotorPins;

pub struct ForceVoltage(pub f32);
pub struct DCCurrent(pub f32);
pub struct FOCCurrent(pub f32);

pub struct PhaseAngle(pub I16F16);
/// Distance from 0-reference to denote position.
/// T could be encoder pulses, Millimeters<i32>, etc
pub struct Displacement<T>(pub T);
/// Used in the derivitives of Displacement
pub struct TimeDelta<T>(pub T);
pub struct Velocity<Dd, Dt> {
    _d_disp: Displacement<Dd>,
    _d_time: TimeDelta<Dt>,
}

pub struct QCurrentPID(pub Pid<f32>);
pub struct DCurrentPID(pub Pid<f32>);
pub struct VelocityPID(pub Pid<f32>);
pub struct VoltagePID(pub Pid<f32>);
pub struct PositionPID(pub Pid<f32>);

pub struct Amps(pub I16F16);

/// Note: P::Output and Instant<C> relies on popping the stack to release memory.
// TODO: setup BufSize so that its length is typed. Len 1: position, 2: vel, and so on
//       ...with arbitrary buffer-len, we can use math-magic like taylor siries and
//       other cool things to get nice analysis
pub struct MotionTracker<T: Counter, P: Counter, const BUF_SIZE: usize> {
    pub clock_source: T,
    pos_source: P,
    mvmnt_buffer: [MaybeUninit<(CountRaw<T>, CountRaw<P>)>; BUF_SIZE],

    head: u8,
    entry_count: u8,
}
// + NonZero + typenum::IsLess<typenum::U255, Output = typenum::True>
impl<T: Counter, P: Counter, const BUF_SIZE: usize> MotionTracker<T, P, BUF_SIZE> {
    pub fn init(
        clock_source: T,
        instant: <T::Reader as CountReader>::RawData,
        pos_source: P,
        pos: <P::Reader as CountReader>::RawData,
    ) -> Self {
        let mut mvmnt_buffer: [_; BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
        mvmnt_buffer[0] = MaybeUninit::new((instant, pos));
        Self {
            clock_source,
            pos_source,
            mvmnt_buffer,
            head: 1,
            entry_count: 1,
        }
    }
    fn push_update(&mut self) {
        let pos = <T::Reader as CountReader>::read();
        let instant = <P::Reader as CountReader>::read();
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

pub struct DefaultMotionCtrl<T: Counter, P: Counter> {
    motion_down_sample: Option<(u32, u32)>,
    pub motion_tracker: MotionTracker<T, P, 4>,
}

// impl<T: Count, P: Counter> DefaultMotionCtrl<T, P> {
//     pub fn new(
//         motion_down_sample: Option<(u32, u32)>,
//         motion_tracker: MotionTracker<T, P, 4>,
//     ) -> Self {
//         Self {
//             motion_down_sample,
//             motion_tracker,
//         }
//     }
// }

impl<T: Counter, P: Counter> MotionControl for DefaultMotionCtrl<T, P>
where
    <T::Reader as CountReader>::ReadErr: core::fmt::Display + core::fmt::Debug,
    <T::Reader as CountReader>::RawData: num::CheckedSub,
{
    type PosSource = P;

    // fn do_motion_impl(&mut self, motion: Displacement<f32>) {
    //     todo!()
    // }
    fn set_displacement(&mut self, _motion: Displacement<f32>) {
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

// temporarily hacked to be for a 3pwm bldc motor, const voltage, svpm
pub trait FOController: MotorPins {
    // fn enable(&mut self);
    // fn disable(&mut self);
    // fn link_sensor/current_sensor(....
    // todo: fn init_foc_algo(&mut self) -> u32; // why the u32?
    // todo: fn foc_loop(&mut self) -> !;
    // todo: fn move_command(motion: MotionCtrl);
    fn set_phase_voltage(&mut self, voltages_q_d: MovingReferenceFrame, phase_angle: PhaseAngle) {
        let (sin_angle, cos_angle) = cordic::sin_cos(phase_angle.0);
        let orth_v = foc::park_clarke::inverse_park(cos_angle, sin_angle, voltages_q_d);
        let [phase_a, phase_b, phase_c] = foc::pwm::svpwm(orth_v);
        self.set_pwms(DutyCycle(phase_a), DutyCycle(phase_b), DutyCycle(phase_c))
    }
}
