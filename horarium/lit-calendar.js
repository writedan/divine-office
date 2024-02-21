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
	for (let w in metadata.prelent) {
		for (let d in metadata.prelent[w]) {
			let kyrie = d == 'sunday' ? 'common/kyrie/xvii.gabc' : 'common/kyrie/xviii.gabc';
			let collect = 'propers/prelent/' + w + '/collect.gabc'
			metadata.prelent[w][d].hours = {
				Vigils: {
					order: d == 'sunday' ? 'vigils/penitential-order-sunday.lit' : 'vigils/penitential-order-feria.lit',
					psalter: 'vigils/' + d + '.lit',
					propers: mergeDeep(vigils_commons(d), lessons('propers/prelent/' + w + '/' + d + '/vigils/'), {
						kyrie: kyrie,
						collect: collect
					}),
				},

				Lauds: {
					order: 'lauds/penitential-order.lit',
					psalter: 'lauds/' + d + '.lit',
					propers: mergeDeep(lauds_commons(d), {
						benedictus: 'propers/prelent/' + w + '/' + d + '/lauds/benedictus.lit',
						kyrie: kyrie,
						collect: collect
					}),
				},

				Prime: mergeDeep(minor_hours('prime', d), {
					propers: {
						collect: collect,
						kyrie: kyrie
					}
				}),

				Terce: mergeDeep(minor_hours('terce', d), {
					propers: {
						collect: collect,
						kyrie: kyrie
					}
				}),

				Sext: mergeDeep(minor_hours('sext', d), {
					propers: {
						collect: collect,
						kyrie: kyrie
					}
				}),
				
				None: mergeDeep(minor_hours('none', d), {
					propers: {
						collect: collect,
						kyrie: kyrie
					}
				}),

				Vespers: {
					order: 'vespers/penitential-order.lit',
					psalter: 'vespers/' + d + '.lit',
					propers: mergeDeep(vespers_commons(d), {
						magnificat: 'propers/prelent/' + w + '/' + d + '/vespers/magnificat.lit',
						kyrie: kyrie,
						collect: collect,
						chapter: (d == 'sunday' || d == 'saturday') ? 'propers/prelent/' + w + '/' + d + '/vespers/chapter.gabc' : vespers_commons(d).chapter
					}),
				},

				Compline: {
					order: 'compline/penitential-order.lit',
					psalter: 'compline/ordinary.lit',
					propers: {
						hymn: 'hymn/te-lucis-ante-terminum.gabc',
						chapter: 'common/compline/chapter(ordinary).gabc',
						versicle: 'common/compline/chapter(ordinary).gabc',
						canticle: 'common/compline/canticle(ordinary).lit',
						anthem: 'anthem/ave-regina-celorum.gabc',
						kyrie: kyrie,
						collect: 'common/compline/collect.gabc'
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
		case 'sunday': return {
			invitatory: 'invitatory/preoccupemus.lit',
			hymn: 'hymn/primo-dierum-omnium.lit',
			'absolution-1': 'common/vigils/1st-nocturn/absolution.gabc',
			'blessing-1': 'common/vigils/1st-nocturn/blessing-1.gabc',
			'blessing-2': 'common/vigils/1st-nocturn/blessing-2.gabc',
			'blessing-3': 'common/vigils/1st-nocturn/blessing-3.gabc',

			'absolution-2': 'common/vigils/2nd-nocturn/absolution.gabc',
			'blessing-4': 'common/vigils/2nd-nocturn/blessing-1.gabc',
			'blessing-5': 'common/vigils/2nd-nocturn/blessing-2.gabc',
			'blessing-6': 'common/vigils/2nd-nocturn/blessing-3.gabc',

			'absolution-3': 'common/vigils/3rd-nocturn/absolution.gabc',
			'blessing-7': 'common/vigils/3rd-nocturn/blessing-1.gabc',
			'blessing-8': 'common/vigils/3rd-nocturn/blessing-2.gabc',
			'blessing-9': 'common/vigils/3rdå-nocturn/blessing-3.gabc',
		}

		case 'friday': {
			return {
				invitatory: 'invitatory/dominum-qui-fecit-nos.lit',
				hymn: 'hymn/tu-trinitatis-unitas.gabc',
				'absolution-1': 'common/vigils/2nd-nocturn/absolution.gabc',
				'blessing-1': 'common/vigils/2nd-nocturn/blessing-1.gabc',
				'blessing-2': 'common/vigils/2nd-nocturn/blessing-2.gabc',
				'blessing-3': 'common/vigils/2nd-nocturn/blessing-3.gabc',
			}
		}

		case 'saturday': {
			return {
				invitatory: 'invitatory/dominum-deum-nostrum.lit',
				hymn: 'hymn/summe-deus-clementie.gabc',
				'absolution-1': 'common/vigils/3rd-nocturn/absolution.gabc',
				'blessing-1': 'common/vigils/3rd-nocturn/blessing-1.gabc',
				'blessing-2': 'common/vigils/3rd-nocturn/blessing-2.gabc',
				'blessing-3': 'common/vigils/3rd-nocturn/blessing-3.gabc',
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
		propers: minor_commons(hour, day),
	}
}

function vespers_commons(day) {
	switch (day) {
		case 'friday': return {
			hymn: 'hymn/plasmator-hominis.gabc',
			chapter: 'common/vespers/chapters/feria.gabc',
			versicle: 'common/vespers/versicle/feria.gabc'
		}

		case 'saturday': return {
			hymn: 'hymn/o-lux-beata-trinitas.lit',
			chapter: 'common/vespers/chapters/sunday.gabc',
			versicle: 'common/vespers/versicle/sunday.gabc' // NOTE: only the saturday office (first vespers) uses the sunday chapter and versicle; second vespers of sundays uses the ferial
		}
	}

	return {}
}