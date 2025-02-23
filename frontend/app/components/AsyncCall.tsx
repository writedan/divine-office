import React, { useEffect, useState } from 'react';
import { View, Text, ActivityIndicator as RNActivityIndicator } from 'react-native';

const AsyncCall = ({ call, message, children }) => {
  const [running, setRunning] = useState(true);

  useEffect(() => {
    const load = async () => {
      setRunning(true);

      try {
        await call(); 
      } catch (error) {
        console.error(error);
      } finally {
        setRunning(false);
      }
    };

    load(); 
  }, []); 

  return running ? (
    <View style={{ flex: 1, justifyContent: 'center', alignItems: 'center' }}>
      <RNActivityIndicator size="large" color="#0000ff" />
      <Text>{message}</Text>
    </View>
  ) : (
    children
  );
};

export default AsyncCall;
