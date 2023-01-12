# sudoku-helper
Helper to solve hard Sudoku puzzles

# Usage

```
sudoku-helper number size
```

It will find groups of **size** numbers of 1 to 9 to sum up to **number**.

```
sudoku-helper number size --require=1,2 --eclude=4,5
```

It will find the same groups, but filter them to have **required** numbers and not having **excluded** ones.

Example of the hard sudoku puzzle:

[![The Sudoku With Only One Visible Cell?!](https://img.youtube.com/vi/b_TdnCGKEAI/maxresdefault.jpg)](https://www.youtube.com/watch?v=b_TdnCGKEAI)
