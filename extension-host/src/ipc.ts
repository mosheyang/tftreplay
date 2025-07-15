// ABOUTME: gRPC client for communicating with the recorder daemon
// ABOUTME: Handles recording control and event streaming over Unix sockets

import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import { EventEmitter } from 'events';
import * as path from 'path';

interface RecorderService {
    StartRecording(request: StartRecordingRequest): Promise<StartRecordingResponse>;
    StopRecording(request: StopRecordingRequest): Promise<StopRecordingResponse>;
    GetStatus(request: GetStatusRequest): Promise<GetStatusResponse>;
    StreamEvents(request: StreamEventsRequest): grpc.ClientReadableStream<RecordingEvent>;
}

interface StartRecordingRequest {
    windowTitle: string;
    width: number;
    height: number;
    bitrate: number;
    outputPath: string;
}

interface StartRecordingResponse {
    success: boolean;
    error?: string;
}

interface StopRecordingRequest {
    recordingId: string;
}

interface StopRecordingResponse {
    success: boolean;
    filePath?: string;
    error?: string;
}

interface GetStatusRequest {}

interface GetStatusResponse {
    isRecording: boolean;
    currentRecordingId?: string;
    duration?: number;
}

interface StreamEventsRequest {}

interface RecordingEvent {
    type: 'started' | 'stopped' | 'error' | 'frame';
    recordingId?: string;
    timestamp: number;
    data?: any;
}

export class RecorderIPC extends EventEmitter {
    private client?: any; // gRPC client
    private eventStream?: grpc.ClientReadableStream<RecordingEvent>;
    private socketPath: string;
    
    constructor(port: number = 0) {
        super();
        this.socketPath = port === 0 
            ? '/tmp/tft-recorder.sock' 
            : `localhost:${port}`;
    }
    
    async connect(): Promise<void> {
        // In a real implementation, we'd load the proto file
        // For now, we'll create a mock connection
        console.log(`Connecting to recorder at ${this.socketPath}`);
        
        // Simulate connection
        this.client = {
            StartRecording: async (req: StartRecordingRequest) => ({ success: true }),
            StopRecording: async (req: StopRecordingRequest) => ({ success: true }),
            GetStatus: async (req: GetStatusRequest) => ({ isRecording: false }),
        };
        
        this.emit('connected');
    }
    
    async disconnect(): Promise<void> {
        if (this.eventStream) {
            this.eventStream.cancel();
        }
        this.client = undefined;
        this.emit('disconnected');
    }
    
    async startRecording(options: {
        windowTitle?: string;
        width?: number;
        height?: number;
        bitrate?: number;
        outputPath?: string;
    }): Promise<boolean> {
        if (!this.client) {
            throw new Error('Not connected to recorder');
        }
        
        const request: StartRecordingRequest = {
            windowTitle: options.windowTitle || 'Teamfight Tactics',
            width: options.width || 1280,
            height: options.height || 720,
            bitrate: options.bitrate || 4000000,
            outputPath: options.outputPath || 'recording.mp4',
        };
        
        const response = await this.client.StartRecording(request);
        return response.success;
    }
    
    async stopRecording(): Promise<string | undefined> {
        if (!this.client) {
            throw new Error('Not connected to recorder');
        }
        
        const status = await this.getStatus();
        if (!status.isRecording || !status.currentRecordingId) {
            throw new Error('No active recording');
        }
        
        const response = await this.client.StopRecording({
            recordingId: status.currentRecordingId,
        });
        
        return response.filePath;
    }
    
    async getStatus(): Promise<GetStatusResponse> {
        if (!this.client) {
            throw new Error('Not connected to recorder');
        }
        
        return await this.client.GetStatus({});
    }
    
    subscribeToEvents(callback: (event: RecordingEvent) => void): void {
        this.on('recording-event', callback);
        
        // In a real implementation, we'd start the event stream here
        // For now, we'll simulate some events
        setTimeout(() => {
            this.emit('recording-event', {
                type: 'started',
                recordingId: 'test-123',
                timestamp: Date.now(),
            });
        }, 1000);
    }
}