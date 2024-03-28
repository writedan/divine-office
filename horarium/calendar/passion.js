function applyPassiontide(metadata) {
	for (let w = 1; w <= 2; w += 1) {
		for (let d in metadata.passion[1]) {
			let kyrie = d == 'sunday' ? 'common/kyrie/xvii.gabc' : 'common/kyrie/xviii.gabc';
			let collectpath = 'propers/passion/' + w + '/' + d + '/';
			metadata.passion[w][d].hours = {
				Vigils: {
					order: (d == 'sunday') ? 'vigils/penitential-order-sunday.lit' : 'vigils/penitential-order-feria.lit',
					psalter: (d == 'sunday') ? 'propers/passion/' + w + '/sunday/vigils/psalter.lit' : 'vigils/' + d + '.lit',
					propers: mergeDeep(vigils_commons(d), lessons('propers/passion/' + w + '/' + d + '/vigils/'), {
						hymn: 'hymn/pange-lingua-gloriosi.lit',
						invitatory: (d == 'sunday') ? 'invitatory/hodie-si-vocem-domini.lit' : 'invitatory/adoremus-dominum(passion).lit',
						hymn: 'hymn/pange-lingua-gloriosi.lit',
						kyrie: kyrie,
						collect: collectpath + 'collect.gabc',
						gospel: 'propers/passion/' + w + '/' + d + '/gospel.lit'
					}),
				},

				Lauds: {
					order: 'lauds/penitential-order.lit',
					psalter: (d == 'sunday' || w == 2) ? 'propers/passion/' + w + '/' + d + '/lauds/psalter.lit' : 'lauds/' + d + '.lit',
					propers: {
						hymn: 'hymn/lustra-sex-qui-jam-peracta.lit',
						kyrie: kyrie,
						collect: collectpath + 'collect.gabc',
						benedictus: 'propers/passion/' + w + '/' + d + '/lauds/benedictus.lit',
						chapter: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/lauds/chapter.lit' : 'common/lauds/chapters/passion.lit',
						versicle: 'common/lauds/versicle/passion.lit',
					}
				},

				Prime: {
					order: 'terce/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/prime/psalter.lit' : 'prime/feria-passion.lit',
					propers: mergeDeep(minor_commons('prime', d), {
						collect: 'common/prime/collect.gabc',
						kyrie: kyrie,
						hymn: (d == 'sunday') ? 'hymn/jam-lucis-orto-sidere(passion).lit' : 'hymn/jam-lucis-orto-sidere(feria).lit'
					})
				},

				Terce: {
					order: 'terce/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/terce/psalter.lit' : 'terce/feria-passion.lit',
					propers: mergeDeep(minor_commons('terce', d), {
						collect: collectpath + 'collect.gabc',
						kyrie: kyrie,
						chapter: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/terce/chapter.lit' : 'common/terce/chapters/passion.lit',
						versicle: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/terce/versicle.lit' : 'common/terce/versicle/passion.lit',
					})
				},

				Sext: {
					order: 'terce/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/sext/psalter.lit' : 'sext/feria-passion.lit',
					propers: mergeDeep(minor_commons('sext', d), {
						collect: collectpath + 'collect.gabc',
						kyrie: kyrie,
						chapter: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/sext/chapter.lit' : 'common/sext/chapters/passion.lit',
						versicle: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/sext/versicle.lit' : 'common/sext/versicle/passion.lit',
					})
				},

				None: {
					order: 'terce/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/none/psalter.lit' : 'none/feria-passion.lit',
					propers: mergeDeep(minor_commons('none', d), {
						collect: collectpath + 'collect.gabc',
						kyrie: kyrie,
						chapter: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/none/chapter.lit' : 'common/none/chapters/passion.lit',
						versicle: (d == 'sunday') ? 'propers/passion/' + w + '/' + d + '/none/versicle.lit' : 'common/none/versicle/passion.lit',
					})
				},

				Vespers: {
					order: 'vespers/penitential-order.lit',
					psalter: 'vespers/' + d + '.lit',
					propers: {
						kyrie: kyrie,
						chapter: (d == 'saturday' || d == 'sunday' || (d == 'wednesday' && w == 2)) ? 'propers/passion/' + w + '/' + d + '/vespers/chapter.lit' : 'common/vespers/chapters/passion.lit',
						hymn: 'hymn/vexilla-regis-prodeunt.lit',
						versicle: 'common/vespers/versicle/passion.lit',
						collect: (d == 'sunday' || d == 'saturday') ? 'propers/passion/' + w + '/sunday/collect.gabc' :collectpath + 'blessing.gabc',
						magnificat: 'propers/passion/' + w + '/' + d + '/vespers/magnificat.lit'
					}
				},

				Compline: {
					order: 'compline/penitential-order.lit',
					psalter: 'compline/passion.lit',
					propers: {
						kyrie: kyrie,
						hymn: 'hymn/cultor-dei-memento.lit',
						chapter: 'common/compline/chapter.lit',
						versicle: 'common/compline/versicle.lit',
						responsory: 'resp/in-manus-tuas(passion).gabc',
						canticle: 'common/compline/passion-canticle.lit',
						collect: 'common/compline/collect.gabc',
						anthem: 'compline/anthems/ave-regina-caelorum.lit'
					}
				}
			}
		}
		metadata.passion[w].sunday.hours.FirstVespers = metadata.passion[w].saturday.hours.Vespers;
		metadata.passion[w].sunday.hours.FirstCompline = metadata.passion[w].saturday.hours.Compline;

		metadata.passion[w].sunday.hours.FirstVespers.propers.kyrie = 'common/kyrie/xvii.gabc';
		metadata.passion[w].sunday.hours.FirstCompline.propers.kyrie = 'common/kyrie/xvii.gabc';
	}
}