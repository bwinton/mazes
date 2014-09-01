/*jshint forin:true, noarg:true, noempty:true, eqeqeq:true, bitwise:true,
strict:true, undef:true, unused:true, curly:true, browser:true, white:true,
moz:true, esnext:false, indent:2, maxerr:50, devel:true, node:true, boss:true,
globalstrict:true, nomen:false, newcap:false */

/*global d3:false */

(function (mazes) {
  if (!mazes) {
    mazes = [];
  }

  var list = d3.select('#mazeList');
  var mazeElem = d3.select('#maze');

  if (!mazes.length) {
    list.append('li').classed('maze error', true)
      .text('No mazes available.');
    return;
  }

  var currentMaze;
  var setMaze = function (maze) {
    if (currentMaze) {
      currentMaze.stop(mazeElem);
    }
    currentMaze = maze;
    currentMaze.init(30, mazeElem);
    d3.timer(function (time) {
      return currentMaze.step(time, mazeElem);
    }, 1000);
  }

  var li = list.selectAll('li.maze').data(mazes)
    .enter().append('li').classed('maze', true);
  li.append('span').classed('name', true)
    .text(d => d.name + ': ')
    .on('click', d => {
      setMaze(d);
    })
  li.append('a').attr('href', d => d.link).text('explanation');

  setMaze(mazes[0]);

})(window.mazes);
