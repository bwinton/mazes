#!/usr/bin/env node

var util = require("./mazeutils");

// Kruskal’s algorithm functions.

var maze = exports;

function shuffle(array) {
  var top = array.length;

  if(top) while(--top) {
    var current = util.randint(1, top);
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
    var nx = x + util.DX[dir];
    var ny = y + util.DY[dir];

    var set1 = sets[x][y];
    var set2 = sets[nx][ny];

    if (connect(set1, set2)) {
      grid[y][x] |= dir;
    }
  }
}

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
        edges.push([x, y, util.S]);
      if (x < size-1)
        edges.push([x, y, util.E]);
    }
  }
  edges = shuffle(edges);
  maze.carve_passages(edges, sets, grid);
  util.draw_grid(grid, maze.name, res);
}
