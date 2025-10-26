/**
 * Configuration options for the IsolateSandbox client
 */
export interface IsolateSandboxConfig {
  /**
   * Base URL of the isolate-sandbox API
   * @example "http://localhost:3000"
   */
  baseUrl: string;

  /**
   * Optional timeout for requests in milliseconds
   * @default 30000
   */
  timeout?: number;

  /**
   * Optional API key for authentication
   * If provided, will be sent as X-API-Key header
   */
  apiKey?: string;
}

/**
 * Request payload for code execution
 */
export interface ExecuteRequest {
  /**
   * Programming language to execute (e.g., "python", "rust", "javascript")
   */
  language: string;

  /**
   * Source code to execute
   */
  code: string;
}

/**
 * Execution metadata returned from the sandbox
 */
export interface MetadataResponse {
  /**
   * Execution time in seconds
   */
  time: number;

  /**
   * Wall clock time in seconds
   */
  time_wall: number;

  /**
   * Memory usage in bytes
   */
  memory: number;

  /**
   * Exit code of the process
   */
  exit_code: number;

  /**
   * Execution status
   */
  status: string;
}

/**
 * Response from code execution
 */
export interface ExecuteResponse {
  /**
   * Standard output from the execution
   */
  stdout: string;

  /**
   * Standard error from the execution
   */
  stderr: string;

  /**
   * Execution metadata
   */
  metadata: MetadataResponse;

  /**
   * Box ID used for execution
   */
  box_id: number;
}

/**
 * Health check response
 */
export interface HealthResponse {
  /**
   * Health status ("ok" or "error")
   */
  status: string;
}

/**
 * Response containing list of supported languages
 */
export interface LanguagesResponse {
  /**
   * List of supported programming languages
   */
  languages: string[];
}

/**
 * Response containing list of files in a sandbox box
 */
export interface BoxFilesResponse {
  /**
   * List of files in the box
   */
  files: string[];
}

/**
 * Response containing file content from a sandbox box
 */
export interface BoxFileResponse {
  /**
   * File content (base64 encoded)
   */
  content: string;

  /**
   * Filename
   */
  filename: string;
}

/**
 * Response from box cleanup operation
 */
export interface CleanupResponse {
  /**
   * Success message
   */
  message: string;
}

/**
 * Error response from the API
 */
export interface ErrorResponse {
  /**
   * Error message
   */
  error: string;
}

