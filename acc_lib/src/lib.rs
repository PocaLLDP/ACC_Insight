use std::ffi::c_void;
use std::ptr::null;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Memory::{
    MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ,
};
use bincode::{deserialize};
use serde::{Serialize, Deserialize};
use serde_big_array::BigArray;
use std::error::Error;


// Constants
pub const ACC_NO_FLAG: i32 = 0;
pub const ACC_BLUE_FLAG: i32 = 1;
pub const ACC_YELLOW_FLAG: i32 = 2;
pub const ACC_BLACK_FLAG: i32 = 3;
pub const ACC_WHITE_FLAG: i32 = 4;
pub const ACC_CHECKERED_FLAG: i32 = 5;
pub const ACC_PENALTY_FLAG: i32 = 6;

pub const ACC_NONE: i32 = 0;
pub const ACC_DRIVE_THROUGH_CUTTING: i32 = 1;
pub const ACC_STOP_AND_GO_10_CUTTING: i32 = 2;
pub const ACC_STOP_AND_GO_20_CUTTING: i32 = 3;
pub const ACC_STOP_AND_GO_30_CUTTING: i32 = 4;
pub const ACC_DISQUALIFIED_CUTTING: i32 = 5;
pub const ACC_REMOVE_BEST_LAPTIME_CUTTING: i32 = 6;
pub const ACC_DRIVE_THROUGH_PIT_SPEEDING: i32 = 7;
pub const ACC_STOP_AND_GO_10_PIT_SPEEDING: i32 = 8;
pub const ACC_STOP_AND_GO_20_PIT_SPEEDING: i32 = 9;
pub const ACC_STOP_AND_GO_30_PIT_SPEEDING: i32 = 10;
pub const ACC_DISQUALIFIED_PIT_SPEEDING: i32 = 11;
pub const ACC_REMOVE_BEST_LAPTIME_PIT_SPEEDING: i32 = 12;
pub const ACC_DISQUALIFIED_IGNORED_MANDATORY_PIT: i32 = 13;
pub const ACC_POST_RACE_TIME: i32 = 14;
pub const ACC_DISQUALIFIED_TROLLING: i32 = 15;
pub const ACC_DISQUALIFIED_PIT_ENTRY: i32 = 16;
pub const ACC_DISQUALIFIED_PIT_EXIT: i32 = 17;
pub const ACC_DISQUALIFIED_WRONGWAY: i32 = 18;
pub const ACC_DRIVE_THROUGH_IGNORED_DRIVER_STINT: i32 = 19;
pub const ACC_DISQUALIFIED_IGNORED_DRIVER_STINT: i32 = 20;
pub const ACC_DISQUALIFIED_EXCEEDED_DRIVER_STINT_LIMIT: i32 = 21;

pub const ACC_UNKNOWN: i32 = -1;
pub const ACC_PRACTICE: i32 = 0;
pub const ACC_QUALIFY: i32 = 1;
pub const ACC_RACE: i32 = 2;
pub const ACC_HOTLAP: i32 = 3;
pub const ACC_TIME_ATTACK: i32 = 4;
pub const ACC_DRIFT: i32 = 5;
pub const ACC_DRAG: i32 = 6;
pub const ACC_HOTSTINT: i32 = 7;
pub const ACC_HOTLAPSUPERPOLE: i32 = 8;

pub const ACC_OFF: i32 = 0;
pub const ACC_REPLAY: i32 = 1;
pub const ACC_LIVE: i32 = 2;
pub const ACC_PAUSE: i32 = 3;

pub const ACC_FRONT_LEFT: i32 = 0;
pub const ACC_FRONT_RIGHT: i32 = 1;
pub const ACC_REAR_LEFT: i32 = 2;
pub const ACC_REAR_RIGHT: i32 = 3;

pub const ACC_GREEN: i32 = 0;
pub const ACC_FAST: i32 = 1;
pub const ACC_OPTIMUM: i32 = 2;
pub const ACC_GREASY: i32 = 3;
pub const ACC_DAMP: i32 = 4;
pub const ACC_WET: i32 = 5;
pub const ACC_FLOODED: i32 = 6;

pub const ACC_NO_RAIN: i32 = 0;
pub const ACC_DRIZZLE: i32 = 1;
pub const ACC_LIGHT_RAIN: i32 = 2;
pub const ACC_MEDIUM_RAIN: i32 = 3;
pub const ACC_HEAVY_RAIN: i32 = 4;
pub const ACC_THUNDERSTORM: i32 = 5;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SPageFilePhysics {
    pub packet_id: i32,
    pub gas: f32,
    pub brake: f32,
    pub fuel: f32,
    pub gear: i32,
    pub rpm: i32,
    pub steer_angle: f32,
    pub speed_kmh: f32,
    pub velocity: [f32; 3],
    pub acc_g: [f32; 3],
    pub wheel_slip: [f32; 4],
    pub wheel_load: [f32; 4],
    pub wheel_pressure: [f32; 4],
    pub wheel_angular_speed: [f32; 4],
    pub tyre_wear: [f32; 4],
    pub tyre_dirty_level: [f32; 4],
    pub tyre_core_temp: [f32; 4],
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
    pub autoshifter_on: i32,
    pub ride_height: [f32; 2],
    pub turbo_boost: f32,
    pub ballast: f32,
    pub air_density: f32,
    pub air_temp: f32,
    pub road_temp: f32,
    pub local_angular_vel: [f32; 3],
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
    pub tyre_contact_point: [[f32; 3]; 4],
    pub tyre_contact_normal: [[f32; 3]; 4],
    pub tyre_contact_heading: [[f32; 3]; 4],
    pub brake_bias: f32,
    pub local_velocity: [f32; 3],
    pub p2p_activation: i32,
    pub p2p_status: i32,
    pub current_max_rpm: f32,
    pub mz: [f32; 4],
    pub fx: [f32; 4],
    pub fy: [f32; 4],
    pub slip_ratio: [f32; 4],
    pub slip_angle: [f32; 4],
    pub tcin_action: i32,
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

impl Default for SPageFilePhysics {
    fn default() -> Self {
        SPageFilePhysics{
            packet_id: 0,
            gas: 0.0,
            brake: 0.0,
            fuel: 0.0,
            gear: 0,
            rpm: 0,
            steer_angle: 0.0,
            speed_kmh: 0.0,
            velocity: [0f32; 3],
            acc_g: [0f32; 3],
            wheel_slip: [0f32; 4],
            wheel_load: [0f32; 4],
            wheel_pressure: [0f32; 4],
            wheel_angular_speed: [0f32; 4],
            tyre_wear: [0f32; 4],
            tyre_dirty_level: [0f32; 4],
            tyre_core_temp: [0f32; 4],
            camber_rad: [0f32; 4],
            suspension_travel: [0f32; 4],
            drs: 0.0,
            tc: 0.0,
            heading: 0.0,
            pitch: 0.0,
            roll: 0.0,
            cg_height: 0.0,
            car_damage: [0f32; 5],
            number_of_tyres_out: 0,
            pit_limiter_on: 0,
            abs: 0.0,
            kers_charge: 0.0,
            kers_input: 0.0,
            autoshifter_on: 0,
            ride_height: [0f32; 2],
            turbo_boost: 0.0,
            ballast: 0.0,
            air_density: 0.0,
            air_temp: 0.0,
            road_temp: 0.0,
            local_angular_vel: [0f32; 3],
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
            brake_temp: [0f32; 4],
            clutch: 0.0,
            tyre_temp_i: [0f32; 4],
            tyre_temp_m: [0f32; 4],
            tyre_temp_o: [0f32; 4],
            is_ai_controlled: 0,
            tyre_contact_point: [[0f32; 3]; 4],
            tyre_contact_normal: [[0f32; 3]; 4],
            tyre_contact_heading: [[0f32; 3]; 4],
            brake_bias: 0.0,
            local_velocity: [0f32; 3],
            p2p_activation: 0,
            p2p_status: 0,
            current_max_rpm: 0.0,
            mz: [0f32; 4],
            fx: [0f32; 4],
            fy: [0f32; 4],
            slip_ratio: [0f32; 4],
            slip_angle: [0f32; 4],
            tcin_action: 0,
            abs_in_action: 0,
            suspension_damage: [0f32; 4],
            tyre_temp: [0f32; 4],
            water_temp: 0.0,
            brake_pressure: [0f32; 4],
            front_brake_compound: 0,
            rear_brake_compound: 0,
            pad_life: [0f32; 4],
            disc_life: [0f32; 4],
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

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SPageFileGraphics {
    pub packet_id: i32,
    pub status: i32,
    pub session: i32,
    pub current_time: [u16; 15],
    pub last_time: [u16; 15],
    pub best_time: [u16; 15],
    pub split: [u16; 15],
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
    #[serde(with = "BigArray")]
    pub tyre_compound: [u16; 33],
    pub replay_time_multiplier: f32,
    pub normalized_car_position: f32,
    pub active_cars: i32,
    #[serde(with = "BigArray")]
    pub car_coordinates: [[f32; 3]; 60],
    #[serde(with = "BigArray")]
    pub car_id: [i32; 60],
    pub player_car_id: i32,
    pub penalty_time: f32,
    pub flag: i32,
    pub penalty: i32,
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
    pub tccut: i32,
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
    pub delta_lap_time: [u16; 15],
    pub i_delta_lap_time: i32,
    pub estimated_lap_time: [u16; 15],
    pub i_estimated_lap_time: i32,
    pub is_delta_positive: i32,
    pub i_split: i32,
    pub is_valid_lap: i32,
    pub fuel_estimated_laps: f32,
    #[serde(with = "BigArray")]
    pub track_status: [u16; 33],
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
    pub track_grip_status: i32,
    pub rain_intensity: i32,
    pub rain_intensity_in_10min: i32,
    pub rain_intensity_in_30min: i32,
    pub current_tyre_set: i32,
    pub strategy_tyre_set: i32,
    pub gap_ahead: i32,
    pub gap_behind: i32,
}

impl Default for SPageFileGraphics {
    fn default() -> Self {
        SPageFileGraphics{
            packet_id: 0,
            status: 0,
            session: 0,
            current_time: [0u16; 15],
            last_time: [0u16; 15],
            best_time: [0u16; 15],
            split: [0u16; 15],
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
            tyre_compound: [0u16; 33],
            replay_time_multiplier: 0.0,
            normalized_car_position: 0.0,
            active_cars: 0,
            car_coordinates: [[0f32; 3]; 60],
            car_id: [0i32; 60],
            player_car_id: 0,
            penalty_time: 0.0,
            flag: 0,
            penalty: 0,
            ideal_line_on: 0,
            is_in_pit_lane: 0,
            surface_grip: 0.0,
            mandatory_pit_done: 0,
            wind_speed: 0.0,
            wind_direction: 0.0,
            is_setup_menu_visible: 0,
            main_display_index: 0,
            tc: 0.0 as i32,
            tccut: 0,
            abs: 0.0 as i32,
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
            delta_lap_time: [0u16; 15],
            i_delta_lap_time: 0,
            estimated_lap_time: [0u16; 15],
            i_estimated_lap_time: 0,
            is_delta_positive: 0,
            i_split: 0,
            is_valid_lap: 0,
            fuel_estimated_laps: 0.0,
            track_status: [0u16; 33],
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
            track_grip_status: 0,
            rain_intensity: 0,
            rain_intensity_in_10min: 0,
            rain_intensity_in_30min: 0,
            current_tyre_set: 0,
            strategy_tyre_set: 0,
            gap_ahead: 0,
            secondary_display_index: 0,
            engine_map: 0,
            gap_behind: 0,
        }
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SPageFileStatic {
    pub sm_version: [u16; 15],
    pub ac_version: [u16; 15],
    pub number_of_sessions: i32,
    pub num_cars: i32,
    #[serde(with = "BigArray")]
    pub car_model: [u16; 33],
    #[serde(with = "BigArray")]
    pub track: [u16; 33],
    #[serde(with = "BigArray")]
    pub player_name: [u16; 33],
    #[serde(with = "BigArray")]
    pub player_surname: [u16; 33],
    #[serde(with = "BigArray")]
    pub player_nick: [u16; 33],
    pub sector_count: i32,
    pub max_torque: f32,
    pub max_power: f32,
    pub max_rpm: i32,
    pub max_fuel: f32,
    pub suspension_max_travel: [f32; 4],
    pub tyre_radius: [f32; 4],
    pub max_turbo_boost: f32,
    pub penalties_enabled: i32,
    pub aid_fuel_rate: f32,
    pub aid_tire_rate: f32,
    pub aid_mechanical_damage: f32,
    pub allow_tyre_blankets: f32,
    pub aid_stability: f32,
    pub aid_autoclutch: i32,
    pub aid_auto_blip: i32,
    pub has_drs: i32,
    pub has_ers: i32,
    pub has_kers: i32,
    pub kers_max_j: f32,
    pub engine_brake_settings_count: i32,
    pub ers_power_controller_count: i32,
    pub track_spline_length: f32,
    pub track_configuration: u16,
    pub ers_max_j: f32,
    pub is_timed_race: i32,
    pub has_extra_lap: i32,
    #[serde(with = "BigArray")]
    pub car_skin: [u16; 33],
    pub reversed_grid_positions: i32,
    pub pit_window_start: i32,
    pub pit_window_end: i32,
    pub is_online: i32,
    #[serde(with = "BigArray")]
    pub dry_tyres_name: [u16; 33],
    #[serde(with = "BigArray")]
    pub wet_tyres_name: [u16; 33],
}

impl Default for SPageFileStatic {
    fn default() -> Self {
        SPageFileStatic{
            sm_version: [0u16; 15],
            ac_version: [0u16; 15],
            number_of_sessions: 0,
            num_cars: 0,
            car_model: [0u16; 33],
            track: [0u16; 33],
            player_name: [0u16; 33],
            player_surname: [0u16; 33],
            player_nick: [0u16; 33],
            sector_count: 0,
            max_torque: 0.0,
            max_power: 0.0,
            max_rpm: 0,
            max_fuel: 0.0,
            suspension_max_travel: [0f32; 4],
            tyre_radius: [0f32; 4],
            max_turbo_boost: 0.0,
            penalties_enabled: 0,
            aid_fuel_rate: 0.0,
            aid_tire_rate: 0.0,
            aid_mechanical_damage: 0.0,
            allow_tyre_blankets: 0.0,
            aid_stability: 0.0,
            aid_autoclutch: 0,
            aid_auto_blip: 0,
            has_drs: 0,
            has_ers: 0,
            has_kers: 0,
            kers_max_j: 0.0,
            engine_brake_settings_count: 0,
            ers_power_controller_count: 0,
            track_spline_length: 0.0,
            track_configuration: 0,
            ers_max_j: 0.0,
            is_timed_race: 0,
            has_extra_lap: 0,
            car_skin: [0u16; 33],
            reversed_grid_positions: 0,
            pit_window_start: 0,
            pit_window_end: 0,
            is_online: 0,
            dry_tyres_name: [0u16; 33],
            wet_tyres_name: [0u16; 33],
        }
    }
}


fn init_phys() -> *const SPageFilePhysics {
    unsafe {
        let mapping_name = HSTRING::from("Local\\acpmf_physics");
        let file_mapping_result =
            OpenFileMappingW(FILE_MAP_READ.0, false, PCWSTR(mapping_name.as_ptr()));
        match file_mapping_result {
            Ok(file_mapping) => {
                let mapped_view: *mut c_void = MapViewOfFile(file_mapping, FILE_MAP_READ, 0, 0, 0);
                let physics_data: *const SPageFilePhysics = mapped_view as *const SPageFilePhysics;
                physics_data
            }
            Err(err) => {
                println!(
                    "Erreur lors de l'ouverture du fichier de mapping : {:?}",
                    err.code()
                );
                println!("Détails : {}", GetLastError().0);
                null()
            }
        }
    }
}

fn init_graph() -> *const SPageFileGraphics {
    unsafe {
        let mapping_name = HSTRING::from("Local\\acpmf_graphics");
        let file_mapping_result =
            OpenFileMappingW(FILE_MAP_READ.0, false, PCWSTR(mapping_name.as_ptr()));
        match file_mapping_result {
            Ok(file_mapping) => {
                let mapped_view: *mut c_void = MapViewOfFile(file_mapping, FILE_MAP_READ, 0, 0, 0);
                let graphics_data: *const SPageFileGraphics =
                    mapped_view as *const SPageFileGraphics;
                graphics_data
            }
            Err(err) => {
                println!(
                    "Erreur lors de l'ouverture du fichier de mapping : {:?}",
                    err.code()
                );
                println!("Détails : {}", GetLastError().0);
                null()
            }
        }
    }
}

fn init_stat() -> *const SPageFileStatic {
    unsafe {
        let mapping_name = HSTRING::from("Local\\acpmf_static");
        let file_mapping_result =
            OpenFileMappingW(FILE_MAP_READ.0, false, PCWSTR(mapping_name.as_ptr()));
        match file_mapping_result {
            Ok(file_mapping) => {
                let mapped_view: *mut c_void = MapViewOfFile(file_mapping, FILE_MAP_READ, 0, 0, 0);
                let static_data: *const SPageFileStatic = mapped_view as *const SPageFileStatic;
                static_data
            }
            Err(err) => {
                println!(
                    "Erreur lors de l'ouverture du fichier de mapping : {:?}",
                    err.code()
                );
                println!("Détails : {}", GetLastError().0);
                null()
            }
        }
    }
}

pub fn init() -> (
    *const SPageFilePhysics,
    *const SPageFileGraphics,
    *const SPageFileStatic,
) {
    (init_phys(), init_graph(), init_stat())
}

fn dismiss_phys(phys: *const SPageFilePhysics) {
    unsafe {
        UnmapViewOfFile(phys as *const _);
    }
}

fn dismiss_graph(graph: *const SPageFileGraphics) {
    unsafe {
        UnmapViewOfFile(graph as *const _);
    }
}

fn dismiss_stat(stat: *const SPageFileStatic) {
    unsafe {
        UnmapViewOfFile(stat as *const _);
    }
}

pub fn dismiss(
    phys: *const SPageFilePhysics,
    graph: *const SPageFileGraphics,
    stat: *const SPageFileStatic,
) {
    dismiss_phys(phys);
    dismiss_graph(graph);
    dismiss_stat(stat);
}

pub fn encode_acc_struct<T: Serialize>(structure: &T) -> Result<Vec<u8>, Box<dyn Error>> {
    let encoded = bincode::serialize(&structure).unwrap();
    Ok(encoded)
}

pub fn decode_acc_struct<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T, Box<dyn Error>> {
    let decoded = deserialize(bytes)?;
    Ok(decoded)
}
