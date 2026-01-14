fn main() {
    let cfg_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let cfg_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let os = match (&*cfg_os, &*cfg_env) {
        ("linux", env) if env.starts_with("gnu") => "GNU/Linux",
        ("linux", "musl") => "Musl/Linux",
        ("linux", env) if env.starts_with("android") => "Android",
        ("none" | "eabi" | "eabihf", _) => "None",
        ("darwin", _) => "MacOS",
        ("redox", _) => "Redox",
        ("fuschia", _) => "Fuschia",
        ("uefi", _) => "Uefi",
        ("ios", _) => "iOS",
        ("lilium", "kernel") => "Lilium Kernel",
        ("lilium", _) => "Lilium",
        (x, _) => x,
    };

    println!("cargo::rustc-env=TARGET_OS={os}");
}
