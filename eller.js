#!/usr/bin/env node

// Eller functions.

var maze = exports;

const S = 2;
const E = 4;

var mu = require("./lib/mu");
mu.templateRoot = "./templates";

function newArray(length, val) {
  var array = [];
  for (var i = 0; i < length; i++) {
    array[i] = val;
  }
  return array;
};

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
        grid[y][x] |= E;
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
      var verticals = Math.floor(Math.random() * set.length) + 1;
      set.sort(function() {return 0.5 - Math.random()});
      for (var j = 0; j < verticals; j++) {
        sets.addToSet(set[j], i);
        grid[y][set[j]] |= S;
      }
    }
    var temp = [];
    for (var x = 0; x < size; x++) {
      temp.push(sets.setForCell(x) == 0 ? "-" : " ");
    }
  }
};

maze.asciify_grid = function(grid) {
  var size = grid.length;
  var rv = "<pre>\n";
  for (var y = 0; y < size; y++) {
    for (var x = 0; x < size; x++) {
      rv += ("0" + grid[y][x]).substr(-2);
      rv += (grid[y][x] == grid[y][x+1]) ? " " : "|";
    }
    rv += "\n";
  }
  rv += "</pre>";
  return rv;
};

maze.draw_grid = function(grid, res) {
  const size = 25;
  var context = {
    "name": maze.name,
    "length": grid.length,
    "width": (grid.length + 1) * size,
    "height": (grid.length + 1) * size,
    "size": size,
    "offset": size / 2,
    "grid": grid,
    "S": S,
    "E": E,
    "ascii": maze.asciify_grid(grid),
  };
  mu.render("nsew.html", context, {}, function (err, output) {
    if (err) {
      throw err;
    }
    output.addListener("data", function(c) { res.write(c); })
          .addListener("end", function() { res.end(); });
  });
};

maze.process = function(req, res) {
  var size = parseInt(req.params[0]);
  var grid = [];
  for (var i = 0; i < size; i++) {
    grid.push(newArray(size, 0));
  }
  maze.carve_passages_from(0, 0, grid);
  maze.draw_grid(grid, res);
}
