function applyPrelent(metadata) {
	for (let w in metadata.prelent) {
		for (let d in metadata.prelent[w]) {
			let kyrie = d == 'sunday' ? 'common/kyrie/xvii.gabc' : 'common/kyrie/xviii.gabc';
			let collect = 'propers/prelent/' + w + '/collect.gabc'
			metadata.prelent[w][d].hours = {
				Vigils: {
					order: d == 'sunday' ? 'vigils/penitential-order-sunday.lit' : 'vigils/penitential-order-feria.lit',
					psalter: 'vigils/' + d + '.lit',
					propers: mergeDeep(vigils_commons(d), lessons('propers/prelent/' + w + '/' + d + '/vigils/'), {
						kyrie: kyrie,
						collect: collect
					}),
				},

				Lauds: {
					order: 'lauds/penitential-order.lit',
					psalter: 'lauds/' + d + '.lit',
					propers: mergeDeep(lauds_commons(d), {
						benedictus: 'propers/prelent/' + w + '/' + d + '/lauds/benedictus.lit',
						kyrie: kyrie,
						collect: collect
					}),
				},

				Prime: mergeDeep(minor_hours('prime', d), {
					propers: {
						collect: collect,
						kyrie: kyrie
					}
				}),

				Terce: mergeDeep(minor_hours('terce', d), {
					propers: {
						collect: collect,
						kyrie: kyrie
					}
				}),

				Sext: mergeDeep(minor_hours('sext', d), {
					propers: {
						collect: collect,
						kyrie: kyrie
					}
				}),
				
				None: mergeDeep(minor_hours('none', d), {
					propers: {
						collect: collect,
						kyrie: kyrie
					}
				}),

				Vespers: {
					order: 'vespers/penitential-order.lit',
					psalter: 'vespers/' + d + '.lit',
					propers: mergeDeep(vespers_commons(d), {
						magnificat: 'propers/prelent/' + w + '/' + d + '/vespers/magnificat.lit',
						kyrie: kyrie,
						collect: collect,
						chapter: (d == 'sunday' || d == 'saturday') ? 'propers/prelent/' + w + '/' + d + '/vespers/chapter.lit' : vespers_commons(d).chapter
					}),
				},

				Compline: {
					order: 'compline/penitential-order.lit',
					psalter: 'compline/ordinary.lit',
					propers: {
						hymn: 'hymn/te-lucis-ante-terminum.lit',
						chapter: 'common/compline/chapter(ordinary).lit',
						versicle: 'common/compline/chapter(ordinary).lit',
						canticle: 'common/compline/canticle(ordinary).lit',
						anthem: 'anthem/ave-regina-celorum.gabc',
						kyrie: kyrie,
						collect: 'common/compline/collect.gabc'
					}
				}
			}
		}

		metadata.prelent[w].sunday.hours.FirstVespers = metadata.prelent[w].saturday.hours.Vespers;
		metadata.prelent[w].sunday.hours.FirstCompline = metadata.prelent[w].saturday.hours.Compline;

		metadata.prelent[w].sunday.hours.FirstVespers.propers.kyrie = 'common/kyrie/xvii.gabc';
		metadata.prelent[w].sunday.hours.FirstCompline.propers.kyrie = 'common/kyrie/xvii.gabc';
	}
}