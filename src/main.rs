// Support building in lilium userspace
#![cfg_attr(lilium_ministd, no_std, no_main)]

#[cfg(lilium_ministd)]
extern crate ministd as std;

#[cfg(lilium_ministd)]
use std::{eprintln, print, println};
#[cfg(lilium_ministd)]
std::def_main!();

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct UnameOption : u32 {
        const KERNEL_NAME = 0x01;
        const NODENAME = 0x02;
        const KERNEL_RELEASE = 0x04;
        const KERNEL_VERSION = 0x08;
        const MACHINE = 0x10;
        const PROCESSOR = 0x20;
        const HARDWARE_PLATFORM = 0x40;
        const OPERATING_SYSTEM = 0x80;
        const GUESS = 0x100;
    }
}

fn main() {
    let uname = sysname::uname().unwrap();

    let mut args = std::env::args();

    let prg_name = args.next().unwrap();

    let mut options = UnameOption::empty();

    while let Some(arg) = args.next() {
        let arg = &*arg;

        match arg {
            "--all" => options |= UnameOption::all() & !UnameOption::GUESS,
            "--kernel-name" => options |= UnameOption::KERNEL_NAME,
            "--nodename" => options |= UnameOption::NODENAME,
            "--kernel-release" => options |= UnameOption::KERNEL_RELEASE,
            "--kernel-version" => options |= UnameOption::KERNEL_VERSION,
            "--machine" => options |= UnameOption::MACHINE,
            "--processor" => options |= UnameOption::PROCESSOR,
            "--hardware-platform" => options |= UnameOption::HARDWARE_PLATFORM,
            "--operating-system" => options |= UnameOption::OPERATING_SYSTEM,
            // #[cfg(feature = "guess")]
            // "--guess" => options |= UnameOption::GUESS,
            "--help" => {
                println!("Usage: {prg_name} [OPTIONS..]");
                println!("Prints system name information with cross-platform support");
                println!("Options:");
                println!("\t--all, -a: Prints all of the following information in that order");
                println!("\t--kernel-name, -s: Prints the Kernel Name");
                println!("\t--nodename, -n: Prints the network node hostname");
                println!("\t--kernel-release, -r: Prints the Kernel Release");
                println!("\t--kernel-version, -v: Prints Kernel Version Information");
                println!("\t--machine, -m: Prints the current machine");
                println!("\t--processor, -p: Prints the host processor (non-portable)");
                println!("\t--hardware-platform, -i: Prints the hardware platform (non-portable)");
                println!("\t--operating-system, -o: Prints the operating system name");
                // #[cfg(feature = "guess")]
                // println!("\t--guess: Prints the target tuple (non-portable)");
                println!("\t--help: Prints this message and exits");
                println!("\t--version: Prints version information and exits");
                println!("Notes:");
                // #[cfg(feature = "guess")]
                // println!(
                //     "\tIf this program is invoked with the name config.guess (or a name that ends in config.guess), --guess is implied."
                // );
                println!(
                    "\tIf this program is invoked without any options, --kernel-name is implied"
                );

                std::process::exit(0)
            }
            "--version" => {
                println!("sysname v{}", core::env!("CARGO_PKG_VERSION"));
                println!("Copyright (C) 2026 Connor Horman");
                println!("This program is dual licensed under the MIT and Apache 2.0 Licenses.");
                println!(
                    "You may copy, distribute, modify, or otherwise use this program under the terms of either license (at your option)"
                );

                std::process::exit(0)
            }
            x if x.starts_with("--") => {
                eprintln!("{prg_name}: Unknown Option: {x}");
                std::process::exit(1)
            }
            x if x.starts_with("-") => {
                for c in x.chars().skip(1) {
                    match c {
                        'a' => options |= UnameOption::all() & !UnameOption::GUESS,
                        's' => options |= UnameOption::KERNEL_NAME,
                        'n' => options |= UnameOption::NODENAME,
                        'r' => options |= UnameOption::KERNEL_RELEASE,
                        'v' => options |= UnameOption::KERNEL_VERSION,
                        'm' => options |= UnameOption::MACHINE,
                        'p' => options |= UnameOption::PROCESSOR,
                        'i' => options |= UnameOption::HARDWARE_PLATFORM,
                        'o' => options |= UnameOption::OPERATING_SYSTEM,
                        v => {
                            eprintln!("{prg_name}: Unknown short option -{v}");
                            std::process::exit(1)
                        }
                    }
                }
            }
            _ => {
                eprintln!("{prg_name}: Usage: {prg_name} [OPTIONS...]");
                std::process::exit(1)
            }
        }
    }

    // if prg_name.ends_with("config.guess") || prg_name.ends_with("config.guess.exe") {
    //     options |= UnameOption::GUESS
    // }

    if options.is_empty() {
        options = UnameOption::KERNEL_NAME;
    }

    let mut sep = "";

    for (name, _) in options.iter_names() {
        match name {
            "KERNEL_NAME" => print!("{sep}{}", uname.kernel_name),
            "NODENAME" => print!("{sep}{}", uname.nodename),
            "KERNEL_RELEASE" => print!("{sep}{}", uname.kernel_release),
            "KERNEL_VERSION" => print!("{sep}{}", uname.kernel_version),
            "MACHINE" => print!("{sep}{}", uname.machine),
            "PROCESSOR" => print!("{sep}{}", uname.processor),
            "HARDWARE_PLATFORM" => print!("{sep}{}", uname.hardware_platform),
            "OPERATING_SYSTEM" => print!("{sep}{}", uname.sysname),
            // #[cfg(feature = "guess")]
            // "GUESS" => todo!("Implement config.guess"),
            x => todo!("Flag {x}"),
        }
        sep = " ";
    }

    println!();
}
