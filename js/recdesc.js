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

  // Metadata
  var maze = {};
  maze.name = 'Recursive Descent';
  maze.link = 'http://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking';


  // Algorithm
  var done = false;
  var grid = null;
  var work = [];
  var lastUpdate = 0;
  const MS_PER_STEP = 200;

  var add_work = function (x, y) {
    var newc = {
      x: x,
      y: y,
      directions: [util.N, util.S, util.E, util.W]
    }
    util.shuffle(newc.directions);
    work.push(newc);
  }

  var carve_passage_from = function (c) {
    if (c.directions.length) {
      var direction = c.directions.pop();
      var nx = c.x + util.DX[direction];
      var ny = c.y + util.DY[direction];
      if (0 <= ny && ny <= grid.length-1 &&
          0 <= nx && nx <= grid[ny].length-1 &&
          grid[ny][nx] == 0) {
        grid[c.y][c.x] |= direction;
        grid[ny][nx] |= util.OPPOSITE[direction];
        add_work(nx, ny);
      }
    }
    c.seen = true;
    return;
  }

  var carve_next_passage = function () {
    var startLength = work.length;
    while (work.length == startLength) {
      var current = work[work.length - 1];
        if (!current) {
          done = true;
          return;
        }
        if (current.directions.length === 0) {
          work.pop();
        } else {
          carve_passage_from(current);
        }
    }
    return current;
  }


  // Drawing
  var draw_current = function (mazeElem) {
    mazeElem.selectAll('rect.current').remove();

    var x = mazeutils.x;
    var y = mazeutils.y;

    mazeElem.selectAll('rect.current').data(work)
      .enter().append('rect').classed('current', true)
      .attr('x', 0).attr('y', 0)
      .attr('width', x(2) - x(1)).attr('height', y(2) - y(1))
      .attr('stroke', 'rgba(0,0,0,0)')
      .attr('fill', (d, i) => i == work.length ? 'rgba(136,255,170,0.3)' : 'rgba(136,170,255,0.3)')
      .attr('transform', d => 'translate(' + x(d.x) + ',' + y(d.y) + ')')
  }

  var update_current = function (mazeElem) {
    var x = mazeutils.x;
    var y = mazeutils.y;

    var rects = mazeElem.selectAll('rect.current').data(work)

    rects.enter().append('rect').classed('current', true)
      .attr('x', 0).attr('y', 0)
      .attr('width', x(2) - x(1)).attr('height', y(2) - y(1))
      .attr('stroke', 'rgba(0,0,0,0)');

    rects.attr('fill', (d, i) => (i === work.length - 1) ? 'rgba(136,170,255,0.3)' : 'rgba(136,255,170,0.3)')
      .attr('transform', d => 'translate(' + x(d.x) + ',' + y(d.y) + ')')

    rects.exit().remove();
  }

  // Scaffolding.
  maze.init = function (size, mazeElem) {
    grid = util.newGrid(size, 0);
    util.draw_grid(grid, mazeElem);
    draw_current(mazeElem);
    done = false;
    var work = [];
    add_work(0, 0);
  }

  maze.step = function (time, mazeElem) {
    if (lastUpdate >= time) {
      return done;
    }
    while (lastUpdate < time) {
      lastUpdate += MS_PER_STEP;
      var current = carve_next_passage();
      util.update_grid(grid, mazeElem);
      update_current(mazeElem);
      if (done) {
        console.log(grid);
      }
    }
    return done;
  };

  maze.stop = function (mazeElem) {
    done = true;
    mazeElem.selectAll('rect.current').remove();
  };

  exports.mazes.push(maze);
})(window);
