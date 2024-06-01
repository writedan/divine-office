function easter_computus(y) {
        var date, a, b, c, m, d;
        date = new Date();
        date.setHours(0, 0, 0, 0);
        date.setFullYear(y);
        a = y % 19;
        b = 2200 <= y && y <= 2299 ? (11 * a + 4) % 30 : (11 * a + 5) % 30;
        c = b === 0 || (b === 1 && a > 10) ? b + 1 : b;
        m = 1 <= c && c <= 19 ? 3 : 2;
        d = (50 - c) % 31;
        date.setMonth(m, d);
        date.setMonth(m, d + (7 - date.getDay()));
        return date;
    }
    function advent_computus(y) {
        var nov27 = new Date(y, 11 - 1, 27);
        while (nov27.getDay() != 0) {
            nov27.setDate(nov27.getDate() + 1);
        }
        return nov27;
    }
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
        return (clocktime[0] < 10 ? "" : "") + clocktime[0] + ":" + (clocktime[1] < 10 ? "0" + clocktime[1] : clocktime[1]) + (clocktime[0] < 10 ? " " : " ") + (am ? "AM" : "PM");
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
        return (n[0] > 0 ? n[0] + "hr" : "") + (n[1] > 0 ? n[1] + "min" : "");
    }

    function romanize (num) {
    if (isNaN(num))
        return NaN;
    var digits = String(+num).split(""),
        key = ["","C","CC","CCC","CD","D","DC","DCC","DCCC","CM",
               "","X","XX","XXX","XL","L","LX","LXX","LXXX","XC",
               "","I","II","III","IV","V","VI","VII","VIII","IX"],
        roman = "",
        i = 3;
    while (i--)
        roman = (key[+digits.pop() + (i * 10)] || "") + roman;
    return Array(+digits.join("") + 1).join("M") + roman;
}

function advdat(dat, n = 1) {
        // advance date by one
        dat.setDate(dat.getDate() + n);
        return dat;
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

    function keyDate(date) {
        var now = date;
        var start = new Date(now.getFullYear(), 0, 0);
        return Math.floor(daysBetween(start, now));
    }

    function ordinal_suffix_of(i) {
        var j = i % 10,
            k = i % 100;
        if (j == 1 && k != 11) {
            return i + "st";
        }
        if (j == 2 && k != 12) {
            return i + "nd";
        }
        if (j == 3 && k != 13) {
            return i + "rd";
        }
        return i + "th";
    }

    function feriaToDay(n) {
            switch (n) {
            case 0: return 'sunday'
            case 1: return 'monday'
            case 2: return 'tuesday'
            case 3: return 'wednesday'
            case 4: return 'thursday'
            case 5: return 'friday'
            case 6: return 'saturday'
            }
        }