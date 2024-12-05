use std::collections::HashMap;

pub fn find_x_mas(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let target = "MAS";

    let directions: [(isize, isize); 4] = [
        (1, 1),   //down-right
        (-1, -1), //up-left
        (1, -1),  //down-left
        (-1, 1),  //up-right
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = target.chars().collect();
    let word_len = word_chars.len();
    let middle = (word_len - 1) / 2;
    let mut midpoints: HashMap<(isize, isize), usize> = HashMap::new();

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
                        let (mid_x, mid_y) = (
                            i as isize + middle as isize * dir_x,
                            j as isize + middle as isize * dir_y,
                        );
                        *midpoints.entry((mid_x, mid_y)).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    midpoints
        .iter()
        .filter(|&(_, &count)| count > 1)
        .map(|(&midpoint, _)| midpoint)
        .collect::<Vec<(isize, isize)>>()
        .len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_xmas() {
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
        assert_eq!(find_x_mas(input), 9);
    }
}
