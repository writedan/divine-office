function applyEaster(metadata) {
	const kyrie = 'common/kyrie/i.gabc'

	// TODO: pascha
	// easter day!
	metadata.pascha[1].sunday.hours = {
		FirstVespers: {
			order: 'vespers/pascha-day-order.lit',
			psalter: 'vespers/pascha-day.lit',
			propers: {
				collect: 'propers/pascha/1/saturday/vespers/collect.gabc',
				magnificat: 'propers/pascha/1/saturday/vespers/magnificat.lit'
			}
		},

		FirstCompline: {
			order: 'compline/pascha-day-order.lit',
			psalter: 'compline/pascha.lit',
			propers: {
				collect: 'propers/pascha/1/saturday/vespers/collect.gabc',
				canticle: 'propers/pascha/1/saturday/compline/canticle.lit',
				anthem: 'compline/anthems/regina-caeli-laetare.lit'
			}
		},

		Vigils: {
			order: 'propers/pascha/1/sunday/vigils/order.lit',
			psalter: 'propers/pascha/1/sunday/vigils/psalter.lit',
			propers: mergeDeep(vigils_commons('sunday'), lessons('propers/pascha/1/sunday/vigils/'), {
				commemoration: 'propers/pascha/1/sunday/vigils/commemoration.lit',
				invitatory: 'invitatory/alleluia-alleluia-christus-hodie-surrexit.lit',
				collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
				gospel: 'propers/pascha/1/sunday/vigils/gospel.lit'
			})
		},

		Lauds: {
			order: 'propers/pascha/1/sunday/lauds/order.lit',
			psalter: 'propers/pascha/1/sunday/lauds/psalter.lit',
			propers: mergeDeep(lauds_commons('sunday'), {
				versicle: 'propers/pascha/1/sunday/lauds/versicle.lit',
				collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
				benedictus: 'propers/pascha/1/sunday/lauds/benedictus.lit'
			})
		}
	}
}