function dateToSuntime(dat) {
    var hour = dat.getHours();
    var mint = dat.getMinutes();
    return hour + mint / 60;
}

function w_sunrise(lat, lon, dat) {
    return dateToSuntime(SunCalc.getTimes(dat, lat, lon).sunrise);
}

function w_sunset(lat, lon, dat) {
    return dateToSuntime(SunCalc.getTimes(dat, lat, lon).sunset);
}

function colorHeading(element, color) {
    color = color.toLowerCase();
    element.style.color = color;
    if (color == "white") {
        element.style['-webkit-text-stroke'] = '0.75px black';
    } else if (color == "blue") {
        element.style.color = "#2B4593";
    } else if (color == "violet") {
        element.style.color = "rgb(76, 0, 153)";
    } else if (color == "rose") {
        element.style.color = "rgb(250, 72, 173)";
    }
}

const roundNearest = (value, nearest) => Math.ceil(value * (1 / nearest)) / (1 / nearest);

function roundTime(time) {
    time = roundNearest(time, 15 / 60);
    if (time > 24) {
        time -= 24;
    }
    if (time < 0) {
        time += 24;
    }
    return time;
}

function suntimeToClocktime(suntime) {
    if (suntime > 24) suntime -= 24
    return [Math.floor(suntime), Math.floor((suntime - Math.floor(suntime)) * 60)];
}

function printClocktime(clocktime) {
    //if (clocktime[0] >= 24) clocktime[0] -= 24;
    let am = clocktime[0] < 12;
    clocktime[0] = am ? clocktime[0] : clocktime[0] - 12;
    if (clocktime[0] == 0) {
        clocktime[0] = 12;
    }
    return clocktime[0] + ":" + (clocktime[1] < 10 ? "0" + clocktime[1] : clocktime[1]) + (clocktime[0] < 10 ? " " : " ") + (am ? "AM" : "PM");
}

function getFormattedDate(date) {
    var year = date.getFullYear();

    var month = (1 + date.getMonth()).toString();
    month = month.length > 1 ? month : '0' + month;

    var day = date.getDate().toString();
    day = day.length > 1 ? day : '0' + day;

    return year + '/' + month + '/' + day;
}

function printDuration(suntime) {
    var n = suntimeToClocktime(suntime);
    if (n[0] == 0 && n[1] == 0) return "0min";
    return (n[0] > 0 ? n[0] + "hr" : "") + (n[1] > 0 ? n[1] + "min" : "");
}

function treatAsUTC(date) {
    var result = new Date(date);
    result.setMinutes(result.getMinutes() - result.getTimezoneOffset());
    return result;
}

function daysBetween(startDate, endDate) {
    var millisecondsPerDay = 24 * 60 * 60 * 1000;
    return (treatAsUTC(endDate) - treatAsUTC(startDate)) / millisecondsPerDay;
}