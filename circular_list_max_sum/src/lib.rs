/*
Given a circular array, compute its maximum subarray sum in O(n) time.
A subarray can be empty, and in this case the sum is 0.
For example, given [8, -1, 3, 4],
return 15 as we choose the numbers 3, 4, and 8 where the 8 is obtained from wrapping around.
Given [-4, 5, 1, 0], return 6 as we choose the numbers 5 and 1.
*/

pub fn max_subarray_sum(list :&Vec<i32>) -> i32 {

    let mut sublists: Vec<[usize; 3]> = vec![]; //first, last, count

    let mut current_start = 0;
    let mut current_count = 0;
    let mut i = 0;
    while i < list.len() {
        if list[i] < 0 {
            if current_start != i {
                sublists.push([current_start, i-1, current_count]);
                current_start = i + 1;
                current_count = 0;
            }
        } else {
            current_count += list[i] as usize;
        }

        i+=1;
    }

    //if last item wasn't neagtive, wrap list around
    if current_start != list.len() {
        let info : [usize; 3] = [current_start, list.len() - 1, current_count];
        if sublists.len() > 0 && sublists[0][0] == 0 {
            sublists[0][0] = info[0];
            sublists[0][2] += info[2];
        } else {
            sublists.push(info);
        }
    }
    let mut largest = 0;

    for [first, last, count] in sublists {
        println!("{} {} {}", first, last, count);
        if count > largest {
            largest = count;
        }
    }

    largest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supplied_tests() {
        let list = vec![8, -1, 3, 4];
        assert_eq!(max_subarray_sum(&list), 15); //[3,4,8]
        let list = vec![-4, 5, 1, 0];
        assert_eq!(max_subarray_sum(&list), 6);  //[5,1]
    }

    #[test]
    fn extremities_test() {
        let list = vec![-7, -1, -10, -1, -9, -11, -300];
        assert_eq!(max_subarray_sum(&list), 0); //[]

        let list = vec![100, 12, 41, 2, 123, 2, 1];
        assert_eq!(max_subarray_sum(&list), list.iter().sum()); //[]

        let list = vec![];
        assert_eq!(max_subarray_sum(&list), 0); //[]
    }
}
