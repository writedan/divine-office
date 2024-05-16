function applyPentecost(metadata) {
	// vigil
	collect = 'propers/pentecost/eve/collect.gabc',
	kyrie = 'common/kyrie/1.gabc'
	metadata.pentecost.vigil.hours = {
		Vigils: {
			order: 'vigils/order-saints.lit',
			psalter: 'propers/pentecost/eve/vigils/psalter.lit',
			propers: mergeDeep(vigils_commons('saturday'), lessons('propers/pentecost/eve/vigils/'), {
				invitatory: 'invitatory/alleluia(pentecost-vigil).lit',
				hymn: 'hymn/eterne-rex-altissime.lit',
				collect: collect,
				kyrie: kyrie
			})
		},

		Lauds: {
			order: 'lauds/order.lit',
			psalter: 'propers/pentecost/eve/lauds/psalter.lit',
			propers: mergeDeep(lauds_commons('saturday'), {
				chapter: 'propers/pentecost/eve/lauds/chapter.lit',
				hymn: 'hymn/tu-christe-nostrum-gaudium.lit',
				versicle: 'common/lauds/versicle/ascension.lit',
				benedictus: 'propers/pentecost/eve/lauds/benedictus.lit',
				collect: collect,
				kyrie: kyrie
			})
		}
	}
}