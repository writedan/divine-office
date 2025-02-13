const fs = require('fs');
const path = require('path');

const webBuildDir = path.join(__dirname, 'web-build');
const indexFile = path.join(webBuildDir, 'index.html');
const targetDir = path.join(webBuildDir, '_expo', 'static', 'js', 'web');
const stringToReplace = '"/assets/';
const replacementString = 'APP_ROOT+"assets/';
const directoriesToKeep = [
    path.join(webBuildDir, '_expo'),
    path.join(webBuildDir, 'assets')
];

fs.readFile(indexFile, 'utf8', (err, data) => {
    if (err) {
        console.error('Error reading index.html:', err);
        return;
    }

    const scriptTagToRemove = /<script src="\/_expo\/static\/js\/web\/entry-[^"]+" defer><\/script>/g;
    const modifiedData = data.replace(scriptTagToRemove, '');

    fs.readdir(targetDir, (err, files) => {
        if (err) {
            console.error('Error reading directory:', err);
            return;
        }

        const targetFile = files.find(file => file.startsWith('entry-') && file.endsWith('.js'));
        
        if (!targetFile) {
            console.error('No target file found that starts with "entry-" and ends with ".js".');
            return;
        }

        const targetFilePath = path.join(targetDir, targetFile);

        const newScriptTagContent = `
<script defer>
// A fix around routing problems in default React Native Expo exports
APP_ROOT = window.location.pathname;
console.log('APP_ROOT', APP_ROOT);

const script = document.createElement('script');
script.src = APP_ROOT + '_expo/static/js/web/${targetFile}';
document.head.appendChild(script);
</script>`;

        const finalData = modifiedData.replace('</head>', `${newScriptTagContent}\n</head>`);

        fs.writeFile(indexFile, finalData, (err) => {
            if (err) {
                console.error('Error writing index.html:', err);
                return;
            }
            console.log('index.html updated successfully.');
        });
    });
});

fs.readdir(targetDir, (err, files) => {
    if (err) {
        console.error('Error reading directory:', err);
        return;
    }

    files.forEach(file => {
        if (file.startsWith('entry-') && file.endsWith('.js')) {
            const targetFilePath = path.join(targetDir, file);

            fs.readFile(targetFilePath, 'utf8', (err, data) => {
                if (err) {
                    console.error(`Error reading file ${file}:`, err);
                    return;
                }

                const modifiedData = data.replace(new RegExp(stringToReplace, 'g'), replacementString);

                fs.writeFile(targetFilePath, modifiedData, (err) => {
                    if (err) {
                        console.error(`Error writing file ${file}:`, err);
                        return;
                    }
                    console.log(`File ${file} updated successfully.`);
                });
            });
        }
    });
});

function deleteFilesAndDirs(dir) {
    fs.readdir(dir, (err, files) => {
        if (err) {
            console.error(`Error reading directory ${dir}:`, err);
            return;
        }

        files.forEach(file => {
            const filePath = path.join(dir, file);
            fs.stat(filePath, (err, stats) => {
                if (err) {
                    console.error(`Error getting stats for ${filePath}:`, err);
                    return;
                }

                if (stats.isDirectory()) {
                    if (!directoriesToKeep.includes(filePath)) {
                        deleteFilesAndDirs(filePath);
                    }
                } else if (stats.isFile()) {
                    if (filePath !== indexFile &&
                        !filePath.startsWith(path.join(webBuildDir, '_expo')) &&
                        !filePath.startsWith(path.join(webBuildDir, 'assets'))) {
                        fs.unlink(filePath, (err) => {
                            if (err) {
                                console.error(`Error deleting file ${filePath}:`, err);
                                return;
                            }
                            console.log(`Deleted file: ${filePath}`);
                        });
                    }
                }
            });
        });
    });
}

deleteFilesAndDirs(webBuildDir);