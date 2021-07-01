fn main() {
    println!("target_arch: {}", target::arch());
    println!("target_os: {}", target::os());
    println!("target_family: {}", target::os_family());
    println!("target_env: {}", target::env());
    println!("target_endian: {}", target::endian());
    println!("target_pointer_width: {}", target::pointer_width());
    println!("target_vendor: {}", target::vendor());
    println!("target_feature: {:?}", target::features());
}
