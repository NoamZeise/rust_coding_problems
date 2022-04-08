/*
Write a program that computes the length of the longest common subsequence of three given strings.
 For example, given "epidemiologist", "refrigeration", and "supercalifragilisticexpialodocious",
 it should return 5, since the longest common subsequence is "eieio".
*/

pub fn subsequence_length(words :Vec<&str>) -> usize {

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        let words = vec!["epidemiologist", "refrigeration", "supercalifragilisticexpialodocious"];
        assert_eq!(subsequence_length(words), 5); //"eieio"
    }
}
