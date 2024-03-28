function applyEaster(metadata) {
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
		}
	}
}