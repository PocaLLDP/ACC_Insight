use std::ffi::c_void;
use std::ptr::null;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Memory::{
    MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ,
};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
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
    pub tyre_compound: [u16; 33],
    pub replay_time_multiplier: f32,
    pub normalized_car_position: f32,
    pub active_cars: i32,
    pub car_coordinates: [[f32; 3]; 60],
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

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SPageFileStatic {
    pub sm_version: [u16; 15],
    pub ac_version: [u16; 15],
    pub number_of_sessions: i32,
    pub num_cars: i32,
    pub car_model: [u16; 33],
    pub track: [u16; 33],
    pub player_name: [u16; 33],
    pub player_surname: [u16; 33],
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
    pub car_skin: [u16; 33],
    pub reversed_grid_positions: i32,
    pub pit_window_start: i32,
    pub pit_window_end: i32,
    pub is_online: i32,
    pub dry_tyres_name: [u16; 33],
    pub wet_tyres_name: [u16; 33],
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
