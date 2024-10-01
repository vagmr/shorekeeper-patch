use windows::core::{PCSTR, s};

pub(crate) struct InjectConfiguration {
    pub(crate) f_pak_file_check: usize,
    pub(crate) kuro_http_get: usize,
    pub(crate) sdk_dll: PCSTR,
    pub(crate) eula_accept: usize,
    pub(crate) sdk_go_away: usize,
}

#[cfg(feature = "cn_live_1_3_0")]
pub(crate) const CN_BETA_1_3_0_CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3D35DF0,
    kuro_http_get: 0xFC9900,
    sdk_dll: s!("KRSDKEx.dll"),
    eula_accept: 0x4A690,
    sdk_go_away: 0x8B9F0
};

#[cfg(feature = "os_live_1_3_0")]
pub(crate) const OS_LIVE_1_3_0_CONFIG: InjectConfiguration = InjectConfiguration {
    f_pak_file_check: 0x3CDC430,
    kuro_http_get: 0xFC6C20,
    sdk_dll: s!("KRSDK.dll"),
    eula_accept: 0x94710,
    sdk_go_away: 0x9FE10
};