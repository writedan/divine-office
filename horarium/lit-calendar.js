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
					commons: vigils_commons(d),
					kyrie: kyrie
				},

				Lauds: {
					order: 'lauds/penitential-order.lit',
					psalter: 'lauds/' + d + '.lit',
					commons: lauds_commons(d),
					kyrie: kyrie
				}
			}
		}
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
				hymn: {
					heading: 'Tu Trinitatis Unitas',
					source: 'hymn/tu_trinitatis_unitas.gabc'
				},
				blessings: 'common/vigils/2nd-nocturn/'
			}
		}

		case 'saturday': {
			return {
				invitatory: 'invitatory/dominum_deum_nostrum.lit',
				hymn: 'hymn/summe-deus-clementie.gabc',
				blessings: 'common/vigils/3rd-nocturn/'
			}
		}
	}
}

function lauds_commons(day) {
	switch (day) {
		case 'friday': {
			return {
				hymn: source: 'hymn/eterna-celi-gloria.gabc',
				chapter: 'common/lauds/chapters/feria.gabc',
				versicle: 'common/lauds/versicle/feria.gabc'
			}
		}
	}
}