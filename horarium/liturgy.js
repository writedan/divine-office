function isObject(item){return(item&&typeof item==='object'&&!Array.isArray(item))}function mergeDeep(target,...sources){if(!sources.length){return target}const source=sources.shift();if(isObject(target)&&isObject(source)){for(const key in source){if(isObject(source[key])){if(!target[key]){Object.assign(target,{[key]:{}})}mergeDeep(target[key],source[key])}else{Object.assign(target,{[key]:source[key]})}}}return mergeDeep(target,...sources)}

function getTemporalMetaData() {
        var metadata = {
            'advent': {},
            'christmas': {},
            'epiphany': {},
            'prelent': {},
            'lent': {},
            'passion': {},
            'pascha': {},
            'ascension': {},
            'pentecost': {},
            'august': {},
            'september': {},
            'october': {},
            'november': {}
        };

        for (var w = 1; w < 5; w++) {
            metadata.advent[w] = {};
            metadata.advent[w].sunday = {
                name: ordinal_suffix_of(w) + ' Sunday of Advent',
                color: (w == 3) ? 'rose' : 'violet',
                rank: 2
            }

            for (var d = 1; d < 7; d++) {
                metadata.advent[w][feriaToDay(d)] = {
                    name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the ' + ordinal_suffix_of(w) + ' Week of Advent',
                    color: 'violet',
                    penance: (d == 3 || d == 5) ? 'fast' : 'none',
                    rank: 8
                }
            }
        }

        metadata.christmas['vigil'] = {
            name: 'Eve of the Nativity of the Lord',
            rank: 8, // technically, it is a feria
            color: 'violet',
            penance: 'vigil'
        }

        metadata.christmas['vigil-sunday'] = {
            name: 'Eve of the Nativity on Sunday',
            rank: 2,
            color: 'violet',
            note: 'The Mass of the Fourth Sunday of Advent is sung after Terce. That of the eve is sung after None. The whole office is of the eve. Penance is abrogated by the Sunday.',
            //vnote: 'Vespers and Compline as on the Fourth Sunday of Advent.'
            ribbons: metadata.advent[4].sunday
        }

        metadata.epiphany['vigil-sunday'] = {
            name: 'Eve of the Epiphany on Sunday',
            rank: 8,
            color: 'violet',
            note: 'First Vespers superseded by Octave of the Innocents. Mass of Christmastide feria is sung after Terce. That of the eve is sung after None. The whole office is of the eve. Penance is abrogated by the Sunday.'
        }

        metadata.christmas['octave'] = {}
        metadata.christmas['octave'][1] = {
            name: 'Nativity of the Lord',
            rank: 1,
            color: 'white',
            note: 'Three Masses are said of the Nativity: one after Vigils, one after Lauds, and one after Terce.'
        }

        metadata.christmas.octave[2] = {
            name: 'Saint Stephen, Protomartyr',
            rank: 3,
            color: 'red'
        }

        metadata.christmas.octave[3] = {
            name: 'Saint John, Apostle and Evangelist',
            rank: 3,
            color: 'white'
        }

        metadata.christmas.octave[4] = {
            name: 'Holy Innocents, Martyrs',
            rank: 3,
            color: 'violet'
        }

        metadata.christmas.octave[5] = {
            name: 'Saint Thomas Becket, Bishop and Martyr',
            rank: 3,
            color: 'red'
        }

        metadata.christmas.octave[6] = {
            name: 'Sixth Day of the Nativity',
            rank: 8,
            color: 'white'
        }

        metadata.christmas.octave[7] = {
            name: 'Seventh Day of the Nativity',
            rank: 8,
            color: 'white'
        }

        metadata.christmas.octave[8] = {
            name: 'Circumcision of the Lord',
            rank: 1,
            color: 'white'
        }

        metadata.christmas.octave[9] = {
            name: 'Octave of Saint Stephen',
            rank: 8,
            color: 'red'
        }

        metadata.christmas.octave[10] = {
            name: 'Octave of Saint John',
            rank: 8,
            color: 'white'
        }

        metadata.christmas.octave[12] = {
            name: 'Octave of the Innocents',
            rank: 8,
            color: 'red'
        }

        metadata.epiphany.vigil = {
            name: 'Eve of the Epiphany',
            rank: 8,
            color: 'violet',
            penance: 'vigil'
        }

        metadata.epiphany.octave = {}
        metadata.epiphany.octave[1] = {
            name: 'Epiphany of the Lord',
            rank: 1,
            color: 'white'
        }

        for (var d = 1; d < 8; d++) {
            metadata.epiphany.octave[d + 1] = {
                name: ordinal_suffix_of(d + 1) + ' Day of the Epiphany',
                rank: 8, 
                color: 'white'
            }
        }

        metadata.epiphany.octave[8] = {
            name: 'Octave of the Epiphany',
            rank: 3,
            color: 'white'
        }

        metadata.christmas.feria = {}
        for (var d = 1; d <= 6; d++) {
            metadata.christmas.feria[d] = {
                name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in Christmastide',
                 rank: 8,
                 color: 'white',
                 penance: (d == 3 || d == 5) ? 'abstinence' : 'none'
             }
        }

        for (var w = 1; w <= 6; w++) {
            metadata.epiphany[w] = {}
            metadata.epiphany[w].sunday = {
                name: ordinal_suffix_of(w) + ' Sunday after Epiphany',
                rank: 4,
                color: 'green'
            }

            for (var d = 1; d < 7; d++) {
                metadata.epiphany[w][feriaToDay(d)] = {
                    name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the ' + ordinal_suffix_of(w) + ' Week after Epiphany',
                    color: 'green',
                    penance: (d == 3 || d == 5) ? 'abstinence' : 'none',
                    rank: 8
                }
            }
        }

        metadata.epiphany.octave.sunday = {
            name: 'Sunday after Epiphany',
            rank: 4,
            color: 'white'
        }

        for (var w = 3; w > 0; w-- ) {
            metadata.prelent[w] = {}
            metadata.prelent[w].sunday = {
                name: ordinal_suffix_of(w) + ' Sunday before Lent',
                color: 'violet',
                rank: 2
            }

            for (var d = 1; d < 7; d++) {
                metadata.prelent[w][feriaToDay(d)] = {
                    name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the ' + ordinal_suffix_of(w) + ' Week before Lent',
                    color: 'violet',
                    penance: (d == 3 || d == 5) ? 'fast' : 'none',
                    rank: 8
                }
            }
        }

        metadata.prelent[1].wednesday = {
            name: 'Ash Wednesday',
            color: 'violet',
            penance: 'strict-fast',
            rank: 7
        }

        metadata.prelent[1].thursday = {
            name: 'Thursday after the Ashes',
            color: 'violet',
            penance: 'fast',
            rank: 8
        }

        metadata.prelent[1].friday = {
            name: 'Friday after the Ashes',
            color: 'violet',
            penance: 'strict-fast',
            rank: 8
        }

        metadata.prelent[1].saturday = {
            name: 'Saturday after the Ashes',
            color: 'violet',
            penance: 'fast',
            rank: 8
        }

        for (var w = 1; w < 5; w++) {
            metadata.lent[w] = {}
            metadata.lent[w].sunday = {
                name: ordinal_suffix_of(w) + ' Sunday in Lent',
                color: (w == 4) ? 'rose' : 'violet',
                penance: (w == 4) ? 'none' : 'abstinence',
                rank: 2
            }

            for (var d = 1; d < 7; d++) {
                metadata.lent[w][feriaToDay(d)] = {
                    name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the ' + ordinal_suffix_of(w) + ' Week of Lent',
                    color: 'violet',
                    penance: (d==3 || d==5) ? 'strict-fast' : 'fast',
                    rank: 8
                }
            }
        }

        metadata.passion = {}
        metadata.passion[1] = {}
        metadata.passion[1].sunday = {
            name: 'Sunday before the Passion',
            color: 'violet',
            penance: 'abstinence',
            rank: 2
        }

        for (var d = 1; d < 7; d++) {
            metadata.passion[1][feriaToDay(d)] = {
                name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the Week before the Passion',
                color: 'violet',
                penance: 'strict-fast',
                rank: 8
            }
        }

        metadata.passion[2] = {}
        metadata.passion[2].sunday = {
            name: 'Palm Sunday',
            color: 'violet',
            penance: 'abstinence',
            rank: 2
        }

        metadata.passion[2].monday = {
            name: 'Monday in Holy Week',
            color: 'violet',
            penance: 'strict-fast',
            rank: 7
        }

        metadata.passion[2].tuesday = {
            name: 'Tuesday in Holy Week',
            color: 'violet',
            penance: 'strict-fast',
            rank: 7
        }

        metadata.passion[2].wednesday = {
            name: 'Wednesday in Holy Week',
            color: 'violet',
            penance: 'strict-fast',
            rank: 7
        }

        metadata.passion[2].thursday = {
            name: "Thursday of the Lord's Supper",
            color: 'violet',
            note: 'Liturgical color of the Mass is white.',
            penance: 'vigil',
            rank: 7
        }

        metadata.passion[2].friday = {
            name: 'Friday of the Preparation',
            color: 'black',
            penance: 'vigil',
            rank: 7
        }

        metadata.passion[2].saturday = {
            name: 'Holy Saturday',
            color: 'violet',
            note: 'Mass begins at the prescribed time for Vespers.<br/>Liturgical color changes to white at the Litany.',
            penance: 'vigil',
            rank: 7
        }

        metadata.pascha[1] = {}
        metadata.pascha[1].sunday = {
            name: 'Holy Pasch of the Lord',
            color: 'white',
            rank: 0
        }

        for (var d = 1; d < 7; d++) {
            metadata.pascha[1][feriaToDay(d)] = {
                name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' after the Holy Pasch',
                color: 'white',
                rank: 8
            }
        }

        for (var w = 2; w <= 6; w++) {
            metadata.pascha[w] = {}
            metadata.pascha[w].sunday = {
                name: ordinal_suffix_of(w - 1) + ' Sunday after the Pasch',
                color: 'white',
                rank: 2
            }

            for (var d = 1; d < 7; d++) {
                metadata.pascha[w][feriaToDay(d)] = {
                    name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the ' + ordinal_suffix_of(w - 1) + ' Week after the Pasch',
                    color: 'white',
                    penance: (d == 3 || d == 5) ? 'abstinence' : 'none',
                    rank: 8
                }
            }
        }

        metadata.ascension.vigil = {
            name: 'Eve of the Ascension',
            color: 'violet',
            penance: 'vigil',
            rank: 8
        }

        metadata.ascension.octave = {}
        metadata.ascension.octave[1] = {
            name: 'Ascension of the Lord',
            color: 'white',
            rank: 1
        }

        for (var d = 2; d <= 8; d++) {
            metadata.ascension.octave[d] = {
                name: (d == 8) ? 'Octave of the Asecnsion' : ordinal_suffix_of(d) + ' Day of the Ascension',
                color: 'white',
                rank: 7
            }
        }

        metadata.ascension.octave[4].name = 'Sunday after Ascension'
        metadata.ascension.octave[4].rank = 2

        metadata.ascension.octave[8].rank = 3

        metadata.ascension.feria = {
            name: 'Friday in Ascensiontide',
            color: 'white',
            penance: 'abstinence',
            rank: 8
        }

        metadata.pentecost.vigil = {
            name: 'Eve of the Holy Pentecost',
            color: 'white',
            note: 'Liturgical color is white until None, violet until the introit of the Mass, and red thereafter.',
            penance: 'vigil',
            rank: 8
        }

        metadata.pentecost[0] = {
            sunday: {
                name: 'Sunday of the Holy Pentecost',
                rank: 2,
                color: 'red'
            },
            monday: {
                name: 'Monday after the Holy Pentecost',
                rank: 8,
                color: 'red'
            },
            tuesday: {
                name: 'Tuesday after the Holy Pentecost',
                rank: 8,
                color: 'red'
            },
            wednesday: {
                name: 'Ember Wednesday after the Holy Pentecost',
                rank: 7,
                color: 'red',
                penance: 'strict-fast',
            },
            thursday: {
                name: 'Thursday after the Holy Pentecost',
                rank: 8,
                color: 'red'
            },
            friday: {
                name: 'Ember Friday after the Holy Pentecost',
                rank: 7,
                color: 'red',
                penance: 'strict-fast'
            },
            saturday: {
                name: 'Ember Saturday after the Holy Pentecost',
                rank: 7,
                color: 'red',
                penance: 'vigil'
            }
        }

        for (var w = 1; w <= 28; w++) {
            metadata.pentecost[w] = {
                sunday: {
                    name: ordinal_suffix_of(w) + ' Sunday after Pentecost',
                    rank: 4,
                    color: 'green'
                }
            }

            for (var d = 1; d < 7; d++) {
                metadata.pentecost[w][feriaToDay(d)] = {
                    name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the ' + ordinal_suffix_of(w) + ' Week after Pentecost',
                    rank: 8,
                    color: 'green',
                    penance: (d == 3 || d == 5) ? 'abstinence' : ''
                }
            }
        }

        var months = ['august', 'september', 'october', 'november'];
        for (var mnum in months) {
            var m = months[mnum]
            var name = m.charAt(0).toUpperCase() + m.slice(1)
            for (var w = 1; w <= 28; w++) {
                metadata[m][w] = {
                    sunday: {
                        name: ordinal_suffix_of(w) + ' Sunday of ' + name,
                        color: 'green',
                        rank: 4
                    }
                }

                for (var d = 1; d < 7; d++) {
                    metadata[m][w][feriaToDay(d)] = {
                        name: feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the ' + ordinal_suffix_of(w) + ' Week of ' + name,
                        color: 'green',
                        rank: 8
                    }
                }
            }
        }

        /*metadata.pentecost[28].sunday.name = 'Sunday before Advent'
        for (var d = 1; d < 7; d++) {
            metadata.pentecost[28][feriaToDay(d)].name = feriaToDay(d).charAt(0).toUpperCase() + feriaToDay(d).slice(1) + ' in the Week before Advent'
        }*/


        metadata.advent[3].wednesday.name = 'Ember Wednesday of Advent'
        metadata.advent[3].wednesday.rank = 7
        metadata.advent[3].wednesday.penance = 'strict-fast'
        metadata.advent[3].friday.name = 'Ember Friday of Advent'
        metadata.advent[3].friday.rank = 7
        metadata.advent[3].friday.penance = 'strict-fast'
        metadata.advent[3].saturday.name = 'Ember Saturday of Advent'
        metadata.advent[3].saturday.rank = 7
        metadata.advent[3].saturday.penance = 'vigil'

        metadata.lent[1].wednesday.name = 'Ember Wednesday of Lent'
        metadata.lent[1].wednesday.penance = 'strict-fast'
        metadata.lent[1].wednesday.rank = 7
        metadata.lent[1].friday.name = 'Ember Friday of Lent'
        metadata.lent[1].friday.penance = 'strict-fast'
        metadata.lent[1].friday.rank = 7
        metadata.lent[1].saturday.name = 'Ember Saturday of Lent'
        metadata.lent[1].saturday.penance = 'vigil'
        metadata.lent[1].saturday.rank = 7

        // pentecost embertide is handled at its octave

        metadata.september[3].wednesday.name = 'Ember Wednesday of September'
            metadata.september[3].wednesday.penance = 'strict-fast'
            metadata.september[3].wednesday.color = 'violet'
            metadata.september[3].wednesday.rank = 7
        metadata.september[3].friday.name = 'Ember Friday of September'
            metadata.september[3].friday.penance = 'strict-fast'
            metadata.september[3].friday.color = 'violet'
            metadata.september[3].friday.rank = 7
        metadata.september[3].saturday.name = 'Ember Saturday of September'
            metadata.september[3].saturday.penance = 'vigil'
            metadata.september[3].saturday.color = 'violet'
            metadata.september[3].saturday.rank = 7

        for (var w in metadata.advent) {
            for (var d in metadata.advent[w]) {
                metadata.advent[w][d].volume = 1
            }
        }

        return metadata;
    }

    function getSanctoralMetaData() {
        return {
            'january': {
                '10': {
                    name: 'Saint Paul, Hermit',
                    color: 'white',
                    rank: 6
                },
                '17': {
                    name: 'Saint Anthony, Abbot',
                    color: 'white',
                    rank: 6
                },
                '21': {
                    name: 'Saint Agnes, Virgin-Martyr',
                    color: 'rose',
                    rank: 5
                },
                '24': {
                    name: 'Saint Timothy, Bishop and Martyr',
                    color: 'red',
                    rank: 6
                },
                '25': {
                    name: 'Conversion of Saint Paul, Apostle',
                    color: 'white',
                    rank: 3
                },
                '30': {
                    name: 'Saint Gregory Nazianzus, Bishop and Doctor',
                    color: 'white',
                    rank: 5
                }
            },
            'february': {
                '2': {
                    name: 'Purification of the Blessed Virgin Mary',
                    color: 'blue',
                    rank: 1,
                    penance: 'none',
                    note: 'Liturgical color of the procession is violet.'
                },
                '3': {
                    name: 'Saint Blase, Bishop and Martyr',
                    color: 'red',
                    rank: 6
                },
                '5': {
                    name: 'Saint Agatha, Virgin-Martyr',
                    color: 'rose',
                    rank: 5
                },
                '10': {
                    name: 'Saint Scholastica, Virgin',
                    color: 'white',
                    rank: 6
                },
                '14': {
                    name: 'Saint Valentine, Priest and Martyr',
                    color: 'red',
                    rank: 6
                }
            },
            'march': {
                '7': {
                    name: 'Saints Perpetua and Felicity, Martyrs',
                    color: 'red',
                    rank: 6
                },
                '12': {
                    name: 'Saint Gregory, Pope and Doctor',
                    color: 'white',
                    rank: 5
                },
                '17': {
                    name: 'Saint Patrick, Bishop',
                    color: 'white',
                    rank: 6
                },
                '21': {
                    name: 'Saint Benedict, Abbot',
                    color: 'white',
                    rank: 5
                },
                '25': {
                    name: 'Annunciation of the Blessed Virgin Mary',
                    rank: 3,
                    color: 'blue'
                },
            },
            'april': {
                '14': {
                    name: 'Saint Justin, Martyr',
                    color: 'red',
                    rank: 6
                },
                '23': {
                    name: 'Saint George, Martyr',
                    color: 'red',
                    rank: 6
                },
                '25': {
                    name: 'Saint Mark, Evangelist',
                    color: 'red',
                    rank: 3
                },
                '26': {
                    name: 'Saint Cletus, Pope and Martyr',
                    color: 'red',
                    rank: 6
                },
                '30': {
                    name: 'Eve of Saints Philip and James',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                },
            },
            'may': {
                '1': {
                    name: 'Saints Philip and James, Apostles',
                    color: 'red',
                    rank: 3
                },
                '2': {
                    name: 'Saint Athanasius, Bishop and Doctor',
                    color: 'white',
                    rank: 6
                },
                '3': {
                    name: 'Saint Alexander, Pope and Martyr',
                    color: 'red',
                    rank: 6
                },
                '6': {
                    name: 'Saint John, Apostle, before the Latin Gate',
                    color: 'red',
                    rank: 3
                },
                '14': {
                    name: 'Saint Matthias, Apostle',
                    color: 'red',
                    rank: 3
                },
                '25': {
                    name: 'Saint Bede, Priest and Doctor',
                    color: 'white',
                    rank: 6
                },
                '31': {
                    name: 'Visitation of the Blessed Virgin Mary',
                    color: 'blue',
                    rank: 1,
                    penance: 'none'
                }
            },
            'june': {
                '2': {
                    name: 'Saints Marcellinus and Peter, Martyrs',
                    color: 'red',
                    rank: 6
                },
                '11': {
                    name: 'Saint Barnabas, Apostle',
                    color: 'red',
                    rank: 5
                },
                '14': {
                    name: 'Saint Basil of Caesarea, Bishop and Doctor',
                    color: 'white',
                    rank: 5
                },
                '18': {
                    name: 'Dedication of Holy Trinity Church',
                    color: 'white',
                    rank: 1,
                    penance: 'none'
                },
                '23': {
                    name: 'Eve of Saint John Baptist',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                },
                '24': {
                    name: 'Nativity of Saint John Baptist',
                    color: 'white',
                    rank: 3
                },
                '26': {
                    name: 'Saints John and Paul, Martyrs',
                    color: 'red',
                    rank: 6
                },
                '28': {
                    name: 'Eve of Saints Peter and Paul',
                    rank: 8,
                    color: 'violet',
                    penance: 'vigil'
                },
                '29': {
                    name: 'Saints Peter and Paul, Apostles',
                    color: 'red',
                    rank: 3
                },
                '30': {
                    name: 'Synaxis of Saint Paul, Apostle',
                    color: 'red',
                    rank: 3
                }
            },
            'july': {
                '2': {
                    name: 'Eve of Saint Thomas',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                },
                '3': {
                    name: 'Saint Thomas, Apostle',
                    color: 'red',
                    rank: 3
                },
                '22': {
                    name: 'Saint Mary Magdalene',
                    color: 'white',
                    rank: 5
                },
                '24': {
                    name: 'Eve of Saint James',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                },
                '25': {
                    name: 'Saint James the Greater, Apostle',
                    color: 'red',
                    rank: 3
                },
                '27': {
                    name: 'Saint Pantaleon, Martyr',
                    color: 'red',
                    rank: 6
                }
            },
            'august': {
                '6': {
                    name: 'Transfiguration of the Lord',
                    color: 'white',
                    rank: 3
                },
                '7': {
                    name: 'Saint Sixtus, Pope and Martyr',
                    color: 'red',
                    rank: 6
                },
                '10': {
                    name: 'Saint Laurence, Deacon and Martyr',
                    color: 'red',
                    rank: 5
                },
                '14': {
                    name: 'Eve of the Assumption',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                },
                '15': {
                    name: 'Assumption of the Blessed Virgin Mary',
                    color: 'blue',
                    rank: 1,
                    penance: 'none'
                },
                '23': {
                    name: 'Eve of Saint Bartholomew',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                },
                '24': {
                    name: 'Saint Bartholomew, Apostle',
                    color: 'red',
                    rank: 3
                },
                '28': {
                    name: 'Saint Augustine of Hippo, Bishop and Doctor',
                    color: 'white',
                    rank: 5
                },
                '29': {
                    name: 'Beheading of Saint John Baptist',
                    color: 'red',
                    rank: 5
                }
            },
            'september': {
                '8': {
                    name: 'Nativity of the Blessed Virgin Mary',
                    color: 'blue',
                    rank: 1,
                    penance: 'none'
                },
                '13': {
                    name: 'Saint John Chrysostom, Bishop and Doctor',
                    color: 'white',
                    rank: 5
                },
                '14': {
                    name: 'Exaltation of the Holy Cross',
                    color: 'red',
                    rank: 3
                },
                '16': {
                    name: 'Saints Cornelius and Cyprian, Bishops and Martyrs',
                    color: 'red',
                    rank: 6
                },
                '20': {
                    name: 'Eve of Saint Matthew',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                },
                '21': {
                    name: 'Saint Matthew, Apostle and Evangelist',
                    color: 'red',
                    rank: 3
                },
                '23': {
                    name: 'Saint Linus, Pope and Martyr',
                    color: 'red',
                    rank: 6
                },
                '26': {
                    name: 'Saints Cosmas and Damian, Martyrs',
                    color: 'red',
                    rank: 6
                },
                '28': {
                    name: 'Saint Anastasia, Virgin-Martyr',
                    color: 'rose',
                    rank: 6
                },
                '29': {
                    name: 'Saint Michael, Archangel',
                    color: 'white',
                    rank: 3
                },
                '30': {
                    name: 'Saint Jerome, Priest and Doctor',
                    color: 'white',
                    rank: 5
                }
            },
            'october': {
                '2': {
                    name: 'Holy Guardian Angels',
                    color: 'white',
                    rank: 5
                },
                '17': {
                    name: 'Saint Ignatius of Antioch, Bishop and Martyr',
                    color: 'red',
                    rank: 6
                },
                '18': {
                    name: 'Saint Luke, Evangelist',
                    color: 'red',
                    rank: 3
                },
                '27': {
                    name: 'Eve of Saints Simon and Jude',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                },
                '28': {
                    name: 'Saints Simon and Jude, Apostles',
                    color: 'red',
                    rank: 3
                },
                '31': {
                    name: 'Eve of the Communion',
                    color: 'violet',
                    rank: 8,
                    penance: 'vigil'
                }
            },
            'november': {
                '1': {
                    name: 'Communion of All Saints',
                    color: 'white',
                    rank: 1,
                    penance: 'none'
                },
                '2': {
                    name: 'Synaxis of the Holy Souls',
                    color: 'black',
                    rank: 8 // this also must be counted with feria
                },
                '11': {
                    name: 'Saint Martin, Confessor',
                    color: 'white',
                    rank: 6
                },
                '21': {
                    name: 'Presentation of the Blessed Virgin Mary',
                    color: 'blue',
                    rank: 5
                },
                '22': {
                    name: 'Saint Cecilia, Virgin-Martyr',
                    color: 'rose',
                    rank: 5
                },
                '23': {
                    name: 'Saint Clement, Pope and Martyr',
                    color: 'red',
                    rank: 5
                },
                '24': {
                    name: 'Saint Chrysogonus, Martyr',
                    color: 'red',
                    rank: 6
                },
                '25': {
                    name: 'Saint Catherine, Virgin-Martyr',
                    color: 'rose',
                    rank: 6
                },
                '29': {
                    name: 'Eve of Saint Andrew, Apostle',
                    color: 'violet',
                    penance: 'vigil',
                    rank: 8 // we will treat it as a feria
                },
                '30': {
                    name: 'Saint Andrew, Apostle',
                    color: 'red',
                    rank: 3
                }
            },
            'december': {
                '6': {
                    name: 'Saint Nicholas, Bishop and Confessor',
                    color: 'white',
                    rank: 5
                },
                '7': {
                    name: 'Saint Ambrose, Bishop and Doctor',
                    color: 'white',
                    rank: 5
                },
                '8': {
                    name: 'Conception of the Blessed Virgin Mary',
                    color: 'blue',
                    rank: 3
                },
                '13': {
                    name: 'Saint Lucy, Virgin-Martyr',
                    color: 'rose',
                    rank: 5
                }
            }
        }
    }

    oMeta = {
        december: {
            '17': {
                name: 'O Wisdom'
            },
            '18': {
                name: 'O Adonay'
            },
            '19': {
                name: 'O Root of Jesse'
            },
            '20': {
                name: 'O Key of David'
            },
            '21': {
                name: 'O Dayspring'
            },
            '22': {
                name: 'O King of the Gentiles'
            },
            '23': {
                name: 'O Emmanuel'
            }
        }
    };

tMeta = getTemporalMetaData();
sMeta = getSanctoralMetaData();

function queryTMeta(calendar, firstVespers = false) {
        // convert calendar data  to its metadata
            var metadata = JSON.parse(JSON.stringify(tMeta));
            var id = calendar.id.split('/');
            for (var i = 0; i < id.length; i++) {
                var part = id[i]
                if (metadata[part] != undefined) {
                    metadata = metadata[part]
                }
            }

            if (calendar.oid != undefined) {
                var oidm = JSON.parse(JSON.stringify(oMeta));
                var id = calendar.oid.split('/');
                    for (var i = 0; i < id.length; i++) {
                    var part = id[i]
                        if (oidm[part] != undefined) {
                            oidm = oidm[part]
                        }
                    }
            }

            if (calendar.subid != undefined) {
                var om = JSON.parse(JSON.stringify(tMeta));
                var id = calendar.subid.split('/');
                    for (var i = 0; i < id.length; i++) {
                    var part = id[i]
                        if (om[part] != undefined) {
                            om = om[part]
                        }
                    }
            }

            if (calendar.stid != undefined) {
                var st = JSON.parse(JSON.stringify(sMeta));
                var id = calendar.stid.split('/');
                    for (var i = 0; i < id.length; i++) {
                    var part = id[i]
                        if (st[part] != undefined) {
                            st = st[part]
                        }
                    }
            }

            if (om != undefined) {
                    if (om.rank < metadata.rank ) {
                        // everything from subid
                        metadata = JSON.parse(JSON.stringify(om))
                    } else if (om.rank == metadata.rank && metadata.rank != 7) {
                        // merge: om has priority
                        metadata.backup = JSON.parse(JSON.stringify(metadata))
                        metadata.backup.id = calendar.id
                        om.id = calendar.subid
                        mergeDeep(metadata, om)
                        var id = metadata.backup.id.split('/')
                        metadata.name = om.name + ' and ' + ordinal_suffix_of(id[1]) + ' after ' + id[0].charAt(0).toUpperCase() + id[0].slice(1)
                    }

                // else, everything is taken from metadata
            }

            if (st != undefined) {
                /*metadata.backup = JSON.parse(JSON.stringify(metadata))
                if (st.rank > 5) {
                    // in this case, merging the two is necessary
                    mergeDeep(metadata, st); // st is given the priority
                } else if (st.rank < metadata.rank) {
                    metadata = st; // this can only be true if >simplex anyways
                } else if (st.rank == metadata.rank) {
                    // this ought only be vigil days
                    mergeDeep(metadata, st);
                }*/

                // if today is feria, always merge, unless strong feria
                if (metadata.rank == 8) {
                    mergeDeep(metadata, st);
                }

                // if the saint is ranked higher than the feria
                if (st.rank < metadata.rank) {
                    if (metadata.rank != 7) {
                        metadata = st;
                    } else if (firstVespers) {
                        metadata = st;
                    }
                }
            }

            if (oidm != undefined) {
                metadata.note = oidm.name;
                mergeDeep(oidm, metadata);
            }
            

            switch (metadata.color) {
            case 'violet': metadata.color = '#4c0099'; break;
            case 'rose': metadata.color = '#fa48ad'; break;
            case 'blue': metadata.color = '#468FEA'; break;
            }

            if (metadata.penance === undefined) metadata.penance = 'none'
            return metadata
    }

    function getLiturgicalCalendar(year) {
        function gendat(m, d, y = year) {
            var n = new Date(y, m - 1, d);
            if (n.getMonth() != (m -1) || n.getDate() != d) {
                console.error('request for ' +m + '/' + d + 'failed')
                alert('gendat failed')
                throw new Error('gendat failed')
            }
            return n;
        }

        var easterSunday = easter_computus(year);
        var adventSunday = advent_computus(year);

        var calendar = {};

        console.log("In the year of Our Lord " + year)

        console.log("Easter is " + easterSunday.toDateString());
        console.log("Advent is " + adventSunday.toDateString());

        /* the sundays of paschaltide */

        var nxtSunday = new Date(easterSunday);

        for (var i = 0; i < 6; i++) {
            calendar[keyDate(nxtSunday)] = {
                //name: (i == 0) ? "Pascha" : ordinal_suffix_of(i) + " Sunday after Pascha",
                id: "pascha/" + (i + 1) + "/sunday",
                date: nxtSunday.toDateString(),
            };

            nxtSunday.setDate(nxtSunday.getDate() + 7);
        }

        /* days of the ascension octave */
        var ascension = new Date(nxtSunday);
        advdat(ascension, -3);
        for (i = 0; i < 8; i++) {
            if (i == 0) {
                calendar[keyDate(ascension)] = {
                    //name: 'Ascension of the Lord',
                    id: "ascension/octave/1",
                    date: ascension.toDateString(),
                };
            } else if (i == 7) {
                calendar[keyDate(ascension)] = {
                    //name: 'Octave of the Ascension',
                    id: "ascension/octave/8",
                    date: ascension.toDateString(),
                };
            } else {
                //if (calendar[keyDate(ascension)] === undefined) {
                calendar[keyDate(ascension)] = {
                    //name: ordinal_suffix_of(i) + ' Day after Ascension',
                    id: "ascension/octave/" + (i + 1),
                    date: ascension.toDateString(),
                };
                //}
            }

            advdat(ascension);
        }

        calendar[keyDate(ascension)] = {
            id: 'ascension/feria',
            date: ascension.toDateString()
        }

        /* pentecost sunday */

        calendar[keyDate(advdat(nxtSunday, 7))] = {
            //name: 'Pentecost',
            id: "pentecost/0/sunday",
            date: nxtSunday.toDateString(),
        };

        /* the sundays after pentecost */

        var seventhSunday;

        for (var i = 0; i < 24; i++) {
            advdat(nxtSunday, 7);
            if (i == 6) {
                seventhSunday = new Date(nxtSunday);
            }
            calendar[keyDate(nxtSunday)] = {
                //name: ordinal_suffix_of(i + 1) + ' Sunday after Pentecost',
                id: "pentecost/" + (i + 1) + "/sunday",
                date: nxtSunday.toDateString(),
            };
        }

        function subidSundays(date, n, id) {
            if (date.getDay() == 0) {
                advdat(date);
            }

            while (date.getDay() != 0) {
                advdat(date);
            }

            for (var i = 0; i < n; i++) {
                if (calendar[keyDate(date)].id.split("/")[0] != "pentecost" && calendar[keyDate(date)].id.split("/")[0] != "epiphany") {
                    //console.log(date.toDateString() + " would be " + id + '/' + (i + 1) + '/sunday')
                    //console.log("\tcannot be included in advent")
                } else {
                    calendar[keyDate(date)].subid = id + "/" + (i + 1) + "/sunday";
                }
                advdat(date, 7);
            }
        }

        function queryid(id) {
            for (var x in calendar) {
                if (calendar[x].id == id) return x;
            }
            return -1;
        }

        function querysubid(id) {
            for (var x in calendar) {
                if (calendar[x].subid == id) return x;
            }
            return -1;
        }

        // before we can add in the november dates, we must ensure we have continuous sundays from pentecost until advent
        // if there are sundays missing, we can add in the octaves of the Assumption and All Saints, in this ordrer
        // the octave-days will overwrite the sunday
        // the sunday will be deffered to the next sunday
        // now let's say we had this scheudle
        /**
         * Nth Sunday after Pentecost
         * Monday
         * Tuesday
         * Assumption (Wednesday)
         * 2nd (Thursday)
         * 3rd (Friday)
         * 4th (Saturday)
         * 5th (Sunday)
         * 6th (Monday)
         * 7th (Tuesday)
         * Octave (Wednesday)
         * */
        // The octave of the assumption repealed Wednesday->Saturday
        // so the days after the octave would be the Thursday, Friday, and Saturday offices removed; Wednesday is lost to the feast itself
        // thus, the Sunday is deferred to next week, and begins there, without losing any offices

        // now, so as to maintain a common calendar, the Assumption and All Saints will always have octaves, and these octaves will always defer the sunday within them to the next

        /* the four sundays of advent */

        nxtSunday = new Date(adventSunday);
        for (var i = 0; i < 4; i++) {
            calendar[keyDate(nxtSunday)] = {
                //name: ordinal_suffix_of(i + 1) + " Sunday of Advent",
                id: "advent/" + (i + 1) + "/sunday",
                date: nxtSunday.toDateString(),
            };
            advdat(nxtSunday, 7);
        }

        /* Sunday next before Advent */

        nxtSunday = advdat(new Date(adventSunday), -7);
        calendar[keyDate(nxtSunday)] = {
            //name: 'Sunday before Advent',
            id: "pentecost/28/sunday",
            date: nxtSunday.toDateString(),
        };

        // we must now fill in the missing sundays, up to 2
        {
            var i = 25;
            var epi = 4;
            while (calendar[keyDate(advdat(nxtSunday, -7))] === undefined) {
                // bring us to an undefined suday
            }

            while (calendar[keyDate(advdat(nxtSunday, 7))] === undefined) {
                calendar[keyDate(nxtSunday)] = {
                    id: "pentecost/" + i + "/sunday",
                    date: nxtSunday.toDateString(),
                    refid: "epiphany/" + epi + "/sunday",
                };
                i += 1;
                epi += 1;
            }
        }

        // we can now fill in the autumn sundays
        /* the sundays of wisdom */
        /* first sunday aftr the fifth kalends of august */
        /* which i understand to be the 28th of july */
        subidSundays(gendat(7, 28), 5, "august");
        /**
         * August I = Proverbs
         * August II = Ecclesiastes
         * August III = Wisdom
         * August IV = Sirach
         * August V = Sirach
         /* 

    /* the sundays of job */
        /* first sunday after the fifth kalends of septmber */
        subidSundays(gendat(8, 28), 5, "september");
        /**
         * September I = Job
         * September II = Job
         * Septemver III = Tobit (where i beleive falls the september embertide)
         * Septemver IV = Tobit (but if there only be 4 weeks, = Setpevmer V)
         * Septemver V = Judith
         * */

        subidSundays(gendat(9, 27), 5, "october");
        /**
         * October I = 1 Macchabbes
         * October II = 1 Maccabees
         * October III = 1 Maccabees
         * Octover IV = 2 Maccabees
         * October V = 2 Maccabess
         * */

        if (querysubid("september/5/sunday") == -1) {
            // apply above rubric
            var x = querysubid("september/4/sunday");
            calendar[x].subid = "september/5/sunday";
            calendar[x].transfers = {
                subid: true,
            };
        }

        subidSundays(gendat(10, 28), 5, "november");
        /**
         * November I = Ezekiel
         * November II = Ezekiel (but if 4 weeks = November III)
         * November III = Daniel (but if 4 weeks = November IV)
         * November IV = Hosea, Joel, Amos, Obadiah, Jonah (but if 4 weeks = Novermber V)
         * November V = Micah, Nahum, Habbakkuk, Zephaniah, Haggai, Zechariah, Malachi
         * */

        /* the two sundays of passiontide */

        nxtSunday = easterSunday;
        for (var i = 0; i < 2; i++) {
            calendar[keyDate(advdat(nxtSunday, -7))] = {
                //name: (i == 0) ? 'Palm Sunday' : 'Passion Sunday',
                id: "passion/" + (2 - i) + "/sunday",
                date: nxtSunday.toDateString(),
            };
        }

        /* the four sundays of lent */
        for (var i = 0; i < 4; i++) {
            calendar[keyDate(advdat(nxtSunday, -7))] = {
                //name: ordinal_suffix_of(4 - i) + ' Sunday in Lent',
                id: "lent/" + (4 - i) + "/sunday",
                date: nxtSunday.toDateString(),
            };
        }

        /* the three sundays before lent */
        for (var i = 0; i < 3; i++) {
            calendar[keyDate(advdat(nxtSunday, -7))] = {
                //name: ordinal_suffix_of(i + 1) + ' Sunday before Lent',
                id: "prelent/" + (i + 1) + "/sunday",
                date: nxtSunday.toDateString(),
            };
        }

        /* epiphany of the lord */
        var epiphany = gendat(1, 6); // jan 6
        calendar[keyDate(epiphany)] = {
            id: 'epiphany/octave/1',
            date: epiphany.toDateString()
        }
        advdat(epiphany)

        /*for (var i = 0; i < 8; i++) {
            calendar[keyDate(epiphany)] = {
                //name: (i == 0) ? 'Epiphany of the Lord' : ordinal_suffix_of(i) + ' Day of the Epiphany',
                id: "epiphany/octave/" + (i + 1),
                date: epiphany.toDateString(),
            };

            if (epiphany.getDay() == 0 && i > 0) {
                calendar[keyDate(epiphany)].id = "epiphany/octave/sunday";
                nxtSunday = new Date(epiphany);
                ;
            }
            advdat(epiphany);
        }*/

        /* days until first sunday after octave of epiphany */

        {
            var i = 1;
            while (epiphany.getDay() != 0) {
                calendar[keyDate(epiphany)] = {
                    // name: ordinal_suffix_of(i) + ' Feria after Epiphanytide',
                    id: "christmas/feria/" + epiphany.getDay(),
                    date: epiphany.toDateString(),
                };
                i += 1;
                advdat(epiphany);
            }
        }

        nxtSunday = epiphany;

        /* the sundays after epiphny */
        {
            //advdat(nxtSunday, 7);
            var i = 1;
            while (calendar[keyDate(nxtSunday)] === undefined) {
                calendar[keyDate(nxtSunday)] = {
                    //name: ordinal_suffix_of(i + 1) + ' Sunday after Epihpany',
                    id: "epiphany/" + i + "/sunday",
                    date: nxtSunday.toDateString(),
                };
                advdat(nxtSunday, 7);
                i += 1;
            }
        }

        /* circumcision &c until epiphany */
        calendar[keyDate(gendat(1, 1))] = {
            //name: 'Circumcision of the Lord',
            id: "christmas/octave/8",
            date: gendat(1, 1).toDateString(),
        };

        calendar[keyDate(gendat(1, 2))] = {
            //name: 'Octave of Saint Stephen',
            id: "christmas/octave/9",
            date: gendat(1, 2).toDateString(),
        };

        calendar[keyDate(gendat(1, 3))] = {
            //name: 'Octave of Saint John',
            id: "christmas/octave/10",
            date: gendat(1, 3).toDateString(),
        };

        calendar[keyDate(gendat(1, 4))] = {
            //name: 'Octave of the Holy Innocents',
            id: "christmas/octave/12",
            date: gendat(1, 4).toDateString(),
        };

        calendar[keyDate(gendat(1, 5))] = {
            //name: 'Eve of the Epiphany',
            id: "epiphany/vigil",
            date: gendat(1, 5).toDateString(),
        };

        /* christmas &c */
        calendar[keyDate(gendat(12, 24))] = {
            // name: 'Eve of the Nativity',
            id: "christmas/vigil",
            date: gendat(12, 24).toDateString(),
        };

        if (gendat(12,24).getDay() == 0) {
            calendar[keyDate(gendat(12, 24))].id = 'christmas/vigil-sunday'
        }

        if (gendat(1,5).getDay() == 0) {
            calendar[keyDate(gendat(1, 5))].id = 'epiphany/vigil-sunday'
        }

        calendar[keyDate(gendat(12, 25))] = {
            //  name: 'Nativity of the Lord',
            id: "christmas/octave/1",
            date: gendat(12, 25).toDateString(),
        };

        calendar[keyDate(gendat(12, 26))] = {
            //name: 'Saint Stephen, Protomartyr',
            id: "christmas/octave/2",
            date: gendat(12, 26).toDateString(),
        };

        calendar[keyDate(gendat(12, 27))] = {
            //name: 'Saint John, Apostle and Evangelist',
            id: "christmas/octave/3",
            date: gendat(12, 27).toDateString(),
        };

        calendar[keyDate(gendat(12, 28))] = {
            //name: 'Holy Innocents, Martyrs',
            id: "christmas/octave/4",
            date: gendat(12, 28).toDateString(),
        };

        calendar[keyDate(gendat(12, 29))] = {
            //name: 'Saint Thomas Becket, Bishop and Martyr',
            id: "christmas/octave/5",
            date: gendat(12, 29).toDateString(),
        };

        calendar[keyDate(gendat(12, 30))] = {
            //name: 'Sixth Day of the Nativity',
            id: "christmas/octave/6",
            date: gendat(12, 30).toDateString(),
        };

        calendar[keyDate(gendat(12, 31))] = {
            // name: 'Seventh Day of the Nativity',
            id: "christmas/octave/7",
            date: gendat(12, 31).toDateString(),
        };

        if (querysubid("november/5/sunday") == -1) {
            var x = querysubid("november/2/sunday");
            calendar[x].subid = "november/3/sunday";
            calendar[x].transfers = {
                subid: true,
            };
            calendar[Number(x) + 7].subid = "november/4/sunday";
            calendar[Number(x) + 7].transfers = {
                subid: true,
            };
            calendar[Number(x) + 7 + 7].subid = "november/5/sunday";
            calendar[Number(x) + 7 + 7].transfers = {
                subid: true,
            };
        }

        {
            var x = queryid("pentecost/28/sunday");
            if (calendar[Number(x) - 7].id != "pentecost/27/sunday") {
                if (calendar[x].transfers === undefined) calendar[x].transfers = {};
                calendar[x].transfers.id = true;
            }
        }

        /* ascension eve */
        var x = queryid("ascension/octave/1");
        calendar[Number(x) - 1] = {
            id: "ascension/vigil",
            date: advdat(new Date(calendar[x].date), -1).toDateString(),
        };

        /* pentecost eve*/
        var x = queryid("pentecost/0/sunday");
        calendar[Number(x) - 1] = {
            id: "pentecost/vigil",
            date: advdat(new Date(calendar[x].date), -1).toDateString(),
        };

        // now we fill in with saints days
        // i see no better way than a manual insertation
        function sanc(date) {
            var id = date.toLocaleString('default', { month: 'long' }).toLowerCase() + '/' + date.getDate();
            if (calendar[keyDate(date)] == undefined) {
                calendar[keyDate(date)] = {
                    id: '',
                    stid: id,
                    date: date.toDateString(),
                }
            } else {
                calendar[keyDate(date)].stid = id;
            }
        }

        //sanc(gendat(1, 10))
        //sanc(gendat(1, 17))
        sanc(gendat(1, 21))
        //sanc(gendat(1, 24))
        sanc(gendat(1, 25))
        //sanc(gendat(1, 30))
        //sanc(gendat(2, 1))
        sanc(gendat(2, 2))
        //sanc(gendat(2, 3))
        sanc(gendat(2, 5))
        //sanc(gendat(2, 10))
        sanc(gendat(2, 14))
        sanc(gendat(3, 7))
        //sanc(gendat(3, 12))
        //sanc(gendat(3, 17))
        sanc(gendat(3, 21))
        sanc(gendat(3, 25))
        //sanc(gendat(4, 14))
        sanc(gendat(4, 23))
        sanc(gendat(4, 25))
        //sanc(gendat(4, 26))
        sanc(gendat(4, 30))
        sanc(gendat(5, 1))
        //sanc(gendat(5, 2))
        sanc(gendat(5, 3))
        sanc(gendat(5, 6))
        sanc(gendat(5, 14))
        sanc(gendat(5, 31))
        sanc(gendat(6, 2))
        sanc(gendat(6, 11))
        //sanc(gendat(6, 14))
        sanc(gendat(6, 18)) // dedication of holy trinity
        sanc(gendat(6, 23))
        sanc(gendat(6, 24))
        sanc(gendat(6, 26))
        sanc(gendat(6, 28))
        sanc(gendat(6, 29))
        sanc(gendat(6, 30))
        sanc(gendat(7, 2))
        sanc(gendat(7, 3))
        sanc(gendat(7, 22))
        sanc(gendat(7, 24))
        sanc(gendat(7, 25))
        sanc(gendat(7, 27))
        sanc(gendat(8, 6))
        sanc(gendat(8, 7))
        sanc(gendat(8, 10))
        sanc(gendat(8, 14))
        sanc(gendat(8, 15))
        sanc(gendat(8, 23))
        sanc(gendat(8, 24))
        //sanc(gendat(8, 28))
        sanc(gendat(8, 29))
        sanc(gendat(9, 8))
        //sanc(gendat(9, 13))
        sanc(gendat(9, 14))
        sanc(gendat(9, 16))
        sanc(gendat(9, 20))
        sanc(gendat(9, 21))
        sanc(gendat(9, 23))
        sanc(gendat(9, 26))
        sanc(gendat(9, 28))
        sanc(gendat(9, 29))
        //sanc(gendat(9, 30))
        //sanc(gendat(10, 2))
        sanc(gendat(10, 17))
        sanc(gendat(10, 18))
        sanc(gendat(10, 27))
        sanc(gendat(10, 28))
        sanc(gendat(10, 31))
        sanc(gendat(11, 1))
        sanc(gendat(11, 2))
        //sanc(gendat(11, 11))
        //sanc(gendat(11, 21))
        sanc(gendat(11, 22))
        sanc(gendat(11, 23))
        sanc(gendat(11, 24))
        //sanc(gendat(11, 25))
        sanc(gendat(11, 29))
        sanc(gendat(11, 30))
        sanc(gendat(12, 6))
        //sanc(gendat(12, 7))
        sanc(gendat(12, 8))
        sanc(gendat(12, 13))

        // we can now fill in the feria
        
        function prevSunday(n) {
            // get the previous sunday from a index on the calendar
            // note that this doesn't look at sundays, but at days of the form '[season]/[week]/sunday'
            while (calendar[n] === undefined) {
                if (n <= 0) {
                    return -1
                }
                n -= 1
            }

            var id = calendar[n].id.split('/');
            if (id.length < 3) {
                return prevSunday(n - 1)
            }

            if (id[2] != 'sunday') {
                return prevSunday(n- 1)
            }

            return n
        }

        for (var i = 1; i <= queryid('christmas/octave/7'); i++) {
            var prev = prevSunday(i);
            if (prev == -1) {
                continue;
            };
            if (calendar[i] === undefined || calendar[i].id == '') {
                var thisdate = new Date(calendar[prev].date);
                var previd = calendar[prev].id.split('/')
               for (var n = 1; n < 7; n++) {
                    var x =keyDate(advdat(thisdate))
                    if (calendar[x] != undefined) {
                        if (calendar[x].id != '') {continue}
                            else {
                                calendar[x].id = previd[0] + '/' + previd[1] + '/' + feriaToDay(n);
                                calendar[x].transfers = calendar[prev].transfers;
                                continue;
                            }
                    }
                    calendar[x] = {
                        id: previd[0] + '/' + previd[1] + '/' + feriaToDay(n),
                        date: thisdate.toDateString(),
                    }
                    if (calendar[prev].transfers != undefined) {
                        calendar[x].transfers = calendar[prev].transfers
                    }
               }
            }
        }

        // now do the same for the sundays of august &c
        for (var i = querysubid('august/1/sunday'); i < queryid('advent/1/sunday'); i++) {
            var prev = prevSunday(i);
            if (prev == -1) continue;
            if (calendar[i].subid === undefined) {
                var thisdate = new Date(calendar[prev].date);
                var previd = calendar[prev].subid.split('/')
                for (var n = 1; n < 7; n++) {
                    var x = keyDate(advdat(thisdate))
                    if (calendar[x].subid != undefined) continue;
                    calendar[x].subid = previd[0] + '/' + previd[1] + '/' + feriaToDay(n)
                }
            }
        }

        /* o antiphons */
        /*var dec17 = gendat(12, 17)
        for (var i = 0; i < 7; i++) {
            calendar[keyDate(dec17)].subid = 'december/' + (i + 17)
            advdat(dec17)
        }*/

        // verify
        for (var i = 1; i <= queryid('christmas/octave/7'); i++) {
            if (calendar[i] === undefined) {
                alert("ERROR: calendar["+i+"] = undefined")
            }
        }

        for (var i = querysubid('august/1/sunday'); i < queryid('advent/1/sunday'); i++) {
            if (calendar[i].subid == undefined) {
                alert("ERROR: calendar["+i+"].subid = undefined")
            }
        }

        for (var i = 1; i <= 366; i++) {
            if (calendar[i] === undefined) {
                console.error(i)
            }
        }

        console.log(calendar)

        for (var x = 17; x < 24; x++) {
            calendar[keyDate(gendat(12, x))].oid = 'december/' + x
        }

        return calendar;
    }

function getLiturgicalDay(date) {
    console.log(date)
    var easter = easter_computus(date.getFullYear());
    var advent = advent_computus(date.getFullYear());
    var calendar = getLiturgicalCalendar(date.getFullYear());

    var today = calendar[keyDate(date)]
    var tomorrow = calendar[keyDate(advdat(new Date(date)))];

    var m1 = queryTMeta(today);
    var m2 = queryTMeta(tomorrow, false);
        
    var today_minus_saint = JSON.parse(JSON.stringify(today));
    delete today_minus_saint.stid;

    var m3 = queryTMeta(today_minus_saint)

    m2.penance = m1.penance;
    m3.penance = m2.penance; // this is necessary to ensure proper supper/dinner after first vespers

    var firstVespers = (m2.rank < 7); // whether tomorrow has first vespers
    var secondVespers = (m1.rank != 6 && m1.rank != 5); // whether today has vespers

    var higherFeast = (m2.rank < m1.rank); // whether tomorrow is a greater feast

    var separateVespers; // whether to separate Vespers and Compline
    var ferialVespers = !higherFeast; // if separateVespers, whether to use the ferial (m3)
                            // if false, uses with saints (m2)
    if (secondVespers && firstVespers) {
            console.log('Today has Vespers, and tomorrow has first Vespers.')
            if (higherFeast) {
                console.log("Vespers and Compline of tomorrow")
                separateVespers = true;
            } else {
                console.log("Vespers and Compline of today")
                separateVespers = false;
            }
        } else if (!secondVespers && firstVespers) {
            console.log('Today has no Vespers, but tomorrow has first Vespers.')
            console.log('Vespers and Compline of tomorrow')
            separateVespers = true;
            ferialVespers = false;
        } else if (secondVespers && !firstVespers) {
            console.log('Today has second Vespers and tomorrow has no first Vespers.')
            console.log('Vespers and Compline of today.')
            separateVespers = false;
        } else {
            console.log('Today has no Vespers and tomorrow has no first Vespers.')
            console.log('We will default to ferial!')
            separateVespers = true;
            ferialVespers = true
        }

        console.log('separateVespers = ' + separateVespers + ', ferialVespers = ' + ferialVespers)

    let hours = {
            Vigils: {
                duration: 1.5,
                liturgy: today,
                metadata: m1
            },
            Lauds: {
                duration: 0.75,
                liturgy: today,
                metadata: m1
            },
            Prime: {
                duration: 0.25,
                liturgy: today,
                metadata: m1
            },
            Terce: {
                duration: 0.25,
                liturgy: today,
                metadata: m1
            },
            Sext: {
                duration: 0.25,
                liturgy: today,
                metadata: m1
            },
            None: {
                duration: 0.25,
                liturgy: today,
                metadata: m1
            },
            Vespers: {
                duration: 0.5,
                liturgy: separateVespers ? (ferialVespers ? today_minus_saint : tomorrow) : today,
                metadata: separateVespers ? (ferialVespers ? m3 : m2) : m1
            },
            Compline: {
                duration: 0.5,
                liturgy: separateVespers ? (ferialVespers ? today_minus_saint : tomorrow) : today,
                metadata: separateVespers ? (ferialVespers ? m3 : m2) : m1
            },
        };
        return {
            hours : hours,
            m1: m1,
            m2: m2,
            m3: m3,
            separateVespers: separateVespers,
            ferialVespers: ferialVespers
        }
}