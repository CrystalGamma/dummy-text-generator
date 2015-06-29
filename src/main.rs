/* dummy text generator
Copyright (C) 2015  Jona Stubbe

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/
extern crate rand;
use rand::Rng;

extern crate pirate;

use std::io::prelude::*;

#[derive(Debug)]
struct Node {
	visits: usize,
	exits: [(usize, usize); 27]
}

impl Node {
	fn split(&mut self, visits: usize) -> Node {
		let mut new_exits = self.exits.clone();
		let mut errors = [0; 27];
		let mut new_visits = 0;
//		println!("splitting {} / {}", visits, self.visits);
		debug_assert!(self.validate());
		for pos in 0..27 {
			let old_exit = self.exits[pos].1;
			let new_exit = old_exit * visits / self.visits;
			new_exits[pos].1 = new_exit;
			let my_exit = old_exit * (self.visits - visits) / self.visits;
			let error = old_exit - my_exit - new_exit;
			assert!(error <= 1);
			errors[pos] = error;
			self.exits[pos].1 -= errors[pos] + new_exit;
			new_visits += new_exit;
		}
		let mut pos = 0;
		while new_visits != visits {
			debug_assert!(new_visits <= visits);
			new_exits[pos].1 += errors[pos];
			new_visits += errors[pos];
			pos += 1;
		}
		for i in pos..27 {
			self.exits[i].1 += errors[i];
		}
		self.visits -= visits;
		debug_assert!(self.validate());
		Node {visits: visits, exits: new_exits}
	}
	fn validate(&self) -> bool {
		self.visits == self.exits.iter().fold(0, |a,b|a+b.1)
	}
}

static START_VALUE: usize = 0;

fn char2num(c: char) -> usize {match c {
	'a'...'z' => (c as usize) - ('a' as usize) + 1,
	_ => 0
}}

fn num2char(n: usize) -> char {match n {
	0 => ' ',
	_ => ::std::char::from_u32('a' as u32 -1 + n as u32).unwrap()
}}

fn main() {
	let args = pirate::parse(std::env::args(), &[
		"s/min-visits:",
		"r/min-remaining:",
		"l/length:",
		"go-back"]).unwrap();
	let SPLIT_MIN_VISITS = args.get("min-visits").map(|s|s.parse().ok().expect("could not parse min-visits")).unwrap_or(4);
	let SPLIT_MIN_REMAINING = args.get("min-remaining").map(|s|s.parse().ok().expect("could not parse min-remaining")).unwrap_or(SPLIT_MIN_VISITS);
	let GO_BACK = args.has_arg("go-back");
	let length = args.get("length").map(|s|s.parse().ok().expect("could not parse length")).unwrap_or(1000);
	let mut text = String::new();
	::std::io::stdin().read_to_string(&mut text).unwrap();
	let iter = text.chars().map(|c|char2num(c.to_lowercase().next().unwrap()));
	let mut graph = vec![Node {visits: START_VALUE*27, exits: [(0, START_VALUE); 27]}];
	let mut pos = 0;
	for c in iter {
		debug_assert!(pos < graph.len());
		debug_assert!(graph[pos].validate());
		graph[pos].visits += 1;
		graph[pos].exits[c].1 += 1;
		if GO_BACK && c == 0 {
			pos = 0;
			continue;
		}
		let (target, num_exits) = graph[pos].exits[c];
		let target_visits = graph[target].visits;
		pos = if num_exits > SPLIT_MIN_VISITS && target_visits + 1 - num_exits > SPLIT_MIN_REMAINING {
			let new_pos = graph.len();
			let new_visits = num_exits - 1;
			let new = graph[target].split(new_visits);
			graph.push(new);
			graph[pos].exits[c].0 = new_pos;
			new_pos
		} else {target};
	}
	println!("num(nodes): {}", graph.len());
//	for node in &graph {println!("{:?}", node);}
	pos = 0;
	let mut rng = ::rand::thread_rng();
	for _ in 0..length {
		let mut rand = rng.gen_range(0, graph[pos].visits);
		let table = &graph[pos].exits;
		for (entry, c) in table.iter().zip(0..) {
			if rand < entry.1 {
				print!("{}", num2char(c));
				rand = 0;
				pos = entry.0;
				if GO_BACK && c == 0 {pos=0;}
				break;
			}
			rand -= entry.1;
		}
		debug_assert!(rand == 0);
	}
	println!("");
}
