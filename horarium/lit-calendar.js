function lessons(prefix) {
	let res = {};
	for (let i = 0; i < 9; i++) {
		res['lesson-' + (i + 1)] = prefix + 'lesson-' + (i + 1) + '.lit'
	}

	res['gospel'] = prefix + 'gospel.lit'

	return res;
}

function annotateTemporalMetadata(metadata) { // attach hour information
	// TODO: advent
	// TODO: christmas
	// TODO: epiphany
	
	applyPrelent(metadata);

	applyLent(metadata);

	applyPassiontide(metadata);

	applyTriduum(metadata);

	applyEaster(metadata);

	applyAscension(metadata);
	
	applyPentecost(metadata);
	
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

	let base = {
		hymn: hymn,
		chapter: 'common/' + hour + '/chapters/' + type + '.lit',
		versicle: (hour == 'prime') ? 'common/prime/versicle.lit' : 'common/' + hour + '/versicle/' + type + '.lit'
	}

	if (hour == 'prime') {
		base.collect = 'common/prime/collect.gabc'
	}

	return base;
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