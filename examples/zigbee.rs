use esp_idf_sys::zigbee::{
    esp_zb_platform_config_t,
    esp_zb_platform_config,
    esp_zb_ncp_config_t,
    esp_zb_host_config_t,
    esp_zb_init,
    esp_zb_cfg_s,
    esp_zb_zed_cfg_t,
    esp_zb_cfg_s__bindgen_ty_1,
};
use esp_idf_sys::esp_err_t;

fn main() {
    esp_idf_sys::link_patches();

    unsafe { 
        let mut platform_cfg: esp_zb_platform_config_t = esp_zb_platform_config_t {
            radio_config: esp_zb_ncp_config_t {
                radio_mode: 0x0,
            },
            host_config: esp_zb_host_config_t {
                host_mode: 0x02,
            },
        };
        let err1: esp_err_t = esp_zb_platform_config(&platform_cfg);

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
        let err2: esp_err_t = esp_zb_init(&mut cfg); 
    };

    println!("Complete");
}
