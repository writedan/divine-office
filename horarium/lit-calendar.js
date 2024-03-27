function lessons(prefix) {
	let res = {};
	for (let i = 0; i < 9; i++) {
		res['lesson-' + (i + 1)] = prefix + 'lesson-' + (i + 1) + '.lit'
	}

	return res;
}

function annotateTemporalMetadata(metadata) { // attach hour information
	// TODO: advent
	// TODO: christmas
	// TODO: epiphany
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

	// TODO: lent
	// lent weeks 1 and 2
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

	// passiontide
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

	// sacred triduum
	metadata.passion[2].thursday.hours = {
		Vigils: {
			order: 'tenebrae/order.lit',
			psalter: 'tenebrae/holy-thursday.lit',
			propers: mergeDeep(lessons('propers/triduum/thursday/tenebrae/'), {
				benedictus: 'propers/triduum/thursday/tenebrae/benedictus.lit',
				preces: 'propers/triduum/thursday/tenebrae/preces.lit',
				'universal-gloria-disabled': true
			})
		},

		Lauds: {
			order: 'tenebrae/order.lit',
			psalter: 'tenebrae/holy-thursday.lit',
			propers: mergeDeep(lessons('propers/triduum/thursday/tenebrae/'), {
				benedictus: 'propers/triduum/thursday/tenebrae/benedictus.lit',
				preces: 'propers/triduum/thursday/tenebrae/preces.lit',
				'universal-gloria-disabled': true
			})
		},

		Prime: {
			order: 'terce/triduum-order.lit',
			psalter: 'prime/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/thursday/tenebrae/preces.lit',
			}
		},

		Terce: {
			order: 'terce/triduum-order.lit',
			psalter: 'terce/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/thursday/tenebrae/preces.lit',
			}
		},

		Sext: {
			order: 'terce/triduum-order.lit',
			psalter: 'sext/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/thursday/tenebrae/preces.lit',
			}
		},

		None: {
			order: 'terce/triduum-order.lit',
			psalter: 'none/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/thursday/tenebrae/preces.lit',
			}
		},

		Vespers: {
			order: 'vespers/triduum-order.lit',
			psalter: 'vespers/triduum.lit',
			propers: {
				magnificat: 'propers/triduum/thursday/vespers/magnificat.lit',
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/thursday/tenebrae/preces.lit',
			}
		},

		Compline: {
			order: 'terce/triduum-order.lit',
			psalter: 'compline/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/thursday/tenebrae/preces.lit',
			}
		}
	}

	// TODO: pascha
	// TODO: ascension
	// TODO: pentecost
	// TODO: august
	// TODO: september
	// TODO: october
	// TODO: november
}


/**
 * Commons for various days and saints
*/

function vigils_commons(day) {
	switch (day) {
		case 'sunday': return {
			invitatory: 'invitatory/preoccupemus.lit',
			hymn: 'hymn/primo-dierum-omnium.lit',
			'absolution-1': 'common/vigils/1st-nocturn/absolution.gabc',
			'blessing-1': 'common/vigils/1st-nocturn/blessing-1.gabc',
			'blessing-2': 'common/vigils/1st-nocturn/blessing-2.gabc',
			'blessing-3': 'common/vigils/1st-nocturn/blessing-3.gabc',

			'absolution-2': 'common/vigils/2nd-nocturn/absolution.gabc',
			'blessing-4': 'common/vigils/2nd-nocturn/blessing-1.gabc',
			'blessing-5': 'common/vigils/2nd-nocturn/blessing-2.gabc',
			'blessing-6': 'common/vigils/2nd-nocturn/blessing-3.gabc',

			'absolution-3': 'common/vigils/3rd-nocturn/absolution.gabc',
			'blessing-7': 'common/vigils/3rd-nocturn/blessing-1(sunday).gabc',
			'blessing-8': 'common/vigils/3rd-nocturn/blessing-2(sunday).gabc',
			'blessing-9': 'common/vigils/3rd-nocturn/blessing-3(sunday).gabc',
		}

		case 'monday': return {
			invitatory: 'invitatory/venite-exultemus.lit',
			hymn: 'hymn/somno-refectis-artubus.lit',
			'absolution-1': 'common/vigils/1st-nocturn/absolution.gabc',
			'blessing-1': 'common/vigils/1st-nocturn/blessing-1.gabc',
			'blessing-2': 'common/vigils/1st-nocturn/blessing-2.gabc',
			'blessing-3': 'common/vigils/1st-nocturn/blessing-3.gabc',
		}

		case 'tuesday': return {
			invitatory: 'invitatory/jubilemus-deo.lit',
			hymn: 'hymn/consors-paterni-luminis.lit',
			'absolution-1': 'common/vigils/2nd-nocturn/absolution.gabc',
			'blessing-1': 'common/vigils/2nd-nocturn/blessing-1.gabc',
			'blessing-2': 'common/vigils/2nd-nocturn/blessing-2.gabc',
			'blessing-3': 'common/vigils/2nd-nocturn/blessing-3.gabc',
		}

		case 'wednesday': return {
			invitatory: 'invitatory/in-manu-tua.lit',
			hymn: 'hymn/rerum-creator-optime.lit',
			'absolution-1': 'common/vigils/3rd-nocturn/absolution.gabc',
			'blessing-1': 'common/vigils/3rd-nocturn/blessing-1.gabc',
			'blessing-2': 'common/vigils/3rd-nocturn/blessing-2.gabc',
			'blessing-3': 'common/vigils/3rd-nocturn/blessing-3.gabc',
		}

		case 'thursday': return {
			invitatory: 'invitatory/adoremus-dominum.lit',
			hymn: 'hymn/nox-atra-rerum-contigit.lit',
			'absolution-1': 'common/vigils/1st-nocturn/absolution.gabc',
			'blessing-1': 'common/vigils/1st-nocturn/blessing-1.gabc',
			'blessing-2': 'common/vigils/1st-nocturn/blessing-2.gabc',
			'blessing-3': 'common/vigils/1st-nocturn/blessing-3.gabc',
		}

		case 'friday': {
			return {
				invitatory: 'invitatory/dominum-qui-fecit-nos.lit',
				hymn: 'hymn/tu-trinitatis-unitas.lit',
				'absolution-1': 'common/vigils/2nd-nocturn/absolution.gabc',
				'blessing-1': 'common/vigils/2nd-nocturn/blessing-1.gabc',
				'blessing-2': 'common/vigils/2nd-nocturn/blessing-2.gabc',
				'blessing-3': 'common/vigils/2nd-nocturn/blessing-3.gabc',
			}
		}

		case 'saturday': {
			return {
				invitatory: 'invitatory/dominum-deum-nostrum.lit',
				hymn: 'hymn/summe-deus-clementie.gabc',
				'absolution-1': 'common/vigils/3rd-nocturn/absolution.gabc',
				'blessing-1': 'common/vigils/3rd-nocturn/blessing-1.gabc',
				'blessing-2': 'common/vigils/3rd-nocturn/blessing-2.gabc',
				'blessing-3': 'common/vigils/3rd-nocturn/blessing-3.gabc',
			}
		}
	}

	return {}
}

function lauds_commons(day) {
	switch (day) {
		case 'friday': {
			return {
				hymn: 'hymn/eterna-celi-gloria.lit',
				chapter: 'common/lauds/chapters/feria.lit',
				versicle: 'common/lauds/versicle/feria.lit'
			}
		}
	}

	return {}
}

function minor_commons(hour, day) {
	let hymn;
	switch(hour) {
		case 'prime': hymn = 'jam-lucis-orto-sidere'; break;
		case 'terce': hymn = 'nunc-sancte-nobis-spiritus'; break;
		case 'sext': hymn = 'rector-potens-verax'; break;
		case 'none': hymn = 'rerum-deus-tenax'; break;
	}

	let type;

	if (day == 'sunday') {
		hymn = 'hymn/' + hymn + '(sunday).lit'
		type = 'sunday'
	} else {
		hymn = 'hymn/' + hymn + '(feria).lit'
		type = 'feria'
	}

	return {
		hymn: hymn,
		chapter: 'common/' + hour + '/chapters/' + type + '.lit',
		versicle: (hour == 'prime') ? 'common/prime/versicle.lit' : 'common/' + hour + '/versicle/' + type + '.lit'
	}
}

function minor_hours(hour, day, kyrie) { // produces the whole for a given minor hour in ordinary time
	return {
		order: 'terce/penitential-order.lit',
		psalter: hour + '/' + (day == 'sunday' ? 'sunday' : 'feria') + '.lit',
		propers: minor_commons(hour, day),
	}
}

function vespers_commons(day) {
	switch (day) {
		case 'friday': return {
			hymn: 'hymn/plasmator-hominis.lit',
			chapter: 'common/vespers/chapters/feria.lit',
			versicle: 'common/vespers/versicle/feria.lit'
		}

		case 'saturday': return {
			hymn: 'hymn/o-lux-beata-trinitas.lit',
			chapter: 'common/vespers/chapters/sunday.lit',
			versicle: 'common/vespers/versicle/sunday.lit' // NOTE: only the saturday office (first vespers) uses the sunday chapter and versicle; second vespers of sundays uses the ferial
		}
	}

	return {}
}