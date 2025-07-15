// ABOUTME: Public API type definitions for extension authors
// ABOUTME: Provides TypeScript support for developing TFT recorder extensions

declare module '@tft-recorder/sdk' {
    export interface ExtensionContext {
        /**
         * Access to the recorder API
         */
        recorder: RecorderAPI;
        
        /**
         * Path where the extension is installed
         */
        extensionPath: string;
        
        /**
         * Global state storage
         */
        globalState: GlobalState;
        
        /**
         * Array to track disposables
         */
        subscriptions: Disposable[];
    }
    
    export interface RecorderAPI {
        /**
         * Start recording with specified options
         */
        startRecording(options?: RecordingOptions): Promise<boolean>;
        
        /**
         * Stop the current recording
         */
        stopRecording(): Promise<string | undefined>;
        
        /**
         * Get current recording status
         */
        getStatus(): Promise<RecordingStatus>;
        
        /**
         * Subscribe to recording events
         */
        onDidChangeRecordingState(callback: (event: RecordingStateEvent) => void): Disposable;
        
        /**
         * Subscribe to frame events during recording
         */
        onDidCaptureFrame(callback: (event: FrameEvent) => void): Disposable;
    }
    
    export interface RecordingOptions {
        windowTitle?: string;
        width?: number;
        height?: number;
        bitrate?: number;
        outputPath?: string;
    }
    
    export interface RecordingStatus {
        isRecording: boolean;
        currentRecordingId?: string;
        duration?: number;
        frameCount?: number;
        fileSize?: number;
    }
    
    export interface RecordingStateEvent {
        type: 'started' | 'stopped' | 'paused' | 'resumed';
        recordingId: string;
        timestamp: number;
    }
    
    export interface FrameEvent {
        recordingId: string;
        frameNumber: number;
        timestamp: number;
        pts: number;
    }
    
    export interface GlobalState {
        get<T>(key: string): T | undefined;
        set<T>(key: string, value: T): void;
        delete(key: string): void;
        keys(): string[];
    }
    
    export interface Disposable {
        dispose(): void;
    }
    
    /**
     * Commands API for registering extension commands
     */
    export namespace commands {
        export function registerCommand(
            command: string,
            callback: (...args: any[]) => any
        ): Disposable;
        
        export function executeCommand(command: string, ...args: any[]): Promise<any>;
        
        export function getCommands(): Promise<string[]>;
    }
    
    /**
     * Window API for UI interactions
     */
    export namespace window {
        export function showInformationMessage(message: string): void;
        export function showWarningMessage(message: string): void;
        export function showErrorMessage(message: string): void;
        
        export function showInputBox(options?: InputBoxOptions): Promise<string | undefined>;
        
        export function showQuickPick<T extends QuickPickItem>(
            items: T[],
            options?: QuickPickOptions
        ): Promise<T | undefined>;
    }
    
    export interface InputBoxOptions {
        prompt?: string;
        placeHolder?: string;
        value?: string;
        password?: boolean;
        validateInput?(value: string): string | null;
    }
    
    export interface QuickPickOptions {
        placeHolder?: string;
        canPickMany?: boolean;
        matchOnDescription?: boolean;
        matchOnDetail?: boolean;
    }
    
    export interface QuickPickItem {
        label: string;
        description?: string;
        detail?: string;
        picked?: boolean;
    }
    
    /**
     * Workspace API for file system operations
     */
    export namespace workspace {
        export function readFile(path: string): Promise<Buffer>;
        export function writeFile(path: string, content: Buffer | string): Promise<void>;
        export function deleteFile(path: string): Promise<void>;
        export function createDirectory(path: string): Promise<void>;
        export function listFiles(path: string): Promise<string[]>;
        
        export function getConfiguration(section?: string): Configuration;
    }
    
    export interface Configuration {
        get<T>(key: string): T | undefined;
        get<T>(key: string, defaultValue: T): T;
        has(key: string): boolean;
        update(key: string, value: any): Promise<void>;
    }
}