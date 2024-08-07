import React, {useState,useEffect} from "react";
import { MapContainer, Marker, Popup, TileLayer, useMap } from "react-leaflet";

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
    if(!bus || bus.lat == undefined || bus.lon == undefined){
        return <div/>
    }

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

export default function Map({new_bus}) {
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