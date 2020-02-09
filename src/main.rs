#![allow(dead_code, unused)]
/**
 * represents a state of the automaton
 */
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
struct IncidenceMatrix{
  nb_of_states: usize,
  incidence_matrix: Vec<Vec<Transition>>
}
impl IncidenceMatrix{
  fn new(incidence_matrix: Vec<Vec<Transition>>) -> IncidenceMatrix {
    IncidenceMatrix {
      nb_of_states: incidence_matrix.len(),
      incidence_matrix: incidence_matrix
    }
  }
  // fn clone(&self) -> IncidenceMatrix {
  //   let mut cloned_matrix:Vec<Vec<Transition>> =  vec![vec![];self.nb_of_states];
  //   for i in 0..self.incidence_matrix.len() {
  //     for j in 0..self.incidence_matrix[i].len() {
  //       cloned_matrix[i][j] = self.incidence_matrix[i][j].clone();
  //     }
  //   }
  //   return IncidenceMatrix {
  //     nb_of_states: self.nb_of_states,
  //     incidence_matrix: cloned_matrix
  //   };
  // }
}
/**
 * Generates a list of words from a language represented by an automaton
 */
struct Generator {
  states: Vec<State>
}
impl Generator {
  fn new(states: Vec<State>) -> Generator {
    Generator {
      states: states
    }
  }
  fn list_all_words(&mut self,words:&mut Vec<String>,state_index:usize, current_word:&str,mut incidence_matrix:IncidenceMatrix) {
    //let mut cloned_matrix = self.incidence_matrix.clone();
   
    println!("state: {:?}", state_index+1);
    if self.states[state_index].is_terminal {
      if !words.contains(&String::from(current_word)) {
        words.push(String::from(current_word));
        println!("words: {:?}", words);
        return;
      }
    };
    for i in 0..incidence_matrix.incidence_matrix[state_index].len() {

          let mut t = &mut incidence_matrix.incidence_matrix[state_index][i];
          if t.current_transit < t.max_transit {
            t.increment_current_transit();
            let together = format!("{}{}", current_word, t.character.as_str());
            self.list_all_words(words,i, together.as_str(),incidence_matrix.clone());
        
      }
     
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
  let matrix: Vec<Vec<Transition>> = vec![
    vec![Transition::e(), Transition::new("a", 1), Transition::e()],
    vec![
      Transition::e(),
      Transition::new("b", 2),
      Transition::new("c", 1),
    ],
    vec![Transition::e(), Transition::e(), Transition::e()],
  ];
  let incidence_matrix: IncidenceMatrix = IncidenceMatrix::new(matrix);
  let mut generator = Generator::new(states);
  let mut words:Vec<String> = vec![];
  generator.list_all_words(&mut words,0,"",incidence_matrix);
  println!("words: {:?}", words);
}
