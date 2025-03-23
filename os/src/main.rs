#![no_std]
#![no_main]

mod langitem;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
