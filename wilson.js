#!/usr/bin/env node

var util = require("./mazeutils");

// Wilson’s algorithm functions.

const IN = 0x10;
const FRONTIER = 0x20;

var maze = exports;

maze.name = "Wilson’s algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/20/maze-generation-wilson-s-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

function walk(grid) {
  var x = util.randint(grid[0].length);
  var y = util.randint(grid.length);
  var visits = {};
  while (grid[y][x] == 0) {
    visits[[x,y]] = 0;

    // where the random walk started
    var start_x = x;
    var start_y = y;
    walking = true;

    while (walking) {
      walking = false;

      var directions = [util.N, util.S, util.E, util.W];
      util.shuffle(directions);
      for (var i in directions) {
        var direction = directions[i];
        var nx = x + util.DX[direction];
        var ny = y + util.DY[direction];
        if (0 <= ny && ny <= grid.length-1 &&
            0 <= nx && nx <= grid[ny].length-1) {
          visits[[x, y]] = direction;
          if (grid[ny][nx] != 0) {
            break;
          } else {
            x = nx;
            y = ny;
            walking = true;
            break;
          }
        }
      }
    }
    var x = util.randint(grid[0].length);
    var y = util.randint(grid.length);
  }

  var path = [];
  x = start_x;
  y = start_y;
  while (visits[[x,y]]) {
    direction = visits[[x,y]];
    path.push([x, y, direction]);
    x = x + util.DX[direction];
    y = y + util.DY[direction];
  }
  return path;
}

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = util.newGrid(size, 0);

  grid[util.randint(size)][util.randint(size)] = IN;
  var remaining = size * size - 1;

  while (remaining > 0) {
    path = walk(grid);
    for (var i in path) {
      var x = path[i][0];
      var y = path[i][1];
      var direction = path[i][2];
      var nx = x + util.DX[direction];
      var ny = y + util.DY[direction];
      grid[y][x] |= direction;
      grid[ny][nx] |= util.OPPOSITE[direction];
      remaining -= 1;
    }
  }

  util.draw_grid(grid, maze.name, res);
}
