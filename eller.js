#!/usr/bin/env node

// Eller functions.

var maze = exports;

const N = 1;
const S = 2;
const E = 4;
const W = 8;
const DX = {1: 0, 2: 0, 4: 1, 8: -1}
const DY = {1: -1, 2: 1, 4: 0, 8: 0}
const OPPOSITE = {1: S, 2: N, 4: W, 8: E}
const NAMES = ["X", "N", "S", "NS", "E", "NE", "SE", "NSE", "W",
               "NW", "SW", "NSW", "EW", "NEW", "SEW", "NSEW"];

function newArray(length, val) {
  var array = [];
  for (var i = 0; i < length; i++) {
    array[i] = val;
  }
  return array;
};

maze.name = "Eller’s Algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2010/12/29/maze-generation-eller-s-algorithm'>explanation</a>";

maze.carve_passages_from = function(cx, cy, grid) {
  // work, work, work
  var directions = [N, S, E, W];
  directions.sort(function() {return 0.5 - Math.random()});
  for (var i in directions) {
    var direction = directions[i];
    var nx = cx + DX[direction];
    var ny = cy + DY[direction];
    if (0 <= ny && ny <= grid.length-1 &&
        0 <= nx && nx <= grid[ny].length-1 &&
        grid[ny][nx] == 0) {
      grid[cy][cx] |= direction
      grid[ny][nx] |= OPPOSITE[direction]
      maze.carve_passages_from(nx, ny, grid)
    }
  }
};

maze.asciify_grid = function(grid) {
  var size = grid.length;
  var rv = "<pre>\n " + new Array(size * 2).join("_") + "\n";
  for (var y = 0; y < size; y++) {
    rv += "|";
    for (var x = 0; x < size; x++) {
      var temp = ((grid[y][x] & S) != 0) ? " " : "_";
      if ((grid[y][x] & E) != 0) {
        temp += (((grid[y][x] | grid[y][x+1]) & S) != 0) ? " " : "_";
      }
      else {
        temp += "|";
      }
      rv += temp;
    }
    rv += "\n";
  }
  rv += "</pre>";
  return rv;
};

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = [];
  for (var i = 0; i < size; i++) {
    grid.push(newArray(size, 0));
  }
  res.write(grid.length + "<br>");
  maze.carve_passages_from(0, 0, grid);
  res.write(maze.asciify_grid(grid));
}
