import './App.css';
import React, { useState, useRef, useEffect } from 'react';

function PhotoUploader() {
  const [selectedFile, setSelectedFile] = useState(null);
  const fileInputRef = useRef(null);

  const handleFileChange = (event) => {
    setSelectedFile(event.target.files[0]); // Update selectedFile with the chosen file
  };

  const handleBoxClick = () => {
    if (fileInputRef.current) {
      fileInputRef.current.click(); // Trigger the file input click event
    }
  };

  const handleSubmit = () => {
    if (selectedFile) {
      const reader = new FileReader();

      reader.onloadend = () => {
        const base64String = reader.result.split(',')[1]; 
        console.log('Base64 Image:', base64String); 
      };

      reader.readAsDataURL(selectedFile);
      setSelectedFile(null); 
    }
  };

  return (
    <div className="relative flex flex-col items-center">
      <input
        type="file"
        accept="image/*"
        onChange={handleFileChange}
        className="hidden"
        ref={fileInputRef}
      />
      <div
        className={`border-dashed border-2 p-4 ${
          selectedFile ? '' : 'border-gray-400'
        } h-64 w-full max-w-xs cursor-pointer flex items-center justify-center`}
        onClick={handleBoxClick}
      >
        {selectedFile ? (
          <img
            src={URL.createObjectURL(selectedFile)}
            alt="Selected"
            className="object-contain h-full w-full"
          />
        ) : (
          <span className="text-gray-500">Click to select an image</span>
        )}
      </div>
      <div className="mt-4 flex justify-center w-full max-w-xs">
        <button
          className="bg-blue-500 text-white font-bold py-2 px-4 rounded"
          onClick={handleSubmit}
        >
          Submit
        </button>
      </div>
    </div>
  );
}

function ProduceLog() {
  const TOPIC_NAME = "image";
  const PARTITION = 0;
  
  return (
    <div className="border border-gray-300 p-4 rounded h-full">
      Produce Log Component

    </div>
  );
}

function ConsumeLog() {
  return (
    <div className="border border-gray-300 p-4 rounded h-full">
      Consume Log Component
    </div>
  );
}

function App() {
  return (
    <div className="grid grid-cols-2 h-screen w-screen gap-4 p-4">
      <div className="flex flex-col h-full gap-4">
        <div className="flex-1 bg-white p-4 rounded shadow-md">
          <PhotoUploader />
        </div>
        <div className="flex-1 bg-white p-4 rounded shadow-md">
          <ProduceLog />
        </div>
      </div>
      <div className="flex-1 bg-white p-4 rounded shadow-md">
        <ConsumeLog />
      </div>
    </div>
  );
}

export default App;
