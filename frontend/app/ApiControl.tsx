import React, { createContext, useContext, useState } from 'react';
import AsyncCall from './components/AsyncCall';

//import * as wasm_bindgen from '../wasm/divine_office_bg.js';
import * as wasm from '../wasm';

const ApiContext = createContext(undefined);

import { Asset, useAssets } from 'expo-asset';

export const ApiControl = ({ children }) => {
  const [wasmModule, setWasmModule] = useState(undefined);

  const getMetadata = async (date) => {
    if (!wasmModule) throw new Error('WASM module not initialized');

    const year = date.getFullYear();
    const month = String(date.getMonth() + 1);
    const day = String(date.getDate());

    return JSON.parse(wasmModule.get_identifier(year, month, day));
  };

  const getMonthCalendar = async (date) => {
    if (!wasmModule) throw new Error('WASM module not initialized');
    
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1);
    
    return JSON.parse(wasmModule.get_monthly_identifiers(year, month));
  };

  const getElements = async (date, hour) => {
    if (!wasmModule) throw new Error('WASM module not initialized');
    
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1);
    const day = String(date.getDate());
    
    return JSON.parse(wasmModule.get_hour(year, month, day, hour));
  };

  return (
    <AsyncCall 
      message="Initializing WASM module" 
      call={async () => {
        const asset = Asset.fromModule(require('../wasm/divine_office_bg.wasm'));
        await asset.downloadAsync();
        const response = await fetch(asset.uri);
        const wasmBuffer = await response.arrayBuffer();
        wasm.initSync({module: wasmBuffer});

        setWasmModule({...wasm});
      }}
    >
      <ApiContext.Provider value={{ getMetadata, getMonthCalendar, getElements }}>
        {children}
      </ApiContext.Provider>
    </AsyncCall>
  );
};

export const useApi = () => {
  const context = useContext(ApiContext);
  if (!context) {
    throw new Error('useApi must be used within a ApiProvider');
  }
  return context;
};

export default ApiControl;