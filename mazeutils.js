#!/usr/bin/env node

var mu = require("mu");
mu.root = "./templates";

// Aldous-Broder algorithm functions.

var mazeutils = exports;

mazeutils.N = 1;
mazeutils.S = 2;
mazeutils.E = 4;
mazeutils.W = 8;

mazeutils.DX = {1: 0, 2: 0, 4: 1, 8: -1}
mazeutils.DY = {1: -1, 2: 1, 4: 0, 8: 0}

mazeutils.OPPOSITE = {1: mazeutils.S, 2: mazeutils.N, 4: mazeutils.W, 8: mazeutils.E}
mazeutils.NAMES = ["X", "N", "S", "NS", "E", "NE", "SE", "NSE", "W",
                   "NW", "SW", "NSW", "EW", "NEW", "SEW", "NSEW"];

mazeutils.HORIZONTAL = 1;
mazeutils.VERTICAL = 2;

mazeutils.shuffle = function(array) {
  array.sort(function() {return 0.5 - Math.random()});
}

mazeutils.newArray = function(length, val) {
  var array = [];
  for (var i = 0; i < length; i++) {
    array[i] = val;
  }
  return array;
};

mazeutils.newGrid = function(size, value) {
  var grid = [];
  for (var i = 0; i < size; i++) {
    grid.push(mazeutils.newArray(size, value));
  }
  return grid;
}

mazeutils.randint = function(startOrEnd, end) {
  var start = 0;
  if (end)
    start = startOrEnd;
  else
    end = startOrEnd;
  return Math.floor(Math.random() * end) + start;
}

mazeutils.asciify_grid = function(grid) {
  var size = grid.length;
  var rv = "<pre>\n " + new Array(size * 2).join("_") + "\n";
  for (var y = 0; y < size; y++) {
    rv += "|";
    for (var x = 0; x < size; x++) {
      var temp = ((grid[y][x] & mazeutils.S) != 0) ? " " : "_";
      if (grid[y][x] & mazeutils.E) {
        temp += (((grid[y][x] | grid[y][x+1]) & mazeutils.S) != 0) ? " " : "_";
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

mazeutils.draw_grid = function(grid, name, res) {
  const size = 25;
  var context = {
    "name": name,
    "length": grid.length,
    "width": (grid.length + 1) * size,
    "height": (grid.length + 1) * size,
    "size": size,
    "offset": size / 2,
    "grid": grid,
    "S": mazeutils.S,
    "E": mazeutils.E,
    "ascii": mazeutils.asciify_grid(grid),
  };
  mu.compile("nsew.html.mu", function (err, output) {
    if (err) {
      throw err;
    }
    stream = mu.render("nsew.html.mu", context, {});
    stream.addListener("data", function(c) { res.write(c); })
          .addListener("end", function() { res.end(); });
  });
};
