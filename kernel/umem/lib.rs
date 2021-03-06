// TODO Copyright Header

#![crate_name="umem"]
#![crate_type="rlib"]
#![doc(html_logo_url = "https://avatars.io/gravatar/d0ad9c6f37bb5aceac2d7ac95ba82607?size=large",
       html_favicon_url="https://avatars.io/gravatar/d0ad9c6f37bb5aceac2d7ac95ba82607?size=small")]
#![feature(asm, concat_idents, lang_items, plugin, intrinsics, unsafe_destructor, box_syntax, core, alloc, libc)]

#![plugin(bassert)]
// TODO I should maybe rename this...
//! The Reenix User memory stuff.
///
/// It has things like the pframe

#[macro_use] #[no_link] extern crate bassert;

#[macro_use] extern crate procs;
#[macro_use] extern crate base;
#[macro_use] extern crate util;
#[macro_use] extern crate mm;
extern crate startup;
extern crate libc;

pub use pframe::pageout::{wakeup, pageoutd_run};

// TODO We should have a MaybePinnedList that uses a LRUCache under the hood...
pub mod mmobj;
pub mod pframe;
//pub mod vnode;

pub fn init_stage1() {
    pframe::init_stage1();
    procs::interrupt::register(procs::interrupt::PAGE_FAULT, handle_pagefault);
}

#[cfg(not(VM))]
extern "Rust" fn handle_pagefault(regs: &mut procs::interrupt::Registers) {
    panic!("Pagefault found! regs were {:?}", regs);
}

pub fn init_stage2() {
    pframe::init_stage2();
}

pub fn init_stage3() {
    pframe::init_stage3();
}

