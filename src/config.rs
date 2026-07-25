use std::sync::OnceLock;

pub fn remap_left_thumbrest_to_menu() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();

    *ENABLED.get_or_init(|| {
        let Some(value) = std::env::var_os("OPENVR_REMAP_THUMBREST_TO_MENU") else {
            return false;
        };

        let value_str = value.to_string_lossy();

        match value_str.trim().to_ascii_lowercase().as_str() {
            "" | "1" | "true" | "yes" | "on" => true,
            "0" | "false" | "no" | "off" => false,
            invalid => {
                log::warn!(
                    "Invalid OPENVR_REMAP_THUMBREST_TO_MENU value {invalid:?}; \
                     remap disabled"
                );
                false
            }
        }
    })
}

