//! # Complex Sound
//! 
//! A structure implementing the ability to run multiple [`Generator`]s and
//! [`Modifier`]s within a single object, granting the ability to create complex
//! systems like those found in digital synthesizers.
//! 
//! [`Generator`]: ../../generators/trait.Generator.html
//! [`Modifier`]: ../../modifiers/trait.Modifier.html

use super::*;
use std::rc::Rc;
use std::collections::VecDeque;
use crate::core::*;
use super::basic_block::*;
use petgraph::graph;

/// Graph type for [`ComplexSound`] 
pub type Graph = graph::DiGraph<BlockRc, ()>;
pub type GraphNode = graph::NodeIndex;
pub type ProcessOrder = VecDeque<GraphNode>;

#[derive(Clone)]
pub struct ComplexSound {
	graph: Graph,
	process_order: ProcessOrder,
	input_gain: GraphNode,
	output_gain: GraphNode,
	driver: Option<ComplexDriverRc>,
	id: Option<usize>,
	is_muted: bool,
	is_paused: bool
}

impl ComplexSound {
	pub fn get_input_gain(&self) -> GraphNode {
		self.input_gain
	}

	pub fn get_output_gain(&self) -> GraphNode {
		self.output_gain
	}

	pub fn add_block(&mut self, block: BlockRc) -> GraphNode {
		self.graph.add_node(block)
	}

	pub fn add_connection(&mut self, from: GraphNode, to: GraphNode) {
		self.graph.update_edge(from, to, ());

		self.process_order();
	}

	pub fn remove_connection(&mut self, from: GraphNode, to: GraphNode) {
		if let Some(e) = self.graph.find_edge(from, to) {
			self.graph.remove_edge(e);
		}

		self.process_order();
	}

	pub fn process(&mut self, input: StereoData) -> StereoData {
		if self.is_paused {
			return Default::default();
		}

		let mut out = Default::default();

		Rc::get_mut(
			self.graph.node_weight_mut(self.input_gain).unwrap()
		).unwrap().prime_input(input);

		for b in &self.process_order {
			let block = Rc::get_mut(self.graph.node_weight_mut(*b).unwrap()).unwrap();
			out = block.process();

			let mut neighbors = self.graph.neighbors(*b).detach();

			while let Some(t) = neighbors.next(&self.graph) {
				Rc::get_mut(self.graph.node_weight_mut(t.1).unwrap()).unwrap().
				prime_input(out);
			}
		}

		if self.is_muted {
			Default::default()
		} else {
			out
		}
	}

	fn process_order(&mut self) {
		self.process_order.clear();

		self.process_order.extend(self.graph.externals(petgraph::Direction::Incoming));
		let mut i = 0;
		while i < self.process_order.len() {
			let neighbors = self.graph.neighbors(self.process_order[i]);
			self.process_order.extend(neighbors);

			let mut j = 0;
			while j < i {
				Self::remove_dups(&mut self.process_order, j);
				j += 1;
			}

			i += 1;
		}

		let ig = self.input_gain;
		let og = self.output_gain;

		self.process_order.retain(|e| *e != ig && *e != og);

		self.process_order.push_front(self.input_gain);
		self.process_order.push_back(self.output_gain);
	}

	fn remove_dups(v: &mut ProcessOrder, whitelist: usize) {
		let mut i = whitelist + 1;

		while i < v.len() {
			if v[i] == v[whitelist] {
				v.remove(i);
			} else {
				i += 1;
			}
		}
	}
}

impl Sound<ComplexSound> for ComplexSound {
	fn new(input_gain: SampleT, output_gain: SampleT) -> Self {
		let mut graph = Graph::new();
		let input_gain = graph.add_node(
			Rc::new(
				ModifierBlock::from_modifier(
					modifiers::Gain::new(input_gain)
				)
			)
		);
		let output_gain = graph.add_node(
			Rc::new(
				ModifierBlock::from_modifier(
					modifiers::Gain::new(output_gain)
				)
			)
		);

		ComplexSound {
			graph,
			process_order: ProcessOrder::new(),
			input_gain,
			output_gain,
			driver: None,
			id: None,
			is_muted: false,
			is_paused: false
		}
	}

	fn toggle_pause(&mut self) {
		self.is_paused = !self.is_paused;
	}

	fn is_paused(&self) -> bool {
		self.is_paused
	}

	fn toggle_mute(&mut self) {
		self.is_muted = !self.is_muted;
	}

	fn is_muted(&self) -> bool {
		self.is_muted
	}

	fn register(this: ComplexSoundRc, mut driver: ComplexDriverRc) {
		Self::unregister(this.clone());

		if let Some(sound) = Rc::get_mut(&mut this.clone()) {
			sound.driver = Some(driver.clone());
			if let Some(driver) = Rc::get_mut(&mut driver) {
				sound.id = Some(driver.add_sound(this));
			}
		}
	}

	fn unregister(mut this: ComplexSoundRc) {
		if this.id != None {
			if let Some(sound) = Rc::get_mut(&mut this) {
				if let Some(mut driver) = sound.driver.clone() {
					if let Some(driver) = Rc::get_mut(&mut driver) {
						if let Some(id) = sound.id {
							let _ = driver.remove_sound(id);
						}
					}
				}

				sound.driver = None;
				sound.id = None;
			}
		}
	}

	fn process(&mut self, input: StereoData) -> StereoData {
		if self.is_paused {
			return Default::default();
		}

		let mut out = Default::default();

		Rc::get_mut(
			self.graph.node_weight_mut(self.input_gain).unwrap()
		).unwrap().prime_input(input);

		for b in &self.process_order {
			let block = Rc::get_mut(self.graph.node_weight_mut(*b).unwrap()).unwrap();
			out = block.process();

			let mut neighbors = self.graph.neighbors(*b).detach();

			while let Some(t) = neighbors.next(&self.graph) {
				Rc::get_mut(self.graph.node_weight_mut(t.1).unwrap()).unwrap().
				prime_input(out);
			}
		}

		if self.is_muted {
			Default::default()
		} else {
			out
		}
	}
}

pub type ComplexSoundRc = Rc<ComplexSound>;