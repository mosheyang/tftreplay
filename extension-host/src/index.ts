#!/usr/bin/env node
// ABOUTME: Extension host entry point that manages plugin lifecycle and IPC
// ABOUTME: Provides VS Code-style extensibility for the TFT recorder

import { ExtensionLoader } from './loader';
import { RecorderIPC } from './ipc';
import { ExtensionContext } from './types';
import * as path from 'path';
import * as os from 'os';

async function main() {
    console.log('TFT Recorder Extension Host starting...');
    
    // Parse command line arguments
    const args = process.argv.slice(2);
    const portIndex = args.indexOf('--port');
    const port = portIndex !== -1 ? parseInt(args[portIndex + 1]) : 0;
    
    // Initialize IPC connection
    const recorder = new RecorderIPC(port);
    await recorder.connect();
    
    // Create extension context
    const context: ExtensionContext = {
        recorder,
        extensionPath: path.join(os.homedir(), '.tft-recorder', 'extensions'),
        globalState: new Map(),
        subscriptions: [],
    };
    
    // Load extensions
    const loader = new ExtensionLoader();
    const extensions = await loader.loadExtensions(context);
    
    console.log(`Loaded ${extensions.length} extensions`);
    
    // Handle shutdown
    process.on('SIGINT', async () => {
        console.log('Shutting down extension host...');
        
        // Deactivate extensions
        for (const ext of extensions) {
            if (ext.deactivate) {
                try {
                    await ext.deactivate();
                } catch (err) {
                    console.error(`Error deactivating extension ${ext.name}:`, err);
                }
            }
        }
        
        // Clean up subscriptions
        for (const disposable of context.subscriptions) {
            disposable.dispose();
        }
        
        await recorder.disconnect();
        process.exit(0);
    });
    
    // Keep process alive
    setInterval(() => {}, 1000);
}

main().catch(err => {
    console.error('Extension host failed:', err);
    process.exit(1);
});