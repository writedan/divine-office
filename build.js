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

const targetPath = path.join(__dirname, 'backend', 'target', 'wasm32-unknown-unknown', 'release', 'deps', 'divine_office.wasm');
if (fs.existsSync(targetPath)) {
    console.log('Deleting', targetPath);
    fs.rmSync(targetPath, { recursive: true, force: true });
} else {
    console.log("No cached build found");
}

executeCommand('wasm-pack build --target web --out-dir ../frontend/wasm', {
    cwd: path.join(__dirname, 'backend')
});

console.log('Building for Lua...')
executeCommand('cargo build --release --features lua_support', {
    cwd: path.join(__dirname, 'backend')
});

// NOTE we assume linux/mac
console.log('Building Lua-compatiable link');
fs.copyFileSync(path.join(__dirname, 'backend', 'target', 'release', 'libdivine_office.dylib'), path.join(__dirname, 'latex', 'libdivine_office.dylib'));
try {
    fs.rmSync(path.join(__dirname, 'latex', 'divine_office.so'));
    console.log('File deleted successfully.');
} catch (err) {
    if (err.code === 'ENOENT') {
        console.log('File does not exist, nothing to delete.');
    } else {
        console.error('Error deleting file:', err);
    }
}
executeCommand('ln -s ./libdivine_office.dylib ./divine_office.so', {
    cwd: path.join(__dirname, 'latex')
});

const divineOfficePath = path.join(__dirname, 'frontend', 'wasm', 'divine_office.js');
removeImportMetaLine(divineOfficePath);