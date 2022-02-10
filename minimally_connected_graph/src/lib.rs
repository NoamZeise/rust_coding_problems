
fn minimally_connected(matrix: &Vec<Vec<u32>>) -> bool {
    let mut connections = 0;
    for node in matrix {
        connections += node.len();
    }
    if connections != (matrix.len() - 1) * 2 {
        return false;
    }
    //check if all nodes are explorable from an arbitrary node
    let mut current_node = 0;
    let mut visited = vec![false; matrix.len()];
    let mut stack = Vec::new();
    loop {
        visited[current_node] = true;
        for n in matrix[current_node].iter() {
            if !visited[*n as usize] {
                stack.push(n);
            }
        }
        if stack.len() == 0 {
            break;
        }
        while visited[current_node] && stack.len() > 0{
        current_node = *stack.pop().unwrap() as usize;
        }
    }

    for v in visited {
        if !v {
            return false;
        }
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_binary_tree_test() {
        /*
                                     0
                                    / \
                                   1   2
                                  /   /\
                                 3   4  5
        */
        //                       0          1             2         3       4          5
        let matrix = vec![vec![1, 2], vec![0, 3], vec![0, 4, 5], vec![1], vec![2], vec![2]];
        assert_eq!(minimally_connected(&matrix), true);
    }

    #[test]
    fn minimal_circular_test() {
        /*
                                     0
                                    / \
                                   5   1
                                  /     \
                                 4 - 3 - 2
        */
        //                       0          1             2         3       4          5
        let matrix = vec![vec![1, 5], vec![0, 2], vec![1,3], vec![2,4], vec![5,3], vec![4, 0]];
        assert_eq!(minimally_connected(&matrix), false);
    }

    #[test]
    fn minimal_broken_circular_test() {
        /*
                                     0
                                    / \
                                   5   1
                                  /     \
                                 4 - 3   2
        */
        //                       0          1         2         3       4            5
        let matrix = vec![vec![1, 5], vec![0, 2], vec![1], vec![4], vec![5,3], vec![4, 0]];
        assert_eq!(minimally_connected(&matrix), true);
    }

    #[test]
    fn minimal_broken_test() {
        /*
                                       0 - 2
                                        \ /
                                6 -5     1
                                  /
                                 4 - 3
        */
        //                       0        1              2         3       4          5          6
        let matrix = vec![vec![1, 2], vec![0, 2], vec![1, 0], vec![4], vec![5,3], vec![4, 6], vec![5]];
        assert_eq!(minimally_connected(&matrix), false);
    }
}
