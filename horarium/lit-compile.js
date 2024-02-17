/**
 * This file is to parse and execute .lit files
 * .lit files have a basic layout: text, and commands
 * All commands take up the whole line and begin with #
 * 
 * INDEX OF COMMANDS
 * #antiphon <antiphon> -- inserts the specified antiphon from antiphon/
 * #gabc <music> -- directly creates a score
 * #gloria <type> -- imports the specified Glory Be from common/glorybe
 * #psalm <psalm> [tone] -- inserts the psalm text from psalter/. It will load the tone previously given by #tone or, in absence, load the text, or you can specify the tone to use. Diacritics in the file will be automatically converted. Also adjoins the psalm number to the following title, if any is given.
 * #include <what> -- paramaters passed in by the calling agent as links to other .lit or .gabc, parses and displays
 * #instruction <text> -- prints as an instruction
 * #import <link> -- includes wholesale a .lit
 * #repeat-antiphon -- repeats the previous antiphon given by #antiphon but without initial lines and without any * or +.
 * #score <link> -- imports a score
 * #title <title> -- prints right-aligned small italic text for a title
 * #tone <tone> <clef> [starting note] -- loads the Euouae from tones/ and adjusts it to the given clef. It will automatically reassign the notes to such clef, unless a different starting note is provided, in which case it will set the first note to such and adjust the others accordingly.
 */

const URL_BASE = 'http://localhost:3000/'

function resolveTone(tone, ending='?') {
	if (ending == '?') {
		let parts = tone.split('-');
		return resolveTone(parts[0], parts[1])
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

function formatPsalm(verses) {

}
 
 class LiturgyContext {
 	constructor(url) {
 		this.url = URL_BASE + url;
 		this.ready = false;
 		this.load();
 	}

 	async load() {
        this.loaded = new Promise(async (resolve, reject) => {
            try {
                const response = await fetch(this.url);
                if (!response.ok) {
                    throw new Error(`Failed to fetch ${this.url}, status: ${response.status}`);
                }

                this.code = await response.text();
                this.ready = true;
                console.log(`Loaded liturgy: ${this.url}`);
                resolve();
            } catch (error) {
                console.error(`Error loading liturgy: ${this.url}:`, error);
                reject(error);
            }
        });
    }

 	async execute() {
 		try {
	 		if (!this.ready) {
	 			await this.loaded;
	 		}

	 		let output = [];
	 		let lines = this.code.split('\n');

	 		lines.forEach(line => {
	 			line = line.trim();
	 			if (line.length == 0) {
	 				return;
	 			}

	 			if (line.startsWith('#')) {
	 				const argsRegex = /"([^"]+)"/g;
	 				const argsMatch = line.match(argsRegex);
	 				const args = argsMatch ? line.match(argsRegex).map(arg => arg.slice(1, -1)) : [];
	            	const command = argsMatch ? line.substring(0, line.indexOf('"', 1)).slice(1).trim() : line.substring(1);
		            output.push(this.handleCommand(command, args));
	 			} else {
	 				let p = document.createElement('p');
	 				p.innerHTML = line;
	 				output.push(p);
	 			}
	 		})

	 		for (let i in output) {
	 			let r = await output[i];
	 			output[i] = r;
	 		}

	 		return output;
	 	} catch (error) {
	 		console.error('Error on LiturgyContext(' + this.url+')')
	 		console.error(error)

	 		let div = document.createElement('div');
	 		div.className = 'error';
	 		div.innerHTML = error;
	 		div.innerHTML += '<br/>' + error.stack;
	 		return [div];
	 	}
 	}

 	setField(name, value) {
 		this[name] = value;
 		return this[name]
 	}

 	async handleCommand(cmd, args) {
 		if (cmd == 'a') {
 			return 'a'
 		}

 		else if (cmd == 'antiphon') {
 			this.setField('antiphon', args[0]);
 			//return undefined;
 		}

 		else if (cmd == 'psalm') {
 			let div = document.createElement('div');
 			div.className = 'psalm'
 			let table = document.createElement('table');

 			table.className = 'psalm';
 			let tone = (this.tone === undefined ? 'text' : resolveTone(this.tone));
 			let ctx = new LiturgyContext('psalter/' + args[0] + '/' + tone + '.lit')
 			let verses = await ctx.execute();

 			let title = document.createElement('p');
 			title.className = 'title';
 			title.innerHTML = 'Psalm ' + args[0] + '.';

 			if (verses[0].className == 'title') {
 				title.innerHTML += '<br/>' + verses[0].innerHTML;
 				verses.splice(0, 1);
 			}

 			let numRows = Math.ceil(verses.length / 2)

 			div.append(title)

 			for (let i = 0; i < numRows; i++) {
 				let left = verses[i];
 				let right = verses[i + numRows];
 				let tr = document.createElement('tr');
 				let left_td = document.createElement('td');
 				left_td.append(left);
 				tr.append(left_td);
 				let right_td = document.createElement('td');
 				right_td.append((right === undefined ? document.createElement('p') : right));
 				tr.append(right_td)
 				table.append(tr)
 				left_td.style.width = '50%'
 				right_td.style.width = '50%'
 				left_td.style.verticalAlign = 'top'
 				right_td.style.verticalAlign = 'top'
 			}

 			div.append(table)


 			return div;
 		}

 		else if (cmd == 'gloria') {
 			return this.handleCommand('score', ['common/glorybe/' + args[0]])
 		}

 		else if (cmd == 'include') {
 			let div = document.createElement('div');
 			let url = this[args[0]];
 			url = (url === undefined ? 'resource:'+args[0] : url)
 			if (url.endsWith(".gabc")) {
 				return this.handleCommand('score', url);
 			}

 			let ctx = new LiturgyContext(url);
 			let res = await ctx.execute();
 			for (let r of res) {
 				div.appendChild(r);
 			}

 			return div;
 		}

 		else if (cmd == 'instruction') {
 			let p = document.createElement('p');
 			p.className = 'instruction';
 			p.innerHTML = args[0];
 			return p;
 		}

 		else if (cmd == 'title') {
 			let p = document.createElement('p');
 			p.className = 'title';
 			p.innerHTML = args[0];
 			return p;
 		}

 		else if (cmd == 'tone') {
 			this.setField('tone', args[0])
 			this.setField('tone-clef', args[1])
 			if (args.length > 2) {
 				this.setField('tone-initial', arg[2])
 			}

 			//return undefined;
 		}

 		 {
 			let div = document.createElement('div');
 			div.className = 'error';
 			div.innerHTML = 'Unknown command: ' + cmd;
 			return div;
 		}

 		//return document.createElement('blank')
 	}
 }