const fs = require("fs");

const input_string = fs.readFileSync("./input.txt").toString().trim();
// const input_string = `XMAS
// XMAS
// XMAS`;

const lines = input_string.split("\n");

const grid = lines.map((l) => l.split(""));
const word = ["X", "M", "A", "S"];
const solutions_grid = grid.map((_) => new Array(140).fill("."));

let outstring = "";

let solutions = 0;

let lineCount = 0;
grid.forEach((row, y0) => {
  row.forEach((char, x0) => {
    if (char !== "X") return;
    if (grid[y0][x0] !== "X") {
      console.log("lineCount, charCount:", lineCount, charCount);
      console.log("grid[lineCount][charCount]:", grid[lineCount][charCount]);
    }
    for (let dx of [-1, 0, 1]) {
      for (let dy of [-1, 0, 1]) {
        if (dx === 0 && dy === 0) continue;

        const found = word.every((letter, i) => {
          const x = x0 + dx * i;
          const y = y0 + dy * i;

          if (grid[y] !== undefined) {
            return grid[y][x] === letter;
          }
        });

        if (found) {
          word.forEach((letter, i) => {
            const x = x0 + dx * i;
            const y = y0 + dy * i;

            if (grid[y] !== undefined && grid[y][x] === letter) {
              solutions_grid[y][x] = letter;
            }
          });

          solutions += 1;

          outstring += `(${x0}, ${y0}), [${dx}, ${dy}]\n`;
        }
      }
    }
  });
});

solutions_grid.forEach((ary) => console.log(ary.join("")));

console.log("solutions:", solutions);

fs.writeFileSync("./solution.txt", outstring);
