// src/App.js
import React, { useEffect, useState, useContext, createContext } from 'react';
import { MapContainer, Marker, Popup, TileLayer, useMap } from "react-leaflet";
import "./App.css"

const BusListContext = createContext();

function App() {
  const [data,setData] = useState({})
  useEffect(() => {
    const socket = new WebSocket('ws://localhost:3000');
    socket.addEventListener('open', (event) => {
      console.log('Connected to WebSocket server');
    });
    socket.addEventListener('message', (event) => {
      const ret = JSON.parse(event.data.replace(/^"|"$/g, '').replace(/\\"/g, '"'));
      setData({id:ret.vehicle, lat: ret.lat, lon: ret.long, route: ret.route})
    });
    socket.addEventListener('close', () => {
      console.log('Disconnected from WebSocket server');
    });
    return () => {
      socket.close();
    };
  }, []);

  return (
    <div>
      <BusListContext >
        <Map new_bus={data}/>
      </BusListContext>
    </div>
  );
}

function MarkerList({updateBus}){
  const [busList,setBusList] = useState({})
  const updateBusList = (newBusData)=>{
      const newBus = newBusData.updateBus
      const new_key = newBus.id + "_" + newBus.route;
      setBusList(prevBusList => ({
          ...prevBusList,
          [new_key]: {
              lat: newBus.lat,
              lon: newBus.lon,
              id: newBus.id,
              route: newBus.route
          }
      }));
  }
  useEffect(()=>{
      if (updateBus && updateBus.lat !== undefined && updateBus.lon !== undefined) {
          updateBusList({ updateBus }) 
          console.log(busList)
      }
  },[updateBus])

  return (
      <div>
          {
              Object.entries(busList).map(([key,value],index)=>(
                  <MarkerCom bus={value} key={index} />
              ))
          }
      </div>
  )
}
function MarkerCom({bus}){
  if(!bus || bus.lat == undefined || bus.lon == undefined) return <div/>
  return (
      <Marker position={[bus.lat, bus.lon]}>
          <Popup>
              Bus: {bus.id} <br/>
              Route: {bus.route} <br/>
              Location: {bus.lat},{bus.lon}
          </Popup>
      </Marker>
  )
}

function Map({new_bus}) {
  return (
      <MapContainer center={[60.1699, 24.9384]} zoom={13} scrollWheelZoom={false}>
      <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      <MarkerList updateBus={new_bus}/>
      </MapContainer>
  );
}

export default App;

