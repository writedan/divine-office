const daily_office = {
	prelent: {
		'1': {
			friday: {
				Vigils: {
					lessons: [
						'Genesis xiv, 1',
						'Genesis xiv, 5',
						'Genesis xiv, 8'
					]
				}
			}
		}
	}
}


function annotateTemporalMetadata(metadata) { // attach hour information
	// TODO: advent
	// TODO: christmas
	// TODO: epiphany
	// TODO: prelent
	for (let w in metadata.prelent) {
		for (let d in metadata.prelent[w]) {
			metadata.prelent[w][d].hours = {
				Vigils: {
					psalter: 'vigils/' + d + '.lit',
					commons: vigils_commons(d),
					propers: daily_office.prelent[w][d].Vigils
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