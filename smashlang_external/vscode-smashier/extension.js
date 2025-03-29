import * as vscode from 'vscode';
import { spawn } from 'child_process';
import * as path from 'path';
import * as fs from 'fs/promises';
import * as os from 'os';
import { existsSync } from 'fs';

// Decorations for live syntax highlighting
let decorationTypes = {};
let activeEditor = null;

/**
 * @param {vscode.ExtensionContext} context
 */
export async function activate(context) {
  console.log('SmashLang Formatter is now active');

  // Register the formatting provider
  const formatProvider = vscode.languages.registerDocumentFormattingEditProvider('smashlang', {
    async provideDocumentFormattingEdits(document) {
      return await formatDocument(document);
    }
  });

  // Register the format on type provider if enabled
  const config = vscode.workspace.getConfiguration('smashier');
  if (config.get('formatOnType')) {
    const formatOnTypeProvider = vscode.languages.registerDocumentRangeFormattingEditProvider('smashlang', {
      async provideDocumentRangeFormattingEdits(document, range) {
        const text = document.getText(range);
        const formatted = await formatWithSmashier(text);
        return [vscode.TextEdit.replace(range, formatted)];
      }
    });
    context.subscriptions.push(formatOnTypeProvider);
  }

  // Register the format command
  const formatCommand = vscode.commands.registerCommand('smashlang.format', async () => {
    const editor = vscode.window.activeTextEditor;
    if (editor) {
      const document = editor.document;
      if (document.languageId === 'smashlang') {
        await vscode.commands.executeCommand('editor.action.formatDocument');
      }
    }
  });

  // Set up format on save if enabled
  if (config.get('formatOnSave')) {
    vscode.workspace.onDidSaveTextDocument(async (document) => {
      if (document.languageId === 'smashlang') {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document === document) {
          await vscode.commands.executeCommand('editor.action.formatDocument');
        }
      }
    }, null, context.subscriptions);
  }

  // Set up live syntax highlighting
  setupLiveSyntaxHighlighting(context);

  // Listen for configuration changes
  vscode.workspace.onDidChangeConfiguration(e => {
    if (e.affectsConfiguration('smashier')) {
      // Refresh decorations when configuration changes
      if (activeEditor) {
        updateDecorations(activeEditor);
      }
    }
  }, null, context.subscriptions);

  context.subscriptions.push(formatProvider, formatCommand);
}

/**
 * Format a SmashLang document using smashier
 * @param {vscode.TextDocument} document - The document to format
 * @returns {Promise<vscode.TextEdit[]>} - The edits to apply
 */
async function formatDocument(document) {
  try {
    const text = document.getText();
    const formatted = await formatWithSmashier(text);
    
    const fullRange = new vscode.Range(
      document.positionAt(0),
      document.positionAt(text.length)
    );
    
    return [vscode.TextEdit.replace(fullRange, formatted)];
  } catch (error) {
    vscode.window.showErrorMessage(`Error formatting SmashLang: ${error.message}`);
    return [];
  }
}

/**
 * Format SmashLang code using the smashier package
 * @param {string} text - The code to format
 * @returns {Promise<string>} - The formatted code
 */
async function formatWithSmashier(text) {
  // Get configuration options
  const config = vscode.workspace.getConfiguration('smashier');
  const options = {
    useTabs: config.get('useTabs'),
    tabWidth: config.get('tabWidth'),
    printWidth: config.get('printWidth'),
    singleQuote: config.get('singleQuote'),
    semi: config.get('semi'),
    trailingComma: config.get('trailingComma')
  };

  // Create a temporary file for the input
  const tmpDir = path.join(os.tmpdir(), 'vscode-smashier');
  
  try {
    // Ensure the temporary directory exists
    await fs.mkdir(tmpDir, { recursive: true });
    
    const inputFile = path.join(tmpDir, 'input.smash');
    const outputFile = path.join(tmpDir, 'output.smash');
    const scriptFile = path.join(tmpDir, 'format.smash');
    
    // Prepare the formatting script without using regex in template literals
    const formatterScript = `
      import { format } from 'tools/smashier';
      import { readFileSync, writeFileSync } from 'std/fs';
      
      const inputFile = '${inputFile.split('\\').join('\\\\')}'; 
      const outputFile = '${outputFile.split('\\').join('\\\\')}'; 
      const options = ${JSON.stringify(options)};
      
      const code = readFileSync(inputFile, 'utf8');
      const formatted = format(code, options);
      writeFileSync(outputFile, formatted, 'utf8');
    `;
    
    // Write the input and script files
    await fs.writeFile(inputFile, text, 'utf8');
    await fs.writeFile(scriptFile, formatterScript, 'utf8');
    
    // Run the SmashLang interpreter with the formatting script
    const formatted = await runSmashLangProcess('smash', [scriptFile], outputFile);
    
    // Clean up temporary files
    await Promise.all([
      fs.unlink(inputFile).catch(() => {}),
      fs.unlink(outputFile).catch(() => {}),
      fs.unlink(scriptFile).catch(() => {})
    ]);
    
    return formatted;
  } catch (error) {
    throw new Error(`Failed to format SmashLang code: ${error.message}`);
  }
}

/**
 * Run the SmashLang process and return the formatted output
 * @param {string} command - The command to run
 * @param {string[]} args - The arguments to pass to the command
 * @param {string} outputFile - The file to read the output from
 * @returns {Promise<string>} - The formatted code
 */
async function runSmashLangProcess(command, args, outputFile) {
  return new Promise((resolve, reject) => {
    const process = spawn(command, args);
    let stderr = '';
    
    process.stderr.on('data', (data) => {
      stderr += data.toString();
    });
    
    process.on('close', async (code) => {
      if (code !== 0) {
        reject(new Error(`SmashLang process exited with code ${code}: ${stderr}`));
        return;
      }
      
      try {
        // Read the formatted output
        const formatted = await fs.readFile(outputFile, 'utf8');
        resolve(formatted);
      } catch (error) {
        reject(new Error(`Failed to read formatted output: ${error.message}`));
      }
    });
  });
}

/**
 * Set up live syntax highlighting for SmashLang files
 * @param {vscode.ExtensionContext} context - The extension context
 */
function setupLiveSyntaxHighlighting(context) {
  // Create decoration types for different token types
  createDecorationTypes();

  // Track the active editor
  activeEditor = vscode.window.activeTextEditor;
  if (activeEditor && activeEditor.document.languageId === 'smashlang') {
    updateDecorations(activeEditor);
  }

  // Update decorations when the active editor changes
  vscode.window.onDidChangeActiveTextEditor(editor => {
    activeEditor = editor;
    if (editor && editor.document.languageId === 'smashlang') {
      updateDecorations(editor);
    }
  }, null, context.subscriptions);

  // Update decorations when the document changes
  vscode.workspace.onDidChangeTextDocument(event => {
    if (activeEditor && event.document === activeEditor.document) {
      updateDecorations(activeEditor);
    }
  }, null, context.subscriptions);
}

/**
 * Create decoration types for different token types
 */
function createDecorationTypes() {
  const config = vscode.workspace.getConfiguration('smashier');
  const style = config.get('highlightStyle') || 'default';
  
  // Define colors for different token types based on the selected style
  const colors = {
    default: {
      keyword: '#569CD6',
      string: '#CE9178',
      number: '#B5CEA8',
      comment: '#6A9955',
      function: '#DCDCAA',
      variable: '#9CDCFE',
      operator: '#D4D4D4',
      type: '#4EC9B0'
    },
    light: {
      keyword: '#0000FF',
      string: '#A31515',
      number: '#098658',
      comment: '#008000',
      function: '#795E26',
      variable: '#001080',
      operator: '#000000',
      type: '#267F99'
    },
    monokai: {
      keyword: '#F92672',
      string: '#E6DB74',
      number: '#AE81FF',
      comment: '#75715E',
      function: '#A6E22E',
      variable: '#F8F8F2',
      operator: '#F8F8F2',
      type: '#66D9EF'
    },
    github: {
      keyword: '#D73A49',
      string: '#032F62',
      number: '#005CC5',
      comment: '#6A737D',
      function: '#6F42C1',
      variable: '#24292E',
      operator: '#24292E',
      type: '#22863A'
    }
  };

  // Create decoration types for each token type
  const selectedColors = colors[style] || colors.default;
  decorationTypes = {
    keyword: vscode.window.createTextEditorDecorationType({
      color: selectedColors.keyword
    }),
    string: vscode.window.createTextEditorDecorationType({
      color: selectedColors.string
    }),
    number: vscode.window.createTextEditorDecorationType({
      color: selectedColors.number
    }),
    comment: vscode.window.createTextEditorDecorationType({
      color: selectedColors.comment,
      fontStyle: 'italic'
    }),
    function: vscode.window.createTextEditorDecorationType({
      color: selectedColors.function
    }),
    variable: vscode.window.createTextEditorDecorationType({
      color: selectedColors.variable
    }),
    operator: vscode.window.createTextEditorDecorationType({
      color: selectedColors.operator
    }),
    type: vscode.window.createTextEditorDecorationType({
      color: selectedColors.type
    })
  };
}

/**
 * Update decorations in the active editor
 * @param {vscode.TextEditor} editor - The active text editor
 */
async function updateDecorations(editor) {
  if (!editor || editor.document.languageId !== 'smashlang') {
    return;
  }

  try {
    // Get the document text
    const text = editor.document.getText();
    
    // Get token information from the smashier package
    const tokenInfo = await getTokenInfo(text);
    
    // Apply decorations for each token type
    for (const [type, ranges] of Object.entries(tokenInfo)) {
      if (decorationTypes[type]) {
        const decorations = ranges.map(range => {
          const startPos = editor.document.positionAt(range.start);
          const endPos = editor.document.positionAt(range.end);
          return new vscode.Range(startPos, endPos);
        });
        editor.setDecorations(decorationTypes[type], decorations);
      }
    }
  } catch (error) {
    console.error('Error updating decorations:', error);
  }
}

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
    
    // Prepare the tokenization script without using regex in template literals
    const script = `
      import { tokenize } from 'tools/smashier';
      import { readFileSync, writeFileSync } from 'std/fs';
      
      const inputFile = '${inputFile.split('\\').join('\\\\')}'; 
      const outputFile = '${outputFile.split('\\').join('\\\\')}'; 
      
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

// This method is called when your extension is deactivated
export function deactivate() {
  // Clean up decoration types
  Object.values(decorationTypes).forEach(decorationType => {
    decorationType.dispose();
  });
  decorationTypes = {};
}
