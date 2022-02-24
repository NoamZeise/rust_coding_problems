/*
Given an array of positive integers, divide the array into two subsets
such that the difference between the sum of the subsets is as small as possible.
*/

pub fn smallest_difference_subsets(list: &mut Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut sub_one: Vec<i32> = Vec::new();
    let mut sub_one_total = 0;
    let mut sub_two: Vec<i32> = Vec::new();
    let mut sub_two_total = 0;

    list.sort_unstable();

    for e in list.iter().rev() {
        if sub_one_total > sub_two_total {
            sub_two.push(*e);
            sub_two_total += sub_two.last().unwrap();
        } else {
            sub_one.push(*e);
            sub_one_total += sub_one.last().unwrap();
        }
    }

    (sub_one, sub_two)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn abs(val: i32) -> i32 {
        match val < 0 {
            true => -val,
            false => val,
        }
    }

    fn difference(sublists: &(Vec<i32>, Vec<i32>)) -> i32 {
        abs(sublists.0.iter().sum::<i32>() - sublists.1.iter().sum::<i32>())
    }

    #[test]
    fn example_test() {
        let mut list = vec![5, 10, 15, 20, 25];
        let sublists = smallest_difference_subsets(&mut list);
        assert_eq!(difference(&sublists), 5);
    }

    #[test]
    fn test_1() {
        let mut list = vec![100, 10, 100, 10, 210];
        let sublists = smallest_difference_subsets(&mut list);
        assert_eq!(difference(&sublists), 10);
    }

    #[test]
    fn test_2() {
        let mut list = vec![1000, 800, 1, 1, 1];
        let sublists = smallest_difference_subsets(&mut list);
        assert_eq!(difference(&sublists), 197);
    }
}
