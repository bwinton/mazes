#!/usr/bin/env node

// Kruskal’s algorithm functions.

var maze = exports;

const S = 2;
const E = 4;
const DX = {2: 0, 4: 1}
const DY = {2: 1, 4: 0}

var mu = require("./lib/mu");
mu.templateRoot = "./templates";

function shuffle(array) {
  var top = array.length;

  if(top) while(--top) {
    var current = Math.floor(Math.random() * (top + 1));
    var tmp = array[current];
    array[current] = array[top];
    array[top] = tmp;
  }

  return array;
}

function connect(set1, set2) {
  var root1 = set1;
  var root2 = set2;
  while (root1[0] != null)
    root1 = root1[0];
  while (root2[0] != null)
    root2 = root2[0];
  var rv = (root1 != root2);
  if (rv)
    root2[0] = root1;
  return rv;
}

maze.name = "Kruskal’s algorithm";
maze.link = "<a href='http://weblog.jamisbuck.org/2011/1/3/maze-generation-kruskal-s-algorithm'>explanation</a>";
maze.handlesOwnEnd = true;

maze.carve_passages = function(edges, sets, grid) {
  while (edges.length) {
    var edge = edges.pop();
    var x = edge[0];
    var y = edge[1];
    var dir = edge[2];
    var nx = x + DX[dir];
    var ny = y + DY[dir];

    var set1 = sets[x][y]
    var set2 = sets[nx][ny]

    if (connect(set1, set2)) {
      grid[y][x] |= dir;
    }
  }
}

maze.asciify_grid = function(grid) {
  var size = grid.length;
  var rv = "<pre>\n " + new Array(size * 2).join("_") + "\n";
  for (var y = 0; y < size; y++) {
    rv += "|";
    for (var x = 0; x < size; x++) {
      var temp = ((grid[y][x] & S) != 0) ? " " : "_";
      if (grid[y][x] & E) {
        temp += (((grid[y][x] | grid[y][x+1]) & S) != 0) ? " " : "_";
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
  var sets = [];
  var edges = [];
  for (var y = 0; y < size; y++) {
    grid[y] = [];
    sets[y] = [];
    for (var x = 0; x < size; x++) {
      grid[y][x] = 0;
      sets[y][x] = [null, (y+","+x)];
      if (y < size-1)
        edges.push([x, y, S]);
      if (x < size-1)
        edges.push([x, y, E]);
    }
  }
  edges = shuffle(edges);
  maze.carve_passages(edges, sets, grid);
  maze.draw_grid(grid, res);
}
