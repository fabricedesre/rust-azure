/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::process::Command;
use std::env;


fn main() {
    let target = env::var("TARGET").unwrap();
    assert!(Command::new("make")
        .args(&["-R", "-f", "makefile.cargo", &format!("-j{}", env::var("NUM_JOBS").unwrap())])
        .status()
        .unwrap()
        .success());
    println!("cargo:rustc-flags=-L native={}", env::var("OUT_DIR").unwrap());

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=static=azure");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=uuid");
    }
}
