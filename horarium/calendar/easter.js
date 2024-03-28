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
				magnificat: 'propers/pascha/1/saturday/vespers/magnificat.lit',
			}
		},

		FirstCompline: {
			order: 'compline/pascha-day-order.lit',
			psalter: 'propers/pascha/1/saturday/compline/psalter.lit',
			propers: {
				collect: 'propers/pascha/1/saturday/vespers/collect.gabc',
				anthem: 'compline/anthems/regina-caeli-laetare.lit',
				canticle: 'propers/pascha/1/saturday/compline/canticle.lit'
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
		},

		Prime: {
			order: 'propers/pascha/1/sunday/terce/order.lit',
			psalter: 'propers/pascha/1/sunday/prime/psalter.lit',
			propers: mergeDeep(minor_commons('prime', 'sunday'), {
				collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
				versicle: 'propers/pascha/1/sunday/prime/versicle.lit'
			})
		},

		Terce: {
			order: 'propers/pascha/1/sunday/terce/order.lit',
			psalter: 'propers/pascha/1/sunday/terce/psalter.lit',
			propers: mergeDeep(minor_commons('terce', 'sunday'), {
				collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
				versicle: 'propers/pascha/1/sunday/terce/versicle.lit'
			})
		},

		Sext: {
			order: 'propers/pascha/1/sunday/terce/order.lit',
			psalter: 'propers/pascha/1/sunday/sext/psalter.lit',
			propers: mergeDeep(minor_commons('sext', 'sunday'), {
				collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
				versicle: 'propers/pascha/1/sunday/sext/versicle.lit'
			})
		},

		None: {
			order: 'propers/pascha/1/sunday/terce/order.lit',
			psalter: 'propers/pascha/1/sunday/none/psalter.lit',
			propers: mergeDeep(minor_commons('none', 'sunday'), {
				collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
				versicle: 'propers/pascha/1/sunday/none/versicle.lit'
			})
		},

		Vespers: {
			order: 'propers/pascha/1/sunday/vespers/order.lit',
			psalter: 'propers/pascha/1/sunday/vespers/psalter.lit',
			propers: mergeDeep(vespers_commons('sunday'), {
				collect: 'propers/pascha/1/sunday/vespers/collect.gabc',
				'hec-dies': 'gradual/hec-dies-confitemini-domino.gabc',
				alleluia: 'alleluia/epulemur-in-azimis.gabc',
				versicle: 'propers/pascha/1/sunday/vespers/versicle.lit',
				magnificat: 'propers/pascha/1/sunday/vespers/magnificat.lit',
				'procession-fount-psalter': 'propers/pascha/1/sunday/vespers/fount-procession.lit',
				'procession-fount-alleluia': 'alleluia/laudate-pueri.gabc',
				'procession-fount-collect': 'propers/pascha/1/sunday/vespers/fount-collect.gabc',
				'procession-altar-psalter': 'propers/pascha/1/sunday/vespers/altar-procession.lit'
			})
		},

		Compline: {
			order: 'propers/pascha/1/sunday/compline/order.lit',
			psalter: 'compline/pascha.lit',
			propers: {
				collect: 'propers/pascha/1/saturday/vespers/collect.gabc',
				anthem: 'compline/anthems/regina-caeli-laetare.lit',
				'hec-dies': 'gradual/hec-dies.gabc'
			}
		}
	}

	//metadata.pascha[1].sunday.hours.Compline = metadata.pascha[1].sunday.hours.FirstCompline;

	// easter week
	for (let d in metadata.pascha[1]) {
		if (d == 'sunday') continue;
		metadata.pascha[1][d].hours = {
			Vigils: {
				order: 'propers/pascha/1/sunday/vigils/order.lit',
				psalter: 'propers/pascha/1/sunday/vigils/psalter.lit',
				propers: mergeDeep(vigils_commons('sunday'), lessons('propers/pascha/1/' + d + '/vigils/'), {
					invitatory: 'invitatory/surrexit-dominus-vere.lit',
					collect: 'propers/pascha/1/' + d + '/vigils/collect.gabc',
					gospel: 'propers/pascha/1/' + d + '/vigils/gospel.lit'
				})
			},

			Lauds: {
				order: 'propers/pascha/1/sunday/lauds/order.lit',
				psalter: 'propers/pascha/1/sunday/lauds/psalter.lit',
				propers: mergeDeep(lauds_commons('sunday'), {
					versicle: 'propers/pascha/1/sunday/lauds/versicle.lit',
					collect: 'propers/pascha/1/' + d + '/vigils/collect.gabc',
					benedictus: 'propers/pascha/1/' + d + '/lauds/benedictus.lit'
				})
			},

			Prime: {
				order: 'propers/pascha/1/sunday/terce/order.lit',
				psalter: 'propers/pascha/1/sunday/prime/psalter.lit',
				propers: mergeDeep(minor_commons('prime', 'sunday'), {
					collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
					versicle: 'propers/pascha/1/sunday/prime/versicle.lit'
				})
			},

			Terce: {
				order: 'propers/pascha/1/sunday/terce/order.lit',
				psalter: 'propers/pascha/1/sunday/terce/psalter.lit',
				propers: mergeDeep(minor_commons('terce', 'sunday'), {
					collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
					versicle: 'propers/pascha/1/sunday/terce/versicle.lit'
				})
			},

			Sext: {
				order: 'propers/pascha/1/sunday/terce/order.lit',
				psalter: 'propers/pascha/1/sunday/sext/psalter.lit',
				propers: mergeDeep(minor_commons('sext', 'sunday'), {
					collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
					versicle: 'propers/pascha/1/sunday/sext/versicle.lit'
				})
			},

			None: {
				order: 'propers/pascha/1/sunday/terce/order.lit',
				psalter: 'propers/pascha/1/sunday/none/psalter.lit',
				propers: mergeDeep(minor_commons('none', 'sunday'), {
					collect: 'propers/pascha/1/sunday/vigils/collect.gabc',
					versicle: 'propers/pascha/1/sunday/none/versicle.lit'
				})
			},

			Vespers: {
				order: 'propers/pascha/1/sunday/vespers/order.lit',
				psalter: 'propers/pascha/1/sunday/vespers/psalter.lit',
				propers: mergeDeep(vespers_commons('sunday'), {
					collect: 'propers/pascha/1/sunday/vespers/collect.gabc',
					'hec-dies': 'propers/pascha/1/' + d + '/vespers/hec-dies.lit',
					alleluia: 'propers/pascha/1/' + d + '/vespers/alleluia.lit',
					versicle: 'propers/pascha/1/' + d + '/vespers/versicle.lit',
					magnificat: 'propers/pascha/1/' + d + '/vespers/magnificat.lit',
					'procession-fount-psalter': 'propers/pascha/1/sunday/vespers/fount-procession.lit',
					'procession-fount-alleluia': 'propers/pascha/1/' + d + '/vespers/fount-alleluia.lit',
					'procession-fount-collect': 'propers/pascha/1/' + d + '/vespers/fount-collect.gabc',
					'procession-altar-psalter': 'propers/pascha/1/sunday/vespers/altar-procession.lit'
				})
			},

			Compline: {
				order: 'propers/pascha/1/sunday/compline/order.lit',
				psalter: 'compline/pascha.lit',
				propers: {
					collect: 'propers/pascha/1/saturday/vespers/collect.gabc',
					anthem: 'compline/anthems/regina-caeli-laetare.lit',
					'hec-dies': 'gradual/hec-dies.gabc'
				}
			}
		}
	}
}