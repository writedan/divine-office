function annotateTemporalMetadata(metadata) { // attach hour information
	// TODO: advent
	// TODO: christmas
	// TODO: epiphany
	// TODO: prelent
	for (let w in metadata.prelent) {
		for (let d in metadata.prelent[w]) {
			metadata.prelent[w][d].hours = {
				Vigils: {
					order: d == 'sunday' ? 'vigils/penitential-order-sunday.lit' : 'vigils/penitential-order-feria.lit',
					psalter: 'vigils/' + d + '.lit',
					commons: vigils_commons(d),
					kyrie: d == 'sunday' ? 'kyrie/xvii.gabc' : 'kyrie/xviii.gabc'
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
				invitatory: 'invitatory/dominum_qui_fecit_nos.lit',
				hymn: {
					heading: 'Tu Trinitatis Unitas',
					source: 'hymn/tu_trinitatis_unitas.gabc'
				}
			}
		}
	}
}