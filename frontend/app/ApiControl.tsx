import React, { createContext, useContext, useState } from 'react';
import AsyncCall from './components/AsyncCall';
import * as wasm from '../wasm';
import { Asset } from 'expo-asset';

const ApiContext = createContext(undefined);

const runAsync = (task) => {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      try {
        const result = task();
        resolve(result);
      } catch (error) {
        reject(error);
      }
    }, 0);
  });
};

export const ApiControl = ({ children }) => {
  const [wasmModule, setWasmModule] = useState(undefined);

  const getMetadata = (date) => {
    return runAsync(() => {
      if (!wasmModule) throw new Error('WASM module not initialized');
      const year = date.getFullYear();
      const month = String(date.getMonth() + 1);
      const day = String(date.getDate());
      return JSON.parse(wasmModule.get_identifiers(year, month, day));
    });
  };

  const getMonthCalendar = (date) => {
    return runAsync(() => {
      if (!wasmModule) throw new Error('WASM module not initialized');
      const year = date.getFullYear();
      const month = String(date.getMonth() + 1);
      return JSON.parse(wasmModule.get_monthly_identifiers(year, month));
    });
  };

  const getElements = (date, hour) => {
    return runAsync(() => {
      if (!wasmModule) throw new Error('WASM module not initialized');
      const year = date.getFullYear();
      const month = String(date.getMonth() + 1);
      const day = String(date.getDate());
      return JSON.parse(wasmModule.get_hour(year, month, day, hour));
    });
  };

  const operationQueue = [];
  const MAX_CONCURRENT_OPERATIONS = 3;
  let activeOperations = 0;

  const processQueue = async () => {
    if (operationQueue.length === 0 || activeOperations >= MAX_CONCURRENT_OPERATIONS) {
      return;
    }

    const operation = operationQueue.shift();
    activeOperations++;

    try {
      const result = await operation.task();
      operation.resolve(result);
    } catch (error) {
      operation.reject(error);
    } finally {
      activeOperations--;
      processQueue();
    }
  };

  const queueOperation = (task) => {
    return new Promise((resolve, reject) => {
      operationQueue.push({ task, resolve, reject });
      processQueue();
    });
  };

  return (
    <AsyncCall 
      message="Initializing WASM module" 
      call={async () => {
        const asset = Asset.fromModule(require('../wasm/divine_office_bg.wasm'));
        await asset.downloadAsync();
        const response = await fetch(asset.uri);
        const wasmBuffer = await response.arrayBuffer();
        await queueOperation(async () => {
          wasm.initSync({module: wasmBuffer});
          setWasmModule({...wasm});
        });
      }}
    >
      <ApiContext.Provider value={{ 
        getMetadata: (date) => queueOperation(() => getMetadata(date)),
        getMonthCalendar: (date) => queueOperation(() => getMonthCalendar(date)),
        getElements: (date, hour) => queueOperation(() => getElements(date, hour)),
        hasFirstVespers: (today, tomorrow) => true
      }}>
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