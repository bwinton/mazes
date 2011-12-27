#!/usr/bin/env node

var util = require("./mazeutils");

// Eller functions.

var maze = exports;

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


maze.name = "Eller’s Algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2010/12/29/maze-generation-eller-s-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

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

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = util.newGrid(size, 0);
  maze.carve_passages_from(0, 0, grid);
  util.draw_grid(grid, maze.name, res);
}
