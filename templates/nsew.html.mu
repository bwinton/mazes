<!DOCTYPE html>
<html>
<head>
<title>{{name}}</title>
<script>
function draw() {
  var canvas = document.getElementById("canvas");
  var ctx = canvas.getContext("2d");
  var grid = [{{grid}}];
  var length = {{length}};

  ctx.fillStyle = "rgb(0,0,0)";
  ctx.strokeStyle = "rgb(0,0,0)";
  ctx.lineWidth = 7;
  ctx.lineCap = "round";
  ctx.lineJoin = "bevel";

  ctx.moveTo({{offset}}, {{offset}});
  ctx.lineTo({{width}} - {{offset}}, {{offset}});

  for (var y = 0; y < length; y++) {
    var yPos = {{offset}} + {{size}} * y;
    var nextYPos = yPos + {{size}};
    ctx.moveTo({{offset}}, yPos);
    ctx.lineTo({{offset}}, nextYPos);
    for (var x = 0; x < length; x++) {
      var xPos = {{offset}} + {{size}} * x;
      var nextXPos = xPos + {{size}};
      if ((grid[x + y * length] & {{S}}) == 0) {
        ctx.moveTo(xPos, nextYPos);
        ctx.lineTo(nextXPos, nextYPos);
      }
      if ((grid[x + y * length] & {{E}}) == 0) {
        ctx.moveTo(nextXPos, yPos);
        ctx.lineTo(nextXPos, nextYPos);
      }
    }
    ctx.stroke();
  };


};
</script>
</head>
<body onload="draw()">{{length}}<br/>
  <canvas id="canvas" width="{{width}}" height="{{height}}"></canvas>
  {{{ascii}}}
</body>
</html>
