#!/usr/bin/env node

var express = require("express");
var http = require("http");
var sys = require("sys");
var url = require("url");

// Mazes.
var recdesc = require("./recdesc");
var eller = require("./eller");
var kruskal = require("./kruskal");
var prim = require("./prim");
var recdiv = require("./recdiv");
var aldous = require("./aldous");
var wilson = require("./wilson");
var huntandkill = require("./huntandkill");
var growingtree = require("./growingtree");
var binarytree = require("./binarytree");
var sidewinder = require("./sidewinder");

// Utility Functions.

function timedResponse(f) {
  function g(req, res) {
    var startTime = Date.now();
    res.writeHead(200, {"Content-Type": "text/html"});
    f(req, res);
    res.end();
    var now = Date.now();
    console.log(req.url + ", t="+(now-startTime)+"ms");
  };
  return g;
};

var app = express.createServer();

app.get("/", timedResponse(function(req, res) {
  res.write("<html><head><title>Mazes</title></head><body>\n");
  res.write("<ul>\n");
  res.write("<li><a href='recdesc/10'>"+recdesc.name+"</a>: "+recdesc.link+"</li>\n");
  res.write("<li><a href='eller/10'>"+eller.name+"</a>: "+eller.link+"</li>\n");
  res.write("<li><a href='kruskal/10'>"+kruskal.name+"</a>: "+kruskal.link+"</li>\n");
  res.write("<li><a href='prim/10'>"+prim.name+"</a>: "+prim.link+"</li>\n");
  res.write("<li><a href='recdiv/10'>"+recdiv.name+"</a>: "+recdiv.link+"</li>\n");
  res.write("<li><a href='aldous/10'>"+aldous.name+"</a>: "+aldous.link+"</li>\n");
  res.write("<li><a href='wilson/10'>"+wilson.name+"</a>: "+wilson.link+"</li>\n");
  res.write("<li><a href='huntandkill/10'>"+huntandkill.name+"</a>: "+huntandkill.link+"</li>\n");
  res.write("<li><a href='growingtree/10'>"+growingtree.name+"</a>: "+growingtree.link+"</li>\n");
  res.write("<li><a href='binarytree/10'>"+binarytree.name+"</a>: "+binarytree.link+"</li>\n");
  res.write("<li><a href='sidewinder/10'>"+sidewinder.name+"</a>: "+sidewinder.link+"</li>\n");
  res.write("</ul>\n</body></html>");
}));

app.get("/recdesc/*", timedResponse(recdesc.process));
app.get("/eller/*", timedResponse(eller.process));
app.get("/kruskal/*", timedResponse(kruskal.process));
app.get("/prim/*", timedResponse(prim.process));
app.get("/recdiv/*", timedResponse(recdiv.process));
app.get("/aldous/*", timedResponse(aldous.process));
app.get("/wilson/*", timedResponse(wilson.process));
app.get("/huntandkill/*", timedResponse(huntandkill.process));
app.get("/growingtree/*", timedResponse(growingtree.process));
app.get("/binarytree/*", timedResponse(binarytree.process));
app.get("/sidewinder/*", timedResponse(sidewinder.process));

app.listen(8123);

console.log("Server running at http://127.0.0.1:8123/");
