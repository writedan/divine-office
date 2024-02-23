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

const URL_BASE = window.location.href.split('horarium')[0];

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

function getHeaders(gabcheaders) {
	const headerLines = gabcheaders.split('\n');
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
 
 class LiturgyContext {

 	constructor(url, base=undefined) {
 		if (url) {
	 		this.url = URL_BASE + url;
	 		this.ready = false;
	 		this.base = base;
	 		this.load();
 		} else {
 			this.ready = true;
 		}

 		this.executed = [];
 	}

 	async load() {
        this.loaded = new Promise(async (resolve, reject) => {
            try {
                const response = await fetch(this.url);
                if (!response.ok) {
                    throw new Error(`Failed to fetch ${this.url}, status: ${response.status}`);
                }

                this.code = (await response.text()).split('\n').map((entry) => entry.trim()).filter(function(entry) {return entry!=''})
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
    	console.error('Error on LiturgyContext(' + this.url+'): ', error)

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

	 		while (this.code.length && !this.yielded) {
	 			const line = this.code.shift();	// dequeue
	 			if (line.startsWith('#')) {
	 				const regex = /"([^"]+)"/g;
	 				const match = line.match(regex);
	 				const args = match ? line.match(regex).map(arg => arg.slice(1, -1)) : [];
	 				const command = (match ? line.substring(0, line.indexOf('"', 1)).slice(1) : line.substring(1)).trim();
	 				output.push(this.handleCommand(command, args));
	 				this.executed.push([command, args].flat())
	 			} else {
	 				let p = document.createElement('p');
	 				p.innerHTML = line;
	 				output.push(p);
	 			}
	 		}

	 		return await Promise.all(output.map(async (r) => {
	 			try {
	 				return (await r);
	 			} catch (error) {
	 				return this.handleError(error);
	 			}
	 		}));

	 	} catch (error) {
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
 		try {
 			return (await this.internal_handleCommand(cmd, args));
 		} catch (error) {
 			return this.handleError(error);
 		}
 	}

 	async internal_handleCommand(cmd, args) {

 		if (cmd == 'a') {
 			return 'a'
 		}

 		else if (cmd == 'antiphon') {
 			let div = document.createElement('div')
 			div.className = 'antiphon'
 			this.setField('antiphon', args[0]);
 			div.append(this.createTitle('Antiphon.'))
 			div.append(await this.handleCommand('score', ['antiphon/' + args[0] + '.gabc']));
 			return div;
 		}

 		else if (cmd == 'psalm') {
 			let div = document.createElement('div');
 			div.className = 'psalm'
 			let table = document.createElement('table');

 			table.className = 'psalm';
 			let tone = (this.tone === undefined ? 'text' : resolveTone(this.tone));
 			let ctx = new LiturgyContext('psalter/' + args[0] + '/' + tone + '.lit', this)
 			let verses = (await ctx.execute()).flat();

 			div.append(this.createTitle('Psalm ' + args[0] + '.'))

 			if (verses[0].className == 'title') {
 				//title.innerHTML += '<br/>' + verses[0].innerHTML;
 				div.append(this.createTitle(verses[0].innerHTML))
 				verses.splice(0, 1);
 			}

 			let numRows = (verses.length / 2);
 			let left_column = [];
 			let right_column = [];

 			for (let i in verses) {
 				if (i < numRows) {
 					if (numRows % 2 != 0 && i == Math.floor(numRows)) {
 						let v = verses[i].textContent.split('*');
 						let v1 = verses[i].cloneNode(true);
 						let v2 = verses[i].cloneNode(true);
 						v1.textContent = v[0] + '*';
 						v2.textContent = v[1];
 						left_column.push(v1)
 						right_column.push(v2)
 					} else {
 						left_column.push(verses[i]);
 					}
 				} else {
 					right_column.push(verses[i]);
 				}
 			}

 			numRows = Math.ceil(verses.length / 2);

 			for (let i = 0; i < numRows; i++) {
 				let left = left_column[i];
 				let right = right_column[i];
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
 				this.setField('tone-initial', args[2])
 			}

 			let div = document.createElement('div')
 			//div.append(this.createTitle('Tone ' + resolveTone(args[0]) + '.'))

 			div.append(await this.handleCommand('score', ['tones/' + args[0] + '.gabc']))
 			return div;
 		}

 		else if (cmd == 'raw-gabc') {
 			let ctxt = new exsurge.ChantContext();
 			ctxt.setFont("'Arial, sans-serif'", 16 * 1.25);
		    //ctxt.dropCapTextFont = ctxt.lyricTextFont;
		    //ctxt.annotationTextFont = ctxt.lyricTextFont;
		    //ctxt.textMeasuringStrategy = exsurge.TextMeasuringStrategy.Canvas;
		    //ctxt.minLyricWordSpacing; = ctxt.hyphenWidth * 0.7;
		    //ctxt.glyphScaling = 0.08
		    ctxt.setGlyphScaling(0.08)

		    let headers = getHeaders(args[0].split('%%')[0]);

		    ctxt.markupSymbolDictionary['^'] = 'c'
		    ctxt.textStyles.al.prefix = '<b>'
		    
		    ctxt.defaultLanguage = (headers['centering-scheme'] == 'english' ? new exsurge.English : new exsurge.Latin);
		    
		    ctxt.textStyles.annotation.size = 16;
		    ctxt.textStyles.annotation.color ='red';

		    window.ctx = ctxt;

 			let gabc = args[0].split('%%')[1].replace(/(<b>[^<]+)<sp>'(?:oe|œ)<\/sp>/g,'$1œ</b>\u0301<b>')
	 			.replaceAll('<sp>v</sp>', '<v>\\Vbar</v>')
	 			.replaceAll('<sp>r</sp>', '<v>\\Rbar</v>')
	 			.replaceAll('<sp>a</sp>', '<v>\\Abar</v>')
	 			.replaceAll('<sp>*</sp>', '<v>\\greheightstar</v>')
	 			.replaceAll('<sp>+</sp>', '<v>+</v>')
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
			
			let score = new exsurge.ChantScore(ctxt, mappings, headers['initial-style'] == '1');
			if (headers['initial-style'] == '1') {
				if (headers['annotation']) {
					let a = [headers['annotation']].flat();
					if (a.length == 1) {
						score.annotation = new exsurge.Annotations(ctxt, a[0], '');
					} else {
						score.annotation = new exsurge.Annotations(ctxt, a[0], a[1])
					}
				}
			}

			score.updateNotations(ctxt);

			let div = document.createElement('div');

			if (headers.heading) {
				div.append(this.createTitle(headers.heading))
			}

			div.className = 'gabc-score'
			window.score = score;

			let lctx = this;
			await score.performLayoutAsync(ctxt, async function() {
			  await score.layoutChantLines(ctxt, document.getElementById('content').offsetWidth, async function() {
			    let svg = await score.createSvgNode(ctxt);
			    for (let e of svg.getElementsByClassName('aboveLinesText')) {
			    	let offset = (e.textContent == '~' ? 15 : 20);
			    	e.setAttribute('test', parseFloat(e.getAttribute('y')) + offset)
			    	e.setAttribute('y', e.getAttribute('test'))
			    	if (e.innerHTML == '') {
			    		e.innerHTML = '^'
			    		e.style.fontWeight = 'bold'
			    	}
			    }
			    
			    div.appendChild(svg); 

			    if (headers.resp) {
					div.appendChild(await lctx.handleCommand('score', ['resp/' + headers.resp]))
				}
			  });
			});

 			return div;
 		}

 		else if (cmd == 'raw-import') {
 			let resp = await fetch(URL_BASE + args[0]);
 			if (!resp.ok) {
 				throw new Error(`Failed to fetch ${URL_BASE + args[0]}, status code: ${resp.status}.`)
 			}
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

 		else if (cmd == 'repeat-antiphon') {
 			let div = document.createElement('div')
 			div.className = 'antiphon'
 			let resp = await fetch(URL_BASE + 'antiphon/' + this.getField('antiphon') + '.gabc')
 			if (!resp.ok) {
 				throw new Error(`Failed to fetch ${URL_BASE + this.getField('antiphon')}, status code: ${resp.status}.`)
 			}
 			let gabc = (await resp.text()).split('%%')[1]
 			if (args[0] == 'partial') {
 				gabc = gabc.split('<sp>+</sp>(:)')[1]
 				console.log(gabc)
 			}

 			gabc = gabc.replaceAll('<sp>*</sp>', '')

 			let newgabc = `
initial-style: 0;
centering-scheme: english;
%%
${gabc}
`
 			
 			div.append(await this.handleCommand('raw-gabc', [newgabc]))
 			return div;
 		}

 		else if (cmd == 'score') {
 			let resp = await fetch(URL_BASE + args[0]);
 			if (!resp.ok) {
 				throw new Error(`Failed to fetch ${URL_BASE + args[0]}, status code: ${resp.status}.`)
 			}

 			let gabc = await resp.text();
 			return this.handleCommand('raw-gabc', [gabc]);
 		}

 		else if (cmd == 'yield') {
 			if (!this.base) {
 				throw new Error("Attempted to yield from main execution!");
 			}

 			this.base.setField('ctx:' + this.url, this);
 			console.log(this, 'YIELDS')
 			this.yielded = true;
 			return document.createElement('blank')
 		}

 		else if (cmd == 'resume') {
 			let div = document.createElement('div')
 			let ref = ('ctx:' + (URL_BASE + this.getField(args[0])))
 			console.log(ref, this.getField(ref))
 			console.log(Object.keys(this));
 			this.getField(ref).yielded = false;
 			let res = await this.getField(ref).execute();
 			console.log(res);
 			return div;
 		}

 		else if (cmd == 'begin-hymn') {
 			this.tolerate = true; // tolerate unknown commands
 			return document.createElement('blank'); // front guard is just to identify where the hymn begins
 		}

 		else if (cmd == 'end-hymn') {
 			this.tolerate = false;
 			// back guard will review the list of executed commands, find the front guard
 			// then send the intervening commands to a HymnContext for execution
 			let execs = this.executed.slice(0);
 			execs.push(['end-hymn'])

 			while (execs.length) {
 				let instr = execs.shift();
 				if (instr[0] != 'begin-hymn') {
 					continue;
 				}

 				break;
 			}

 			let htx = new HymnContext(execs, this);
 			let div = document.createElement('div');
 			console.log(htx);
 			(await htx.execute()).forEach(e => div.appendChild(e));
 			return div;
 		}


 		// unrecognized command control
 		if (!this.tolerate) {
 			let div = document.createElement('div');
 			div.className = 'error';
 			div.innerHTML = 'Unknown command: ' + cmd;
 			return div;
 		} else {
 			return document.createElement('blank')
 		}

 		//return document.createElement('blank')
 	}
 }

 class HymnContext extends LiturgyContext {
 	constructor(code, base) {
 		super(); // we don't need to load anything
 		this.code = code;
 		this.base = base;
 		this.verses = [];
 		this.combined = [];
 	}

 	async execute() {
 		let output = [];
 		while (this.code.length) {
 			let instr = this.code.shift();
 			
 			let command = instr[0];
 			let args = instr.slice(1);

 			output.push(await this.handleCommand(command, args));
 		}

 		return output;
 	}

 	async handleCommand(cmd, args) {
 		try {
 			return this.internal_handleCommand(cmd, args);
 		} catch (error) {
 			return this.handleError(error);
 		}
 	}

 	async internal_handleCommand(cmd, args) {
 		if (cmd == 'a') {
 			return 'a';
 		}

 		else if (cmd == 'clef') {
 			this.clef = args[0];
 			return document.createElement('blank')
 		}

 		else if (cmd == 'melody') {
 			if (!this.clef) {
 				throw new Error("Attempted to have hymn melody without a defined clef!")
 			}

 			this.melody = args;
 			return document.createElement('blank')
 		}

 		else if (cmd == 'verse') {
 			this.verses.push(args);
 			return document.createElement('blank')
 		}

 		else if (cmd == 'make') {
 			if (!this.clef) {
 				throw new Error("Cannot build without clef!");
 			}

 			if (this.verses.length == 0) {
 				throw new Error("Cannot build without verses!");
 			}

 			for (let v of this.verses) {
 				this.combined.push([v, this.melody]);
 			}
 			this.vlen = this.verses.length;
 			this.verses = [];

 			return document.createElement('blank'); // make is used for compiling to LaTeX, not now
 		}

 		else if (cmd == 'end-hymn') {
 			let gabc = 'initial-style: 0;\ncentering-scheme: english;\n%%\n'

 			for (let vi of Array(this.vlen).keys()) {
 				for (let i = 0; i < this.combined.length; i += this.vlen) {
 					let v = this.combined[vi + i];
 					let melody = v[1];
 					let verse = v[0];

 					for (let verse_idx in verse) {
 						let syllable = verse[verse_idx];
 						let notes = melody[verse_idx];
 						if (verse_idx == 0 && i==0) {
 							gabc += (vi + 1) + '. ' + (vi == 0 ? '' : ' (::)')
 						}

 						let continuous = syllable.endsWith('-');
 						gabc += (continuous ? syllable : syllable + ' ') + '(' + notes + ')';
 					}

 					if ((i + this.vlen) >= this.combined.length) {
 						continue;
 					}
 					
 					gabc += (i % 2 == 0) ? '(,)' : '(;)'
 				}
 			}

 			return this.base.handleCommand('raw-gabc', [gabc])
 		}

 		throw new Error("Unknown command: " + cmd);
 	}
 }