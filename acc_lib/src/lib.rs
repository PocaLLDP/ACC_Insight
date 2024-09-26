mod shared_file_out;

use std::ffi::c_void;
use std::ptr::null;
use windows::Win32::System::Memory::{
    OpenFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ
};
use windows::Win32::Foundation::{GetLastError};
use windows::core::{HSTRING, PCWSTR};
use shared_file_out::SPageFilePhysics;
use shared_file_out::SPageFileGraphics;
use shared_file_out::SPageFileStatic;


fn init_phys() -> *const SPageFilePhysics {
    unsafe {
        let mapping_name = HSTRING::from("Local\\acpmf_physics");
        let file_mapping_result = OpenFileMappingW(FILE_MAP_READ.0, false, PCWSTR(mapping_name.as_ptr()));
        match file_mapping_result {
            Ok(file_mapping) => {
                let mapped_view: *mut c_void = MapViewOfFile(file_mapping, FILE_MAP_READ, 0, 0, 0);
                let physics_data: *const SPageFilePhysics = mapped_view as *const SPageFilePhysics;
                physics_data
            }
            Err(err) => {
                println!("Erreur lors de l'ouverture du fichier de mapping : {:?}", err.code());
                println!("Détails : {}", GetLastError().0);
                null()
            }
        }
    }
}

fn init_graph() -> *const SPageFileGraphics {
    unsafe {
        let mapping_name = HSTRING::from("Local\\acpmf_graphics");
        let file_mapping_result = OpenFileMappingW(FILE_MAP_READ.0, false, PCWSTR(mapping_name.as_ptr()));
        match file_mapping_result {
            Ok(file_mapping) => {
                let mapped_view: *mut c_void = MapViewOfFile(file_mapping, FILE_MAP_READ, 0, 0, 0);
                let graphics_data: *const SPageFileGraphics = mapped_view as *const SPageFileGraphics;
                graphics_data
            }
            Err(err) => {
                println!("Erreur lors de l'ouverture du fichier de mapping : {:?}", err.code());
                println!("Détails : {}", GetLastError().0);
                null()
            }
        }
    }
}

fn init_stat() -> *const SPageFileStatic {
    unsafe {
        let mapping_name = HSTRING::from("Local\\acpmf_static");
        let file_mapping_result = OpenFileMappingW(FILE_MAP_READ.0, false, PCWSTR(mapping_name.as_ptr()));
        match file_mapping_result {
            Ok(file_mapping) => {
                let mapped_view: *mut c_void = MapViewOfFile(file_mapping, FILE_MAP_READ, 0, 0, 0);
                let static_data: *const SPageFileStatic = mapped_view as *const SPageFileStatic;
                static_data
            }
            Err(err) => {
                println!("Erreur lors de l'ouverture du fichier de mapping : {:?}", err.code());
                println!("Détails : {}", GetLastError().0);
                null()
            }
        }
    }
}

pub fn init() -> (*const SPageFilePhysics, *const SPageFileGraphics, *const SPageFileStatic) {
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

pub fn dismiss(phys: *const SPageFilePhysics, graph: *const SPageFileGraphics, stat: *const SPageFileStatic){
    dismiss_phys(phys);
    dismiss_graph(graph);
    dismiss_stat(stat);
}