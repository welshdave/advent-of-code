pub fn find_xmas(input: &str) -> i32 {
    let grid: Vec<Vec<char>> =
    input.lines().map(|line| line.trim().chars().collect()).collect();
    let target = "XMAS";
    let mut result = 0;

    let directions:[(isize,isize); 8] = [
        (0, 1), //right
        (0, -1), //left
        (1, 0), //down
        (-1, 0), //up
        (1, 1), //down-right
        (-1, -1), //up-left
        (1, -1), //down-left
        (-1, 1), //up-right
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = target.chars().collect();
    let word_len = word_chars.len();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == word_chars[0] {
                for &(dir_x, dir_y) in &directions {
                    let mut match_found = true;

                    for k in 0..word_len {
                        let x = i as isize + k as isize * dir_x;
                        let y = j as isize + k as isize * dir_y;

                        if x < 0 || y < 0 || x >= rows as isize || y >= cols as isize {
                            match_found = false;
                            break;
                        }

                        if grid[x as usize][y as usize] != word_chars[k] {
                            match_found = false;
                            break;
                        }
                    }

                    if match_found {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_corrupt_numbers() {
        let input = "\
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX";
        assert_eq!(find_xmas(input), 18);
    }
}
