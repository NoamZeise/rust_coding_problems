/*
Given the sequence of keys visited by a postorder traversal of a binary search tree, reconstruct the tree.
*/
pub fn parse_post_order(elems: &Vec<i32>) -> Vec<Node> {
    let mut current = 0;
    let mut next = 1;
    let mut nodes: Vec<Node> = vec![];
    nodes.resize(elems.len() + 1, Node::new());

    for (i, e) in Iterator::enumerate(elems.iter().rev()) {
        let new_value = Value::new(*e, next);
        loop {
            let current_value =
                match &nodes[current].this {
                    None => {
                        nodes[current].this = Some(Value::new(*e, i));
                        break;
                    },
                    Some(n) => *n
            };

            if *e >= current_value.value {
                nodes[current].right_child = Some(new_value);
                nodes[next].parent = Some(current_value);
                nodes[next].this = Some(new_value);
                println!("\\{} parent:{}", e, nodes[next].parent.unwrap().value);
                current = next;
                break;
            } else if nodes[current].parent.is_some() {
                if nodes[current].parent.unwrap().value > *e && nodes[current].parent.unwrap().value < nodes[current].this.unwrap().value {
                    current = nodes[current].parent.unwrap().index;
                    println!("^{}", nodes[current].this.unwrap().value);
                    continue;
                }
            }
            nodes[current].left_child = Some(new_value);
            nodes[next].parent = Some(current_value);
            nodes[next].this = Some(new_value);
            println!("/{} parent:{}", e, nodes[next].parent.unwrap().value);
            current = next;
            break;
        }
        next += 1;
    }

    nodes
}

#[derive(Copy, Clone)]
pub struct Value {
        pub value: i32,
        pub index: usize,
}

impl Value {
    pub fn new(value: i32, index: usize) -> Value {
        Value {
            value,
            index
        }
    }
}

pub struct Node {
    pub parent: Option<Value>,
    pub this: Option<Value>,
    pub left_child: Option<Value>,
    pub right_child: Option<Value>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            parent: None,
            this: None,
            left_child: None,
            right_child: None,
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Node{
        let mut node = Node::new();
        node.parent = self.parent;
        node.this = self.this;
        node.left_child = self.left_child;
        node.right_child = self.right_child;
        node
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eg_test() {
        let test_input1 = vec![2, 4, 3, 8, 7, 5];
        let test_input2 = vec![3, 5, 4, 11, 6];

        parse_post_order(&test_input1);
        println!("");
        parse_post_order(&test_input2);

        assert!(false);
    }


    /* -------test input 1-------
            5
            /\
           3  7
          /\   \
         2  4    8

    OUT:

    \7 parent:5
    \8 parent:7
    ^7
    ^5
    /3 parent:5
    \4 parent:3
    ^3
    /2 parent:3

------- test input 2 -------
                 6
                /\
               4  11
              /\
             3 5

    OUT:

    \11 parent:6
    ^6
    /4 parent:6
    \5 parent:4
    ^4
    /3 parent:4

    */

}
