#[repr(C)]
struct PluginSpecVersion(u32);
const ATS_VERSION: PluginSpecVersion = PluginSpecVersion(0x00020000);
#[no_mangle]
extern "system" fn GetPluginVersion() -> PluginSpecVersion {
    ATS_VERSION
}
