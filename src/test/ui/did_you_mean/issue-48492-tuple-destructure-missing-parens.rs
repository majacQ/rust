// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused)]

#[derive(Copy, Clone)]
enum Nucleotide {
    Adenine,
    Thymine,
    Cytosine,
    Guanine
}

#[derive(Clone)]
struct Autosome;

#[derive(Clone)]
enum Allosome {
    X(Vec<Nucleotide>),
    Y(Vec<Nucleotide>)
}

impl Allosome {
    fn is_x(&self) -> bool {
        match *self {
            Allosome::X(_) => true,
            Allosome::Y(_) => false,
        }
    }
}

#[derive(Clone)]
struct Genome {
    autosomes: [Autosome; 22],
    allosomes: (Allosome, Allosome)
}

fn find_start_codon(strand: &[Nucleotide]) -> Option<usize> {
    let mut reading_frame = strand.windows(3);
    while let b1, b2, b3 = reading_frame.next().expect("there should be a start codon") {
        //~^ ERROR unexpected `,` in pattern
        // ...
    }
    None
}

fn find_thr(strand: &[Nucleotide]) -> Option<usize> {
    let mut reading_frame = strand.windows(3);
    let mut i = 0;
    if let b1, b2, b3 = reading_frame.next().unwrap() {
        //~^ ERROR unexpected `,` in pattern
        match (b1, b2, b3) {
            //~^ ERROR cannot find value `b2` in this scope
            //~| ERROR cannot find value `b3` in this scope
            Nucleotide::Adenine, Nucleotide::Cytosine, _ => {
                //~^ ERROR unexpected `,` in pattern
                return Some(i);
            },
            _ => {}
        }
        i += 1;
    }
    None
}

fn analyze_sex_chromosomes(women: &[Genome], men: &[Genome]) {
    for x, _barr_body in women.iter().map(|woman| woman.allosomes.clone()) {
        //~^ ERROR unexpected `,` in pattern
        // ...
    }
    for x, y @ Allosome::Y(_) in men.iter().map(|man| man.allosomes.clone()) {
        //~^ ERROR unexpected `,` in pattern
        // ...
    }
}

fn main() {
    let genomes = Vec::new();
    let women, men: (Vec<Genome>, Vec<Genome>) = genomes.iter().cloned()
    //~^ ERROR unexpected `,` in pattern
        .partition(|g: &Genome| g.allosomes.0.is_x() && g.allosomes.1.is_x());
}
