/*jshint forin:true, noarg:true, noempty:true, eqeqeq:true, bitwise:true,
strict:true, undef:true, unused:true, curly:true, browser:true, white:true,
moz:true, esnext:false, indent:2, maxerr:50, devel:true, node:true, boss:true,
globalstrict:true, nomen:false, newcap:false */

// Recursive Descent functions.

(function(exports) {
  if (!exports.mazes) {
    exports.mazes = [];
  }

  var maze = {};

  maze.name = 'Recursive Descent';
  maze.link = 'http://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking';
  maze.handlesOwnEnd = true;

  maze.carve_passages_from = function(cx, cy, grid) {
    // work, work, work
    var directions = [util.N, util.S, util.E, util.W];
    util.shuffle(directions);
    for (var i in directions) {
      var direction = directions[i];
      var nx = cx + util.DX[direction];
      var ny = cy + util.DY[direction];
      if (0 <= ny && ny <= grid.length-1 &&
          0 <= nx && nx <= grid[ny].length-1 &&
          grid[ny][nx] == 0) {
        grid[cy][cx] |= direction;
        grid[ny][nx] |= util.OPPOSITE[direction];
        maze.carve_passages_from(nx, ny, grid);
      }
    }
  };

  maze.process = function(req, res) {
    var size = parseInt(req.params[0]);
    var grid = util.newGrid(size, 0);
    maze.carve_passages_from(0, 0, grid);
    util.draw_grid(grid, maze.name, res);
  }

  exports.mazes.push(maze);
})(window);