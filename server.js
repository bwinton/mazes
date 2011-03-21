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

function timedResponse(module, handlesOwnEnd) {
  function g(req, res) {
    var startTime = Date.now();
    res.writeHead(200, {"Content-Type": "text/html"});
    module.process(req, res);
    if (!module.handlesOwnEnd)
      res.end();
    var now = Date.now();
    console.log(req.url + ", t="+(now-startTime)+"ms");
  };
  return g;
};

var app = express.createServer();

app.get("/", timedResponse({"process": function(req, res) {
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
}}));

app.get("/recdesc/*", timedResponse(recdesc));
app.get("/eller/*", timedResponse(eller));
app.get("/kruskal/*", timedResponse(kruskal));
app.get("/prim/*", timedResponse(prim));
app.get("/recdiv/*", timedResponse(recdiv));
app.get("/aldous/*", timedResponse(aldous));
app.get("/wilson/*", timedResponse(wilson));
app.get("/huntandkill/*", timedResponse(huntandkill));
app.get("/growingtree/*", timedResponse(growingtree));
app.get("/binarytree/*", timedResponse(binarytree));
app.get("/sidewinder/*", timedResponse(sidewinder));

app.listen(8123);

console.log("Server running at http://127.0.0.1:8123/");
