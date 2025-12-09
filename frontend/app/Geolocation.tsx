import React, { createContext, useContext, useState, useEffect } from 'react';
import AsyncCall from './components/AsyncCall';

const GeolocationContext = createContext(undefined);

export const Geolocation = ({ children }) => {
  const [geolocation, setGeolocation] = useState(null);
  const [loaded, setLoaded] = useState(false);

  const load = async () => {
    if (loaded) return;

    if (!navigator.geolocation) {
      console.error('Geolocation is not supported by this browser.');
      setGeolocation({ lat: 0, lon: 0 });
      setLoaded(true);
      return;
    }

    navigator.geolocation.getCurrentPosition(
      (position) => {
        setGeolocation({
          lat: position.coords.latitude,
          lon: position.coords.longitude,
        });
        setLoaded(true);
      },
      (error) => {
        console.error('Error fetching geolocation:', error);
        setGeolocation({ lat: 0, lon: 0 });
        setLoaded(true);
      }
    );
  };

  useEffect(() => console.log('[Geolocation]', geolocation), [geolocation]);

  return (
    <GeolocationContext.Provider value={geolocation || { lat: 0, lon: 0 }}>
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
