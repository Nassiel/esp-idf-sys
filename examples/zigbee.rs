use std::default;

use esp_idf_sys::zigbee::{
    esp_zb_platform_config_t,
    esp_zb_platform_config,
    esp_zb_radio_config_t,
    esp_zb_host_config_t,
    esp_zb_uart_config_t,
    esp_zb_init,
    esp_zb_cfg_s,
    esp_zb_zed_cfg_t,
    esp_zb_cfg_s__bindgen_ty_1,
    esp_zb_radio_mode_t_ZB_RADIO_MODE_NATIVE,
    esp_zb_host_connection_mode_t_ZB_HOST_CONNECTION_MODE_NONE,
};
use esp_idf_sys::esp_err_t;

fn main() {
    esp_idf_sys::link_patches();

    unsafe { 
        let mut platform_cfg: esp_zb_platform_config_t = esp_zb_platform_config_t {
            radio_config: esp_zb_radio_config_t {
                radio_mode: esp_zb_radio_mode_t_ZB_RADIO_MODE_NATIVE,
                radio_uart_config: default(esp_zb_uart_config_t),
            },
            host_config: esp_zb_host_config_t {
                host_connection_mode: esp_zb_host_connection_mode_t_ZB_HOST_CONNECTION_MODE_NONE,
                host_uart_config: default(esp_zb_uart_config_t)
            },
        };
        let err1 = esp_zb_platform_config(&platform_cfg);

        let mut cfg: esp_zb_cfg_s = esp_zb_cfg_s{
            esp_zb_role: 0x0, 
            install_code_policy: false,
            nwk_cfg: esp_zb_cfg_s__bindgen_ty_1 {
                    zed_cfg: esp_zb_zed_cfg_t {
                    ed_timeout: 6,
                    keep_alive: 3000,
                },
            },
        };
        let err2 = esp_zb_init(&mut cfg); 
    };

    println!("Complete");
}
