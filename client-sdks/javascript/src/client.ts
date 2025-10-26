import type {
  IsolateSandboxConfig,
  ExecuteRequest,
  ExecuteResponse,
  HealthResponse,
  LanguagesResponse,
  BoxFilesResponse,
  BoxFileResponse,
  CleanupResponse,
} from './types.js';
import { IsolateSandboxError } from './errors.js';

/**
 * Client for interacting with the IsolateSandbox API
 */
export class IsolateSandboxClient {
  private readonly baseUrl: string;
  private readonly timeout: number;

  /**
   * Creates a new IsolateSandbox client
   * @param config - Configuration options
   */
  constructor(config: IsolateSandboxConfig) {
    this.baseUrl = config.baseUrl.replace(/\/$/, ''); // Remove trailing slash
    this.timeout = config.timeout ?? 30000;
  }

  /**
   * Makes a fetch request with timeout and error handling
   */
  private async request<T>(
    path: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);

    try {
      const response = await fetch(url, {
        ...options,
        signal: controller.signal,
        headers: {
          'Content-Type': 'application/json',
          ...options.headers,
        },
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        throw await IsolateSandboxError.fromResponse(response);
      }

      return (await response.json()) as T;
    } catch (error) {
      clearTimeout(timeoutId);

      if (error instanceof IsolateSandboxError) {
        throw error;
      }

      if (error instanceof Error) {
        if (error.name === 'AbortError') {
          throw new IsolateSandboxError(
            `Request timeout after ${this.timeout}ms`,
            408
          );
        }
        throw new IsolateSandboxError(error.message, 0);
      }

      throw new IsolateSandboxError('Unknown error occurred', 0);
    }
  }

  /**
   * Check the health status of the service
   * @returns Health status
   * @throws {IsolateSandboxError} If the request fails
   */
  async health(): Promise<HealthResponse> {
    return this.request<HealthResponse>('/health', {
      method: 'GET',
    });
  }

  /**
   * List all supported programming languages
   * @returns List of supported languages
   * @throws {IsolateSandboxError} If the request fails
   */
  async listLanguages(): Promise<LanguagesResponse> {
    return this.request<LanguagesResponse>('/languages', {
      method: 'GET',
    });
  }

  /**
   * Execute code in a sandboxed environment
   * @param request - Code execution request
   * @returns Execution result with stdout, stderr, and metadata
   * @throws {IsolateSandboxError} If the request fails
   */
  async execute(request: ExecuteRequest): Promise<ExecuteResponse> {
    return this.request<ExecuteResponse>('/execute', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }

  /**
   * List files in a sandbox box
   * @param boxId - Box ID to list files from
   * @returns List of files in the box
   * @throws {IsolateSandboxError} If the request fails
   */
  async listBoxFiles(boxId: number): Promise<BoxFilesResponse> {
    return this.request<BoxFilesResponse>(`/boxes/${boxId}/files`, {
      method: 'GET',
    });
  }

  /**
   * Get a file from a sandbox box
   * @param boxId - Box ID to get file from
   * @param filename - Name of the file to retrieve
   * @returns File content (decoded from base64) and filename
   * @throws {IsolateSandboxError} If the request fails
   */
  async getBoxFile(
    boxId: number,
    filename: string
  ): Promise<{ content: string; filename: string }> {
    const response = await this.request<BoxFileResponse>(
      `/boxes/${boxId}/files/${encodeURIComponent(filename)}`,
      {
        method: 'GET',
      }
    );

    // Decode base64 content
    const decoded = this.decodeBase64(response.content);

    return {
      content: decoded,
      filename: response.filename,
    };
  }

  /**
   * Get a file from a sandbox box as raw base64
   * @param boxId - Box ID to get file from
   * @param filename - Name of the file to retrieve
   * @returns File content (base64 encoded) and filename
   * @throws {IsolateSandboxError} If the request fails
   */
  async getBoxFileRaw(boxId: number, filename: string): Promise<BoxFileResponse> {
    return this.request<BoxFileResponse>(
      `/boxes/${boxId}/files/${encodeURIComponent(filename)}`,
      {
        method: 'GET',
      }
    );
  }

  /**
   * Cleanup a sandbox box
   * @param boxId - Box ID to cleanup
   * @returns Cleanup confirmation message
   * @throws {IsolateSandboxError} If the request fails
   */
  async cleanupBox(boxId: number): Promise<CleanupResponse> {
    return this.request<CleanupResponse>(`/boxes/${boxId}`, {
      method: 'DELETE',
    });
  }

  /**
   * Decode base64 string to UTF-8
   * Works in both Node.js and browsers
   */
  private decodeBase64(base64: string): string {
    // Check if we're in Node.js or browser
    if (typeof Buffer !== 'undefined') {
      // Node.js
      return Buffer.from(base64, 'base64').toString('utf-8');
    } else {
      // Browser
      return atob(base64);
    }
  }
}

