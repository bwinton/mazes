// Aldous-Broder algorithm functions.

(function (exports) {
  var mazeutils = {};

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
    if (grid === null) {
      return "null";
    }

    var size = grid.length;
    var rv = new Array(size * 2).join("_") + "\n";
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
    return rv;
  };

  var margin = {top: 20, right: 25, bottom: 25, left: 20};
  var width = 800;
  var height = 800;

  var x = d3.scale.linear()
    .range([margin.left, width - margin.left - margin.right]);

  var y = d3.scale.linear()
    .range([margin.top, height - margin.top - margin.bottom]);

  mazeutils.x = x;
  mazeutils.y = y;

  mazeutils.draw_grid = function(grid, mazeElem) {
    x.domain([0, grid.length - 1]);
    y.domain([0, grid[0].length - 1]);

    mazeElem.selectAll('g.row').remove();

    var cells = mazeElem.selectAll('g.row').data(grid)
      .enter().append('g').classed('row', true)
      .attr('transform', (d, i) => 'translate(0,' + y(i) + ')')
      .selectAll('g.cell').data(d => d)
        .enter().append('g').classed('cell', true)
        .attr('transform', (d, i) => 'translate(' + x(i) + ',0)');
    cells.append('line').classed('top', true)
      .attr('x1', 0).attr('y1', 0)
      .attr('x2', x(2) - x(1)).attr('y2', 0);
    cells.append('line').classed('left', true)
      .attr('x1', 0).attr('y1', 0)
      .attr('x2', 0).attr('y2', y(2) - y(1));
    cells.append('line').classed('bottom', true)
      .attr('x1', 0).attr('y1', y(2) - y(1))
      .attr('x2', x(2) - x(1) ).attr('y2', y(2) - y(1));
    cells.append('line').classed('right', true)
      .attr('x1', x(2) - x(1)).attr('y1', 0)
      .attr('x2', x(2) - x(1)).attr('y2', y(2) - y(1));
  };

  mazeutils.update_grid = function(grid, mazeElem) {
    var cells = mazeElem.selectAll('g.cell').data(d3.merge(grid));
    cells.select('line.top').classed('hidden', d => d & mazeutils.N);
    cells.select('line.left').classed('hidden', d => d & mazeutils.W);
    cells.select('line.bottom').classed('hidden', d => d & mazeutils.S);
    cells.select('line.right').classed('hidden', d => d & mazeutils.E);
  };

  exports.mazeutils = mazeutils;
})(window);