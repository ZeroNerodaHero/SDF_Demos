"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
/* tslint:disable:no-console */
var client_1 = require("@fluvio/client");
var express = require("express");
var app = express();
var port = 3000;
var TOPIC_NAME = 'tabulated-high';
var PARTITION = 0;
// Create an array to hold the response objects
var clients = [];
// Function to broadcast messages to all connected clients
var broadcastToClients = function (message) {
    clients.forEach(function (client) {
        client.write("data: ".concat(message, "\n\n"));
    });
};
function consume() {
    return __awaiter(this, void 0, void 0, function () {
        var fluvio, consumer, ex_1;
        var _this = this;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    _a.trys.push([0, 4, , 5]);
                    fluvio = new client_1.default();
                    console.log('Connecting client to Fluvio');
                    // Connect to the Fluvio cluster
                    return [4 /*yield*/, fluvio.connect()];
                case 1:
                    // Connect to the Fluvio cluster
                    _a.sent();
                    return [4 /*yield*/, fluvio.partitionConsumer(TOPIC_NAME, PARTITION)];
                case 2:
                    consumer = _a.sent();
                    console.log('Reading from the beginning');
                    return [4 /*yield*/, consumer.stream(client_1.Offset.FromBeginning(), function (record) { return __awaiter(_this, void 0, void 0, function () {
                            var message;
                            return __generator(this, function (_a) {
                                message = "".concat(record.valueString());
                                console.log(message);
                                broadcastToClients(message); // Broadcast the message to all clients
                                return [2 /*return*/];
                            });
                        }); })];
                case 3:
                    _a.sent();
                    return [3 /*break*/, 5];
                case 4:
                    ex_1 = _a.sent();
                    console.log('Error', ex_1);
                    return [3 /*break*/, 5];
                case 5: return [2 /*return*/];
            }
        });
    });
}
app.get('/', function (req, res) {
    res.send("\n    <!DOCTYPE html>\n    <html>\n      <head>\n      <head>\n        <script src=\"https://cdn.jsdelivr.net/npm/echarts@5.5.1/dist/echarts.min.js\"></script>\n      </head>\n      <body>\n        <div id=\"main\" style=\"width: 600px;height:400px;\"></div>\n        <div id=\"messages\"></div>\n        <script>\n          var myChart = echarts.init(document.getElementById('main'));\n          var error_data = Array.from({ length: 11 }, () => Array(1).fill(0))\n\n          function updateOption(appendValue){\n            series = []\n            for(let i = 1; i <= 10; i++){\n              error_data[i].push(appendValue[i])\n              series.push({\n                name: 'Location'+i,\n                type: 'line',\n                stack: 'Total',\n                data: error_data[i]\n              })\n            }\n            error_data.push(appendValue)\n            return{\n              tooltip: {\n                trigger: 'axis'\n              },\n              xAxis: {\n                type: 'category',\n              },\n              yAxis: {\n                type: 'value'\n              },\n              series: series\n            };\n          }\n          function mapInput(input){\n            var ret = Array(11).fill(0)\n            for (const item of input) {\n              console.log(item)\n              ret[parseInt(item.location)] = item.error_count;\n            }\n            return ret\n          }\n          \n          myChart.setOption(updateOption(1));\n          const eventSource = new EventSource(\"/events\");\n          eventSource.onmessage = (event) => {\n            const messagesDiv = document.getElementById(\"messages\");\n            const newMessage = document.createElement(\"p\");\n\n            let message = event.data;\n            newMessage.textContent = message;\n            messagesDiv.appendChild(newMessage);\n\n            \n            const jsonArray = JSON.parse(message);\n            myChart.setOption(updateOption(mapInput(jsonArray)));\n          };\n\n          \n\n          // Display the chart using the configuration items and data just specified.\n        </script>\n      </body>\n    </html>\n  ");
});
// Endpoint to handle SSE connections
app.get('/events', function (req, res) {
    res.setHeader('Content-Type', 'text/event-stream');
    res.setHeader('Cache-Control', 'no-cache');
    res.setHeader('Connection', 'keep-alive');
    // Add the client response object to the clients array
    clients.push(res);
    // Remove the client from the array when the connection closes
    req.on('close', function () {
        clients.splice(clients.indexOf(res), 1);
    });
});
// Start the server
app.listen(port, function () {
    console.log("Server is running on http://localhost:".concat(port));
    consume();
});
