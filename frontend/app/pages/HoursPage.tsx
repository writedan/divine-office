import React, { useState, useEffect, useRef } from 'react';
import { StyleSheet, Text, View, Pressable, ScrollView, Modal } from 'react-native';
import SunCalc from 'suncalc';
import { useGeolocation } from '../Geolocation';
import { useApi } from '../ApiControl';
import { useNavigation } from '../Navigation';

const colors = {
  White: '#ffffff',
  Blue: '#0066cc',
  Green: '#008000',
  Red: '#cc0000',
  Black: '#000000',
  Violet: '#5a2a83',
  Rose: '#ff66b2',
};

const Hours = ({ now }) => {
  const [rows, setRows] = useState([]);
  const [loading, setLoading] = useState(false);
  const [startDate, setStartDate] = useState(new Date(now));
  const [endDate, setEndDate] = useState(new Date(now));
  const [hoveredRow, setHoveredRow] = useState(null);
  const [selectedLiturgicalDays, setSelectedLiturgicalDays] = useState({});
  const [showModal, setShowModal] = useState(false);
  const [modalOptions, setModalOptions] = useState([]);
  const [modalKey, setModalKey] = useState(null);

  const { getMetadata, hasFirstVespers } = useApi();
  const { lat: latitude, lon: longitude } = useGeolocation();
  const { goto } = useNavigation();
  const scrollViewRef = useRef(null);

  useEffect(() => {
    loadInitialData();
  }, []);

  const loadInitialData = async () => {
    const start = new Date(now);
    const end = new Date(now);
    end.setDate(end.getDate() + 7);

    await loadDateRange(start, end, 'down');
    setStartDate(start);
    setEndDate(end);
  };

  const loadDateRange = async (start, end, direction) => {
    if (loading) return;
    setLoading(true);

    const newRows = [];
    let current = new Date(start);

    while (current <= end) {
      const dateRows = await buildRowsForDate(new Date(current), false);
      newRows.push(...dateRows);
      current.setDate(current.getDate() + 1);
    }

    setRows(prev => {
      const combined = direction === 'up' ? [...newRows, ...prev] : [...prev, ...newRows];
      return combined.sort((a, b) => {
        if (a.calendarDate.getTime() !== b.calendarDate.getTime()) {
          return a.calendarDate - b.calendarDate;
        }
        return a.hourIndex - b.hourIndex;
      });
    });

    setLoading(false);
  };

  const buildRowsForDate = async (date, isFirstDate = false) => {
    const response = await getMetadata(date);
    const todayLiturgical = response[0] || [];
    const tomorrowLiturgical = response[1] || [];

    const hours = calculateHours(date);
    const rows = [];
    const dateKey = date.toDateString();

    const vigilsKey = `${dateKey}-vigils`;
    const vespersKey = `${dateKey}-vespers`;

    const selectedToday = selectedLiturgicalDays[vigilsKey] || todayLiturgical[0];
    const selectedTomorrow = selectedLiturgicalDays[vespersKey] || tomorrowLiturgical[0];

    const tomorrowBeginsAtVespers = await hasFirstVespers(selectedToday, selectedTomorrow);

    const yesterday = new Date(date);
    yesterday.setDate(yesterday.getDate() - 1);
    const yesterdayKey = yesterday.toDateString();
    const yesterdayVigilsKey = `${yesterdayKey}-vigils`;
    const yesterdayLiturgicalResponse = await getMetadata(yesterday);
    const yesterdayLiturgical = yesterdayLiturgicalResponse[0] || [];
    const selectedYesterday = selectedLiturgicalDays[yesterdayVigilsKey] || yesterdayLiturgical[0];

    const todayHasFirstVespers = await hasFirstVespers(selectedYesterday, selectedToday);

    const fastingLiturgicalDay = selectedToday;

    hours.forEach((hour, index) => {
      const row = {
        calendarDate: new Date(date),
        hour: {
          key: hour.name.toLowerCase(),
          ...hour
        },
        hourIndex: index,
        liturgicalDayOptions: null,
        liturgicalDay: selectedToday,
        fastingLiturgicalDay: index === 0 ? fastingLiturgicalDay : null,
      };

      if (isFirstDate && hour.name === 'Nocturns') {
        const key = vigilsKey;
        row.liturgicalDayOptions = todayLiturgical;
        row.liturgicalDayKey = key;

        if (!selectedLiturgicalDays[key] && todayLiturgical.length > 0) {
          setSelectedLiturgicalDays(prev => ({
            ...prev,
            [key]: todayLiturgical[0]
          }));
        }
      }
      else if (hour.name === 'Vespers' && tomorrowBeginsAtVespers) {
        const key = vespersKey;
        row.liturgicalDayOptions = tomorrowLiturgical;
        row.liturgicalDayKey = key;
        row.hourLabel = 'First Vespers';
        row.hour.key = 'first-vespers';
        row.liturgicalDay = selectedTomorrow;

        if (!selectedLiturgicalDays[key] && tomorrowLiturgical.length > 0) {
          setSelectedLiturgicalDays(prev => ({
            ...prev,
            [key]: tomorrowLiturgical[0]
          }));
        }
      }
      else if (hour.name === 'Nocturns' && !isFirstDate && !todayHasFirstVespers) {
        const key = vigilsKey;
        row.liturgicalDayOptions = todayLiturgical;
        row.liturgicalDayKey = key;

        if (!selectedLiturgicalDays[key] && todayLiturgical.length > 0) {
          setSelectedLiturgicalDays(prev => ({
            ...prev,
            [key]: todayLiturgical[0]
          }));
        }
      }
      else if (hour.name === 'Compline' && tomorrowBeginsAtVespers) {
        row.hourLabel = 'First Compline';
        row.hour.key = 'first-compline';
        row.liturgicalDay = selectedTomorrow;
      }

      rows.push(row);
    });

    return rows;
  };

  const calculateHours = (date) => {
    const times = SunCalc.getTimes(date, latitude, longitude);
    const daylightDuration = times.sunset - times.sunrise;

    const addSunlightHours = (startTime, fraction) => {
      const timeInMs = startTime.getTime() + (daylightDuration * fraction);
      return new Date(timeInMs);
    };

    return [
      { name: 'Nocturns', time: times.nadir },
      { name: 'Matins', time: times.dawn },
      { name: 'Prime', time: addSunlightHours(times.sunrise, 1 / 12) },
      { name: 'Terce', time: addSunlightHours(times.sunrise, 3 / 12) },
      { name: 'Sext', time: addSunlightHours(times.sunrise, 6 / 12) },
      { name: 'None', time: addSunlightHours(times.sunrise, 9 / 12) },
      { name: 'Vespers', time: times.sunset },
      { name: 'Compline', time: times.night },
    ];
  };

  const loadMore = async (direction) => {
    if (loading) return;
    if (direction === 'up') {
      const newStart = new Date(startDate);
      newStart.setDate(newStart.getDate() - 7);
      await loadDateRange(newStart, new Date(startDate), 'up');
      setStartDate(newStart);
      // Optionally adjust scroll to maintain position
      scrollViewRef.current.scrollTo({ y: 7 * 200, animated: false });
    } else {
      const newEnd = new Date(endDate);
      newEnd.setDate(newEnd.getDate() + 7);
      await loadDateRange(new Date(endDate), newEnd, 'down');
      setEndDate(newEnd);
    }
  };

  const handleScroll = ({ nativeEvent }) => {
    const { contentOffset, contentSize, layoutMeasurement } = nativeEvent;

    if (contentOffset.y < 200) {
      loadMore('up');
    }

    if (contentOffset.y + layoutMeasurement.height > contentSize.height - 200) {
      loadMore('down');
    }
  };

  const handleLiturgicalDayClick = (key, options) => {
    setModalKey(key);
    setModalOptions(options);
    setShowModal(true);
  };

  const selectLiturgicalDay = (day) => {
    setSelectedLiturgicalDays(prev => ({
      ...prev,
      [modalKey]: day
    }));
    setShowModal(false);
  };

  const formatTime = (date) => {
    if (!date) return null;
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const formatDate = (date) => {
    return date.toLocaleDateString([], { weekday: 'short', month: 'short', day: 'numeric' });
  };

  const renderPenanceMessage = (penance) => {
    if (penance === null) {
      return { short: 'No penance.', long: 'Meat and fish may be taken at dinner.' };
    } else if (penance === 'Abstinence') {
      return { short: 'Abstinence.', long: 'Refrain from meat, dairy, and eggs.' };
    } else if (penance === 'Fasting' || penance === 'Vigil') {
      return { short: 'Fasting.', long: 'Refrain from meat, fish, oil, wine, dairy, and eggs.' };
    }
    return { short: '', long: '' };
  };

  const groupedRows = rows.reduce((acc, row) => {
    const dateKey = row.calendarDate.toDateString();
    if (!acc[dateKey]) acc[dateKey] = { date: row.calendarDate, rows: [] };
    acc[dateKey].rows.push(row);
    return acc;
  }, {});

  const sortedDateKeys = Object.keys(groupedRows).sort((a, b) => groupedRows[a].date - groupedRows[b].date);

  return (
    <View style={styles.container}>
      <ScrollView
        ref={scrollViewRef}
        onScroll={handleScroll}
        scrollEventThrottle={400}
      >
        <View style={styles.contentContainer}>
          {sortedDateKeys.map(dateKey => {
            const dateRows = groupedRows[dateKey].rows;
            const date = groupedRows[dateKey].date;

            return (
              <View key={dateKey} style={styles.dateSection}>
                <View style={styles.dateColumn}>
                  <Text style={styles.dateText}>{formatDate(date)}</Text>
                  {dateRows[0]?.fastingLiturgicalDay && (
                    <View style={styles.fastingInfo}>
                      <Text style={styles.fastingTitle}>
                        {renderPenanceMessage(dateRows[0].fastingLiturgicalDay.penance).short}
                      </Text>
                      <Text style={styles.fastingDetail}>
                        {renderPenanceMessage(dateRows[0].fastingLiturgicalDay.penance).long}
                      </Text>
                    </View>
                  )}
                </View>

                <View style={styles.hoursColumn}>
                  {dateRows.map((row, idx) => (
                    <View key={idx} style={styles.rowContainer}>
                      {row.liturgicalDayOptions && (
                        <Pressable
                          style={styles.liturgicalDayRow}
                          onPress={() => handleLiturgicalDayClick(row.liturgicalDayKey, row.liturgicalDayOptions)}
                        >
                          {selectedLiturgicalDays[row.liturgicalDayKey] && (
                            <View style={styles.liturgicalContent}>
                              <Text
                                style={[
                                  styles.liturgicalName,
                                  { color: colors[selectedLiturgicalDays[row.liturgicalDayKey].color] || colors.Black },
                                  selectedLiturgicalDays[row.liturgicalDayKey].color === 'White' && styles.whiteTextOutline
                                ]}
                              >
                                {selectedLiturgicalDays[row.liturgicalDayKey].name}
                                {row.liturgicalDayOptions.length > 1 && ' ▼'}
                              </Text>
                            </View>
                          )}
                        </Pressable>
                      )}

                      <Pressable
                        style={[styles.hourRow, hoveredRow === `${dateKey}-${idx}` && styles.hoveredRow]}
                        onHoverIn={() => setHoveredRow(`${dateKey}-${idx}`)}
                        onHoverOut={() => setHoveredRow(null)}
                        onPress={() => goto('hour', { celebration: row.liturgicalDay, hour: row.hour.key })}
                      >
                        <View style={styles.hourInfo}>
                          <Text style={styles.hourName}>{row.hourLabel || row.hour.name}</Text>
                          <Text style={styles.hourTime}>{formatTime(row.hour.time)}</Text>
                        </View>
                      </Pressable>
                    </View>
                  ))}
                </View>
              </View>
            );
          })}

          {loading && (
            <View style={styles.loadingContainer}>
              <Text style={styles.loadingText}>Loading...</Text>
            </View>
          )}
        </View>
      </ScrollView>

      <Modal
        visible={showModal}
        transparent
        animationType="fade"
        onRequestClose={() => setShowModal(false)}
      >
        <Pressable style={styles.modalOverlay} onPress={() => setShowModal(false)}>
          <View style={styles.modalContent}>
            <Text style={styles.modalTitle}>Select Liturgical Day</Text>
            {modalOptions.map((day, idx) => (
              <Pressable key={idx} style={styles.modalOption} onPress={() => selectLiturgicalDay(day)}>
                <Text style={[styles.modalOptionText, { color: colors[day.color] || colors.Black }]}>
                  {day.name}
                </Text>
                <Text style={styles.modalOptionPenance}>{day.rank}</Text>
              </Pressable>
            ))}
          </View>
        </Pressable>
      </Modal>
    </View>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#f5f1e8' },
  contentContainer: { padding: 20 },
  dateSection: { flexDirection: 'row', marginBottom: 2 },
  dateColumn: {
    width: 180,
    backgroundColor: '#f9f6f0',
    padding: 15,
    justifyContent: 'flex-start',
    alignItems: 'center',
    borderRightWidth: 1,
    borderRightColor: '#d1c7b7',
  },
  dateText: { fontSize: 16, fontWeight: '600', color: '#4a3c31', textAlign: 'center' },
  fastingInfo: { marginTop: 12, paddingTop: 12, width: '100%' },
  fastingTitle: { fontSize: 13, fontWeight: 'bold', color: '#4a3c31', textAlign: 'center', marginBottom: 6 },
  fastingDetail: { fontSize: 11, color: '#6b5d52', textAlign: 'center', lineHeight: 16 },
  hoursColumn: { flex: 1, backgroundColor: '#fff' },
  rowContainer: { borderBottomWidth: 1, borderBottomColor: '#e8e0d5' },
  hourRow: { padding: 12 },
  hoveredRow: { backgroundColor: '#f4f0f8' },
  hourInfo: { flexDirection: 'row', justifyContent: 'space-between' },
  hourName: { fontSize: 16, color: '#4a3c31', fontWeight: '500' },
  hourTime: { fontSize: 14, color: '#6b5d52' },
  liturgicalDayRow: { padding: 12, backgroundColor: '#faf8f5', borderTopWidth: 1, borderTopColor: '#e8e0d5' },
  liturgicalContent: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center' },
  liturgicalName: { fontSize: 18, fontWeight: 'bold' },
  whiteTextOutline: { WebkitTextStroke: '1px black' },
  loadingContainer: { padding: 20, alignItems: 'center' },
  loadingText: { fontSize: 14, color: '#6b5d52' },
  modalOverlay: { flex: 1, backgroundColor: 'rgba(0,0,0,0.5)', justifyContent: 'center', alignItems: 'center' },
  modalContent: { backgroundColor: '#fff', borderRadius: 8, padding: 20, minWidth: 300, maxWidth: 400 },
  modalTitle: { fontSize: 18, fontWeight: 'bold', marginBottom: 15, color: '#4a3c31' },
  modalOption: { padding: 15, borderBottomWidth: 1, borderBottomColor: '#e8e0d5' },
  modalOptionText: { fontSize: 16, fontWeight: 'bold', marginBottom: 5 },
  modalOptionPenance: { fontSize: 12, color: '#6b5d52', fontStyle: 'italic' },
});

export default Hours;
