// src-tauri/src/macos_m1_visual_effect.rs

extern crate objc;
extern crate cocoa;

use cocoa::appkit::{NSWindow, NSWindowOrderingMode, NSVisualEffectState, NSVisualEffectMaterial, NSVisualEffectBlendingMode, NSWindowTitleVisibility};
use cocoa::base::{nil, YES};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

pub fn apply_visual_effect(window: tauri::Window) {
    unsafe {
        // Get the underlying NSWindow object from the Tauri window
        let ns_window: *mut Object = window.ns_window().unwrap() as *mut Object;
        let _: () = msg_send![ns_window, setTitlebarAppearsTransparent: YES];
        let _: () = msg_send![ns_window, setTitleVisibility: NSWindowTitleVisibility::NSWindowTitleHidden];

        // Create an NSVisualEffectView instance
        let ve_class = class!(NSVisualEffectView);
        let visual_effect_view: *mut Object = msg_send![ve_class, alloc];
        let _: *mut Object = msg_send![visual_effect_view, initWithFrame:NSWindow::frame(ns_window)];

        // Configure the NSVisualEffectView
        let _: () = msg_send![visual_effect_view, setMaterial: NSVisualEffectMaterial::Titlebar];
        let _: () = msg_send![visual_effect_view, setBlendingMode: NSVisualEffectBlendingMode::BehindWindow];
        let _: () = msg_send![visual_effect_view, setState: NSVisualEffectState::Active];

        // Get the content view of the NSWindow
        let content_view: *mut Object = msg_send![ns_window, contentView];

        // Add the NSVisualEffectView as a subview to the NSWindow's content view
        let _: () = msg_send![content_view, addSubview: visual_effect_view positioned: NSWindowOrderingMode::NSWindowBelow relativeTo: nil];
    }
}

