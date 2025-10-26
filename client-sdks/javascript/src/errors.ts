import type { ErrorResponse } from './types.js';

/**
 * Custom error class for IsolateSandbox API errors
 */
export class IsolateSandboxError extends Error {
  /**
   * HTTP status code of the error response
   */
  public readonly statusCode: number;

  /**
   * Raw error response from the API
   */
  public readonly response?: ErrorResponse;

  constructor(message: string, statusCode: number, response?: ErrorResponse) {
    super(message);
    this.name = 'IsolateSandboxError';
    this.statusCode = statusCode;
    this.response = response;

    // Maintains proper stack trace for where our error was thrown (only available on V8)
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, IsolateSandboxError);
    }
  }

  /**
   * Creates an error from a fetch response
   */
  static async fromResponse(response: Response): Promise<IsolateSandboxError> {
    let errorData: ErrorResponse | undefined;
    let message = `HTTP ${response.status}: ${response.statusText}`;

    try {
      const contentType = response.headers.get('content-type');
      if (contentType?.includes('application/json')) {
        errorData = await response.json() as ErrorResponse;
        if (errorData?.error) {
          message = errorData.error;
        }
      } else {
        const text = await response.text();
        if (text) {
          message = text;
        }
      }
    } catch {
      // If we can't parse the error, use the default message
    }

    return new IsolateSandboxError(message, response.status, errorData);
  }
}

