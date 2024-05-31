// for the pentecostal cycle
function applyPostPentecost(metadata) {
	for (let w = 1; w <= 28; w += 1) {
		let collect = 'propers/post-pentecost/' + w + '/collect.lit'
		for (let d in metadata.pentecost[w]) {
			let kyrie = (d == 'sunday') ? 'common/kyrie/xi.gabc' : 'common/kyrie/xvi.gabc'
			metadata.pentecost[w][d].hours = {
				Vigils: {
					order: (d == 'sunday') ? 'vigils/order-sunday.lit' : 'vigils/order-feria.lit',
					psalter: 'vigils/' + d + '.lit',
					propers: mergeDeep(vigils_commons(d), lessons('propers/post-pentecost/' + w + '/' + d + '/vigils/'), {
						kyrie: kyrie,
						collect: collect
					})
				},

				Lauds: {
					order: 'lauds/order.lit',
					psalter: 'lauds/' + d + '.lit',
					propers: mergeDeep(lauds_commons(d), {
						kyrie: kyrie,
						collect: collect,
						benedictus: (d == 'sunday' || d == 'saturday') ? 'propers/post-pentecost/' + w + '/' + d + '/lauds/benedictus.lit' : lauds_commons(d).benedictus
					})
				},

				Prime: {
					order: 'terce/order.lit',
					psalter: (d == 'sunday') ? 'prime/sunday.lit' : 'prime/feria.lit',
					propers: mergeDeep(minor_commons('prime', d), {
						kyrie: kyrie,
						collect: collect
					})
				},

				Terce: {
					order: 'terce/order.lit',
					psalter: (d == 'sunday') ? 'terce/sunday.lit' : 'terce/feria.lit',
					propers: mergeDeep(minor_commons('terce', d), {
						kyrie: kyrie,
						collect: collect
					})
				},

				Sext: {
					order: 'terce/order.lit',
					psalter: (d == 'sunday') ? 'sext/sunday.lit' : 'sext/feria.lit',
					propers: mergeDeep(minor_commons('sext', d), {
						kyrie: kyrie,
						collect: collect
					})
				},

				None: {
					order: 'terce/order.lit',
					psalter: (d == 'sunday') ? 'none/sunday.lit' : 'none/feria.lit',
					propers: mergeDeep(minor_commons('none', d), {
						kyrie: kyrie,
						collect: collect
					})
				},

				Vespers: {
					order: 'vespers/order.lit',
					psalter: 'vespers/' + d + '.lit',
					propers: mergeDeep(vespers_commons(d), {
						kyrie: kyrie,
						collect: collect,
						magnificat: (d == 'sunday' || d == 'saturday') ? 'propers/post-pentecost/' + w + '/' + d + '/vespers/magnificat.lit' : vespers_commons(d).magnificat
					})
				},

				Compline: {
					order: 'compline/order.lit',
					psalter: 'compline/feria.lit',
					propers: {
						kyrie: kyrie,
						collect: 'common/compline/collect.gabc',
						chapter: 'common/compline/chapter.lit',
						responsory: 'resp/in-manus-tuas.gabc',
						canticle: 'common/compline/ordinary.lit',
						hymn: 'hymn/te-lucis-ante-terminum.lit',
						versicle: 'common/compline/versicle.lit',
						anthem: 'compline/anthems/salve-regina.lit'
					}
				}
			}
		}

		metadata.pentecost[w].sunday.hours.FirstVespers = metadata.pentecost[w].saturday.hours.Vespers;
		metadata.pentecost[w].sunday.hours.FirstCompline = metadata.pentecost[w].saturday.hours.Compline;
	}
}