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
		},

		Prime: {
			order: 'terce/order.lit',
			psalter: 'prime/easter-feria.lit',
			propers: mergeDeep(minor_commons('prime', 'saturday'), {
				kyrie: kyrie,
				responsory: 'resp/jesu-christe-fili-dei(ascension).gabc',
				hymn: 'hymn/jam-lucis-orto-sidere(ascension).lit',
				chapter: 'common/prime/chapters/advent.lit'
			})
		},

		Terce: {
			order: 'terce/order.lit',
			psalter: 'terce/easter-feria.lit',
			propers: mergeDeep(minor_commons('terce', 'saturday'), {
				kyrie: kyrie,
				collect: collect,
				chapter: 'propers/pentecost/eve/lauds/chapter.lit',
				responsory: 'resp/elevata-est-magnificentia-tua.gabc',
				versicle: 'common/lauds/versicle/ascension.lit',
				hymn: 'hymn/nunc-sancte-nobis-spiritus(ascension).lit'
			})
		},

		Sext: {
			order: 'terce/order.lit',
			psalter: 'sext/easter-feria.lit',
			propers: mergeDeep(minor_commons('sext', 'saturday'), {
				kyrie: kyrie,
				collect: collect,
				hymn: 'hymn/rector-potens-verax(ascension).lit',
				chapter: 'propers/pentecost/eve/sext/chapter.lit',
				versicle: 'common/sext/versicle/ascension.lit',
				responsory: 'resp/ascendit-deus-in-jubilatione.gabc'
			})
		},

		None: {
			order: 'terce/order.lit',
			psalter: 'none/easter-feria.lit',
			propers: mergeDeep(minor_commons('none', 'saturday'), {
				kyrie: kyrie,
				collect: collect,
				hymn: 'hymn/rerum-deus-tenax(ascension).lit',
				chapter: 'propers/pentecost/eve/none/chapter.lit',
				versicle: 'common/none/versicle/ascension.lit',
				responsory: 'resp/ascendens-christus-in-altum.gabc'
			})
		}
	}

	for (d in metadata.pentecost[0]) {
		metadata.pentecost[0][d].hours = {
			Vigils: {
				order: 'vigils/order-saints.lit',
				psalter: 'vigils/pentecost.lit',
				propers: mergeDeep(vigils_commons(d), lessons('propers/pentecost/' + d + '/vigils/'), {
					invitatory: 'invitatory/alleluia-spiritus-domini.lit',
					hymn: 'hymn/jam-christus-astra-ascenderat.lit',
					collect: 'propers/pentecost/' + d + '/collect.gabc'
				})
			},

			Lauds: {
				order: 'lauds/order.lit',
				psalter: 'lauds/pentecost.lit',
				propers: {
					chapter: d == 'sunday' ? 'propers/pentecost/eve/vespers/chapter.lit' : 'propers/pentecost/sunday/sext/chapter.lit',
					hymn: 'hymn/impleta-gaudent-viscera.lit',
					versicle: 'propers/pentecost/sunday/lauds/versicle.lit',
					benedictus: 'propers/pentecost/' + d + '/lauds/benedictus.lit',
					kyrie: kyrie,
					collect: 'propers/pentecost/' + d + '/collect.gabc'
				}
			},

			Prime: {
				order: 'terce/order.lit',
				psalter: 'prime/pentecost.lit',
				propers: mergeDeep(minor_commons('prime', 'sunday'), {
					kyrie: kyrie,
					hymn: 'hymn/jam-lucis-orto-sidere(pentecost).lit',
					responsory: 'resp/jesu-christe-fili-dei(ascension).gabc'
				})
			},

			Terce: {
				order: 'terce/order.lit',
				psalter: 'terce/pentecost.lit',
				propers: mergeDeep(minor_commons('terce', 'sunday'), {
					kyrie: kyrie,
					collect: 'propers/pentecost/' + d + '/collect.gabc',
					hymn: 'hymn/nunc-sancte-nobis-spiritus(pentecost).lit',
					chapter: d == 'sunday' ? 'propers/pentecost/eve/vespers/chapter.lit' : 'propers/pentecost/sunday/sext/chapter.lit',
					responsory: 'resp/repleti-sunt-omnes-spiritusancto.gabc',
					versicle: 'propers/pentecost/sunday/lauds/versicle.lit'
				})
			},

			Sext: {
				order: 'terce/order.lit',
				psalter: 'sext/pentecost.lit',
				propers: mergeDeep(minor_commons('sext', 'sunday'), {
					kyrie: kyrie,
					collect: 'propers/pentecost/' + d + '/collect.gabc',
					hymn: 'hymn/rector-potens-verax(pentecost).lit',
					chapter: d == 'sunday' ? 'propers/pentecost/sunday/sext/chapter.lit' : 'propers/pentecost/sunday/none/chapter.lit',
					responsory: 'resp/loquebantur-variis-linguis(simple).gabc',
					versicle: 'propers/pentecost/sunday/sext/versicle.lit'
				})
			},

			None: {
				order: 'terce/order.lit',
				psalter: 'none/pentecost.lit',
				propers: mergeDeep(minor_commons('none', 'sunday'), {
					kyrie: kyrie,
					collect: 'propers/pentecost/' + d + '/collect.gabc',
					hymn: 'hymn/rerum-deus-tenax(pentecost).lit',
					chapter: d == 'sunday' ? 'propers/pentecost/sunday/none/chapter.lit' : 'propers/pentecost/monday/none/chapter.lit',
					responsory: 'resp/spiritus-domini-replevit.gabc',
					versicle: 'propers/pentecost/sunday/none/versicle.lit'
				})
			},

			Vespers: {
				order: 'vespers/order.lit',
				psalter: 'vespers/pentecost.lit',
				propers: {
					kyrie: kyrie,
					collect: 'propers/pentecost/' + d + '/collect.gabc',
					chapter: 'propers/pentecost/sunday/vespers/chapter.lit',
					hymn: 'hymn/beata-nobis-gaudia.lit',
					versicle: 'propers/pentecost/sunday/sext/versicle.lit',
					magnificat: 'propers/pentecost/' + d + '/vespers/magnificat.lit'
				}
			},

			Compline: {
				order: 'compline/order.lit',
				psalter: 'compline/pentecost.lit',
				propers: {
					kyrie: kyrie,
					collect: 'common/compline/collect.gabc',
					chapter: 'common/compline/chapter.lit',
					responsory: 'resp/in-manus-tuas.gabc',
					canticle: 'common/compline/pentecost-octave-canticle.lit',
					hymn: 'hymn/salvator-mundi-domine(ascension).lit',
					versicle: 'common/compline/versicle.lit',
					anthem: 'compline/anthems/regina-caeli-laetare.lit'
				}
			}
		}
	}

	// pentecost day prior evening
	metadata.pentecost[0].sunday.hours.FirstVespers = {
			order: 'vespers/order.lit',
			psalter: 'propers/pentecost/eve/vespers/psalter.lit',
			propers: {
				kyrie: kyrie,
				collect: collect,
				responsory: 'resp/loquebantur-variis-linquis.gabc',
				chapter: 'propers/pentecost/eve/vespers/chapter.lit',
				hymn: 'hymn/jam-christus-astra-ascenderat.lit',
				versicle: 'propers/pentecost/eve/vespers/versicle.lit',
				magnificat: 'propers/pentecost/eve/vespers/magnificat.lit'
			}
		};

	metadata.pentecost[0].sunday.hours.FirstCompline = {
			order: 'compline/order.lit',
			psalter: 'compline/pentecost.lit',
			propers: {
				kyrie: kyrie,
				collect: 'common/compline/collect.gabc',
				chapter: 'common/compline/chapter.lit',
				responsory: 'resp/in-manus-tuas.gabc',
				canticle: 'common/compline/pentecost-canticle.lit',
				hymn: 'hymn/salvator-mundi-domine(ascension).lit',
				versicle: 'common/compline/versicle.lit',
				anthem: 'compline/anthems/regina-caeli-laetare.lit'
			}
		}
}