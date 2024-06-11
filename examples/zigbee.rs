use esp_idf_sys::zigbee::esp_zb_init;
use esp_idf_sys::zigbee::{
    esp_zb_cfg_s,
    esp_zb_zed_cfg_t,
    esp_zb_cfg_s__bindgen_ty_1,
};

fn main() {
    esp_idf_sys::link_patches();

    unsafe { 
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
        esp_zb_init(&mut cfg); 
    };

    println!("Complete");
}
