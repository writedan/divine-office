function applyLent(metadata) {
	for (let w = 1; w <= 2; w += 1) {
		for (let d in metadata.lent[w]) {
			let kyrie = d == 'sunday' ? 'common/kyrie/xvii.gabc' : 'common/kyrie/xviii.gabc';
			let collectpath = 'propers/lent/' + w + '/' + d + '/';

			metadata.lent[w][d].hours = {
				Vigils: {
					order: d == 'sunday' ? 'vigils/penitential-order-sunday.lit' : 'vigils/penitential-order-feria.lit',
					psalter: 'vigils/' + d + '.lit',
					propers: mergeDeep(vigils_commons(d), lessons('propers/lent/' + w + '/' + d + '/vigils/'), {
						invitatory: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/vigils/invitatory.lit' : vigils_commons(d).invitatory,
						hymn: 'hymn/summi-largitor-premii.lit',
						kyrie: kyrie,
						collect: collectpath + 'collect.gabc',
						gospel: 'propers/lent/' + w + '/' + d + '/gospel.lit'
					}),
				},

				Lauds: {
					order: 'lauds/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/lauds/psalter.lit' : 'lauds/' + d +'.lit',
					propers: mergeDeep(lauds_commons(d), {
						benedictus: 'propers/lent/' + w + '/' + d + '/lauds/benedictus.lit',
						kyrie: kyrie,
						collect: 'propers/lent/' + w + '/' + d + '/collect.gabc',
						hymn: 'hymn/audi-benigne-conditor.lit',
						chapter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/lauds/chapter.lit' : 'common/lauds/chapters/lent-feria.lit',
						versicle: 'common/lauds/versicle/lent.lit'
					})
				},

				Prime: {
					order: 'terce/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/prime/psalter.lit' : 'prime/feria-lent.lit',
					propers: mergeDeep(minor_commons('prime', d), {
						collect: 'common/prime/collect.gabc',
						kyrie: kyrie,
						hymn: (d == 'sunday') ? 'hymn/jam-lucis-orto-sidere(lent-1st-2nd-sunday).lit' : 'hymn/jam-lucis-orto-sidere(feria).lit'
					})
				},

				Terce: {
					order: 'terce/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/terce/psalter.lit' : 'terce/feria-lent.lit',
					propers: mergeDeep(minor_commons('terce', d), {
						kyrie: kyrie,
						collect: 'propers/lent/' + w + '/' + d + '/collect.gabc',
						chapter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/terce/chapter.lit' : 'common/terce/chapters/lent-feria.lit',
						versicle: (d== 'sunday') ? 'propers/lent/' + w + '/' + d + '/terce/versicle.lit' : 'common/terce/versicle/lent.lit'
					})
				},

				Sext: {
					order: 'terce/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/sext/psalter.lit' : 'sext/feria-lent.lit',
					propers: mergeDeep(minor_commons('sext', d), {
						kyrie: kyrie,
						collect: 'propers/lent/' + w + '/' + d + '/collect.gabc',
						chapter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/sext/chapter.lit' : 'common/sext/chapters/lent-feria.lit',
						versicle: (d== 'sunday') ? 'propers/lent/' + w + '/' + d + '/sext/versicle.lit' : 'common/sext/versicle/lent.lit'
					})
				},

				None: {
					order: 'terce/penitential-order.lit',
					psalter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/none/psalter.lit' : 'none/feria-lent.lit',
					propers: mergeDeep(minor_commons('none', d), {
						kyrie: kyrie,
						collect: 'propers/lent/' + w + '/' + d + '/collect.gabc',
						chapter: (d == 'sunday') ? 'propers/lent/' + w + '/' + d + '/none/chapter.lit' : 'common/none/chapters/lent-feria.lit',
						versicle: (d== 'sunday') ? 'propers/lent/' + w + '/' + d + '/none/versicle.lit' : 'common/none/versicle/lent.lit'
					})
				},

				Vespers: {
					order: 'vespers/penitential-order.lit',
					psalter: 'vespers/' + d + '.lit',
					propers: mergeDeep(vespers_commons(d), {
						magnificat: 'propers/lent/' + w + '/' + d + '/vespers/magnificat.lit',
						kyrie: kyrie,
						hymn: 'hymn/ex-more-docte-mystico.lit',
						collect: (d == 'sunday' || d == 'saturday') ? 'propers/lent/' + w + '/sunday/collect.gabc' : collectpath + 'blessing.gabc',
						chapter: (d == 'sunday' || d == 'saturday') ? 'propers/lent/' + w + '/' + d + '/vespers/chapter.lit' : 'common/vespers/chapters/lent-feria.lit',
						versicle: 'common/vespers/versicle/lent.lit',
					}),
				},

				Compline: {
					order: 'compline/penitential-order.lit',
					psalter: 'compline/1st-2nd-lent.lit',
					propers: {
						kyrie: kyrie,
						canticle: 'common/compline/lent-canticle.lit',
						chapter: 'common/compline/chapter.lit',
						versicle: 'common/compline/versicle.lit',
						hymn: 'hymn/christe-qui-lux-est.lit',
						responsory: 'resp/in-pace-in-idipsum.gabc',
						collect: 'common/compline/collect.gabc',
						anthem: 'compline/anthems/ave-regina-caelorum.lit'
					}
				}
			}
		}

		metadata.lent[w].sunday.hours.FirstVespers = metadata.lent[w].saturday.hours.Vespers;
		metadata.lent[w].sunday.hours.FirstCompline = metadata.lent[w].saturday.hours.Compline;

		metadata.lent[w].sunday.hours.FirstVespers.propers.kyrie = 'common/kyrie/xvii.gabc';
		metadata.lent[w].sunday.hours.FirstCompline.propers.kyrie = 'common/kyrie/xvii.gabc';
	}

	// TODO lent 3 and 4
}