// https://leetcode.com/problems/word-search/description/

#[allow(dead_code)]
struct Solution;
struct Solution1;
struct Solution2;


#[allow(dead_code)]
// Runtime: 130ms - Beats 54.65% of users with Rust
// Memory: 2.07mb - Beats 66.86% of users with Rust
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let mut board = board;
        let word: Vec<char> = word.chars().collect();
        for i in 0..m {
            for j in 0..n {
                if Solution::dfs(&mut board, &word, i, j) {
                    return true;
                }
            }
        }
        false
    }
    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], x: usize, y: usize) -> bool {
        if word.len() == 1 {
            return board[x][y] == word[0];
        }
        if board[x][y] != word[0] {
            return false;
        }
        let dirs = [[-1, 0], [0, 1], [1, 0], [0, -1]];
        board[x][y] = '*';
        for dir in dirs {
            let x = x as i32 + dir[0];
            let y = y as i32 + dir[1];
            if x < 0 || y < 0 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if x >= board.len() || y >= board[0].len() {
                continue;
            }
            if Solution::dfs(board, &word[1..], x, y) {
                board[x][y] = word[0];
                return true;
            }
        }
        board[x][y] = word[0];
        false
    }
}




// Runtime: 1ms - Beats 98.26% of users with Rust
// Memory: 1.98mb - Beats 98.84% of users with Rust
impl Solution2 {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();    // row (total of sub vecs in the board vec)
        let n = board[0].len(); // number of elements in the sub vec
        let mut board = board;

        // First, converts the input `word` from a string to a character vector `wrd`, 
        let mut wrd: Vec<char> = word.chars().collect();

        // initializes a 128-elements vector `boardf` to keep track of the occurrences of each character in the board.
        let mut boardf = vec![0; 128];

        // Then, iterates through the inputed `board` to populate the `boardf` vec with the frequency of each character.
        for i in 0..m {
            // first sub vec (row)
            for j in 0..n {
                boardf[board[i][j] as usize] += 1;
            }
        }

        // Next, checks if all characters of the word can be found in the board using the boardf array. 
        // If a character is not present in the board, it returns false as it is not possible to construct the word.
        for ch in wrd.iter() {
            if boardf[*ch as usize] == 0 {
                return false;
            }
            boardf[*ch as usize] -= 1;
        }

        // If the word can be constructed from the board, it checks whether the first character of the word is
        // more frequent in the board than the last character. If so, it reverses the wrd array.
        if boardf[wrd[0] as usize] > boardf[wrd[wrd.len() - 1] as usize] {
            Self::reverse(&mut wrd);
        }


        // Finally, performs a depth-first search (DFS) on the board to find the word starting from each cell.
        // Marks the cells as visited while searching and backtracks if a valid path is not found, unmarking the visited cells.
        //let dirs = [0, -1, 0, 1, 0];
        for i in 0..m {
            for j in 0..n {
                if wrd[0] == board[i][j] && Self::found(&mut board, i, j, &wrd, &mut vec![vec![false; n]; m], 0) {
                    return true;
                }
            }
        }

        false
    }

    fn reverse(word: &mut Vec<char>) {
        let n = word.len();
        for i in 0..n / 2 {
            word.swap(i, n - i - 1);
        }
    }

    // dfs:
    fn found(board: &mut Vec<Vec<char>>, row: usize, col: usize, word: &Vec<char>, visited: &mut Vec<Vec<bool>>, index: usize) -> bool {
        if index == word.len() {
            return true;
        }

        if row >= board.len() || col >= board[0].len() || board[row][col] != word[index] || visited[row][col] {
            return false;
        }

        visited[row][col] = true;
        let directions = [0, -1, 0, 1, 0];

        for i in 0..4 {
            if Self::found(board, (row as i32 + directions[i]) as usize, (col as i32 + directions[i + 1]) as usize, word, visited, index + 1) {
                return true;
            }
        }

        visited[row][col] = false;
        false
    }
}



// Runtime: 164ms -  Beats 40.70%of users with Rust
// Memory:2.13mb - Beats 30.23%of users with Rust
// error: dfs's found: bool -> thread 'main' panicked at 'attempt to subtract with overflow'
#[allow(dead_code)]
impl Solution1 {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word_chars: Vec<char> = word.chars().collect();
        let rows = board.len();
        let cols = board[0].len();
        let mut visited = vec![vec![false; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                if Self::dfs(&board, &word_chars, &mut visited, i, j, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        visited: &mut Vec<Vec<bool>>,
        row: usize,
        col: usize,
        index: usize,
    ) -> bool {
        if index == word.len() {
            return true;
        }

        let rows = board.len();
        let cols = board[0].len();

        if row >= rows || col >= cols || visited[row][col] || board[row][col] != word[index] {
            return false;
        }

        visited[row][col] = true;

        // Explore all four possible directions
        let found = Self::dfs(board, word, visited, row + 1, col, index + 1)
            || Self::dfs(board, word, visited, row - 1, col, index + 1)
            || Self::dfs(board, word, visited, row, col + 1, index + 1)
            || Self::dfs(board, word, visited, row, col - 1, index + 1);

        visited[row][col] = false;

        found
    }
}


// .NET solution 1:
// use traditional recursive approach of dfs implementation
// Runtime: 340ms - Beats 79.72% of users with C#
// Memory: 40.86mb - Beats 64.37% of users with C#
/*
public class Solution {
    public bool Exist(char[][] board, string word) {
        int m = board.Length, n = board[0].Length;

        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                if (Dfs(i, j, 0))
                    return true;
        return false;
        
        // traditional recursive
        bool Dfs(int i, int j, int start) {
            if (start == word.Length)
                return true;
            if (i < 0 || i >= m || j < 0 || j >= n || board[i][j] != word[start])
                return false;

            char c = board[i][j];
            board[i][j] = '~';
            bool result = Dfs(i + 1, j, start + 1) || Dfs(i - 1, j, start + 1) 
                    || Dfs(i, j + 1, start + 1) || Dfs(i, j - 1, start + 1);
            board[i][j] = c;
            return result;
        }
    }
}
*/



// .NET solution 2:
// use stack-like recursive approach of dps implementation
// Runtime: 113ms - Beats 99.80 % of users with C#
// Memory: 40.72 - Beats 78.54 % of users with C#
/*
public class Solution {
    public bool Exist(char[][] board, string word) {
        int m = board.Length;
        int n = board[0].Length;
        if (m * n < word.Length)
            return false;
        char[] wrd = word.ToCharArray();
        int[] boardf = new int[128];
        for (int i = 0; i < m; ++i)
        {
            for (int j = 0; j < n; ++j)
            {
                ++boardf[board[i][j]];
            }
        }
        foreach (char ch in wrd)
        {
            if (--boardf[ch] < 0)
            {
                return false;
            }
        }
        if (boardf[wrd[0]] > boardf[wrd[wrd.Length - 1]])
            Array.Reverse(wrd);
        for (int i = 0; i < m; ++i)
        {
            for (int j = 0; j < n; ++j)
            {
                if (wrd[0] == board[i][j]
                    && Found(board, i, j, wrd, new bool[m, n], 0))
                    return true;
            }
        }
        return false;
    }

    private void Reverse(char[] word)
    {
        int n = word.Length;
        for (int i = 0; i < n / 2; ++i)
        {
            char temp = word[i];
            word[i] = word[n - i - 1];
            word[n - i - 1] = temp;
        }
    }

    private static readonly int[] dirs = { 0, -1, 0, 1, 0 };
    private bool Found(char[][] board, int row, int col, char[] word,
        bool[,] visited, int index)
    {
        if (index == word.Length)
            return true;
        if (row < 0 || col < 0 || row == board.Length || col == board[0].Length
            || board[row][col] != word[index] || visited[row, col])
            return false;
        visited[row, col] = true;
        for (int i = 0; i < 4; ++i)
        {
            if (Found(board, row + dirs[i], col + dirs[i + 1],
                word, visited, index + 1))
                return true;
        }
        visited[row, col] = false;
        return false;
    }
}

*/


fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    let word1 = "ABCCED".to_string();
    let word2 = "SEE".to_string();
    let word3 = "ABCB".to_string();

    println!("Word 'ABCCED' exists: {}", Solution2::exist(board.clone(), word1));
    println!("Word 'SEE' exists: {}", Solution2::exist(board.clone(), word2));
    println!("Word 'ABCB' exists: {}", Solution2::exist(board.clone(), word3));

    
    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}