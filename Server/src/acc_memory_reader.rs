use std::ffi::c_void;
use std::ops::Bound::Unbounded;
use std::ptr::null;
use windows::Win32::System::Memory::{
    OpenFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ
};
use windows::Win32::Foundation::{GetLastError, HANDLE};
use windows::core::{HSTRING, PCWSTR};
use crate::shared_file_out::SPageFilePhysics;
use crate::shared_file_out::SPageFileGraphics;
use crate::shared_file_out::SPageFileStatic;

pub(crate) fn init_phys() -> *const SPageFilePhysics {
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


pub(crate) fn init_graph() -> *const SPageFileGraphics {
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

pub(crate) fn init_stat() -> *const SPageFileStatic {
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

pub(crate) fn dismiss_phys(phys: *const SPageFilePhysics) {
    unsafe {
        UnmapViewOfFile(phys as *const _);
    }
}

pub(crate) fn dismiss_graph(graph: *const SPageFileGraphics) {
    unsafe {
        UnmapViewOfFile(graph as *const _);
    }
}

pub(crate) fn dismiss_stat(stat: *const SPageFileStatic) {
    unsafe {
        UnmapViewOfFile(stat as *const _);
    }
}