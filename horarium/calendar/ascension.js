function applyAscension(metadata) {
	metadata.ascension.vigil.hours = {
		Vigils: {
			order: 'vigils/order-feria.lit',
			psalter: 'propers/ascension/vigil/psalter.lit'
		}
	}

	metadata.ascension.octave[1].hours = {
		FirstVespers: {
			order: 'vespers/order.lit',
			psalter: 'propers/ascension/vigil/vespers/psalter.lit'
		}
	}
}