// server.ts
import express from 'express';
import http from 'http';
import WebSocket, { Server as WebSocketServer } from 'ws';
import Fluvio, { Offset, Record } from '@fluvio/client';

const app = express();
const server = http.createServer(app);
const wss = new WebSocketServer({ server });
app.use(express.static('public'));

const TOPIC_NAME = 'helsinki';
const TOPIC_NAME_2 = 'top-vehicle';
const PARTITION = 0;

function appendJSONString(oldJSON:string,newKey: string,newValue: string){
  const newRecord = '"'+newKey+'":"'+newValue+'"';
  return oldJSON.slice(0,-1) + "," + newRecord + "}"
}

wss.on('connection', (ws: WebSocket) => {
  console.log('Client connected');
  const fluvio = new Fluvio();
  const intervalId = setInterval(async () => {
    await fluvio.connect();
    const consumer_position = await fluvio.partitionConsumer(TOPIC_NAME, PARTITION);
    await consumer_position.stream(Offset.FromEnd(), async (record: Record) => {
      let message = `${record.valueString()}`;
      message = appendJSONString(message,"type","vehicle_status");
      const exportData = JSON.stringify(message);
      ws.send(exportData);
    });
    const consumer_door_open = await fluvio.partitionConsumer(TOPIC_NAME_2, PARTITION);
    await consumer_door_open.stream(Offset.FromEnd(), async (record: Record) => {
      let message = `{"topvehicle":${record.valueString()}}`;
      message = appendJSONString(message,"type","top-vehicle");
      console.log(message)
      const exportData = JSON.stringify(message);
      ws.send(exportData);
    });
  }, 1000);

  ws.on('close', () => {
    clearInterval(intervalId);
    console.log('Client disconnected');
  });

  ws.on('message', (message: WebSocket.MessageEvent) => {
    console.log('Received message:', message);
  });
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
  console.log(`Server is listening on port ${PORT}`);
});
