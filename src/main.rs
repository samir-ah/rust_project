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
        let together = format!("{}{}", current_word, t.character);
        self.list_all_words(words, i, together.as_str(), &mut incidence_matrix.clone());
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
  ///////************création des états**************
  // let mut state1 = State::new(true, false, 0);
  // let mut state2 = State::new(false, false, 1);
  // let mut state3 = State::new(false, true, 2);
  // let states: Vec<State> = vec![state1, state2, state3];
  /////// *******création de la matrice d'incidence********
  // let mut matrix: Vec<Vec<Transition>> = vec![
  //   vec![Transition::e(), Transition::new("a", 2), Transition::e()],
  //   vec![
  //     Transition::e(),
  //     Transition::new("b", 2),
  //     Transition::new("c", 2),
  //   ],
  //   vec![Transition::new("d", 1), Transition::e(), Transition::e()],
  // ];
  //////**********constructeur à partir des variable creés***********
  //let serializable_input = Automaton::new(states, matrix);
  //////********sauvegarder l'automate dans un fichier au format json**********
  //serializable_input.save_to_json("output.json");


  //////************charger l'automate à partir d'un fichier json**********
  let mut serializable_input = Automaton::load_from_json("input.json");

  serializable_input.save_to_dot_script("dotScript.txt");

  let mut generator = Generator::new(&serializable_input.states);
  let mut words: Vec<String> = vec![];
  generator.list_all_words(&mut words, 0, "", &mut serializable_input.matrix);
  println!("words: {:?}", words);

  let mut path_in_automaton: Vec<usize> = vec![];
  let wanted_word = "acda";
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
