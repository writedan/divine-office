function lessons(prefix) {
	let res = {};
	for (let i = 0; i < 9; i++) {
		res['lesson-' + (i + 1)] = prefix + 'lesson-' + (i + 1) + '.gabc'
	}

	return res;
}

function annotateTemporalMetadata(metadata) { // attach hour information
	// TODO: advent
	// TODO: christmas
	// TODO: epiphany
	// TODO: prelent
	for (let w in metadata.prelent) {
		for (let d in metadata.prelent[w]) {
			let kyrie = d == 'sunday' ? 'common/kyrie/xvii.gabc' : 'kyrie/xviii.gabc';
			metadata.prelent[w][d].hours = {
				Vigils: {
					order: d == 'sunday' ? 'vigils/penitential-order-sunday.lit' : 'vigils/penitential-order-feria.lit',
					psalter: 'vigils/' + d + '.lit',
					commons: mergeDeep(vigils_commons(d), lessons('propers/prelent/' + w + '/' + d + '/vigils/'), {
						kyrie: kyrie
					}),
				},

				Lauds: {
					order: 'lauds/penitential-order.lit',
					psalter: 'lauds/' + d + '.lit',
					commons: mergeDeep(lauds_commons(d), {
						benedictus: 'propers/prelent/' + w + '/' + d + '/lauds/benedictus.lit',
						kyrie: kyrie,
					}),
				},

				Prime: minor_hours('prime', d, kyrie),
				Terce: minor_hours('terce', d, kyrie),
				Sext: minor_hours('sext', d, kyrie),
				None: minor_hours('none', d, kyrie),

				Vespers: {
					order: 'vespers/penitential-order.lit',
					psalter: 'vespers/' + d + '.lit',
					commons: mergeDeep(vespers_commons(d), {
						magnificat: 'propers/prelent/' + w + '/' + d + '/vespers/magnificat.lit'
					}),
				},

				Compline: {
					order: 'compline/penitential-order.lit',
					psalter: 'compline/ordinary.lit',
					commons: {
						hymn: 'hymn/te-lucis-ante-terminum.gabc',
						chapter: 'common/compline/chapter(ordinary).gabc',
						versicle: 'common/compline/chapter(ordinary).gabc',
						canticle: 'common/compline/canticle(ordinary).lit',
						anthem: 'anthem/ave-regina-celorum.gabc',
						kyrie: kyrie
					}
				}
			}
		}

		metadata.prelent[w].sunday.hours.FirstVespers = metadata.prelent[w].saturday.hours.Vespers;
		metadata.prelent[w].sunday.hours.FirstCompline = metadata.prelent[w].saturday.hours.Compline;
	}

	// TODO: lent
	// TODO: passion
	// TODO: pascha
	// TODO: ascension
	// TODO: pentecost
	// TODO: august
	// TODO: september
	// TODO: october
	// TODO: november
}


/**
 * Commons for various days and saints
*/

function vigils_commons(day) {
	switch (day) {
		case 'friday': {
			return {
				invitatory: 'invitatory/dominum-qui-fecit-nos.lit',
				hymn: 'hymn/tu-trinitatis-unitas.gabc',
				blessings: 'common/vigils/2nd-nocturn/'
			}
		}

		case 'saturday': {
			return {
				invitatory: 'invitatory/dominum-deum-nostrum.lit',
				hymn: 'hymn/summe-deus-clementie.gabc',
				blessings: 'common/vigils/3rd-nocturn/'
			}
		}
	}

	return {}
}

function lauds_commons(day) {
	switch (day) {
		case 'friday': {
			return {
				hymn: 'hymn/eterna-celi-gloria.gabc',
				chapter: 'common/lauds/chapters/feria.gabc',
				versicle: 'common/lauds/versicle/feria.gabc'
			}
		}
	}

	return {}
}

function minor_commons(hour, day) {
	let hymn;
	switch(hour) {
		case 'prime': hymn = 'jam-lucis-orto-sidere'; break;
		case 'terce': hymn = 'nunc-sancte-nobis-spiritus'; break;
		case 'sext': hymn = 'rector-potens-verax'; break;
		case 'none': hymn = 'rerum-deus-tenax'; break;
	}

	let type;

	if (day == 'sunday') {
		hymn = 'hymn/' + hymn + '(sunday).gabc'
		type = 'sunday'
	} else {
		hymn = 'hymn/' + hymn + '(feria).gabc'
		type = 'feria'
	}

	return {
		hymn: hymn,
		chapter: 'common/' + hour + '/chapters/' + type + '.gabc',
		versicle: 'common/' + hour + '/versicle/' + type + '.gabc'
	}
}

function minor_hours(hour, day, kyrie) { // produces the whole for a given minor hour in ordinary time
	return {
		order: 'terce/penitential-order.lit',
		psalter: hour + '/' + (day == 'sunday' ? 'sunday' : 'feria') + '.lit',
		commons: mergeDeep(minor_commons(hour, day), {
			kyrie: kyrie
		}),
	}
}

function vespers_commons(day) {
	switch (day) {
		case 'friday': return {
			hymn: 'hymn/plasmator-hominis.gabc',
			chapter: 'common/vespers/chapters/feria.gabc',
			versicle: 'common/vespers/versicle/feria.gabc'
		}
	}

	return {}
}