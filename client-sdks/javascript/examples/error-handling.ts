/**
 * Error Handling Example
 * 
 * This example demonstrates proper error handling when using the SDK.
 */

import { IsolateSandboxClient, IsolateSandboxError } from '../src/index';

async function main() {
  const client = new IsolateSandboxClient({
    baseUrl: 'http://localhost:3000',
    timeout: 5000, // Short timeout for demonstration
    // Optional: Uncomment if your API requires authentication
    // apiKey: process.env.ISOLATE_SANDBOX_API_KEY,
  });

  console.log('=== IsolateSandbox Error Handling Example ===\n');

  // Example 1: Invalid language
  console.log('1. Testing invalid language error...');
  try {
    await client.execute({
      language: 'invalid-language',
      code: 'print("test")',
    });
  } catch (error) {
    if (error instanceof IsolateSandboxError) {
      console.log(`   ✓ Caught IsolateSandboxError`);
      console.log(`   Message: ${error.message}`);
      console.log(`   Status Code: ${error.statusCode}`);
      if (error.response) {
        console.log(`   API Error: ${error.response.error}`);
      }
    } else {
      console.log(`   ✗ Unexpected error type:`, error);
    }
  }
  console.log();

  // Example 2: Invalid box ID
  console.log('2. Testing invalid box ID error...');
  try {
    await client.listBoxFiles(99999);
  } catch (error) {
    if (error instanceof IsolateSandboxError) {
      console.log(`   ✓ Caught IsolateSandboxError`);
      console.log(`   Message: ${error.message}`);
      console.log(`   Status Code: ${error.statusCode}`);
    } else {
      console.log(`   ✗ Unexpected error type:`, error);
    }
  }
  console.log();

  // Example 3: File not found
  console.log('3. Testing file not found error...');
  try {
    // First create a box
    const result = await client.execute({
      language: 'python',
      code: 'print("test")',
    });

    // Then try to get a non-existent file
    await client.getBoxFile(result.box_id, 'nonexistent.txt');

    // Cleanup
    await client.cleanupBox(result.box_id);
  } catch (error) {
    if (error instanceof IsolateSandboxError) {
      console.log(`   ✓ Caught IsolateSandboxError`);
      console.log(`   Message: ${error.message}`);
      console.log(`   Status Code: ${error.statusCode}`);
    } else {
      console.log(`   ✗ Unexpected error type:`, error);
    }
  }
  console.log();

  // Example 4: Authentication errors (if API key is required)
  console.log('4. Testing authentication errors...');
  
  // 4a. Missing API key
  const noAuthClient = new IsolateSandboxClient({
    baseUrl: 'http://localhost:3000',
    // No apiKey provided
  });
  
  try {
    await noAuthClient.listLanguages();
    console.log(`   Note: No authentication required on this server`);
  } catch (error) {
    if (error instanceof IsolateSandboxError) {
      if (error.statusCode === 403) {
        console.log(`   ✓ Caught 403 Forbidden (missing X-API-Key header)`);
        console.log(`   Message: ${error.message}`);
      } else {
        console.log(`   Status Code: ${error.statusCode}, Message: ${error.message}`);
      }
    }
  }
  
  // 4b. Invalid API key
  const badAuthClient = new IsolateSandboxClient({
    baseUrl: 'http://localhost:3000',
    apiKey: 'invalid-api-key',
  });
  
  try {
    await badAuthClient.listLanguages();
    console.log(`   Note: No authentication required on this server`);
  } catch (error) {
    if (error instanceof IsolateSandboxError) {
      if (error.statusCode === 401) {
        console.log(`   ✓ Caught 401 Unauthorized (invalid API key)`);
        console.log(`   Message: ${error.message}`);
      } else {
        console.log(`   Status Code: ${error.statusCode}, Message: ${error.message}`);
      }
    }
  }
  console.log();

  // Example 5: Connection error (wrong URL)
  console.log('5. Testing connection error...');
  const badClient = new IsolateSandboxClient({
    baseUrl: 'http://localhost:9999', // Wrong port
    timeout: 2000,
  });

  try {
    await badClient.health();
  } catch (error) {
    if (error instanceof IsolateSandboxError) {
      console.log(`   ✓ Caught IsolateSandboxError`);
      console.log(`   Message: ${error.message}`);
      console.log(`   Status Code: ${error.statusCode}`);
    } else {
      console.log(`   ✗ Unexpected error type:`, error);
    }
  }
  console.log();

  // Example 6: Proper error handling pattern
  console.log('6. Demonstrating proper error handling pattern...');
  
  async function safeExecute(language: string, code: string) {
    try {
      const result = await client.execute({ language, code });
      console.log(`   ✓ Execution successful`);
      console.log(`   Exit code: ${result.metadata.exit_code}`);
      return result;
    } catch (error) {
      if (error instanceof IsolateSandboxError) {
        // Handle API errors
        console.log(`   ✗ API Error: ${error.message}`);
        
        // Different handling based on status code
        if (error.statusCode === 400) {
          console.log(`   → Bad request, check your input`);
        } else if (error.statusCode === 401) {
          console.log(`   → Unauthorized, check your API key`);
        } else if (error.statusCode === 403) {
          console.log(`   → Forbidden, API key required`);
        } else if (error.statusCode === 408) {
          console.log(`   → Request timeout, increase timeout or simplify code`);
        } else if (error.statusCode === 500) {
          console.log(`   → Server error, try again later`);
        }
      } else {
        // Handle unexpected errors
        console.log(`   ✗ Unexpected error:`, error);
      }
      return null;
    }
  }

  await safeExecute('python', 'print("Hello, World!")');
  console.log();

  console.log('=== Example completed successfully ===');
}

// Run the example
main().catch(console.error);

