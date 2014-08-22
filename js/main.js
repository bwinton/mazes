/*jshint forin:true, noarg:true, noempty:true, eqeqeq:true, bitwise:true,
strict:true, undef:true, unused:true, curly:true, browser:true, white:true,
moz:true, esnext:false, indent:2, maxerr:50, devel:true, node:true, boss:true,
globalstrict:true, nomen:false, newcap:false */

/*global d3:false */

(function(mazes) {
  if (!mazes) {
    mazes = [];
  }

  var list = d3.select('#mazeList');
  var li = list.selectAll('li.maze').data(mazes)
    .enter().append('li').classed('maze', true);
  li.append('span').classed('name', true)
    .text(d => d.name + ': ')
    .on('click', d => {console.log('BW1', d);})
  li.append('a').attr('href', d => d.link).text('explanation');

})(window.mazes);