use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum AccStatus {
    Off = 0,
    Replay = 1,
    Live = 2,
    Pause = 3,
}

impl fmt::Display for AccStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccStatus::Off => write!(f, "Off"),
            AccStatus::Replay => write!(f, "Replay"),
            AccStatus::Live => write!(f, "Live"),
            AccStatus::Pause => write!(f, "Pause"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum AccSessionType {
    Unknown = -1,
    Practice = 0,
    Qualify = 1,
    Race = 2,
    Hotlap = 3,
    TimeAttack = 4,
    Superpole = 5,
    Hotstint = 6,
    HotlapSuperpole = 7,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum AccFlagType {
    NoFlag = 0,
    BlueFlag = 1,
    YellowFlag = 2,
    BlackFlag = 3,
    WhiteFlag = 4,
    CheckeredFlag = 5,
    PenaltyFlag = 6,
    GreenFlag = 7,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum AccPenaltyType {
    None = 0,
    DriveThrough = 1,
    StopAndGo = 2,
    TimePenalty = 3,
    PitLane = 4,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum AccTrackGripStatus {
    Green = 0,
    Fast = 1,
    Optimum = 2,
    Greasy = 3,
    Wet = 4,
    Flooded = 5,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum AccRainIntensity {
    NoRain = 0,
    Drizzle = 1,
    LightRain = 2,
    MediumRain = 3,
    HeavyRain = 4,
    Thunderstorm = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Physics {
    pub packet_id: i32,
    pub gas: f32,
    pub brake: f32,
    pub fuel: f32,
    pub gear: i32,
    pub rpms: i32,
    pub steer_angle: f32,
    pub speed_kmh: f32,
    pub velocity: [f32; 3],
    pub acc_g: [f32; 3],
    pub wheel_slip: [f32; 4],
    pub wheel_load: [f32; 4],
    pub wheels_pressure: [f32; 4],
    pub wheel_angular_speed: [f32; 4],
    pub tyre_wear: [f32; 4],
    pub tyre_dirty_level: [f32; 4],
    pub tyre_core_temperature: [f32; 4],
    pub camber_rad: [f32; 4],
    pub suspension_travel: [f32; 4],
    pub drs: f32,
    pub tc: f32,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub cg_height: f32,
    pub car_damage: [f32; 5],
    pub number_of_tyres_out: i32,
    pub pit_limiter_on: i32,
    pub abs: f32,
    pub kers_charge: f32,
    pub kers_input: f32,
    pub auto_shifter_on: i32,
    pub ride_height: [f32; 2],
    pub turbo_boost: f32,
    pub ballast: f32,
    pub air_density: f32,
    pub air_temp: f32,
    pub road_temp: f32,
    pub local_angular_velocity: [f32; 3],
    pub final_ff: f32,
    pub performance_meter: f32,
    pub engine_brake: i32,
    pub ers_recovery_level: i32,
    pub ers_power_level: i32,
    pub ers_heat_charging: i32,
    pub ers_is_charging: i32,
    pub kers_current_kj: f32,
    pub drs_available: i32,
    pub drs_enabled: i32,
    pub brake_temp: [f32; 4],
    pub clutch: f32,
    pub tyre_temp_i: [f32; 4],
    pub tyre_temp_m: [f32; 4],
    pub tyre_temp_o: [f32; 4],
    pub is_ai_controlled: i32,
    pub tyre_contact_point: [Coordinates; 4],
    pub tyre_contact_normal: [Coordinates; 4],
    pub tyre_contact_heading: [Coordinates; 4],
    pub brake_bias: f32,
    pub local_velocity: [f32; 3],
    // ACC specific fields
    pub p2p_activation: i32,
    pub p2p_status: i32,
    pub current_max_rpm: f32,
    pub mz: [f32; 4],
    pub fx: [f32; 4],
    pub fy: [f32; 4],
    pub slip_ratio: [f32; 4],
    pub slip_angle: [f32; 4],
    pub tc_in_action: i32,
    pub abs_in_action: i32,
    pub suspension_damage: [f32; 4],
    pub tyre_temp: [f32; 4],
    pub water_temp: f32,
    pub brake_pressure: [f32; 4],
    pub front_brake_compound: i32,
    pub rear_brake_compound: i32,
    pub pad_life: [f32; 4],
    pub disc_life: [f32; 4],
    pub ignition_on: i32,
    pub starter_engine_on: i32,
    pub is_engine_running: i32,
    pub kerb_vibration: f32,
    pub slip_vibrations: f32,
    pub g_vibrations: f32,
    pub abs_vibrations: f32,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Graphics {
    pub packet_id: i32,
    pub status: AccStatus,
    pub session: AccSessionType,
    pub current_time: [u16; 15], // UTF-16 string
    pub last_time: [u16; 15],    // UTF-16 string
    pub best_time: [u16; 15],    // UTF-16 string
    pub split: [u16; 15],        // UTF-16 string
    pub completed_laps: i32,
    pub position: i32,
    pub i_current_time: i32,
    pub i_last_time: i32,
    pub i_best_time: i32,
    pub session_time_left: f32,
    pub distance_traveled: f32,
    pub is_in_pit: i32,
    pub current_sector_index: i32,
    pub last_sector_time: i32,
    pub number_of_laps: i32,
    pub tyre_compound: [u16; 34], // UTF-16 string
    pub replay_time_multiplier: f32,
    pub normalized_car_position: f32,
    pub active_cars: i32,
    pub car_coordinates: [[f32; 3]; 60],
    pub car_id: [i32; 60],
    pub player_car_id: i32,
    pub penalty_time: f32,
    pub flag: AccFlagType,
    pub penalty: AccPenaltyType,
    pub ideal_line_on: i32,
    pub is_in_pit_lane: i32,
    pub surface_grip: f32,
    pub mandatory_pit_done: i32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub is_setup_menu_visible: i32,
    pub main_display_index: i32,
    pub secondary_display_index: i32,
    pub tc: i32,
    pub tc_cut: i32,
    pub engine_map: i32,
    pub abs: i32,
    pub fuel_x_lap: f32,
    pub rain_lights: i32,
    pub flashing_lights: i32,
    pub lights_stage: i32,
    pub exhaust_temperature: f32,
    pub wiper_lv: i32,
    pub driver_stint_total_time_left: i32,
    pub driver_stint_time_left: i32,
    pub rain_tyres: i32,
    pub session_index: i32,
    pub used_fuel: f32,
    pub delta_lap_time: [u16; 16], // UTF-16 string
    pub i_delta_lap_time: i32,
    pub estimated_lap_time: [u16; 16], // UTF-16 string
    pub i_estimated_lap_time: i32,
    pub is_delta_positive: i32,
    pub i_split: i32,
    pub is_valid_lap: i32,
    pub fuel_estimated_laps: f32,
    pub track_status: [u16; 34], // UTF-16 string
    pub missing_mandatory_pits: i32,
    pub clock: f32,
    pub direction_lights_left: i32,
    pub direction_lights_right: i32,
    pub global_yellow: i32,
    pub global_yellow1: i32,
    pub global_yellow2: i32,
    pub global_yellow3: i32,
    pub global_white: i32,
    pub global_green: i32,
    pub global_chequered: i32,
    pub global_red: i32,
    pub mfd_tyre_set: i32,
    pub mfd_fuel_to_add: f32,
    pub mfd_tyre_pressure_lf: f32,
    pub mfd_tyre_pressure_rf: f32,
    pub mfd_tyre_pressure_lr: f32,
    pub mfd_tyre_pressure_rr: f32,
    pub track_grip_status: AccTrackGripStatus,
    pub rain_intensity: AccRainIntensity,
    pub rain_intensity_in_10min: AccRainIntensity,
    pub rain_intensity_in_30min: AccRainIntensity,
    pub current_tyre_set: i32,
    pub strategy_tyre_set: i32,
}

impl Graphics {
    pub fn current_time_str(&self) -> String {
        utf16_to_string(&self.current_time)
    }

    pub fn last_time_str(&self) -> String {
        utf16_to_string(&self.last_time)
    }

    pub fn best_time_str(&self) -> String {
        utf16_to_string(&self.best_time)
    }

    pub fn split_str(&self) -> String {
        utf16_to_string(&self.split)
    }

    pub fn tyre_compound_str(&self) -> String {
        utf16_to_string(&self.tyre_compound)
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct StaticInfo {
    pub sm_version: [u16; 15],  // UTF-16 string
    pub acc_version: [u16; 15], // UTF-16 string
    pub number_of_sessions: i32,
    pub num_cars: i32,
    pub car_model: [u16; 33],      // UTF-16 string
    pub track: [u16; 33],          // UTF-16 string
    pub player_name: [u16; 33],    // UTF-16 string
    pub player_surname: [u16; 33], // UTF-16 string
    pub player_nick: [u16; 34],    // UTF-16 string
    pub sector_count: i32,
    pub max_torque: f32,
    pub max_power: f32,
    pub max_rpm: i32,
    pub max_fuel: f32,
    pub suspension_max_travel: [f32; 4],
    pub tyre_radius: [f32; 4],
    pub max_turbo_boost: f32,
    pub deprecated1: f32, // AirTemp since 1.6 in physics
    pub deprecated2: f32, // RoadTemp since 1.6 in physics
    pub penalties_enabled: i32,
    pub aid_fuel_rate: f32,
    pub aid_tire_rate: f32,
    pub aid_mechanical_damage: f32,
    pub aid_allow_tyre_blankets: i32,
    pub aid_stability: i32,
    pub aid_auto_clutch: i32,
    pub aid_auto_blip: i32,
    pub has_drs: i32,
    pub has_ers: i32,
    pub has_kers: i32,
    pub kers_max_joules: f32,
    pub engine_brake_settings_count: i32,
    pub ers_power_controller_count: i32,
    pub track_spline_length: f32,
    pub track_configuration: [u16; 34], // UTF-16 string
    pub ers_max_j: f32,
    pub is_timed_race: i32,
    pub has_extra_lap: i32,
    pub car_skin: [u16; 34], // UTF-16 string
    pub reversed_grid_positions: i32,
    pub pit_window_start: i32,
    pub pit_window_end: i32,
    pub is_online: i32,
    pub dry_tyres_name: [u16; 33], // UTF-16 string
    pub wet_tyres_name: [u16; 33], // UTF-16 string
}

impl StaticInfo {
    pub fn sm_version_str(&self) -> String {
        utf16_to_string(&self.sm_version)
    }

    pub fn acc_version_str(&self) -> String {
        utf16_to_string(&self.acc_version)
    }

    pub fn car_model_str(&self) -> String {
        utf16_to_string(&self.car_model)
    }

    pub fn track_str(&self) -> String {
        utf16_to_string(&self.track)
    }

    pub fn player_name_str(&self) -> String {
        utf16_to_string(&self.player_name)
    }

    pub fn player_surname_str(&self) -> String {
        utf16_to_string(&self.player_surname)
    }

    pub fn player_nick_str(&self) -> String {
        utf16_to_string(&self.player_nick)
    }

    pub fn track_configuration_str(&self) -> String {
        utf16_to_string(&self.track_configuration)
    }

    pub fn car_skin_str(&self) -> String {
        utf16_to_string(&self.car_skin)
    }
}

pub fn utf16_to_string(utf16: &[u16]) -> String {
    let end = utf16.iter().position(|&c| c == 0).unwrap_or(utf16.len());
    String::from_utf16_lossy(&utf16[..end])
}

// Helper functions for creating default instances
impl Default for Physics {
    fn default() -> Self {
        Self {
            packet_id: 0,
            gas: 0.0,
            brake: 0.0,
            fuel: 0.0,
            gear: 0,
            rpms: 0,
            steer_angle: 0.0,
            speed_kmh: 0.0,
            velocity: [0.0; 3],
            acc_g: [0.0; 3],
            wheel_slip: [0.0; 4],
            wheel_load: [0.0; 4],
            wheels_pressure: [0.0; 4],
            wheel_angular_speed: [0.0; 4],
            tyre_wear: [0.0; 4],
            tyre_dirty_level: [0.0; 4],
            tyre_core_temperature: [0.0; 4],
            camber_rad: [0.0; 4],
            suspension_travel: [0.0; 4],
            drs: 0.0,
            tc: 0.0,
            heading: 0.0,
            pitch: 0.0,
            roll: 0.0,
            cg_height: 0.0,
            car_damage: [0.0; 5],
            number_of_tyres_out: 0,
            pit_limiter_on: 0,
            abs: 0.0,
            kers_charge: 0.0,
            kers_input: 0.0,
            auto_shifter_on: 0,
            ride_height: [0.0; 2],
            turbo_boost: 0.0,
            ballast: 0.0,
            air_density: 0.0,
            air_temp: 0.0,
            road_temp: 0.0,
            local_angular_velocity: [0.0; 3],
            final_ff: 0.0,
            performance_meter: 0.0,
            engine_brake: 0,
            ers_recovery_level: 0,
            ers_power_level: 0,
            ers_heat_charging: 0,
            ers_is_charging: 0,
            kers_current_kj: 0.0,
            drs_available: 0,
            drs_enabled: 0,
            brake_temp: [0.0; 4],
            clutch: 0.0,
            tyre_temp_i: [0.0; 4],
            tyre_temp_m: [0.0; 4],
            tyre_temp_o: [0.0; 4],
            is_ai_controlled: 0,
            tyre_contact_point: [Coordinates { x: 0.0, y: 0.0, z: 0.0 }; 4],
            tyre_contact_normal: [Coordinates { x: 0.0, y: 0.0, z: 0.0 }; 4],
            tyre_contact_heading: [Coordinates { x: 0.0, y: 0.0, z: 0.0 }; 4],
            brake_bias: 0.0,
            local_velocity: [0.0; 3],
            p2p_activation: 0,
            p2p_status: 0,
            current_max_rpm: 0.0,
            mz: [0.0; 4],
            fx: [0.0; 4],
            fy: [0.0; 4],
            slip_ratio: [0.0; 4],
            slip_angle: [0.0; 4],
            tc_in_action: 0,
            abs_in_action: 0,
            suspension_damage: [0.0; 4],
            tyre_temp: [0.0; 4],
            water_temp: 0.0,
            brake_pressure: [0.0; 4],
            front_brake_compound: 0,
            rear_brake_compound: 0,
            pad_life: [0.0; 4],
            disc_life: [0.0; 4],
            ignition_on: 0,
            starter_engine_on: 0,
            is_engine_running: 0,
            kerb_vibration: 0.0,
            slip_vibrations: 0.0,
            g_vibrations: 0.0,
            abs_vibrations: 0.0,
        }
    }
}

impl Default for Graphics {
    fn default() -> Self {
        Self {
            packet_id: 0,
            status: AccStatus::Off,
            session: AccSessionType::Unknown,
            current_time: [0; 15],
            last_time: [0; 15],
            best_time: [0; 15],
            split: [0; 15],
            completed_laps: 0,
            position: 0,
            i_current_time: 0,
            i_last_time: 0,
            i_best_time: 0,
            session_time_left: 0.0,
            distance_traveled: 0.0,
            is_in_pit: 0,
            current_sector_index: 0,
            last_sector_time: 0,
            number_of_laps: 0,
            tyre_compound: [0; 34],
            replay_time_multiplier: 0.0,
            normalized_car_position: 0.0,
            active_cars: 0,
            car_coordinates: [[0.0; 3]; 60],
            car_id: [0; 60],
            player_car_id: 0,
            penalty_time: 0.0,
            flag: AccFlagType::NoFlag,
            penalty: AccPenaltyType::None,
            ideal_line_on: 0,
            is_in_pit_lane: 0,
            surface_grip: 0.0,
            mandatory_pit_done: 0,
            wind_speed: 0.0,
            wind_direction: 0.0,
            is_setup_menu_visible: 0,
            main_display_index: 0,
            secondary_display_index: 0,
            tc: 0,
            tc_cut: 0,
            engine_map: 0,
            abs: 0,
            fuel_x_lap: 0.0,
            rain_lights: 0,
            flashing_lights: 0,
            lights_stage: 0,
            exhaust_temperature: 0.0,
            wiper_lv: 0,
            driver_stint_total_time_left: 0,
            driver_stint_time_left: 0,
            rain_tyres: 0,
            session_index: 0,
            used_fuel: 0.0,
            delta_lap_time: [0; 16],
            i_delta_lap_time: 0,
            estimated_lap_time: [0; 16],
            i_estimated_lap_time: 0,
            is_delta_positive: 0,
            i_split: 0,
            is_valid_lap: 0,
            fuel_estimated_laps: 0.0,
            track_status: [0; 34],
            missing_mandatory_pits: 0,
            clock: 0.0,
            direction_lights_left: 0,
            direction_lights_right: 0,
            global_yellow: 0,
            global_yellow1: 0,
            global_yellow2: 0,
            global_yellow3: 0,
            global_white: 0,
            global_green: 0,
            global_chequered: 0,
            global_red: 0,
            mfd_tyre_set: 0,
            mfd_fuel_to_add: 0.0,
            mfd_tyre_pressure_lf: 0.0,
            mfd_tyre_pressure_rf: 0.0,
            mfd_tyre_pressure_lr: 0.0,
            mfd_tyre_pressure_rr: 0.0,
            track_grip_status: AccTrackGripStatus::Green,
            rain_intensity: AccRainIntensity::NoRain,
            rain_intensity_in_10min: AccRainIntensity::NoRain,
            rain_intensity_in_30min: AccRainIntensity::NoRain,
            current_tyre_set: 0,
            strategy_tyre_set: 0,
        }
    }
}

impl Default for StaticInfo {
    fn default() -> Self {
        Self {
            sm_version: [0; 15],
            acc_version: [0; 15],
            number_of_sessions: 0,
            num_cars: 0,
            car_model: [0; 33],
            track: [0; 33],
            player_name: [0; 33],
            player_surname: [0; 33],
            player_nick: [0; 34],
            sector_count: 0,
            max_torque: 0.0,
            max_power: 0.0,
            max_rpm: 0,
            max_fuel: 0.0,
            suspension_max_travel: [0.0; 4],
            tyre_radius: [0.0; 4],
            max_turbo_boost: 0.0,
            deprecated1: 0.0,
            deprecated2: 0.0,
            penalties_enabled: 0,
            aid_fuel_rate: 0.0,
            aid_tire_rate: 0.0,
            aid_mechanical_damage: 0.0,
            aid_allow_tyre_blankets: 0,
            aid_stability: 0,
            aid_auto_clutch: 0,
            aid_auto_blip: 0,
            has_drs: 0,
            has_ers: 0,
            has_kers: 0,
            kers_max_joules: 0.0,
            engine_brake_settings_count: 0,
            ers_power_controller_count: 0,
            track_spline_length: 0.0,
            track_configuration: [0; 34],
            ers_max_j: 0.0,
            is_timed_race: 0,
            has_extra_lap: 0,
            car_skin: [0; 34],
            reversed_grid_positions: 0,
            pit_window_start: 0,
            pit_window_end: 0,
            is_online: 0,
            dry_tyres_name: [0; 33],
            wet_tyres_name: [0; 33],
        }
    }
}
