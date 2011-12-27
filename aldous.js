#!/usr/bin/env node

var util = require("./mazeutils");

// Aldous-Broder algorithm functions.

var maze = exports;

maze.name = "Aldous-Broder algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;


function chooseOrientation(width, height) {
  if (width < height)
    return util.HORIZONTAL;
  else if (height < width)
    return util.VERTICAL;
  else
    return [util.HORIZONTAL, util.VERTICAL][util.randint(2)];
}

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = util.newGrid(size, 0);

  var y = util.randint(grid.length);
  var x = util.randint(grid[y].length);
  var remaining = size * size - 1;
  var directions = [util.N, util.S, util.E, util.W];

  while (remaining > 0) {
    util.shuffle(directions);
    for (var i in directions) {
      var direction = directions[i];
      var nx = x + util.DX[direction];
      var ny = y + util.DY[direction];
      if (0 <= ny && ny <= grid.length-1 &&
          0 <= nx && nx <= grid[ny].length-1){
        if (grid[ny][nx] == 0) {
          grid[y][x] |= direction;
          grid[ny][nx] |= util.OPPOSITE[direction];
          remaining -= 1;
        }
        x = nx;
        y = ny;
        break;
      }
    }
  }
  util.draw_grid(grid, maze.name, res);
}

