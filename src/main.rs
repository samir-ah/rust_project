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
#[derive(Serialize, Deserialize, Clone, Copy)]
struct State {
  is_terminal: bool,
  index: usize,
}
/**
 * constructor of State class
 */
impl State {
  fn new(is_initial: bool, is_terminal: bool, index: usize) -> State {
    State {
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
  character: String
}

impl Transition {
  fn new(character: &str) -> Transition {
    Transition {
      character: String::from(character)
    }
  }
  fn e() -> Transition {
    Transition {
      character: String::from("0"),
    }
  }
}

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
    mut incidence_matrix: &mut Vec<Vec<Transition>>,
    max_word_length:usize
  ) {
    if self.states[state_index].is_terminal {
      if !words.contains(&String::from(current_word)) {
        words.push(String::from(current_word));
      }
    };
    for i in 0..incidence_matrix[state_index].len() {
      let mut t = &mut incidence_matrix[state_index][i];
      // if t.current_transit < t.max_transit {
      if  t.character != "0" && current_word.len() < 7 {
        let together = format!("{}{}", current_word, t.character);
        self.list_all_words(words, i, together.as_str(), &mut incidence_matrix.clone(),max_word_length);
      }
    }
  }

  fn match_word_path(
    &mut self,
    word: &str,
    char_index: usize,
    state_index: usize,
    current_word: &str,
    incidence_matrix: &Vec<Vec<Transition>>,
    path_in_automaton: &mut Vec<usize>,
  ) -> bool {
    if state_index == 0 && char_index == 0 {
      path_in_automaton.push(state_index)
    }
    if self.states[state_index].is_terminal && (word.len() == current_word.len()) {
      if current_word == word {
        return true;
      }
    };
    for index_next_state in 0..incidence_matrix[state_index].len() {
      let t = &incidence_matrix[state_index][index_next_state];
      if t.character != "0" {
        let new_current_word = format!("{}{}", current_word, t.character);
        if word.get(char_index..char_index + 1)
          == new_current_word.as_str().get(char_index..char_index + 1)
        {
          path_in_automaton.push(index_next_state);

          if (self.match_word_path(
            word,
            char_index + 1,
            index_next_state,
            new_current_word.as_str(),
            incidence_matrix,
            path_in_automaton,
          )) {
            return true;
          };
        }
      }
    }
    return false;
  }

  fn union(
    &mut self,
    words_union: &mut Vec<String>,
    mut incidence_matrix: &mut Vec<Vec<Transition>>,
    mut incidence_matrix_2: &mut Vec<Vec<Transition>>,
    max_word_length:usize
  ) {
    let mut words_1: Vec<String> = vec![];
    self.list_all_words(&mut words_1,0,"",&mut incidence_matrix,max_word_length);
    let mut words_2: Vec<String> = vec![];
    self.list_all_words(&mut words_2,0,"",&mut incidence_matrix_2,max_word_length);

    words_union.append(&mut words_1);
    words_union.append(&mut words_2);
    
  }

  fn inter(
    &mut self,
    words_inter: &mut Vec<String>,
    mut incidence_matrix: &mut Vec<Vec<Transition>>,
    mut incidence_matrix_2: &mut Vec<Vec<Transition>>,
    max_word_length:usize
  ) {
    let mut words_1: Vec<String> = vec![];
    self.list_all_words(&mut words_1,0,"",&mut incidence_matrix,max_word_length);
    let mut words_2: Vec<String> = vec![];
    self.list_all_words(&mut words_2,0,"",&mut incidence_matrix_2,max_word_length);

    for i_word in &words_1 {
      for j_word in &words_2 {
        if i_word == j_word {
          words_inter.push(i_word.clone());
        }
      }
    }
    
  }
}
#[derive(Serialize, Deserialize)]
struct Automaton {
  states: Vec<State>,
  matrix: Vec<Vec<Transition>>,
}
impl Automaton {
  fn new(states: Vec<State>, matrix: Vec<Vec<Transition>>) -> Automaton {
    Automaton {
      states: states,
      matrix: matrix,
    }
  }
  fn load_from_json(file_name: &str) -> Automaton {
    // Create a path to the desired file
    let path = Path::new(file_name);
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
      // The `description` method of `io::Error` returns a string that
      // describes the error
      Err(why) => panic!("couldn't open {}: {}", display, why.description()),
      Ok(file) => file,
    };
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s);
    return serde_json::from_str(s.as_str()).unwrap();
  }

  fn save_to_json(&self, file_name: &str) {
    let output = serde_json::to_string(self);
    match output {
      Ok(o) => {
        let path = Path::new(file_name);
        let display = path.display();
        let mut file = match File::create(&path) {
          Err(why) => panic!("couldn't create {}: {}", display, why.description()),
          Ok(file) => file,
        };
        match file.write_all(o.as_bytes()) {
          Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
          Ok(_) => println!("successfully wrote to {}", display),
        }
      }
      Err(e) => println!("error parsing header: {:?}", e),
    }
  }
  fn save_to_dot_script(&self, file_name: &str) {
    let path = Path::new(file_name);
    let display = path.display();
    let mut file = File::create(&path).expect("couldn't create file");
    file.write_all(
      "digraph finite_state_machine {
  rankdir = LR;
  node [shape = circle]; 0;\n"
        .as_bytes(),
    );
    for i in &self.states {
      if i.is_terminal {
        file.write_all(format!("    node [shape = doublecircle]; {};\n", i.index).as_bytes());
      }
    }
    file.write_all(
      "   node [shape = plaintext];
  \"\" -> 0 [label = \"start\"];
  node [shape = circle];\n"
        .as_bytes(),
    );

    for i in 0..self.matrix.len() {
      for j in 0..self.matrix[i].len() {
        if self.matrix[i][j].character != "0" {
          file.write_all(
            format!(
              "   {}->{} [label=\"{}\"];\n",
              i, j, self.matrix[i][j].character
            )
            .as_bytes(),
          );
        }
      }
    }
    file.write_all("}".as_bytes());
  }
}

/**
 * Creation of the incidence matrix and launch of word generation
 */
fn main() {
  //////************charger l'automate à partir d'un fichier json**********
  let mut serializable_input = Automaton::load_from_json("input.json");
  let mut serializable_input_2 = Automaton::load_from_json("input2.json");

//////********sauvegarder l'automate dans un fichier au format json**********
  //serializable_input.save_to_json("output.json");

  serializable_input.save_to_dot_script("dotScript.txt");

  let mut generator = Generator::new(&serializable_input.states);
  let mut words: Vec<String> = vec![];
  generator.union(&mut words,&mut serializable_input.matrix,&mut serializable_input_2.matrix,6);
  println!("words union: {:?}", words);

  words = vec![];

  generator.inter(&mut words,&mut serializable_input.matrix,&mut serializable_input_2.matrix,6);
  println!("words inter: {:?}", words);

  let mut path_in_automaton: Vec<usize> = vec![];
  let wanted_word = "abc";
  if generator.match_word_path(
    wanted_word,
    0,
    0,
    "",
    &serializable_input.matrix,
    &mut path_in_automaton,
  ) {
    println!("path in automaton: {:?}", path_in_automaton);
  } else {
    println!("{} Not found", wanted_word);
  }
}
