function applyTriduum(metadata) {
	// holy thurday
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

	//good friday
	metadata.passion[2].friday.hours = {
		Vigils: {
			order: 'tenebrae/order.lit',
			psalter: 'tenebrae/good-friday.lit',
			propers: mergeDeep(lessons('propers/triduum/friday/tenebrae/'), {
				benedictus: 'propers/triduum/friday/tenebrae/benedictus.lit',
				preces: 'propers/triduum/friday/tenebrae/preces.lit',
				'universal-gloria-disabled': true
			})
		},

		Lauds: {
			order: 'tenebrae/order.lit',
			psalter: 'tenebrae/good-friday.lit',
			propers: mergeDeep(lessons('propers/triduum/friday/tenebrae/'), {
				benedictus: 'propers/triduum/friday/tenebrae/benedictus.lit',
				preces: 'propers/triduum/friday/tenebrae/preces.lit',
				'universal-gloria-disabled': true
			})
		},

		Prime: {
			order: 'terce/triduum-order.lit',
			psalter: 'prime/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/friday/tenebrae/preces.lit',
			}
		},

		Terce: {
			order: 'terce/triduum-order.lit',
			psalter: 'terce/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/friday/tenebrae/preces.lit',
			}
		},

		Sext: {
			order: 'terce/triduum-order.lit',
			psalter: 'sext/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/friday/tenebrae/preces.lit',
			}
		},

		None: {
			order: 'terce/triduum-order.lit',
			psalter: 'none/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/friday/tenebrae/preces.lit',
			}
		},

		Vespers: {
			order: 'vespers/triduum-order.lit',
			psalter: 'vespers/triduum.lit',
			propers: {
				magnificat: 'propers/triduum/friday/vespers/magnificat.lit',
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/friday/tenebrae/preces.lit',
			}
		},

		Compline: {
			order: 'terce/triduum-order.lit',
			psalter: 'compline/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/friday/tenebrae/preces.lit',
			}
		}
	}

	//holy saturday
	metadata.passion[2].saturday.hours = {
		Vigils: {
			order: 'tenebrae/order.lit',
			psalter: 'tenebrae/holy-saturday.lit',
			propers: mergeDeep(lessons('propers/triduum/saturday/tenebrae/'), {
				benedictus: 'propers/triduum/saturday/tenebrae/benedictus.lit',
				preces: 'propers/triduum/saturday/tenebrae/preces.lit',
				'universal-gloria-disabled': true
			})
		},

		Lauds: {
			order: 'tenebrae/order.lit',
			psalter: 'tenebrae/holy-saturday.lit',
			propers: mergeDeep(lessons('propers/triduum/saturday/tenebrae/'), {
				benedictus: 'propers/triduum/saturday/tenebrae/benedictus.lit',
				preces: 'propers/triduum/saturday/tenebrae/preces.lit',
				'universal-gloria-disabled': true
			})
		},

		Prime: {
			order: 'terce/triduum-order.lit',
			psalter: 'prime/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/saturday/tenebrae/preces.lit',
			}
		},

		Terce: {
			order: 'terce/triduum-order.lit',
			psalter: 'terce/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/saturday/tenebrae/preces.lit',
			}
		},

		Sext: {
			order: 'terce/triduum-order.lit',
			psalter: 'sext/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/saturday/tenebrae/preces.lit',
			}
		},

		None: {
			order: 'terce/triduum-order.lit',
			psalter: 'none/triduum.lit',
			propers: {
				'universal-gloria-disabled': true,
				preces: 'propers/triduum/saturday/tenebrae/preces.lit',
			}
		}
	}
}