const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

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

const isWindows = os.platform() === 'win32';
console.log(`Detected platform: ${isWindows ? 'Windows' : 'Unix/Mac'}`);

console.log('Building for Lua...')
executeCommand('cargo build --release --features lua_support', {
    cwd: path.join(__dirname, 'backend')
});

console.log('Building Lua-compatible link');

if (isWindows) {
  const sourceFile = path.join(__dirname, 'backend', 'target', 'i686-pc-windows-msvc', 'release', 'divine_office.dll');
  const destFile = path.join(__dirname, 'latex', 'divine_office.dll');
  
  fs.copyFileSync(sourceFile, destFile);
  console.log('DLL file copied successfully');
  
  fs.copyFileSync(destFile, path.join(__dirname, 'latex', 'divine_office.so'));
  console.log('Added .so compatibility file');
} else {
  const sourceFile = path.join(__dirname, 'backend', 'target', 'release', 'libdivine_office.dylib');
  const destFile = path.join(__dirname, 'latex', 'libdivine_office.dylib');
  
  fs.copyFileSync(sourceFile, destFile);
  console.log('Dylib file copied successfully');
  
  executeCommand('ln -s ./libdivine_office.dylib ./divine_office.so', {
    cwd: path.join(__dirname, 'latex')
  });
  console.log('Symbolic link created');
}

const divineOfficePath = path.join(__dirname, 'frontend', 'wasm', 'divine_office.js');
removeImportMetaLine(divineOfficePath);