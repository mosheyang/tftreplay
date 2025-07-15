// ABOUTME: Type definitions for the extension host and plugin API
// ABOUTME: Defines the contract between the host and extensions

import { RecorderIPC } from './ipc';

export interface ExtensionContext {
    recorder: RecorderIPC;
    extensionPath: string;
    globalState: Map<string, any>;
    subscriptions: Disposable[];
}

export interface Disposable {
    dispose(): void;
}

export interface Extension {
    name: string;
    version: string;
    manifest: ExtensionManifest;
    exports: any;
    deactivate?: () => void | Promise<void>;
}

export interface ExtensionManifest {
    name: string;
    version?: string;
    displayName?: string;
    description?: string;
    main: string;
    activationEvents?: string[];
    contributes?: ExtensionContributions;
    engines?: {
        recorder?: string;
    };
    dependencies?: Record<string, string>;
}

export interface ExtensionContributions {
    commands?: Command[];
    configuration?: ConfigurationContribution;
    keybindings?: Keybinding[];
}

export interface Command {
    command: string;
    title: string;
    category?: string;
    icon?: string;
}

export interface ConfigurationContribution {
    title: string;
    properties: Record<string, ConfigurationProperty>;
}

export interface ConfigurationProperty {
    type: 'string' | 'number' | 'boolean' | 'array' | 'object';
    default?: any;
    description?: string;
    enum?: any[];
    minimum?: number;
    maximum?: number;
}

export interface Keybinding {
    command: string;
    key: string;
    when?: string;
    mac?: string;
    win?: string;
    linux?: string;
}