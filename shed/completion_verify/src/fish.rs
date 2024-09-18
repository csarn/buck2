/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::io;

use crate::extract_from_outputs;
use crate::runtime::FishRuntime;
use crate::runtime::Term;

pub(crate) fn run_fish(script: &str, input: &str) -> io::Result<Vec<String>> {
    let home = tempfile::tempdir()?;
    let home = home.path();

    let mut r = FishRuntime::new(home.to_owned())?;
    r.register("buck2", script)?;

    extract_from_outputs(
        input,
        [
            r.complete(&format!("{}\t", input), &Term::new()),
            r.complete(&format!("{}\t\t", input), &Term::new()),
        ],
    )
}
