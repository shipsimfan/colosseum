use crate::{info, logging::Logger};
use std::{arch::x86_64::__cpuid, path::PathBuf};
use time::DateTime;
use win32::{
    GetPhysicallyInstalledSystemMemory, GetProductInfo, GetSystemInfo, SYSTEM_INFO,
    ntddk::RtlGetVersion, wdm::RTL_OSVERSIONINFOEXW,
};

#[cfg(debug_assertions)]
const BUILD_TYPE: &str = "Debug";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &str = "Release";

#[cfg(target_arch = "x86_64")]
const CPU_ARCHITECTURE: &str = "x86-64";

/// Logs the metadata about the program and the system
pub(in crate::run) fn log_metadata<Game: crate::Game>(
    logger: &Logger,
    start_time: DateTime,
    game_hash: Option<&str>,
    game_build_time: Option<&str>,
) {
    // Log starting
    info!(logger, "Starting {} v{} . . .", Game::NAME, Game::VERSION);
    info!(logger, "Start Time: {}", start_time.iso8601());

    // Log working directory
    info!(
        logger,
        "Working directory: {}",
        std::env::current_dir().unwrap_or(PathBuf::new()).display()
    );

    // Log command arguments
    let mut arguments = String::new();
    let mut first = true;
    for argument in std::env::args_os() {
        if first {
            first = false;
        } else {
            arguments.push(' ');
        }
        arguments.push('"');
        arguments.push_str(&argument.to_string_lossy());
        arguments.push('"');
    }
    info!(logger, "Command line args: {}", arguments);

    // Log game info
    info!(
        logger,
        "Company: {}{}, Build Type: {}",
        Game::COMPANY,
        match game_hash {
            Some(commit) => format!(", Game Commit: #{}", commit),
            None => String::new(),
        },
        BUILD_TYPE,
    );
    if let Some(game_build_time) = game_build_time {
        info!(logger, "Game Build Time: {}", game_build_time);
    }

    // Log engine info
    info!(
        logger,
        "Engine: {} v{}, Engine Commit: #{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("COLOSSEUM_ENGINE_COMMIT")
    );
    info!(
        logger,
        "Engine Build Time: {}",
        env!("COLOSSEUM_ENGINE_BUILD_TIME")
    );

    // Log operating system info
    info!(logger, "Operating System: {}", get_os_version());

    // Log CPU
    info!(
        logger,
        "CPU Architecture: {}, CPU Model: {}, Cores: {}",
        CPU_ARCHITECTURE,
        get_cpu_model(),
        get_cpu_cores(),
    );

    // Log total memory
    let mut memory = 0;
    unsafe { GetPhysicallyInstalledSystemMemory(&mut memory) };

    info!(logger, "Installed Memory: {} MB", memory / 1024);
}

fn get_cpu_model() -> String {
    // Get the highest extended result
    let highest_extended_result = unsafe { __cpuid(0x80000000) }.eax;

    // If doesn't support extended brand, fallback to legacy
    if highest_extended_result < 0x8000004 {
        let legacy_result = unsafe { __cpuid(0) };
        let ebx = legacy_result.ebx.to_le_bytes();
        let ecx = legacy_result.ecx.to_le_bytes();
        let edx = legacy_result.edx.to_le_bytes();
        let bytes = [
            ebx[0], ebx[1], ebx[2], ebx[3], edx[0], edx[1], edx[2], edx[3], ecx[0], ecx[1], ecx[2],
            ecx[3],
        ];
        let mut length = 0;
        for byte in bytes {
            if byte == 0 {
                break;
            }

            length += 1;
        }
        return String::from_utf8_lossy(&bytes[..length]).to_string();
    }

    // Pull extended brand name
    let brand_name1 = unsafe { __cpuid(0x80000002) };
    let eax1 = brand_name1.eax.to_le_bytes();
    let ebx1 = brand_name1.ebx.to_le_bytes();
    let ecx1 = brand_name1.ecx.to_le_bytes();
    let edx1 = brand_name1.edx.to_le_bytes();

    let brand_name2 = unsafe { __cpuid(0x80000003) };
    let eax2 = brand_name2.eax.to_le_bytes();
    let ebx2 = brand_name2.ebx.to_le_bytes();
    let ecx2 = brand_name2.ecx.to_le_bytes();
    let edx2 = brand_name2.edx.to_le_bytes();

    let brand_name3 = unsafe { __cpuid(0x80000004) };
    let eax3 = brand_name3.eax.to_le_bytes();
    let ebx3 = brand_name3.ebx.to_le_bytes();
    let ecx3 = brand_name3.ecx.to_le_bytes();
    let edx3 = brand_name3.edx.to_le_bytes();

    let bytes = [
        eax1[0], eax1[1], eax1[2], eax1[3], ebx1[0], ebx1[1], ebx1[2], ebx1[3], ecx1[0], ecx1[1],
        ecx1[2], ecx1[3], edx1[0], edx1[1], edx1[2], edx1[3], eax2[0], eax2[1], eax2[2], eax2[3],
        ebx2[0], ebx2[1], ebx2[2], ebx2[3], ecx2[0], ecx2[1], ecx2[2], ecx2[3], edx2[0], edx2[1],
        edx2[2], edx2[3], eax3[0], eax3[1], eax3[2], eax3[3], ebx3[0], ebx3[1], ebx3[2], ebx3[3],
        ecx3[0], ecx3[1], ecx3[2], ecx3[3], edx3[0], edx3[1], edx3[2], edx3[3],
    ];

    let mut length = 0;
    for byte in bytes {
        if byte == 0 {
            break;
        }

        length += 1;
    }
    return String::from_utf8_lossy(&bytes[..length]).to_string();
}

fn get_cpu_cores() -> u32 {
    let mut system_info = SYSTEM_INFO::default();
    unsafe { GetSystemInfo(&mut system_info) };

    system_info.number_of_processors
}

fn get_os_version() -> String {
    let mut version_information = RTL_OSVERSIONINFOEXW::default();
    unsafe { RtlGetVersion(&mut version_information as *mut _ as _) };

    let name = get_windows_name(&version_information);

    let mut service_pack_length = 0;
    for byte in version_information.csd_version {
        if byte == 0 {
            break;
        }

        service_pack_length += 1;
    }

    let service_pack =
        String::from_utf16_lossy(&version_information.csd_version[..service_pack_length]);

    let product_info = get_windows_product_version(&version_information);

    format!(
        "{}{}{}{}{} ({}.{} Build {})",
        name,
        if product_info.len() == 0 { "" } else { " " },
        product_info,
        if service_pack.len() == 0 { "" } else { " " },
        service_pack,
        version_information.major_version,
        version_information.minor_version,
        version_information.build_number
    )
}

fn get_windows_name(info: &RTL_OSVERSIONINFOEXW) -> &'static str {
    if info.major_version == 5 {
        if info.minor_version == 0 {
            return "Windows 2000";
        }

        if info.minor_version == 1 || info.minor_version == 2 {
            return "Windows XP";
        }
    }

    if info.major_version == 6 {
        if info.minor_version == 0 {
            return "Windows Vista";
        }

        if info.minor_version == 1 {
            return "Windows 7";
        }

        if info.minor_version == 2 {
            return "Windows 8";
        }

        if info.minor_version == 3 {
            return "Windows 8.1";
        }
    }

    if info.major_version == 10 {
        if info.build_number < 22000 {
            return "Windows 10";
        } else {
            return "Windows 11";
        }
    }

    "unknown Windows"
}

fn get_windows_product_version(info: &RTL_OSVERSIONINFOEXW) -> &'static str {
    let mut product_info = 0;
    if unsafe {
        GetProductInfo(
            info.major_version,
            info.minor_version,
            info.service_pack_major as _,
            info.service_pack_minor as _,
            &mut product_info,
        )
    } == 0
    {
        panic!("TESTING");
    };

    match product_info {
        win32::PRODUCT_BUSINESS => "Business",
        win32::PRODUCT_BUSINESS_N => "Business N",
        win32::PRODUCT_CLUSTER_SERVER => "HPC Edition",
        win32::PRODUCT_CLUSTER_SERVER_V => "Server Hyper Core V",
        win32::PRODUCT_CORE => "Home",
        win32::PRODUCT_CORE_COUNTRYSPECIFIC => "Home China",
        win32::PRODUCT_CORE_N => "Home N",
        win32::PRODUCT_CORE_SINGLELANGUAGE => "Home Single Language",
        win32::PRODUCT_DATACENTER_EVALUATION_SERVER => "Server Datacenter",
        win32::PRODUCT_DATACENTER_A_SERVER_CORE => "Server Datacenter, Semi-Annual Channel",
        win32::PRODUCT_STANDARD_A_SERVER_CORE => "Server Standard, Semi-Annual Channel",
        win32::PRODUCT_DATACENTER_SERVER => "Server Datacenter",
        win32::PRODUCT_DATACENTER_SERVER_CORE => "Server Datacenter",
        win32::PRODUCT_DATACENTER_SERVER_CORE_V => "Server Datacenter without Hyper-V",
        win32::PRODUCT_DATACENTER_SERVER_V => "Server Datacenter without Hyper-V",
        win32::PRODUCT_EDUCATION => "Education",
        win32::PRODUCT_EDUCATION_N => "Education N",
        win32::PRODUCT_ENTERPRISE => "Enterprise",
        win32::PRODUCT_ENTERPRISE_E => "Enterprise E",
        win32::PRODUCT_ENTERPRISE_EVALUATION => "Enterprise Evaluation",
        win32::PRODUCT_ENTERPRISE_N => "Enterprise N",
        win32::PRODUCT_ENTERPRISE_N_EVALUATION => "Enterprise N Evaluation",
        win32::PRODUCT_ENTERPRISE_S => "Enterprise 2015 LTSB",
        win32::PRODUCT_ENTERPRISE_S_EVALUATION => "Enterprise 2015 LTSB Evaluation",
        win32::PRODUCT_ENTERPRISE_S_N => "Enterprise 2015 LTSB N",
        win32::PRODUCT_ENTERPRISE_S_N_EVALUATION => "Enterprise 2015 LTSB N Evaluation",
        win32::PRODUCT_ENTERPRISE_SERVER => "Server Enterprise",
        win32::PRODUCT_ENTERPRISE_SERVER_CORE => "Server Enterprise",
        win32::PRODUCT_ENTERPRISE_SERVER_CORE_V => "Server Enterprise without Hyper-V",
        win32::PRODUCT_ENTERPRISE_SERVER_IA64 => "Server Enterprise for Itanium-based Systems",
        win32::PRODUCT_ENTERPRISE_SERVER_V => "Server Enterprise without Hyper-V",
        win32::PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL => "Essential Server Solution Additional",
        win32::PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC => {
            "Essential Server Solution Additional SVC"
        }
        win32::PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT => "Essential Server Solution Management",
        win32::PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC => {
            "Essential Server Solution Management SVC"
        }
        win32::PRODUCT_HOME_BASIC => "Home Basic",
        win32::PRODUCT_HOME_BASIC_E => "Not supported",
        win32::PRODUCT_HOME_BASIC_N => "Home Basic N",
        win32::PRODUCT_HOME_PREMIUM => "Home Premium",
        win32::PRODUCT_HOME_PREMIUM_E => "Not supported",
        win32::PRODUCT_HOME_PREMIUM_N => "Home Premium N",
        win32::PRODUCT_HOME_PREMIUM_SERVER => "Home Server 2011",
        win32::PRODUCT_HOME_SERVER => "Storage Server 2008 R2 Essentials",
        win32::PRODUCT_HYPERV => "Microsoft Hyper-V Server",
        win32::PRODUCT_IOTENTERPRISE => "IoT Enterprise",
        win32::PRODUCT_IOTENTERPRISE_S => "IoT Enterprise LTSC",
        win32::PRODUCT_IOTUAP => "IoT Core",
        win32::PRODUCT_IOTUAPCOMMERCIAL => "IoT Core Commercial",
        win32::PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT => {
            "Essential Business Server Management Server"
        }
        win32::PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING => {
            "Essential Business Server Messaging Server"
        }
        win32::PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY => {
            "Essential Business Server Security Server"
        }
        win32::PRODUCT_MOBILE_CORE => "Mobile",
        win32::PRODUCT_MOBILE_ENTERPRISE => "Mobile Enterprise",
        win32::PRODUCT_MULTIPOINT_PREMIUM_SERVER => "MultiPoint Server Premium",
        win32::PRODUCT_MULTIPOINT_STANDARD_SERVER => "MultiPoint Server Standard ",
        win32::PRODUCT_PPI_PRO => "Team",
        win32::PRODUCT_PRO_FOR_EDUCATION => "Pro Education",
        win32::PRODUCT_PRO_WORKSTATION => "Pro for Workstations",
        win32::PRODUCT_PRO_WORKSTATION_N => "Pro for Workstations N",
        win32::PRODUCT_PROFESSIONAL => "Pro",
        win32::PRODUCT_PROFESSIONAL_E => "Not supported",
        win32::PRODUCT_PROFESSIONAL_N => "Pro N",
        win32::PRODUCT_PROFESSIONAL_WMC => "Professional with Media Center",
        win32::PRODUCT_SB_SOLUTION_SERVER => "Small Business Server 2011 Essentials",
        win32::PRODUCT_SB_SOLUTION_SERVER_EM => "Server For SB Solutions EM",
        win32::PRODUCT_SERVER_FOR_SB_SOLUTIONS => "Server For SB Solutions",
        win32::PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM => "Server For SB Solutions EM",
        win32::PRODUCT_SERVER_FOR_SMALLBUSINESS => {
            "Server 2008 for Windows Essential Server Solutions"
        }
        win32::PRODUCT_SERVER_FOR_SMALLBUSINESS_V => {
            "Server 2008 without Hyper-V for Windows Essential Server Solutions"
        }
        win32::PRODUCT_SERVER_FOUNDATION => "Server Foundation",
        win32::PRODUCT_SERVERRDSH => "Enterprise for Virtual Desktops",
        win32::PRODUCT_SMALLBUSINESS_SERVER => "Small Business Server",
        win32::PRODUCT_SMALLBUSINESS_SERVER_PREMIUM => "Small Business Server Premium",
        win32::PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE => "Small Business Server Premium",
        win32::PRODUCT_SOLUTION_EMBEDDEDSERVER => "MultiPoint Server",
        win32::PRODUCT_STANDARD_EVALUATION_SERVER => "Server Standard",
        win32::PRODUCT_STANDARD_SERVER => "Server Standard",
        win32::PRODUCT_STANDARD_SERVER_CORE => "Server Standard",
        win32::PRODUCT_STANDARD_SERVER_CORE_V => "Server Standard without Hyper-V",
        win32::PRODUCT_STANDARD_SERVER_V => "Server Standard without Hyper-V",
        win32::PRODUCT_STANDARD_SERVER_SOLUTIONS => "Server Solutions Premium",
        win32::PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE => "Server Solutions Premium",
        win32::PRODUCT_STARTER => "Starter",
        win32::PRODUCT_STARTER_E => "Not supported",
        win32::PRODUCT_STARTER_N => "Starter N",
        win32::PRODUCT_STORAGE_ENTERPRISE_SERVER => "Storage Server Enterprise",
        win32::PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE => "Storage Server Enterprise",
        win32::PRODUCT_STORAGE_EXPRESS_SERVER => "Storage Server Express",
        win32::PRODUCT_STORAGE_EXPRESS_SERVER_CORE => "Storage Server Express",
        win32::PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER => "Storage Server Standard",
        win32::PRODUCT_STORAGE_STANDARD_SERVER => "Storage Server Standard",
        win32::PRODUCT_STORAGE_STANDARD_SERVER_CORE => "Storage Server Standard",
        win32::PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER => "Storage Server Workgroup",
        win32::PRODUCT_STORAGE_WORKGROUP_SERVER => "Storage Server Workgroup",
        win32::PRODUCT_STORAGE_WORKGROUP_SERVER_CORE => "Storage Server Workgroup",
        win32::PRODUCT_ULTIMATE => "Ultimate",
        win32::PRODUCT_ULTIMATE_E => "Not supported",
        win32::PRODUCT_ULTIMATE_N => "Ultimate N",
        win32::PRODUCT_UNDEFINED => "An unknown product",
        win32::PRODUCT_WEB_SERVER => "Web Server",
        win32::PRODUCT_WEB_SERVER_CORE => "Web Server",
        _ => "",
    }
}
