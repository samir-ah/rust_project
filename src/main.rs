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
}
/**
 * Generates a list of words from a language represented by an automaton
 */
struct Generator {
  states: Vec<State>,
  incidence_matrix: Vec<Vec<Transition>>,
}
impl Generator {
  fn new(states: Vec<State>, incidence_matrix: Vec<Vec<Transition>>) -> Generator {
    Generator {
      states: states,
      incidence_matrix: incidence_matrix,
    }
  }
  fn list_all_words(&mut self,words:&mut Vec<String>,state_index:usize, current_word:&str) {
    if self.states[state_index].is_terminal {
      if !words.contains(&String::from(current_word)) {
        words.push(String::from(current_word));
        println!("words: {:?}", words);
        return;
      }
    };
    for i in 0..self.incidence_matrix[state_index].len() {
      let mut t = &mut self.incidence_matrix[state_index][i];
      if t.current_transit < t.max_transit {
        t.increment_current_transit();
        let together = format!("{}{}", current_word, t.character.as_str());
        self.list_all_words(words,i, together.as_str());
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
  let incidence_matrix: Vec<Vec<Transition>> = vec![
    vec![Transition::e(), Transition::new("a", 1), Transition::e()],
    vec![
      Transition::e(),
      Transition::new("b", 2),
      Transition::new("c", 1),
    ],
    vec![Transition::e(), Transition::e(), Transition::e()],
  ];

  let mut generator = Generator::new(states, incidence_matrix);
  let mut words:Vec<String> = vec![];
  generator.list_all_words(&mut words,0,"");
  println!("words: {:?}", words);
}
