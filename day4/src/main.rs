use std::fs::read_to_string;

fn check_for_word<Fi, Fj>(matrix: &Vec<Vec<char>>, h:i32, w:i32, mut i: i32, mut j: i32,
                          word: &Vec<char>, mut fi: Fi, mut fj: Fj) -> bool
    where Fi: FnMut(i32) -> i32, Fj: FnMut(i32) -> i32
{
    for c in 0..word.len() {
        if i < 0 || i >= h || j < 0 || j >= w || word[c] != matrix[i as usize][j as usize] { return false }
        i = fi(i); j = fj(j);
    }
    true
}

fn find_words(matrix: &Vec<Vec<char>>, word: &Vec<char>) -> i32 {
    let h = matrix.len() as i32;
    let w = matrix.get(0).unwrap().len() as i32;
    let mut count: i32 = 0;

    for i in 0..h {
        for j in 0..w {
            if matrix[i as usize][j as usize] != word[0] { continue; }
            if check_for_word(&matrix, h, w, i, j, word, |i|i, |j|j+1) { count += 1; }
            if check_for_word(&matrix, h, w, i, j, word, |i|i+1, |j|j) { count += 1; }
            if check_for_word(&matrix, h, w, i, j, word, |i|i, |j|j-1) { count += 1; }
            if check_for_word(&matrix, h, w, i, j, word, |i|i-1, |j|j) { count += 1; }
            if check_for_word(&matrix, h, w, i, j, word, |i|i+1, |j|j+1) { count += 1; }
            if check_for_word(&matrix, h, w, i, j, word, |i|i-1, |j|j-1) { count += 1; }
            if check_for_word(&matrix, h, w, i, j, word, |i|i+1, |j|j-1) { count += 1; }
            if check_for_word(&matrix, h, w, i, j, word, |i|i-1, |j|j+1) { count += 1; }
        }
    }
    count
}

fn find_shape(matrix: &Vec<Vec<char>>, word: &Vec<char>) -> i32 {
    let h = matrix.len() as i32;
    let w = matrix.get(0).unwrap().len() as i32;
    let m = word.len() as i32 / 2;
    let p = word.len() as i32 - m - 1;
    let mut count: i32 = 0;

    for i in 0..h {
        for j in 0..w {
            if matrix[i as usize][j as usize] != word[m as usize] { continue; }

            if check_for_word(&matrix, h, w, i-p, j-p, word, |i|i+1, |j|j+1) &&
                check_for_word(&matrix, h, w, i+p, j-p, word, |i|i-1, |j|j+1) {
                count = count +1;
            }
            if check_for_word(&matrix, h, w, i-p, j+p, word, |i|i+1, |j|j-1) &&
                check_for_word(&matrix, h, w, i+p, j+p, word, |i|i-1, |j|j-1) {
                count = count +1;
            }
            if check_for_word(&matrix, h, w, i+p, j-p, word, |i|i-1, |j|j+1) &&
                check_for_word(&matrix, h, w, i+p, j+p, word, |i|i-1, |j|j-1) {
                count = count +1;
            }
            if check_for_word(&matrix, h, w, i-p, j-p, word, |i|i+1, |j|j+1) &&
                check_for_word(&matrix, h, w, i-p, j+p, word, |i|i+1, |j|j-1) {
                count = count +1;
            }
        }
    }
    count
}

fn main() {
    let matrix: Vec<Vec<char>> = read_to_string( "src/input.txt").unwrap().lines().map(|l|l.chars().collect()).collect();

    let word1: Vec<char> = "XMAS".chars().collect();
    let part1 = find_words(&matrix, &word1);
    println!("Part 1: {}", part1); assert_eq!(2571, part1);

    let word2: Vec<char> = "MAS".chars().collect();
    let part2 = find_shape(&matrix, &word2);
    println!("Part 2: {}", part2); assert_eq!(1992, part2);
}
