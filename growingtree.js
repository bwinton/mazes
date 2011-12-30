#!/usr/bin/env node

var util = require("./mazeutils");

// Growing tree algorithm functions.

var maze = exports;

maze.name = "Growing tree algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

var indexFunctions = {
  "newest": function(limit) { return limit - 1 },
  "oldest": function(limit) { return 0 },
  "random": function(limit) { return util.randint(limit) },
}

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var type = req.params[1];
  if (!(type in indexFunctions)) {
    type = "random";
  }
  var chooseIndex = indexFunctions[type];
  var grid = util.newGrid(size, 0);

  var x = util.randint(size);
  var y = util.randint(size);
  var cells = [];
  cells.push([x, y]);

  while (cells.length) {
    var index = chooseIndex(cells.length);
    x = cells[index][0];
    y = cells[index][1];
    var directions = [util.N, util.S, util.E, util.W];
    util.shuffle(directions);
    for (var i in directions) {
      var direction = directions[i];
      var nx = x + util.DX[direction];
      var ny = y + util.DY[direction];
      if (0 <= ny && ny <= grid.length-1 &&
          0 <= nx && nx <= grid[ny].length-1 &&
          grid[ny][nx] == 0) {
        grid[y][x] |= direction;
        grid[ny][nx] |= util.OPPOSITE[direction];
        cells.push([nx, ny]);
        index = null;
        break;
      }
    }
    if (index !== null)
      cells.splice(index, 1);
  }
  util.draw_grid(grid, maze.name, res);
}
