/* tslint:disable:no-console */
import Fluvio, { Offset, Record } from '@fluvio/client';
import * as express from 'express';
import { Request, Response } from 'express';
import * as echarts from 'echarts';

const app = express();
const port = 3000;
const TOPIC_NAME = 'tabulated-high';
const PARTITION = 0;

// Create an array to hold the response objects
const clients: Response[] = [];

// Function to broadcast messages to all connected clients
const broadcastToClients = (message: string) => {
  clients.forEach(client => {
    client.write(`data: ${message}\n\n`);
  });
};

async function consume() {
  try {
    const fluvio = new Fluvio();

    console.log('Connecting client to Fluvio');

    // Connect to the Fluvio cluster
    await fluvio.connect();

    // Create partition consumer
    const consumer = await fluvio.partitionConsumer(TOPIC_NAME, PARTITION);

    console.log('Reading from the beginning');

    await consumer.stream(Offset.FromBeginning(), async (record: Record) => {
      const message = `${record.valueString()}`;
      console.log(message);
      broadcastToClients(message);  // Broadcast the message to all clients
    });
  } catch (ex) {
    console.log('Error', ex);
  }
}

app.get('/', (req: Request, res: Response) => {
  res.send(`
    <!DOCTYPE html>
    <html>
      <head>
      <head>
        <script src="https://cdn.jsdelivr.net/npm/echarts@5.5.1/dist/echarts.min.js"></script>
      </head>
      <body>
        <div id="main" style="width: 600px;height:400px;"></div>
        <div id="messages"></div>
        <script>
          var myChart = echarts.init(document.getElementById('main'));
          var error_data = Array.from({ length: 11 }, () => Array(1).fill(0))

          function updateOption(appendValue){
            series = []
            for(let i = 1; i <= 10; i++){
              error_data[i].push(appendValue[i])
              series.push({
                name: 'Location '+i,
                type: 'line',
                stack: 'Total',
                data: error_data[i]
              })
            }
            error_data.push(appendValue)
            return{
              tooltip: {
                trigger: 'axis'
              },
              xAxis: {
                type: 'category',
              },
              yAxis: {
                type: 'value'
              },
              series: series
            };
          }
          function mapInput(input){
            var ret = Array(11).fill(0)
            for (const item of input) {
              ret[parseInt(item.location)] = item.error_count;
            }
            return ret
          }
          
          myChart.setOption(updateOption(1));
          const eventSource = new EventSource("/events");
          eventSource.onmessage = (event) => {
            const messagesDiv = document.getElementById("messages");
            const newMessage = document.createElement("p");

            let message = event.data;
            newMessage.textContent = message;
            messagesDiv.appendChild(newMessage);

            const jsonArray = JSON.parse(message);
            myChart.setOption(updateOption(mapInput(jsonArray)));
          };
        </script>
      </body>
    </html>
  `);
});

// Endpoint to handle SSE connections
app.get('/events', (req: Request, res: Response) => {
  res.setHeader('Content-Type', 'text/event-stream');
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Connection', 'keep-alive');

  // Add the client response object to the clients array
  clients.push(res);

  // Remove the client from the array when the connection closes
  req.on('close', () => {
    clients.splice(clients.indexOf(res), 1);
  });
});

// Start the server
app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
  consume();
});
