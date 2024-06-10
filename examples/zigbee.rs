use esp_idf_sys::esp_zb_init;

fn main() {
    esp_idf_sys::link_patches();

    unsafe { 
        esp_zb_init(&core::ptr::null); 
    };

    println!("Complete");
}