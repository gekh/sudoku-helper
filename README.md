# sudoku-helper
Helper to solve hard Sudoku puzzles

# Usage

```
sudoku-helper number size
```

So it will find groups of **size** number of 1 to 9 to sum up to **number**

```
sudoku-helper number size --require=1,2 --eclude=4,5
```

It will find the same groups, but filter them to have **required** numbers and not having **excluded** ones.