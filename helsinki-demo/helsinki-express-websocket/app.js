"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
// server.ts
const express_1 = __importDefault(require("express"));
const http_1 = __importDefault(require("http"));
const ws_1 = require("ws");
const client_1 = __importStar(require("@fluvio/client"));
const app = (0, express_1.default)();
const server = http_1.default.createServer(app);
const wss = new ws_1.Server({ server });
app.use(express_1.default.static('public'));
const TOPIC_NAME = 'helsinki-subset';
const TOPIC_NAME_2 = 'top-vehicle';
const PARTITION = 0;
function appendJSONString(oldJSON, newKey, newValue) {
    const newRecord = '"' + newKey + '":"' + newValue + '"';
    return oldJSON.slice(0, -1) + "," + newRecord + "}";
}
wss.on('connection', (ws) => {
    console.log('Client connected');
    const fluvio = new client_1.default();
    const intervalId = setInterval(() => __awaiter(void 0, void 0, void 0, function* () {
        yield fluvio.connect();
        const consumer_position = yield fluvio.partitionConsumer(TOPIC_NAME, PARTITION);
        yield consumer_position.stream(client_1.Offset.FromEnd(), (record) => __awaiter(void 0, void 0, void 0, function* () {
            let message = `${record.valueString()}`;
            message = appendJSONString(message, "type", "vehicle_status");
            const exportData = JSON.stringify(message);
            ws.send(exportData);
        }));
        const consumer_door_open = yield fluvio.partitionConsumer(TOPIC_NAME_2, PARTITION);
        yield consumer_door_open.stream(client_1.Offset.FromEnd(), (record) => __awaiter(void 0, void 0, void 0, function* () {
            let message = `{"topvehicle":${record.valueString()}}`;
            message = appendJSONString(message, "type", "top-vehicle");
            console.log(message);
            const exportData = JSON.stringify(message);
            ws.send(exportData);
        }));
    }), 1000);
    ws.on('close', () => {
        clearInterval(intervalId);
        console.log('Client disconnected');
    });
    ws.on('message', (message) => {
        console.log('Received message:', message);
    });
});
const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
    console.log(`Server is listening on port ${PORT}`);
});
