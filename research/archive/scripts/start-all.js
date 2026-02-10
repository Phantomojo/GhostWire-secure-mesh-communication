#!/usr/bin/env node
console.log('Script entry');
try {
  console.log('Node.js version:', process.versions.node);
  console.log('Process argv:', process.argv);
  const { spawn } = require('child_process');
  const getPort = require('get-port');
  const fs = require('fs');
  const path = require('path');

  console.log('Starting GhostWire dynamic launcher...');

  (async () => {
    try {
      // 1. Find free ports
      console.log('Finding free ports...');
      let BACKEND_PORT, FRONTEND_PORT;
      try {
        BACKEND_PORT = await getPort({ port: getPort.makeRange(3001, 3999) });
        FRONTEND_PORT = await getPort({ port: getPort.makeRange(5173, 5999) });
        console.log(`Selected ports: backend=${BACKEND_PORT}, frontend=${FRONTEND_PORT}`);
      } catch (err) {
        console.error('Error finding ports:', err);
        throw err;
      }

      // 2. Update frontend config (webui/src/services/api.ts)
      const apiPath = path.join(__dirname, 'webui', 'src', 'services', 'api.ts');
      console.log('Checking for frontend API config at', apiPath);
      if (!fs.existsSync(apiPath)) {
        console.error('api.ts file does not exist at', apiPath);
        throw new Error('api.ts file not found');
      }
      console.log('Reading frontend API config...');
      let apiContent;
      try {
        apiContent = fs.readFileSync(apiPath, 'utf8');
        console.log('Successfully read api.ts');
      } catch (err) {
        console.error('Error reading api.ts:', err);
        throw err;
      }
      console.log('Updating API_BASE_URL in api.ts...');
      apiContent = apiContent.replace(/const API_BASE_URL = .+?;/, `const API_BASE_URL = 'http://localhost:${BACKEND_PORT}/api';`);
      try {
        fs.writeFileSync(apiPath, apiContent, 'utf8');
        console.log('Successfully wrote api.ts');
      } catch (err) {
        console.error('Error writing api.ts:', err);
        throw err;
      }

      // 3. Start backend
      console.log('Starting backend...');
      let backend;
      try {
        backend = spawn('cargo', ['run', '--', '--host', '0.0.0.0', '--port', BACKEND_PORT], {
          cwd: path.join(__dirname, 'ghostwire'),
          stdio: 'inherit',
          shell: true,
        });
        console.log('Backend started.');
      } catch (err) {
        console.error('Error starting backend:', err);
        throw err;
      }

      // 4. Start frontend
      console.log('Starting frontend...');
      let frontend;
      try {
        frontend = spawn('npm', ['run', 'dev', '--', '--port', FRONTEND_PORT], {
          cwd: path.join(__dirname, 'webui'),
          stdio: 'inherit',
          shell: true,
        });
        console.log('Frontend started.');
      } catch (err) {
        console.error('Error starting frontend:', err);
        throw err;
      }

      // 5. Print info
      console.log('\n==============================');
      console.log('✓ GhostWire started successfully!');
      console.log(`Backend API:   http://localhost:${BACKEND_PORT}/api`);
      console.log(`Frontend UI:   http://localhost:${FRONTEND_PORT}`);
      console.log('==============================\n');

      // 6. Handle exit
      process.on('SIGINT', () => {
        backend.kill('SIGINT');
        frontend.kill('SIGINT');
        process.exit();
      });

      console.log('All steps completed.');
    } catch (err) {
      console.error('Error in start-all.js:', err && err.stack ? err.stack : err);
      process.exit(1);
    }
  })();
} catch (err) {
  console.error('Top-level error in start-all.js:', err && err.stack ? err.stack : err);
  process.exit(1);
} 