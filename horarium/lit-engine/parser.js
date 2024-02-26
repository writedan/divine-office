async function fetch_text(url) {
	let resp = await fetch(window.location.href.split('horarium')[0] + url);
	if (!resp.ok) {
		throw new Error(`Fetch failed for "${url}". Status code ${resp.status}.`);
	}

	return await resp.text();
}

function resolveTone(tone, ending='?') {
	if (ending == '?') {
		let parts = tone.split('-');
		return resolveTone(parts[0], (parts[1] === undefined ? '' : parts[1]))
	}

	if (tone == '3') {
		if (ending == 'i' || ending == 'ii' || ending == 'iii') return '3a';
		if (ending == 'iv' || ending == 'v') return '3b';
		if (ending == 'vi') return '3c';
	}

	if (tone == '4') {
		if (ending == 'i' || ending == 'ii' || ending == 'iii' || ending == 'iv' || ending == 'v') return '4a';
		if (ending == 'vi') return '4b';
		if (ending == 'vii' || ending == 'viii' || ending == 'ix') return '4c';
	}

	if (tone == '5') {
		if (ending == 'i' || ending == 'ii') return '5a';
		if (ending == 'iii') return '5b';
	}

	return tone;
}

class Directive {
	constructor(type, args=[]) {
		this.type = type;
		this.args = args;
	}

	static new(type, args=[]) {
		return new Directive(type, args);
	}
}

class Node {
	constructor(directive, children=[]) {
		this.directive = directive;
		this.children = children;
		this.attributes = {};
	}

	add(node) {
		node.root = this;
		this.children.push(node);
	}

	addBefore(node, before) {
		node.root = this;
		this.children.splice(this.children.indexOf(before), 0, node);
	}

	remove(node) {
		node.root = undefined;
		return this.children.splice(this.children.indexOf(node), 1);
	}

	childrenAfter(node) {
		console.log(this.children.indexOf(node))
		return this.children.slice(this.children.indexOf(node))
	}

	async cleanTree() {
		for (let node of this.children) {
			node = await node;

			if (node.directive.type == 'root') {
				for (let subnode of node.children) {
					this.addBefore(subnode, node);
				}

				this.remove(node);

				await node.cleanTree();
			}
		}
	}

	getAttribute(attr) {
		return this.attributes[attr]
	}

	setAttribute(attr, value) {
		this.attributes[attr] = value;
	}

	async getPromisedAttribute(attr) {
		let node = this;
	    return await new Promise((resolve, reject) => {
	        const timeout = 2000;
	        const startTime = Date.now();

	        const loop = () => {
	            if (node.getAttribute(attr) !== undefined) {
	                resolve(node.getAttribute(attr));
	            } else {
	                if (Date.now() - startTime >= timeout) {
	                    reject(new Error(`Timeout: Unable to get attribute ${attr} within ${timeout} milliseconds`));
	                } else {
	                    setTimeout(loop);
	                }
	            }
	        };

	        loop();
	    });
	}

	async preprocess(ctx) {
		if (!ctx) {
			throw new Error('LiturgyContext is necessary to preprocess nodes.');
		}

		await this.cleanTree();

		if (this.directive.type == 'score') {
			return (await GabcParser.fromUrl(this.directive.args[0])).buildTree();
		} 

		else if (this.directive.type == 'include') {
			let root = await new Node(Directive.new('import', [await ctx.getPromisedField(this.directive.args[0])])).preprocess(ctx);
			root.setAttribute('name', this.directive.args[0])
			return root;

		} else if (this.directive.type == 'import') {
			let url = this.directive.args[0];
			let new_ctx = new LiturgyContext(url, ctx);
			let root = await (await new_ctx.parser).buildTree();
			root.setAttribute('source', url);
			return root;

		} else if (this.directive.type == 'if-include') {
			let url = ctx.getField(this.directive.args[0]);
			if (!url) {
				return undefined;
			} else {
				return await new Node(Directive.new('import', [url])).preprocess(ctx);
			}
		}

		else if (this.directive.type == 'antiphon') {
			let path = 'antiphon/' + this.directive.args[0] + '.gabc';
			ctx.setField('last-antiphon', this.directive.args[0])
			let root = new Node(Directive.new('root'));
			root.add(new Node(Directive.new('title', ['Antiphon.'])))
			root.add(new Node(Directive.new('score', [path])));
			return await root.preprocess(ctx);

		} else if (this.directive.type == 'gabc') {
			let gabc = `initial-style: 0;\ncentering-scheme:english;\n%%\n${this.directive.args[0]}`
			let root = new Node(Directive.new('raw-gabc', [gabc]));
			return await root.preprocess(ctx);

		} else if (this.directive.type == 'tone') {
			ctx.setField('last-tone', this.directive.args[0]);
			let path = 'tones/' + this.directive.args[0] + '.gabc';
			let root = new Node(Directive.new('root'));
			root.add(new Node(Directive.new('score', [path])));
			return await root.preprocess(ctx);

		} else if (this.directive.type == 'psalm') {
			//console.log('PSALM', this)
			let tone = await ctx.getPromisedField('last-tone'); // I really don't know if we can trust this value but it seems to work
			let num = this.directive.args[0];
			let root = new Node(Directive.new('root'));
			root.add(new Node(Directive.new('title', ['Psalm ' + num + '.'])));
			let psalmody = new Node(Directive.new('psalmody'))
			psalmody.add(new Node(Directive.new('import', ['psalter/' + num + '/' + resolveTone(tone) + '.lit'])))
			root.add(psalmody);
			psalmody = await psalmody.preprocess(ctx);
			await root.cleanTree();
			for (let idx in psalmody.children[0].children) {
				let node = await psalmody.children[0].children[idx];
				if (node.directive.type == 'title') {
					psalmody.children[0].children.splice(idx, 1); // i don't know why remove doesnt work but it doesnt
					root.addBefore(node, psalmody)
				}
			}
			return await root.preprocess(ctx);

		} else if (this.directive.type == 'repeat-antiphon') {
			let antiphon = await ctx.getPromisedField('last-antiphon'); // ibid.
			let partial = this.directive.args[1] == 'partial';
			let gabcbase = "initial-style: 0;\ncentering-scheme: english;\n%%\n";
			let rest = (await fetch_text('antiphon/' + antiphon + '.gabc')).split('%%')[1];
			let root = new Node(Directive.new('raw-gabc', [gabcbase + rest]))
			return await root.preprocess(ctx);

		} else if (this.directive.type == 'gloria') {
			let link = this.directive.args[0];
			if (link == 'alleluia' || link == 'laus-tibi') {
				return await new Node(Directive.new('score', ['common/gloria/' + link + '.gabc'])).preprocess(ctx);
			} else {
				return await new Node(Directive.new('import', ['common/gloria/' + link + '.lit'])).preprocess(ctx);
			}

		} else if (this.directive.type == 'begin-hymn') {
			let node = new Node(Directive.new('hymn'))
			ctx.setField('hymn', node);
			return undefined;
		}

		else if (this.directive.type == 'clef' || this.directive.type == 'melody' || this.directive.type == 'verse' || this.directive.type == 'make' || this.directive.type == 'amen') {
			let node = await ctx.getPromisedField('hymn');
			node.add(this);
			return undefined;
		}

		else if (this.directive.type == 'end-hymn') {
			return await (await ctx.getPromisedField('hymn')).preprocess(ctx);
		}

		else if (this.directive.type == 'hymn') {
			await this.cleanTree();
			let melody;
			let verses = [];
			let combined = [];
			let clef;
			let vlen;
			for (let n of this.children) {
				if (n.directive.type == 'clef') {
					clef = n.directive.args[0];
				} else if (n.directive.type == 'melody') {
					melody = n.directive.args;
				} else if (n.directive.type == 'verse') {
					verses.push(n.directive.args);
				} else if (n.directive.type == 'amen') {
					melody.push('::')
					melody.push(n.directive.args[0]);
					melody.push(n.directive.args[1]);
					verses[verses.length - 1].push(' ')
					verses[verses.length - 1].push('A-')
					verses[verses.length - 1].push('men.')
				} else if (n.directive.type == 'make') {
					for (let v of verses) {
						combined.push([v, melody]);
					}

					vlen = verses.length;
					verses = [];
				} else {
					throw new Error('Unknown directive while parsing hymn: ' + n.directive.type + '['  + n.directive.args + ']');
				}
			}

			let gabc = '(' + clef + ') ';

			for (let vi of Array(vlen).keys()) {
				for (let i = 0; i < combined.length; i+= vlen) {
					let v = combined[vi + i];
					let verse = v[0];
					let melody = v[1];

					for (let verse_idx in verse) {
						let syllable = verse[verse_idx];
						let notes = melody[verse_idx];
						if (verse_idx == 0 && i == 0) {
							gabc += (vi + 1) + '. ' + (vi == 0 ? '' : ' (::)');
						}

						let continuous = syllable.endsWith('-')
						gabc += (continuous ? syllable : syllable + ' ') + '(' + notes + ')';
					}

					if ((i + vlen) >= combined.length) {
						continue;
					}

					gabc += (i % 2 == 0) ? '(;)' : '(,)';
				}
			}

			return await new Node(Directive.new('gabc', [gabc])).preprocess(ctx);
		}

		for (let child_idx in this.children) {
			if (!this.children[child_idx]) {
				this.children.splice(child_idx, 1);
			}

			this.children[child_idx] = await (await this.children[child_idx]).preprocess(ctx);
		}

		return this;
	}
}

class LiturgyParser {
	constructor(lines, ctx) {
		this.lines = lines;
		this.ctx = ctx;
	}

	static async fromUrl(url, ctx) {
		return new LiturgyParser((await fetch_text(url)).split('\n').map((entry) => entry.trim()).filter(function(entry) {return entry!=''}), ctx);
	}

	buildTree() {
		let root = new Node(Directive.new('root'));
		
		this.lines.forEach(line => {
			if (line.startsWith('#')) {
	 			const regex = /"([^"]+)"/g;
	 			const match = line.match(regex);
	 			const args = match ? line.match(regex).map(arg => arg.slice(1, -1)) : [];
	 			const command = (match ? line.substring(0, line.indexOf('"', 1)).slice(1) : line.substring(1)).trim();
	 			root.add(new Node(Directive.new(command, args)));
	 		} else {
	 			root.add(new Node(Directive.new('text', [line])));
	 		}
		});

		let ctx = this.ctx;
		root.children = root.children.map(async n => {
			return n.preprocess(ctx);
			// some nodes need to be preprocessed
			// notable, gabc nodes must be processed into raw-gabc nodes
			// include/import must be processed into new root nodes with the result of the query
			// include resolving to .gabc must be resolved as score
			// score must be resolved as raw-gabc
			// raw-gabc must resolve into a root with possible headers
		});

		return root;
	}
}

class GabcParser {
	constructor(music, ctx) {
		this.lines = music;
		this.ctx = ctx;
	}

	static async fromUrl(url, ctx) {
		return new GabcParser((await fetch_text(url)));
	}

	buildTree() {
		this.headers = GabcParser.parseHeaders(this.lines);
		let root = new Node(Directive.new('root'));
		if (this.headers.heading) {
			root.add(new Node(Directive.new('title', [this.headers.heading])))
		}

		root.add(new Node(Directive.new('raw-gabc', [this.lines])))
		return root;
	}

	static parseHeaders(lines) {
		const headerLines = lines.split('%%')[0].split('\n');
	    const headerObject = {};

	    for (let line of headerLines) {
	        line = line.trim();
	        if (line !== '') {
	            let [key, value] = line.split(':').map(part => part.trim());
	            value = value.substring(0, value.length - 1)
	            if (headerObject.hasOwnProperty(key)) {
	                if (Array.isArray(headerObject[key])) {
	                    headerObject[key].push(value);
	                } else {
	                    headerObject[key] = [headerObject[key], value];
	                }
	            } else {
	                headerObject[key] = value;
	            }
	        }
	    }

	    return headerObject;
	}
}