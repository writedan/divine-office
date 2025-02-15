const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

function executeCommand(command, options = {}) {
    try {
        console.log(`Executing: ${command}`);
        execSync(command, { stdio: 'inherit', ...options });
    } catch (error) {
        console.error(`Error executing command: ${command}`);
        console.error(error);
        process.exit(1);
    }
}

function removeImportMetaLine(filePath) {
    try {
        console.log(`Removing import.meta from ${filePath}`);
        const content = fs.readFileSync(filePath, 'utf8');
        const updatedContent = content.split('\n')
            .filter(line => !line.includes('import.meta'))
            .join('\n');
        fs.writeFileSync(filePath, updatedContent);
    } catch (error) {
        console.error('Error modifying divine_office.js');
        console.error(error);
        process.exit(1);
    }
}

executeCommand('wasm-pack build --target web --out-dir ../frontend/wasm', {
    cwd: path.join(__dirname, 'backend')
});

const divineOfficePath = path.join(__dirname, 'frontend', 'wasm', 'divine_office.js');
removeImportMetaLine(divineOfficePath);