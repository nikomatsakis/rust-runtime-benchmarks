// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate test;

use test::Bencher;

use regex::Regex;

macro_rules! bench {
    ($name:ident, $pattern:expr, $count:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            use std::sync::Mutex;

            lazy_static! {
                static ref RE: Mutex<Regex> =
                    Mutex::new(Regex::new($pattern).unwrap());
                static ref TEXT: Mutex<String> =
                    Mutex::new(include_str!("sherlock.txt").to_owned());
            };
            let re = RE.lock().unwrap();
            let text = TEXT.lock().unwrap();
            b.bytes = text.len() as u64;
            b.iter(|| {
                let count = re.find_iter(&text).count();
                assert_eq!($count, count)
            });
        }
    }
}

bench!(name_sherlock, r"Sherlock", 97);
bench!(name_holmes, r"Holmes", 461);
bench!(name_sherlock_holmes, r"Sherlock Holmes", 91);

bench!(name_sherlock_nocase, r"(?i)Sherlock", 102);
bench!(name_holmes_nocase, r"(?i)Holmes", 467);
bench!(name_sherlock_holmes_nocase, r"(?i)Sherlock Holmes", 96);

bench!(name_whitespace, r"Sherlock\s+Holmes", 97);

bench!(name_alt1, r"Sherlock|Street", 158);
bench!(name_alt2, r"Sherlock|Holmes", 558);
bench!(name_alt3, r"Sherlock|Holmes|Watson|Irene|Adler|John|Baker", 740);
bench!(
    name_alt3_nocase,
    r"(?i)Sherlock|Holmes|Watson|Irene|Adler|John|Baker",
    753);
bench!(name_alt4, r"Sher[a-z]+|Hol[a-z]+", 582);
bench!(name_alt4_nocase, r"(?i)Sher[a-z]+|Hol[a-z]+", 697);
bench!(name_alt5, r"Sherlock|Holmes|Watson", 639);
bench!(name_alt5_nocase, r"(?i)Sherlock|Holmes|Watson", 650);

bench!(no_match_uncommon, r"zqj", 0);
bench!(no_match_common, r"aqj", 0);
bench!(no_match_really_common, r"aei", 0);

bench!(the_lower, r"the", 7218);
bench!(the_upper, r"The", 741);
bench!(the_nocase, r"(?i)the", 7987);

bench!(the_whitespace, r"the\s+\w+", 5410);

bench!(everything_greedy, r".*", 13053);
bench!(everything_greedy_nl, r"(?s).*", 1);

bench!(letters, r"\p{L}", 447160);
bench!(letters_upper, r"\p{Lu}", 14180);
bench!(letters_lower, r"\p{Ll}", 432980);

bench!(words, r"\w+", 109214);

bench!(before_holmes, r"\w+\s+Holmes", 319);
bench!(before_after_holmes, r"\w+\s+Holmes\s+\w+", 137);

bench!(holmes_cochar_watson, r"Holmes.{0,25}Watson|Watson.{0,25}Holmes", 7);
bench!(
    holmes_coword_watson,
    r"Holmes(?:\s*.+\s*){0,10}Watson|Watson(?:\s*.+\s*){0,10}Holmes",
    51);

bench!(quotes, r#"["'][^"']{0,30}[?!.]["']"#, 767);

bench!(
    line_boundary_sherlock_holmes,
    r"(?m)^Sherlock Holmes|Sherlock Holmes$",
    34);

bench!(word_ending_n, r"\b\w+n\b", 8366);
bench!(repeated_class_negation, r"[a-q][^u-z]{13}x", 142);
bench!(ing_suffix, r"[a-zA-Z]+ing", 2824);
bench!(ing_suffix_limited_space, r"\s[a-zA-Z]{0,12}ing\s", 2081);
