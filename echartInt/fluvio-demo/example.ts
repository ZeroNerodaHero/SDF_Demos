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
      const message = `Key=${record.keyString()}, Value=${record.valueString()}`;
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
        <script>
          var myChart = echarts.init(document.getElementById('main'));

          // Specify the configuration items and data for the chart
          var option = {
            title: {
              text: 'ECharts Getting Started Example'
            },
            tooltip: {},
            legend: {
              data: ['sales']
            },
            xAxis: {
              data: ['Shirts', 'Cardigans', 'Chiffons', 'Pants', 'Heels', 'Socks']
            },
            yAxis: {},
            series: [
              {
                name: 'sales',
                type: 'bar',
                data: [5, 20, 36, 10, 10, 20]
              }
            ]
          };

          // Display the chart using the configuration items and data just specified.
          myChart.setOption(option);
        </script>
        <div id="messages"></div>
        <script>
          const eventSource = new EventSource("/events");
          eventSource.onmessage = (event) => {
            const messagesDiv = document.getElementById("messages");
            const newMessage = document.createElement("p");
            newMessage.textContent = event.data;
            messagesDiv.appendChild(newMessage);

            let message = event.data;
            const regex = /Value=(\[\{[^]*?\}\])/;
            const match = message.match(regex);

            if (match) {
              const jsonArray = JSON.parse(match[1]);
              console.log(jsonArray);
            }
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
