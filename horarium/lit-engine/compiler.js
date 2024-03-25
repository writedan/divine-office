function uuidv4() {
  return "10000000-1000-4000-8000-100000000000".replace(/[018]/g, c =>
    (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
  );
} // https://stackoverflow.com/a/2117523

function applySpecialCharactersToNextVowel(text) {
    let newText = '';
    let accentMap = {
    	'\u00B0': '\u030A',   // Combining Ring Above (◌̊)
        '^': '\u0302',  // Combining Circumflex Accent
        '~': '\u0303',  // Combining Tilde
        "`": '\u0301'   // Combining Acute Accent
    };

    let vowelAccents = ''; // Store the current accent symbol
    let boldStartIndex = -1; // Store the start index to bold
    let vowelIndex = -1;

    for (let i = 0; i < text.length; i++) {
        if (text[i] in accentMap) {
            // Store the accent symbol for the next vowel
            vowelAccents = accentMap[text[i]];
            // Store the start index for bolding
            boldStartIndex = newText.length;
        } else if (isVowel(text[i])) {
            // Add the vowel with the current accent symbol
            newText += text[i] + vowelAccents;
            vowelIndex = i + 1;
            // Reset the accent symbol for the next vowel
            vowelAccents = '';
        } else if (boldStartIndex !== -1 && i == vowelIndex) {
            // If we find a space after the special character, add the bold tag
            newText = newText.substring(0, boldStartIndex) + '<b>' + newText.substring(boldStartIndex) + '</b>';
            boldStartIndex = -1;
            newText += text[i];
        } else {
            // Add other characters as they are
            newText += text[i];
        }
    }

    // In case the special character was at the end, add bold tag
    if (boldStartIndex !== -1) {
        newText = newText.substring(0, boldStartIndex) + '<b>' + newText.substring(boldStartIndex) + '</b>';
    }

    return newText;

    // Function to check if a character is a vowel
    function isVowel(char) {
        return 'aeiouyAEIOUY'.indexOf(char) !== -1;
    }
}

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
        const timeout = 2000;
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

			else if (node.directive.type == 'heading') {
				let heading = document.createElement('h2');
				heading.innerHTML = node.directive.args[0];
				return heading;
			}

			else if (node.directive.type == 'subheading') {
				let heading = document.createElement('h3');
				heading.innerHTML = node.directive.args[0];
				return heading;
			}

			else if (node.directive.type == 'instruction') {
				let p = document.createElement('p');
				p.innerHTML = node.directive.args[0];
				p.className = 'instruction'
				return p;
			}

			else if (node.directive.type == 'no-gloria') {
				if (this.getField('universal-gloria-disabled')) {
					return undefined;
				}
			}

			else if (node.directive.type == 'text') {
				let text = node.directive.args[0];
				let newtext = '';

				if (text.includes('+')) {
					let d = text.split('+');
					let intone = applySpecialCharactersToNextVowel(d[0]);
					let flex = applySpecialCharactersToNextVowel(d[1]);
					newtext += intone + ' <span class="symbol">+</span><br/>';
					text = flex;
				}

				if (text.includes('*')) {
					let d = text.split('*');
					let meditation = applySpecialCharactersToNextVowel(d[0]);
					let ending = applySpecialCharactersToNextVowel(d[1]);

					newtext += meditation + ' <span class="symbol">*</span><br/>&emsp; ' + ending;
				} else {
					newtext = text;
				}

				let p = document.createElement('p');
				p.innerHTML = newtext;
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
					} else if (header == 'gospel-blessing') {
						this.setField(header, 'common/vigils/3rd-nocturn/matthew.gabc');
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
				if (true) {
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

			else if (node.directive.type == 'error') {
				return this.handleError(node.directive.args[0])
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