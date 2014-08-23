/*jshint forin:true, noarg:true, noempty:true, eqeqeq:true, bitwise:true,
strict:true, undef:true, unused:true, curly:true, browser:true, white:true,
moz:true, esnext:false, indent:2, maxerr:50, devel:true, node:true, boss:true,
globalstrict:true, nomen:false, newcap:false */

// Recursive Descent functions.

(function (exports) {
  if (!exports.mazes) {
    exports.mazes = [];
  }
  var util = exports.mazeutils;

  var maze = {};

  maze.name = 'Recursive Descent';
  maze.link = 'http://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking';

  var done = false;
  var grid = null;
  var work = [];

  var carve_passage_from = function (c) {
    var directions = [util.N, util.S, util.E, util.W];
    util.shuffle(directions);
    for (var i in directions) {
      var direction = directions[i];
      var nx = c.x + util.DX[direction];
      var ny = c.y + util.DY[direction];
      if (0 <= ny && ny <= grid.length-1 &&
          0 <= nx && nx <= grid[ny].length-1 &&
          grid[ny][nx] == 0) {
        grid[c.y][c.x] |= direction;
        grid[ny][nx] |= util.OPPOSITE[direction];
        work.push({x: nx, y: ny, seen: false});
      }
    }
    c.seen = true;
    return;
  }

  var carve_next_passage = function () {
    var current = work[work.length - 1];
    if (!current) {
      done = true;
      return;
    }
    if (current.seen) {
      work.pop();
    } else {
      carve_passage_from(current);
    }
    return current;
  }


  maze.init = function (size, mazeElem) {
    grid = util.newGrid(size, 0);
    // console.log(util.asciify_grid(grid));
    util.draw_grid(grid, mazeElem);
    done = false;
    work.push({x: 0, y: 0, seen: false});
  }

  maze.step = function (time, mazeElem) {
    var current = carve_next_passage();
    // console.log(util.asciify_grid(grid, current));
    util.update_grid(grid, current, mazeElem);
    if (done) {
      console.log(grid);
    }
    return done;
    // return true;
  };

  maze.stop = function () {
    done = true;
  };

  exports.mazes.push(maze);
})(window);