/*jshint forin:true, noarg:true, noempty:true, eqeqeq:true, bitwise:true,
strict:true, undef:true, unused:true, curly:true, browser:true, white:true,
moz:true, esnext:false, indent:2, maxerr:50, devel:true, node:true, boss:true,
globalstrict:true, nomen:false, newcap:false */

// Eller functions.

(function (exports) {
  if (!exports.mazes) {
    exports.mazes = [];
  }
  var util = exports.mazeutils;

  // Metadata
  var maze = {};
  maze.name = 'Eller’s Algorithm';
  maze.link = 'http://weblog.jamisbuck.org/2010/12/29/maze-generation-eller-s-algorithm';


  // Algorithm
  var done = false;
  var grid = null;
  var work = [];
  var setScale = d3.scale.category20();

  var sets = {
    current: 1,
    allSets: {},

    next: function() {
      return sets.current++;
    },

    addToSet: function(x, set) {
      if (!sets.allSets[set])
        sets.allSets[set] = {};
      sets.allSets[set][x] = true;
    },

    removeFromSet: function(x, set) {
      delete sets.allSets[set][x];
      if (Object.keys(sets.allSets[set]).length == 0) {
        delete sets.allSets[set];
      }
    },

    setForCell: function(x) {
      for (var i in sets.allSets) {
        var set = sets.allSets[i];
        if (set[x]) {
          return i;
        }
      }
      return 0;
    },

    merge: function(x) {
      var newSet = sets.setForCell(x);
      var oldSet = sets.setForCell(x+1);
      for (var i in sets.allSets[oldSet]) {
        sets.removeFromSet(i, oldSet);
        sets.addToSet(i, newSet);
      }
    },

    clear: function() {
      sets.allSets = {};
    },
  };


  var add_work = function (row, size) {
    var newc = {
      row: row,
      size, size
    }
    work.push(newc);
  }

  maze.carve_passages_from = function(cx, cy, grid) {
    sets.current = 1;
    var size = grid.length;

    // Start with the first row.
    for (var y = 0; y < size; y++) {

      // Assign each remaining cell to its own set.
      for (var x = 0; x < size; x++) {
        var set = sets.setForCell(x);
        if (set == 0)
          set = sets.next();
        sets.addToSet(x, set);
      }

      // Randomly merge adjacent sets.
      for (var x = 0; x < size-1; x++) {
        if (((Math.random() > 0.5) ||
             (y == size - 1)) &&
           (sets.setForCell(x) != sets.setForCell(x+1))) {
          grid[y][x] |= util.E;
          sets.merge(x);
        }
      }

      var prevSets = sets.allSets;
      sets.clear();

      if (y == size - 1)
        break;

      // Randomly determine the vertical connections, at least one per set.
      for (var i in prevSets) {
        var set = Object.keys(prevSets[i]);
        var verticals = util.randint(1, set.length);
        util.shuffle(set);
        for (var j = 0; j < verticals; j++) {
          sets.addToSet(set[j], i);
          grid[y][set[j]] |= util.S;
        }
      }
      var temp = [];
      for (var x = 0; x < size; x++) {
        temp.push(sets.setForCell(x) == 0 ? "-" : " ");
      }
    }
  };

  var carve_next_passage = function () {
    var current = work[work.length - 1];
    if (!current) {
      done = true;
      return;
    }
    work.pop();
    var y = current.row;
    var size = current.size;

    // Assign each remaining cell to its own set.
    for (var x = 0; x < size; x++) {
      var set = sets.setForCell(x);
      if (set == 0)
        set = sets.next();
      sets.addToSet(x, set);
    }

    // Randomly merge adjacent sets.
    for (var x = 0; x < size-1; x++) {
      if (((Math.random() > 0.5) ||
           (y == size - 1)) &&
         (sets.setForCell(x) != sets.setForCell(x+1))) {
        grid[y][x] |= util.E;
        grid[y][x + 1] |= util.W;
        sets.merge(x);
      }
    }

    var prevSets = sets.allSets;
    sets.clear();

    if (y == size - 1)
      return;

    // Randomly determine the vertical connections, at least one per set.
    for (var i in prevSets) {
      var set = Object.keys(prevSets[i]);
      var verticals = util.randint(1, set.length);
      util.shuffle(set);
      for (var j = 0; j < verticals; j++) {
        sets.addToSet(set[j], i);
        grid[y][set[j]] |= util.S;
        grid[y + 1][set[j]] |= util.N;
      }
    }
    return current;
  }

  // Drawing
  var draw_current = function (mazeElem, size) {
    mazeElem.selectAll('rect.current').remove();

    var x = mazeutils.x;
    var y = mazeutils.y;

    mazeElem.selectAll('rect.current').data(d3.range(0, size))
      .enter().append('rect').classed('current', true)
      .attr('x', 0).attr('y', 0)
      .attr('width', x(2) - x(1)).attr('height', y(2) - y(1))
      .attr('stroke', 'rgba(0,0,0,0)')
      .attr('fill', (d, i) => 'rgba(136,170,255,0.3)')
      .attr('transform', d => 'translate(' + x(d) + ',' + y(0) + ')')
  };

  var update_current = function (mazeElem, current) {
    var x = mazeutils.x;
    var y = mazeutils.y;

    var rects = mazeElem.selectAll('rect.current')

    if (current) {
      rects.attr('fill', (d, i) => setScale(sets.setForCell(d)))
        .attr('transform', d => 'translate(' + x(d) + ',' + y(current.row) + ')')
    } else {
      rects.remove();
    }
  };

  // Scaffolding.
  maze.init = function (size, mazeElem) {
    grid = util.newGrid(size, 0);
    util.draw_grid(grid, mazeElem);
    draw_current(mazeElem, size);
    done = false;
    // Add some work for each row.
    var work = [];
    for (var y = size - 1; y >= 0; y--) {
      add_work(y, size);
    }
  };

  maze.step = function (time, mazeElem) {
    var current = carve_next_passage();
    util.update_grid(grid, mazeElem);
    update_current(mazeElem, current);
    if (done) {
      console.log(grid);
    }
    return done;
  };

  maze.stop = function (mazeElem) {
    done = true;
    mazeElem.selectAll('rect.current').remove();
  };


  exports.mazes.push(maze);
})(window);
