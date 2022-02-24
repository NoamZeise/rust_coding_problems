/*

You are given a starting state start, a list of transition probabilities for a Markov chain, and a number of steps num_steps. Run the Markov chain starting from start for num_steps and compute the number of times we visited each state.

For example, given the starting state a, number of steps 5000, and the following transition probabilities:

[
  ('a', 'a', 0.9),
  ('a', 'b', 0.075),
  ('a', 'c', 0.025),
  ('b', 'a', 0.15),
  ('b', 'b', 0.8),
  ('b', 'c', 0.05),
  ('c', 'a', 0.25),
  ('c', 'b', 0.25),
  ('c', 'c', 0.5)
]
One instance of running this Markov chain might produce { 'a': 3012, 'b': 1656, 'c': 332 }.

*/

pub fn run_markov_chain(chain: &mut MarkovChain, step: usize) -> Vec<(String, usize)> {
    let mut steps: Vec<(usize,usize)> = vec!();

    for _i in 0..step {
        let node = chain.step();
        let mut found = false;
        for v in &mut steps {
            if v.0 == node {
                v.1+=1;
                found = true;
                break;
            }
        }
        if !found {
            steps.push((node, 1));
        }
    }

    let mut final_steps: Vec<(String,usize)> = vec!();
    for e in steps {
        final_steps.push((chain.get_node_value(e.0), e.1));
    }

    final_steps
}

pub struct Node {
    value: String,
    edges: Vec<(usize, f32)>
}

pub struct MarkovChain {
    nodes: Vec::<Node>,
    current: usize
}

impl MarkovChain {
    pub fn new(start_node: String, node_names: Vec::<String>, transitons: Vec::<f32>) -> MarkovChain {

        assert_eq!(transitons.len(), node_names.len() * node_names.len());

        let mut nodes : Vec::<Node> = vec!();

        let mut current = 0;
        for i in 0..node_names.len() {
            if node_names[i] == start_node {
                current = i;
            }
            let mut edges: Vec<(usize, f32)> = vec!();
            for j in 0..node_names.len() {
                edges.push((j, transitons[i*node_names.len() + j]));
            }
            nodes.push(
                Node {
                    value: node_names[i].clone(),
                    edges
                }
            );

        }

        MarkovChain {
            nodes,
            current
        }
    }

    pub fn step(&mut self) -> usize {
        let val = rand::random::<f32>();
        let mut current: f32 = 0.0;
        for e in self.nodes[self.current].edges.iter() {
            current += e.1;
            if current > val {
                self.current = e.0;
                break;
            }
        }
        self.current
    }

    pub fn get_node_value(&self, index: usize) -> String {
        self.nodes[index].value.clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let mut chain = MarkovChain::new(String::from("a"),
                                    vec!(String::from("a"), String::from("b"), String::from("c")),
                                    vec!(0.9, 0.075, 0.025, 0.15, 0.8, 0.05, 0.25, 0.25, 0.5));
        let steps = run_markov_chain(&mut chain, 5000);
        let mut total = 0;
        for e in steps {
            total += e.1;
            println!("{}, {}", e.0, e.1);
        }
        assert_eq!(total, 5000);
    }

    #[test]
    fn test1() {
        let mut chain = MarkovChain::new(String::from("second"),
                                    vec!(String::from("first"), String::from("second")),
                                    vec!(0.1, 0.9, 0.5, 0.5));
        let steps = run_markov_chain(&mut chain, 5000);
        let mut total = 0;
        for e in steps {
            total += e.1;
            println!("{}, {}", e.0, e.1);
        }
        assert_eq!(total, 5000);
    }
}
