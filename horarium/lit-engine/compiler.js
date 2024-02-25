function uuidv4() {
  return "10000000-1000-4000-8000-100000000000".replace(/[018]/g, c =>
    (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
  );
} // https://stackoverflow.com/a/2117523

class LiturgyContext {
	parameters = {};
	constructor(url, base) {
		this.url = url;
		this.base = base;
		if (url.endsWith('.gabc')) {
			this.parser = GabcParser.fromUrl(url, this);
			this.gabc = true;
		} else {
			this.parser = LiturgyParser.fromUrl(url, this);
		}
	}

	async execute() {
		try {
			this.parser = await this.parser;
			let root = this.parser.buildTree();
			this.setField('scores', [])
			let output = [];
			for (let node of root.children) {
				output.push(this.compile(node))
			}

			return output;
		} catch (error) {
			return this.handleError(error);
		}
	}

	setField(field, value) {
		this.parameters[field] = value;
	}

	getField(field) {
		if (!this.parameters[field] && this.base) {
			return this.base.getField(field);
		}

		return this.parameters[field];
	}

	async getPromisedField(field) {
		let ctx = this;
		return await new Promise((resolve, reject) => {
			const loop = () => ctx.getField(field) !== undefined ? resolve(ctx.getField(field)) : setTimeout(loop);
			loop();
		})
	}

	async compile(node) {
		try {
			node = await node; // wait for the node to become available
			if (!node) {
				return undefined;
			}

			if (node.directive.type == 'root') {
				let output = [];
				for (let n of node.children) {
					output.push(await this.compile(n)); // any sub-array of promises must be resolved
														// before we can propagate up
				}

				return output;
			} 

			else if (node.directive.type == 'instruction') {
				let p = document.createElement('p');
				p.innerHTML = node.directive.args[0];
				p.className = 'instruction'
				return p;
			}

			else if (node.directive.type == 'text') {
				let p = document.createElement('p');
				p.innerHTML = node.directive.args[0];
				return p;
			}

			else if (node.directive.type == 'title') {
				let p = document.createElement('p');
				p.innerHTML = node.directive.args[0];
				p.className = 'title'
				return p;
			} 

			else if (node.directive.type == 'raw-gabc') {
				let div = document.createElement('div');
				//div.innerHTML = node.directive.args[0];
				let uuid = uuidv4();
				div.setAttribute('score-id', uuid);
				this.setField('score:' + uuid, {
					div: div,
					node: node
				})
				div.className = 'gabc-score';
				div.innerHTML = node.directive.args[0];
				return div;
			}

			throw new Error('Unknown node directive: ' + node.directive.type + '[' + node.directive.args +']')
		} catch (error) {
			return this.handleError(error)
		}
	}

	handleError(error) {
		console.error('LiturgyContext(' + this.url + '): ' + error);
		let div = document.createElement('div');
		div.className = 'error';
		div.innerHTML = error;
		error.stack.split('\n').forEach(msg => {
			div.innerHTML += '<br/>' + msg + '\n'
		})
		return [div];
	}
}