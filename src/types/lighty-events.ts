export enum LightyEvent {
  DownloadProgress = 'lighty://download-progress',
  LaunchStatus = 'lighty://launch-status',
  ConsoleOutput = 'lighty://console-output',
  Error = 'lighty://error',
}

export interface LaunchStatusPayload {
  status: LaunchStatus,
  phase: string,
  instance_name: string,
  pid: number,
}

export enum LaunchStatus {
  Running = "running",
  Launched = "launched",
  Exited = "exited",
  Installing = "installing",
  Downloading = "downloading",
  Failed = "failed",
}

export interface DownloadProgressPayload {
  phase: DownloadPhase,
  current_bytes: number,
  total_bytes: number,
  percentage: number,
  message: string,
  instance_name: string,
}

export enum DownloadPhase {
  Java = "java",
  Extracting = "extracting",
  Game = "game",
}

export enum StdStream {
  Stdout = "stdout",
  Stderr = "stderr",
}

export interface ConsoleLinePayload {
  instance_name: string,
  pid: number,
  stream: StdStream,
  line: string,
  timestamp: number,
}

export interface ErrorPayload {
  category: string,
  message: string,
  details?: string,
  timestamp: number,
}
