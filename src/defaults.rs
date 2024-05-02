/// Default configuration values
/// Change this file to optimal values for your application

const DEF_POWER_SUPPLY: f32 = 12.0;

// Velocity PI controller params

/// Default PID controller Position value
const DEF_PID_VEL_P: f32 = 0.5;
/// Default PID controller Integral value
const DEF_PID_VEL_I: f32 = 10.0;
/// Default PID controller Derivitive value
const DEF_PID_VEL_D: f32 = 0.0;
/// Default PID controller voltage ramp value
const DEF_PID_VEL_RAMP: f32 = 1000.0;
/// Default PID controller voltage limit
const DEF_PID_VEL_LIMIT: f32 = DEF_POWER_SUPPLY;

// Current sensing PID values
// For 16MHz controllers like Arduino Uno and Mega
// TODO: AVR_ATmega[328P || 168 || 328PB || 2560] only
const DEF_PID_CURR_P_16MHZ: i32 = 2;
const DEF_PID_CURR_I_16MHZ: i32 = 100;
const DEF_PID_CURR_D_16MHZ: f32 = 0.0;
const DEF_PID_CURR_RAMP_16MHZ: f32 = 1000.0;
const DEF_PID_CURR_LIMIT_16MHZ: f32 = DEF_POWER_SUPPLY;
const DEF_CURR_FILTER_TF_16MHZ: f32 = 0.01;

//TODO: For STM32, Due, Teensy, ESP32, and similar
const DEF_PID_CURR_P_32BIT: i32 = 3;
const DEF_PID_CURR_I_32BIT: f32 = 300.0;
const DEF_PID_CURR_D_32BIT: f32 = 0.0;
const DEF_PID_CURR_RAMP_32BIT: i32 = 0;
const DEF_PID_CURR_LIMIT_32BIT: f32 = DEF_POWER_SUPPLY;
const DEF_CURR_FILTER_TF_32BIT: f32 = 0.005;

/// Default current limit values
const DEF_CURRENT_LIM: f32 = 2.0;

/// Default monitor downsample
const DEF_MON_DOWNSAMPLE: i32 = 100;
const DEF_MOTION_DOWNSAMPLE: i32 = 0;

/// Angle P params
const DEF_P_ANGLE_P: f32 = 20.0;
const DEF_VEL_LIM: f32 = 20.0;

/// Index search
const DEF_INDEX_SEARCH_TARGET_VELOCITY: f32 = 1.0;

/// Align voltage
const DEF_VOLTAGE_SENSOR_ALIGN: f32 = 3.0;

/// Low pass filter velocity
const DEF_VEL_FILTER_TF: f32 = 0.005;

/// Current sense default parameters
const DEF_LPF_PER_PHASE_CURRENT_SENSE_TF: f32 = 0.0;
