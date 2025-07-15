// ABOUTME: Extension loader that discovers and activates plugins
// ABOUTME: Implements VS Code-style extension discovery and lifecycle management

import * as path from 'path';
import * as fs from 'fs/promises';
import { glob } from 'glob';
import { ExtensionContext, Extension, ExtensionManifest } from './types';

export class ExtensionLoader {
    async loadExtensions(context: ExtensionContext): Promise<Extension[]> {
        const extensions: Extension[] = [];
        
        // Ensure extensions directory exists
        try {
            await fs.mkdir(context.extensionPath, { recursive: true });
        } catch (err) {
            console.error('Failed to create extensions directory:', err);
        }
        
        // Find all extension directories
        const pattern = path.join(context.extensionPath, '*/package.json');
        const manifestPaths = await glob(pattern);
        
        for (const manifestPath of manifestPaths) {
            try {
                const extension = await this.loadExtension(manifestPath, context);
                if (extension) {
                    extensions.push(extension);
                }
            } catch (err) {
                console.error(`Failed to load extension from ${manifestPath}:`, err);
            }
        }
        
        return extensions;
    }
    
    private async loadExtension(
        manifestPath: string,
        context: ExtensionContext
    ): Promise<Extension | null> {
        // Read manifest
        const manifestData = await fs.readFile(manifestPath, 'utf-8');
        const manifest: ExtensionManifest = JSON.parse(manifestData);
        
        // Validate manifest
        if (!manifest.name || !manifest.main) {
            console.error(`Invalid manifest at ${manifestPath}`);
            return null;
        }
        
        // Load extension module
        const extensionDir = path.dirname(manifestPath);
        const mainPath = path.join(extensionDir, manifest.main);
        
        try {
            const module = require(mainPath);
            
            // Create extension object
            const extension: Extension = {
                name: manifest.name,
                version: manifest.version || '0.0.0',
                manifest,
                exports: module,
            };
            
            // Check activation events
            if (this.shouldActivate(manifest, context)) {
                // Activate extension
                if (typeof module.activate === 'function') {
                    console.log(`Activating extension: ${manifest.name}`);
                    const api = await module.activate(context);
                    extension.exports = api || module;
                }
                
                // Store deactivate function if present
                if (typeof module.deactivate === 'function') {
                    extension.deactivate = module.deactivate;
                }
            }
            
            return extension;
        } catch (err) {
            console.error(`Failed to load extension module ${mainPath}:`, err);
            return null;
        }
    }
    
    private shouldActivate(manifest: ExtensionManifest, context: ExtensionContext): boolean {
        // Always activate if no activation events specified
        if (!manifest.activationEvents || manifest.activationEvents.length === 0) {
            return true;
        }
        
        // Check activation events
        for (const event of manifest.activationEvents) {
            if (event === '*' || event === 'onStartup') {
                return true;
            }
            
            // TODO: Implement other activation events like onCommand, onRecordingStart, etc.
        }
        
        return false;
    }
}