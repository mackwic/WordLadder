#[macro_use]
extern crate structopt;

mod word_graph;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use word_graph::WordGraph;

#[derive(Debug, StructOpt)]
#[structopt(name = "word_ladder", about = "Finding ladders between words.")]
struct Opt {
    /// Path to the word dictionnary
    #[structopt(parse(from_os_str))]
    words_input: PathBuf,
    /// Origin of the ladder
    origin: String,
    /// Target of the ladder
    target: String,
}

fn main() {
    let opt = Opt::from_args();

    let mut word_graph = WordGraph::default();

    let mut file = File::open(opt.words_input).expect("file should exists");
    let mut content = String::with_capacity(128_000);
    file.read_to_string(&mut content).expect("file should be readable");

    for word in content.lines() {
        word_graph.add(word);
    }

    let output = word_graph.ladder(opt.origin, opt.target);
    println!("{:?}", output);
}

