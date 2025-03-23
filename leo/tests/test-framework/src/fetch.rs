// Copyright (C) 2019-2025 Provable Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

pub fn find_tests(path: &Path) -> impl Iterator<Item = (PathBuf, String)> {
    WalkDir::new(path).into_iter().flatten().filter_map(move |f| {
        // Get the path.
        let path = f.path();
        // Check if the file is a .leo file.
        let is_leo_file = path.extension().filter(|s| *s == "leo").is_some();
        // Read the test filter from the environment.
        let filter = std::env::var("TEST_FILTER").unwrap_or_default();
        // Check if the path contains the filter.
        let satisfies_filter = filter.is_empty() || path.to_string_lossy().contains(&filter);
        // If the file is a .leo file and satisfies the filter, return the path and the file contents.
        if is_leo_file && satisfies_filter {
            Some((path.to_path_buf(), fs::read_to_string(path).expect("failed to read test")))
        } else {
            None
        }
    })
}

pub fn split_tests_one_line(source: &str) -> Vec<&str> {
    source.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).collect()
}

pub fn split_tests_two_line(source: &str) -> Vec<String> {
    let mut out = vec![];
    let mut lines = vec![];
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !lines.is_empty() {
                out.push(lines.join("\n"));
            }
            lines.clear();
            continue;
        }
        lines.push(line);
    }
    let last_test = lines.join("\n");
    if !last_test.trim().is_empty() {
        out.push(last_test.trim().to_string());
    }
    out
}
