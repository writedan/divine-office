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

	if (ending == 'solemn') {
		return tone + '-' + ending;
	}

	if (tone == '1') {
		return '8';
	}

	if (tone == '6a') {
		return '8';
	}

	if (tone == '2') {
		if (ending == 'i') {
			return '2';
		} else if (ending == 'ii') {
			return '8' // identical stress pattern
		}
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
		if (ending == 'iii') return '8';
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

	syncCleanTree() {
		for (let node of this.children) {
			if (node.directive.type == 'root') {
				for (let subnode of node.children) {
					this.addBefore(subnode, node);
				}

				this.remove(node);
				node.syncCleanTree();
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

	unfold(ctx) {
		try {
			if (!ctx) {
				throw new Error('LiturgyContext is necessary to unfold nodes.');
			}

			this.syncCleanTree();

			if (this.directive.type == 'if-include') {
				let url = ctx.getField(this.directive.args[0]);
				if (!url) {
					return this; // its possible the field will be set during execution
				}

				return new Node(Directive.new('import', [url])).unfold(this);
			}

			else if (this.directive.type == 'antiphon') {
				let path = 'antiphon/' + this.directive.args[0] + '.gabc';
				ctx.setField('last-antiphon', this.directive.args[0])
				let root = new Node(Directive.new('root'));
				root.add(new Node(Directive.new('score', [path])));
				return root.unfold(this);
			} 

			else if (this.directive.type == 'repeat-antiphon') {
				this.setAttribute('antiphon', ctx.getField('last-antiphon'));
				return this; //nothing further we can do synchronously
			}

			else if (this.directive.type == 'define') {
				ctx.setField(this.directive.args[0], this.directive.args[1]);
				return undefined;
			}

			else if (this.directive.type == 'gabc') {
				let gabc = `initial-style: 0;\ncentering-scheme:english;\n%%\n${this.directive.args[0]}`
				let root = new Node(Directive.new('raw-gabc', [gabc]));
				return root.unfold(this);
			}

			else if (this.directive.type == 'tone') {
				ctx.setField('last-tone', this.directive.args[0]);
				let path = 'tones/' + this.directive.args[0] + '.gabc';
				let root = new Node(Directive.new('root'));
				root.add(new Node(Directive.new('score', [path])));
				return root.unfold(this);
			}

			else if (this.directive.type == 'psalm') {
				let tone = ctx.getField('last-tone');
				this.setAttribute('tone', tone);
				return this; // we can do nothing further synchronously
			}

			else if (this.directive.type == 'gloria') {
				let link = this.directive.args[0];
				if (link == 'alleluia' || link == 'laus-tibi') {
					return new Node(Directive.new('score', ['common/gloria/' + link + '.gabc'])).unfold(ctx);
				} else {
					if (ctx.getField('universal-gloria-disabled')) {
						return undefined;
					}
					return new Node(Directive.new('import', ['common/gloria/' + resolveTone(link) + '.lit'])).unfold(ctx);
				}
			}

			else if (this.directive.type == 'begin-hymn') {
				let node = new Node(Directive.new('hymn'))
				ctx.setField('hymn', node);
				return undefined;
			}

			else if (this.directive.type == 'clef' || this.directive.type == 'melody' || this.directive.type == 'verse' || this.directive.type == 'make' || this.directive.type == 'amen') {
				let node = ctx.getField('hymn');
				node.add(this);
				return undefined;
			}

			else if (this.directive.type == 'end-hymn') {
				return ctx.getField('hymn').unfold(ctx);
			}

			else if (this.directive.type == 'hymn') {
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

						if (verse.length < melody.length) {
							if (verse.length < melody.length - 3) {
								throw new Error(`On verse "${verse}", less syllables than melody, and amen presumably not present.`)
							}
						}

						for (let verse_idx in verse) {
							let syllable = verse[verse_idx];
							let notes = melody[verse_idx];
							if (!syllable || !notes) {
								throw new Error(`On verse "${verse}", syllable "${syllable}" and notes "${notes}" are mismatched."`);
							}

							if (verse_idx == 0 && i == 0) {
								gabc += (vi + 1) + '. ' + (vi == 0 ? '' : ' (::)');
							}

							let continuous = syllable.endsWith('-')
							gabc += (continuous ? syllable.substring(0, syllable.length - 1) : syllable + ' ') + '(' + notes + ')';
						}

						if ((i + vlen) >= combined.length) {
							continue;
						}

						gabc += (i % 2 == 0) ? '(;)' : '(,)';
					}
				}

				return new Node(Directive.new('gabc', [gabc])).unfold(ctx);
			}

			else if (this.directive.type == 'yield') {
				// yield must be given a unqie name in args[0]
				// anything which comes after a yield in a given root must be grouped under it into a new resumable node
				// resumable nodes recur only when a #resume is called with their name
				// we will not presume to derive the name of a resumable
				if (!this.directive.args[0]) {
					throw new Error('Cannot yield without resumable name.');
				}
				
				let nodes = this.root.childrenAfter(this);
				let resumable = new Node(Directive.new('root'));
				nodes.shift(); // remove this
				for (let n of nodes) {
					this.root.remove(n);
					resumable.add(n);
				}

				ctx.setField('resumable:' + this.directive.args[0], resumable);
				//resumable.children.push(...nodes);
				//resumable = resumable.unfold(ctx).preprocess(ctx);
				return undefined;
			}

			this.children = this.children.map(n => {
				if (!n.unfold) {
					throw new Error('We cannot accept promises!');
				}

				return n.unfold(ctx);
			})

			return this;
		} catch (error) {
			return new Node(Directive.new('error', [error]))
		}
	}

	async preprocess(ctx) {
		try {
			if (!ctx) {
				throw new Error('LiturgyContext is necessary to preprocess nodes.');
			}

			if (this.directive.type == 'score') {
				return (await GabcParser.fromUrl(this.directive.args[0])).buildTree();
			} 

			else if (this.directive.type == 'include') {
				let root = await new Node(Directive.new('import', [await ctx.getPromisedField(this.directive.args[0])]));
				root.setAttribute('name', this.directive.args[0])
				root = root.preprocess(ctx);
				return root;

			} else if (this.directive.type == 'if-include') {
				let url;
				try {
					url = await ctx.getPromisedField(this.directive.args[0]);
				} catch (error) {
					console.warn(`Field ${this.directive.args[0]} did not resolve within timeout.`)
					console.warn('Ensure this was intended behavior.')
					console.warn(error);
					return undefined;
				}

				return new Node(Directive.new('import', [url])).unfold(ctx).preprocess(ctx);
			}

			else if (this.directive.type == 'import') {
				let url = this.directive.args[0];
				let new_ctx = new LiturgyContext(url, ctx);
				let root = await (await new_ctx.parser).buildTree();
				root.setAttribute('source', url);
				return root;

			} else if (this.directive.type == 'psalm') {
				let tone = this.getAttribute('tone');
				let num = this.directive.args[0];
				let root = new Node(Directive.new('root'));
				root.add(new Node(Directive.new('title', ['Psalm ' + num + '.'])));
				let psalmody = new Node(Directive.new('psalmody'))
				psalmody.add(new Node(Directive.new('import', ['psalter/' + num + '/' + resolveTone(tone) + '.lit'])))
				root.add(psalmody);
				psalmody = await psalmody.preprocess(ctx);
				await root.cleanTree();
				for (let node of root.children[1].children[0].children) {
					if ((await node).directive.type == 'title') {
						root.children[1].children[0].remove(node);
						root.addBefore(node, psalmody);
					}
				} 

				return await root.preprocess(ctx);

			} else if (this.directive.type == 'repeat-antiphon') {
				let antiphon = this.getAttribute('antiphon'); // ibid.
				let partial = this.directive.args[0] == 'partial';
				let gabcbase = "initial-style: 0;\ncentering-scheme: english;\n%%\n";
				let rest = (await fetch_text('antiphon/' + antiphon + '.gabc')).split('%%')[1];

				if (partial) {
					rest = rest.split('<sp>+</sp>(:)')[1];
				}

				let root = new Node(Directive.new('raw-gabc', [gabcbase + rest]))
				return await root.preprocess(ctx);

			} 

			else if (this.directive.type == 'resume') {
				let resumable = await ctx.getPromisedField('resumable:' + this.directive.args[0]);
				return await (await resumable).unfold(ctx).preprocess(ctx);
			}

			for (let child_idx in this.children) {
				if (!(this.children[child_idx])) {
					this.children.splice(child_idx, 1);
					continue;
				}

				this.children[child_idx] = await (await this.children[child_idx]).preprocess(ctx);
			}

			return this;
		} catch (error) {
			return new Node(Directive.new('error', [error]))
		}
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
		root.unfold(ctx);
		root.children = root.children.filter(n => n!==undefined);

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