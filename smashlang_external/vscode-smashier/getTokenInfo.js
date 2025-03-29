/**
 * Get token information from the smashier package
 * @param {string} text - The code to tokenize
 * @returns {Promise<Object>} - Token information with ranges for each token type
 */
async function getTokenInfo(text) {
  try {
    // Create a temporary file for the input
    const tmpDir = path.join(os.tmpdir(), 'vscode-smashier');
    await fs.mkdir(tmpDir, { recursive: true });
    
    const inputFile = path.join(tmpDir, 'input.smash');
    const outputFile = path.join(tmpDir, 'tokens.json');
    const scriptFile = path.join(tmpDir, 'tokenize.smash');
    
    // Prepare the tokenization script
    const script = `
      import { tokenize } from 'tools/smashier';
      import { readFileSync, writeFileSync } from 'std/fs';
      
      const inputFile = '${inputFile.replace(/\\/g, '\\\\')}';
      const outputFile = '${outputFile.replace(/\\/g, '\\\\')}';
      
      const code = readFileSync(inputFile, 'utf8');
      const tokens = tokenize(code);
      
      // Convert tokens to ranges by token type
      const tokenRanges = {};
      
      tokens.forEach(token => {
        if (!tokenRanges[token.type]) {
          tokenRanges[token.type] = [];
        }
        tokenRanges[token.type].push({
          start: token.start,
          end: token.end
        });
      });
      
      writeFileSync(outputFile, JSON.stringify(tokenRanges), 'utf8');
    `;
    
    // Write the input and script files
    await fs.writeFile(inputFile, text, 'utf8');
    await fs.writeFile(scriptFile, script, 'utf8');
    
    // Run the SmashLang interpreter with the tokenization script
    await new Promise((resolve, reject) => {
      const process = spawn('smash', [scriptFile]);
      let stderr = '';
      
      process.stderr.on('data', (data) => {
        stderr += data.toString();
      });
      
      process.on('close', (code) => {
        if (code !== 0) {
          reject(new Error(`SmashLang process exited with code ${code}: ${stderr}`));
        } else {
          resolve();
        }
      });
    });
    
    // Read the token information
    const tokenData = await fs.readFile(outputFile, 'utf8');
    const tokenInfo = JSON.parse(tokenData);
    
    // Clean up temporary files
    await Promise.all([
      fs.unlink(inputFile).catch(() => {}),
      fs.unlink(outputFile).catch(() => {}),
      fs.unlink(scriptFile).catch(() => {})
    ]);
    
    return tokenInfo;
  } catch (error) {
    console.error('Error getting token information:', error);
    return {};
  }
}
