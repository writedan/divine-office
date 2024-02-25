async function fetch_text(url) {
	let resp = await fetch('/' + url);
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
	}

	add(node) {
		node.root = this;
		this.children.push(node);
	}

	addBefore(node, before) {
		node.root = this;
		this.children.splice(this.children.indexOf(before), 0, node);
	}

	findPrevious(type) {
		// finds the previous directive of the given type
		let ancestor = this.root;
		while (ancestor !== undefined) {
			if (ancestor.root !== undefined) {
				ancestor = ancestor.root;
			} else {
				break;
			}
		}

		console.log('ANESTROY:',ancestor);
	}

	async preprocess(ctx) {
		if (!ctx) {
			throw new Error('LiturgyContext is necessary to preprocess nodes.');
		}

		if (this.directive.type == 'score') {
			return (await GabcParser.fromUrl(this.directive.args[0])).buildTree();
		} else if (this.directive.type == 'include') {
			let url = await ctx.getPromisedField(this.directive.args[0]);
			let new_ctx = new LiturgyContext(url, ctx);
			return (await new_ctx.parser).buildTree();

		} else if (this.directive.type == 'import') {
			let url = this.directive.args[0];
			let new_ctx = new LiturgyContext(url, ctx);
			return (await new_ctx.parser).buildTree();

		} else if (this.directive.type == 'antiphon') {
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
			let tone = await ctx.getPromisedField('last-tone'); // I really don't know if we can trust this value but it seems to work
			let num = this.directive.args[0];
			let root = new Node(Directive.new('root'));
			root.add(new Node(Directive.new('title', ['Psalm ' + num + '.'])));
			let psalmody = new Node(Directive.new('psalmody'))
			psalmody.add(new Node(Directive.new('import', ['psalter/' + num + '/' + resolveTone(tone) + '.lit'])))
			root.add(psalmody);
			psalmody = await psalmody.preprocess(ctx);
			for (let idx in psalmody.children[0].children) {
				let node = await psalmody.children[0].children[idx];
				if (node.directive.type != 'text' && node.directive.type != 'gloria') {
					root.addBefore(node, psalmody);
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

			}

		} else if (this.directive.type == 'begin-hymn') {
			ctx.setField('hymn', []);
			return undefined;
		}


		for (let child_idx in this.children) {
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
		this.parseHeaders();
		let root = new Node(Directive.new('root'));
		if (this.headers.heading) {
			root.add(new Node(Directive.new('title', [this.headers.heading])))
		}

		root.add(new Node(Directive.new('raw-gabc', [this.lines])))
		return root;
	}

	parseHeaders() {
		const headerLines = this.lines.split('%%')[0].split('\n');
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

	    this.headers = headerObject;
	}
}