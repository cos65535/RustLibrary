// rad[i] is longest palindrome radius if center is (i + 1) / 2
#[allow(dead_code)]
fn longest_palindrome(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut rad = vec![0; 2 * n];
    let mut i = 0;
    let mut j = 0;
    while i < 2 * n {
        while i >= j && i + j + 1 < 2 * n && s[(i - j) / 2] == s[(i + j + 1) / 2] {
            j += 1;
        }
        rad[i] = j;
        let mut k = 1;
        while i >= k && rad[i] >= k && rad[i - k] != rad[i] - k {
            rad[i + k] = std::cmp::min(rad[i - k], rad[i] - k);
            k += 1;
        }
        i += k;
        j = if j >= k { j - k } else { 0 };
    }
    for i in 0..2 * n {
        rad[i] = (rad[i] + 1) / 2;
    }
    rad
}

#[test]
fn palindrome_test() {
    let rad = longest_palindrome(&vec!['a', 'b', 'c']);
    assert!(rad == vec![1, 0, 1, 0, 1, 0]);
    let rad = longest_palindrome(&vec!['a', 'a']);
    assert!(rad == vec![1, 1, 1, 0]);
    let rad = longest_palindrome(&vec!['a', 'b', 'a']);
    assert!(rad == vec![1, 0, 2, 0, 1, 0]);
}
