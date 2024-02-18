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
 * #import <link> -- includes wholesale a .lit (for internal use only?)
 * #nogloria -- suppresses any gloria until the next appears
 * #raw-gabc <gabc> -- for internal use only! directly formulates a gabc element
 * #repeat-antiphon -- repeats the previous antiphon given by #antiphon but without initial lines and without any * or +.
 * #score <link> -- imports a score
 * #title <title> -- prints right-aligned small italic text for a title
 * #tone <tone> <clef> [starting note] -- loads the Euouae from tones/ and adjusts it to the given clef. It will automatically reassign the notes to such clef, unless a different starting note is provided, in which case it will set the first note to such and adjust the others accordingly.
 */

const URL_BASE = 'http://localhost:3000/'

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
 
 class LiturgyContext {
 	constructor(url, base=undefined) {
 		this.url = URL_BASE + url;
 		this.ready = false;
 		this.base = base;
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

    handleError(error) {
    	console.error('Error on LiturgyContext(' + this.url+')')
	 	console.error(error)

	 	let div = document.createElement('div');
	 	div.className = 'error';
	 	div.innerHTML = error;
	 	div.innerHTML += '<br/>' + error.stack;
	 	return div;
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
		            try {
		            	output.push(this.handleCommand(command, args));
		        	} catch (error) {
		        		console.error('Error while executing:', line)
		        		output.push(this.handleError(error));
		        	}
	 			} else {
	 				let p = document.createElement('p');
	 				p.innerHTML = line;
	 				output.push(p);
	 			}
	 		})

	 		for (let i in output) {
	 			try {
	 				let r = await output[i];
	 				output[i] = r;
	 			} catch (error) {
	 				console.error('Error while awaiting execution:', output[i]);
	 				output[i] = this.handleError(error);
	 			}
	 		}

	 		return output;
	 	} catch (error) {
	 		console.error('General error in execution.')
	 		return [this.handleError(error)];
	 	}
 	}

 	setField(name, value) {
 		this[name] = value;
 		return this[name]
 	}

 	getField(name) {
 		let r = this[name];
 		if (r === undefined) {
 			if (this.base != undefined) {
 				return this.base[name];
 			} else {
 				return r;
 			}
 		} else {
 			return r;
 		}
 	}

 	createTitle(text) {
 		let p = document.createElement('p')
 		p.className = 'title'
 		p.innerHTML = text;
 		return p;
 	}

 	async handleCommand(cmd, args) {

 		if (cmd == 'a') {
 			return 'a'
 		}

 		else if (cmd == 'antiphon') {
 			this.setField('antiphon', args[0]);
 			return this.handleCommand('score', ['anitphon/' + args[0] + '.gabc']);
 		}

 		else if (cmd == 'psalm') {
 			let div = document.createElement('div');
 			div.className = 'psalm'
 			let table = document.createElement('table');

 			table.className = 'psalm';
 			let tone = (this.tone === undefined ? 'text' : resolveTone(this.tone));
 			let ctx = new LiturgyContext('psalter/' + args[0] + '/' + tone + '.lit', this)
 			let verses = (await ctx.execute()).flat();

 			console.log(verses)

 			div.append(this.createTitle('Psalm ' + args[0] + '.'))

 			if (verses[0].className == 'title') {
 				//title.innerHTML += '<br/>' + verses[0].innerHTML;
 				div.append(this.createTitle(verses[0].innerHTML))
 				verses.splice(0, 1);
 			}

 			let numRows = Math.ceil(verses.length / 2);

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

 		else if (cmd == 'gabc') {
 			let gabc = `
initial-style: 0;
centering-scheme: english;
%%
${args[0]}
 			`
 			return this.handleCommand('raw-gabc', [gabc])
 		}

 		else if (cmd == 'gloria') {
 			if (this.getField('nogloria') == true) {
 				this.setField('nogloria', false);
 				return document.createElement('blank')
 			}

 			let end = (args[0] == 'laus-tibi' || args[0] == 'alleluia') ? '.gabc' : '.lit'
 			if (end == '.gabc') {
 				return this.handleCommand('score', ['common/gloria/' + args[0] + end]);
 			} else {
 				return this.handleCommand('raw-import', ['common/gloria/' + args[0] + end])
 			}
 		}

 		else if (cmd == 'include') {
 			let url = this.getField(args[0]);
 			url = (url === undefined ? 'resource:'+args[0] : url)
 			if (url.endsWith(".gabc")) {
 				return this.handleCommand('score', [url]);
 			}

 			return this.handleCommand('import', [url]);
 		}

 		else if (cmd == 'instruction') {
 			let p = document.createElement('p');
 			p.className = 'instruction';
 			p.innerHTML = args[0];
 			return p;
 		}

 		else if (cmd == 'import') {
 			let div = document.createElement('div');
 			let ctx = new LiturgyContext(args[0], this);
 			let res = await ctx.execute();
 			for (let r of res) {
 				div.appendChild(r);
 			}

 			return div;
 		}

 		else if (cmd == 'nogloria') {
 			this.setField('nogloria', true);
 			return document.createElement('blank')
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

 			return this.handleCommand('score', ['tones/' + args[0] + '.gabc'])
 		}

 		else if (cmd == 'raw-gabc') {
 			let ctxt = new exsurge.ChantContext();
 			ctxt.setFont("'Arial, sans-serif'", 16 * 1.2);
		    ctxt.dropCapTextFont = ctxt.lyricTextFont;
		    ctxt.annotationTextFont = ctxt.lyricTextFont;
		    ctxt.textMeasuringStrategy = exsurge.TextMeasuringStrategy.Canvas;
		    ctxt.minLyricWordSpacing = ctxt.hyphenWidth * 0.7;
		    console.log(ctxt.textStyles);

 			let gabc = args[0].replace(/(<b>[^<]+)<sp>'(?:oe|œ)<\/sp>/g,'$1œ</b>\u0301<b>')
	 			.replaceAll('<sp>v</sp>', '<v>\\Vbar</v>')
	 			.replaceAll('<sp>r</sp>', '<v>\\Rbar</v>')
	 			.replaceAll('<sp>a</sp>', '<v>\\Abar</v>')
	 			.replaceAll('<sp>*</sp>', '<v>\\greheightstar</v>')
			      .replaceAll(/<v>\\([VRAvra])bar<\/v>/g,'$1/.')
			      .replaceAll(/<sp>([VRAvra])\/<\/sp>\.?/g,'$1/.')
			      .replaceAll(/<b><\/b>/g,'')
			      .replaceAll(/<sp>'(?:ae|æ)<\/sp>/g,'ǽ')
			      .replaceAll(/<sp>'(?:oe|œ)<\/sp>/g,'œ́')
			      .replaceAll(/<v>\\greheightstar<\/v>/g,'*')
			      .replaceAll(/<\/?sc>/g,'%')
			      .replaceAll(/<\/?b>/g,'*')
			      .replaceAll(/<\/?i>/g,'_')
			        .replaceAll(/(\s)_([^\s*]+)_(\(\))?(\s)/g,"$1^_$2_^$3$4")
			        .replaceAll(/(\([cf][1-4]\)|\s)(\d+\.)(\s\S)/g,"$1^$2^$3");
 			let mappings = exsurge.Gabc.createMappingsFromSource(ctxt, gabc);
			let score = new exsurge.ChantScore(ctxt, mappings, true);
			let div = document.createElement('div');
			div.className = 'gabc-score'
			await score.performLayoutAsync(ctxt, async function() {
			  await score.layoutChantLines(ctxt, document.getElementById('content').offsetWidth, async function() {
			    let svg = await score.createSvgNode(ctxt);
			    div.appendChild(svg);
			  });
			});
 			return div;
 		}

 		else if (cmd == 'raw-import') {
 			let resp = await fetch(URL_BASE + args[0]);
 			let text = await resp.text();
 			let lines = text.split('\n')
 			let output = []
 			for (let line of lines) {
 				line = line.trim();
 				if (line.length == 0) {
 					continue;
 				}

 				let p = document.createElement('p')
 				p.innerHTML = line;
 				output.push(p)
 			}

 			return output;
 		}

 		else if (cmd == 'score') {
 			let resp = await fetch(URL_BASE + args[0]);
 			if (!resp.ok) {
 				throw new Error(`Failed to fetch ${args[0]}, status code: ${resp.status}.`)
 			}

 			let gabc = await resp.text();
 			return this.handleCommand('raw-gabc', [gabc]);
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