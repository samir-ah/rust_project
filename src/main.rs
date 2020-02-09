
/**
 * represents a state of the automaton
 */
struct Node{
    is_terminal: bool,
    is_initial:bool,
    index:u32,
  }
  /**
   * constructor of Node class
   */
  impl Node{
     fn new(is_initial: bool, is_terminal: bool,index:u32) -> Node {
          Node { is_initial: is_initial, 
                  is_terminal: is_terminal,
                  index: index }
      }
      
  }
  /**
   * represents a transition and its maximum number of flows
   */
  struct Transition{
  
    value:String,
    max_transit:u32,
    current_transit:u32,
  }
  
  impl Transition{
     fn new(value: String,max_transit:u32) -> Transition {
          Transition { value: value, 
                  max_transit: max_transit,
                  current_transit: 0 }
      }
      fn e() -> Transition {
          Transition { value: String::from("0"), 
                  max_transit: 0,
                  current_transit: 0 }
      }
      
  }
  /**
   * Generates a list of words from a language represented by an automaton
   */
  struct Generator{
    nodes:Vec<Node>,
    transitions_matrix: Vec<Vec<Transition>>,
    words: Vec<String>
  }
  impl Generator{
    fn new(nodes:Vec<Node>,transitions_matrix: Vec<Vec<Transition>>) -> Generator {
        Generator{
          nodes:nodes,
          transitions_matrix: transitions_matrix,
          words: vec![]
        }
    }
    fn listAllwords(&self,word:String, index:u32) {
      // let mut word2 = word +"abffj";
      // self.listAllwords(word2.to_string(),1);
      // println!(" {}",word2);
      // for i in &self.transitions_matrix {
      //   for j in i {
      //     print!(" {}",j.value);
      //   }
      //   println!();  
      // }
      
  
     /* thread 'main' has overflowed its stack
  fatal runtime error: stack overflow*/
    }
  }
  
  
  
  /**
   * Creation of the incidence matrix and launch of word generation
   */
  fn main() {
    let mut node1 = Node::new(true,false,0);
    let mut node2 = Node::new(false,false,1);
    let mut node3 = Node::new(false,true,2);
  
    let nodes: Vec<Node> = vec![node1,node2,node3];
    let transitions_matrix : Vec<Vec<Transition>> =
    vec![
        vec![Transition::e(),Transition::new(String::from("a"),1),Transition::e()],
        vec![Transition::e(),Transition::new(String::from("b"),2),Transition::new(String::from("c"),1)],
        vec![Transition::e(),Transition::e(),Transition::e()]
        ];
  
    let generator = Generator::new(nodes,transitions_matrix);
    generator.listAllwords(String::from(""),0);
  
  
    
  }