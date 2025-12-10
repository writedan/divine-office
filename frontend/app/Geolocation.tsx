import React, { createContext, useContext, useState, useEffect, useRef } from 'react';
import AsyncCall from './components/AsyncCall';
import { View } from 'react-native';

const GeolocationContext = createContext(undefined);

export const Geolocation = ({ children }) => {
  const [geolocation, setGeolocation] = useState(null);
  const [loaded, setLoaded] = useState(false);

  const load = async () => {
    console.log("load");
    if (loaded) return;

    if (!navigator.geolocation) {
      console.error('Geolocation is not supported by this browser.');
      setGeolocation({ lat: 0, lon: 0 });
      setLoaded(true);
      return;
    }

    await new Promise((resolve) => {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          setGeolocation({
            lat: position.coords.latitude,
            lon: position.coords.longitude,
          });
          setLoaded(true);
          resolve();
        },
        (error) => {
          console.error('Error fetching geolocation:', error);
          setGeolocation({ lat: 0, lon: 0 });
          setLoaded(true);
          resolve();
        }
      );
    });
  };

  useEffect(() => console.log('[Geolocation]', geolocation), [geolocation]);

  return (
    <GeolocationContext.Provider value={geolocation || {}}>
      <AsyncCall call={load} message={'Fetching geolocation'}>
        {children}
      </AsyncCall>
    </GeolocationContext.Provider>

  );
};

export const useGeolocation = () => {
  const context = useContext(GeolocationContext);
  if (context === undefined) {
    throw new Error('useGeolocation must be used within a Geolocation provider');
  }
  return context;
};
