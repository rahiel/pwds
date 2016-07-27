// pwds - print the path of the current working directory, shortly
// Copyright (C) 2016 Rahiel Kasim
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;


fn main() {
    let max_length: usize;

    match env::var("PWDS_LENGTH") {
        Ok(val) => max_length = val.parse::<usize>().unwrap(),
        Err(_) => max_length = 16,
    }

    let path = env::current_dir().unwrap();
    let mut p = path.to_str().unwrap().to_string();

    let in_home: bool;
    let home_path = env::home_dir().unwrap();
    let home = home_path.to_str().unwrap();
    if p.starts_with(home) {
        p = p.replace(home, "~");
        in_home = true;
    } else {
        in_home = false;
    }

    let mut length = p.chars().count();

    if length > max_length {
        let mut dirs: Vec<&str> = p.split('/').collect();
        if !in_home {
            dirs.remove(0);    // empty string
        }
        let last = dirs.len() - 1;   // never shorten the current dir name
        let mut i = 0;
        while length > max_length && i < last {
            let char_count = dirs[i].chars().count();
            if char_count > 1 {
                length = length - char_count + 1;
                let mut chars = dirs[i].char_indices();
                let start = chars.next().unwrap().0;
                let end = chars.next().unwrap().0;
                dirs[i] = &dirs[i][start..end];
            }
            i = i + 1;
        }
        let mut pwds = dirs.join("/");
        if !in_home {
            pwds = "/".to_string() + &pwds;
        }
        println!("{}", &pwds);
    } else {
        println!("{}", &p);
    }
}
