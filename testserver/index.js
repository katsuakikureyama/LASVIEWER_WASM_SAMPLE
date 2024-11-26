// Expressフレームワーク                                                                           
var express = require('express');
var app = express();

// HTTPSサーバー起動                                                                               
app.use(express.static('public'));
app.use(express.static('files'));

var fs = require('fs');
var https = require('https');
var options = {
  key:  fs.readFileSync('./localhost.com-key.pem'),
  cert: fs.readFileSync('./localhost.com.pem')
};
var server = https.createServer(options,app);

app.use('/static', express.static('public'));
app.use('/static/pkg', express.static('../pkg'));

// イベント待機                                                                                    
server.listen(3000);
