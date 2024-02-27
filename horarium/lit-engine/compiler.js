function uuidv4() {
  return "10000000-1000-4000-8000-100000000000".replace(/[018]/g, c =>
    (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
  );
} // https://stackoverflow.com/a/2117523

class LiturgyContext {
	parameters = {};
	constructor(url, base) {
		if (!url) {
			console.warn('LiturgyContext created without an url!');
			return;
		}

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
		if (this.base) {
			this.base.setField(field, value);
		}
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
        const timeout = 250;
        const startTime = Date.now();

        const loop = () => {
            if (ctx.getField(field) !== undefined) {
                resolve(ctx.getField(field));
            } else {
                if (Date.now() - startTime >= timeout) {
                    reject(new Error(`Timeout: Unable to get field ${field} within ${timeout} milliseconds`));
                } else {
                    setTimeout(loop);
                }
            }
        };

        loop();
    });
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
				let headers = GabcParser.parseHeaders(node.directive.args[0].split('%%')[0]);
				for (let header in headers) {
					if (header.startsWith('responsory')) {
						this.setField(header, 'resp/' + headers[header] + '.gabc')
					} 
				}

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

			else if (node.directive.type == 'psalmody') {
				let nodes = node.children; 
				let contentWidth = document.getElementById('content').offsetWidth;
				if (contentWidth < 800) {
					// display in single list
					let output = [];
					for (let n of nodes) {
						output.push(await this.compile(n));
					}
					
					return output;
				} else {
					// display as table
					let elements = (await this.compile(nodes[0])).flat();
					let numRows = (elements.length / 2);
					let left_column = [];
					let right_column = [];
					for (let i in elements) {
						let element = elements[i];
						if (i < numRows) {
							if (numRows % 2 != 0 && i == Math.floor(numRows)) {
								let v = element.textContent.split('*');
								let v1 = element.cloneNode(true);
								let v2 = element.cloneNode(true);
								v1.textContent = v[0] + '*';
								v2.textContent = v[1];
								left_column.push(v1);
								right_column.push(v2);
							} else {
								left_column.push(element)
							}
						} else {
							right_column.push(element);
						}
					}

					let table = document.createElement('table')
					table.className = 'psalm';
					numRows = Math.ceil(elements.length / 2);
					for (let i = 0; i < numRows; i++) {
						let left = left_column[i];
						let right = right_column[i];
						let tr = document.createElement('tr');
						let left_td = document.createElement('td');
						let right_td = document.createElement('td');
						left_td.append(...[left].flat());
						right_td.append(...[right].flat());
						left_td.style.width = '50%';
						right_td.style.width = '50%'
						left_td.style.verticalAlign = 'top'
						right_td.style.verticalAlign = 'top'
						tr.append(left_td);
						tr.append(right_td);
						table.append(tr);
					}

					return table;
				}
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