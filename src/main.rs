#![allow(dead_code, unused)]
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/**
 * represents a state of the automaton
 */
#[derive(Serialize, Deserialize,Clone,Copy)]
struct State {
  is_terminal: bool,
  is_initial: bool,
  index: usize,
}
/**
 * constructor of State class
 */
impl State {
  fn new(is_initial: bool, is_terminal: bool, index: usize) -> State {
    State {
      is_initial: is_initial,
      is_terminal: is_terminal,
      index: index,
    }
  }
}
/**
 * represents a transition and its maximum number of flows
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transition {
  character: String,
  max_transit: usize,
  current_transit: usize,
}

impl Transition {
  fn new(character: &str, max_transit: usize) -> Transition {
    Transition {
      character: String::from(character),
      max_transit: max_transit,
      current_transit: 0,
    }
  }
  fn e() -> Transition {
    Transition {
      character: String::from("0"),
      max_transit: 0,
      current_transit: 0,
    }
  }
  fn increment_current_transit(&mut self) {
    self.current_transit += 1;
  }
}
// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct IncidenceMatrix {
//   matrix: Vec<Vec<Transition>>,
// }
// impl IncidenceMatrix {
//   fn new(incidence_matrix: Vec<Vec<Transition>>) -> IncidenceMatrix {
//     IncidenceMatrix {
//       matrix: incidence_matrix,
//     }
//   }
// }
/**
 * Generates a list of words from a language represented by an automaton
 */
struct Generator<'a> {
  states: &'a Vec<State>,
}
impl Generator<'_> {
  fn new(states: &'_ Vec<State>) -> Generator {
    Generator { states: states }
  }
  fn list_all_words(
    &mut self,
    words: &mut Vec<String>,
    state_index: usize,
    current_word: &str,
    mut incidence_matrix: Vec<Vec<Transition>>,
  ) {
    //let mut cloned_matrix = self.incidence_matrix.clone();
    //println!("state: {:?}", state_index+1);
    if self.states[state_index].is_terminal {
      if !words.contains(&String::from(current_word)) {
        words.push(String::from(current_word));
        //println!("words: {:?}", words);
        //return;
      }
    };
    for i in 0..incidence_matrix[state_index].len() {
      let mut t = &mut incidence_matrix[state_index][i];
      if t.current_transit < t.max_transit {
        t.increment_current_transit();
        let together = format!("{}{}", current_word, t.character.as_str());
        self.list_all_words(words, i, together.as_str(), incidence_matrix.clone());
      }
    }
  }
}
#[derive(Serialize, Deserialize)]
struct SerializableInputFromFile {
  states: Vec<State>,
  matrix: Vec<Vec<Transition>>,
}
impl SerializableInputFromFile {
  fn new(states: Vec<State>, matrix: Vec<Vec<Transition>>) -> SerializableInputFromFile {
    SerializableInputFromFile {
      states: states,
      matrix: matrix,
    }
  }
  fn serialize_to_json(&self, file_name: &str) {
    let output = serde_json::to_string(self);
    match output {
      Ok(o) => {
        println!("working with version: {:?}", o);
        let path = Path::new(file_name);
        let display = path.display();
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
          Err(why) => panic!("couldn't create {}: {}", display, why.description()),
          Ok(file) => file,
        };
        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(o.as_bytes()) {
          Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
          Ok(_) => println!("successfully wrote to {}", display),
        }
      }
      Err(e) => println!("error parsing header: {:?}", e),
    }
  }
}

/**
 * Creation of the incidence matrix and launch of word generation
 */
fn main() {
  let mut state1 = State::new(true, false, 0);
  let mut state2 = State::new(false, false, 1);
  let mut state3 = State::new(false, true, 2);

  let states: Vec<State> = vec![state1, state2, state3];
  let mut matrix: Vec<Vec<Transition>> = vec![
    vec![Transition::e(), Transition::new("a", 2), Transition::e()],
    vec![
      Transition::e(),
      Transition::new("b", 2),
      Transition::new("c", 2),
    ],
    vec![Transition::new("d", 1), Transition::e(), Transition::e()],
  ];
  

  let mut serializable_input = SerializableInputFromFile::new(states.clone(), matrix.clone());

  let mut generator = Generator::new(&serializable_input.states);
  let mut words: Vec<String> = vec![];
  generator.list_all_words(&mut words, 0, "", serializable_input.matrix);
  println!("words: {:?}", words);
}
