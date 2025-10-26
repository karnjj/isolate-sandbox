/**
 * Basic Usage Example
 * 
 * This example demonstrates basic code execution using the IsolateSandbox SDK.
 */

import { IsolateSandboxClient } from '../src/index';

async function main() {
  // Create a client instance
  // If your API requires authentication, provide an API key:
  // apiKey: 'your-api-key-here' or process.env.ISOLATE_SANDBOX_API_KEY
  const client = new IsolateSandboxClient({
    baseUrl: 'http://localhost:3000',
    timeout: 30000,
    // Optional: Uncomment the line below if your API requires authentication
    // apiKey: process.env.ISOLATE_SANDBOX_API_KEY,
  });

  console.log('=== IsolateSandbox Basic Usage Example ===\n');

  // 1. Check service health
  console.log('1. Checking service health...');
  const health = await client.health();
  console.log(`   Status: ${health.status}\n`);

  // 2. List supported languages
  console.log('2. Listing supported languages...');
  const languages = await client.listLanguages();
  console.log(`   Available languages: ${languages.languages.join(', ')}\n`);

  // 3. Execute Python code
  console.log('3. Executing Python code...');
  const pythonResult = await client.execute({
    language: 'python',
    code: `
print("Hello from Python!")
print("2 + 2 =", 2 + 2)

# This will write to stderr
import sys
sys.stderr.write("This is stderr\\n")
`,
  });

  console.log('   Python Output:');
  console.log(`   stdout: ${pythonResult.stdout.trim()}`);
  console.log(`   stderr: ${pythonResult.stderr.trim()}`);
  console.log(`   Exit code: ${pythonResult.metadata.exit_code}`);
  console.log(`   Execution time: ${pythonResult.metadata.time.toFixed(3)}s`);
  console.log(`   Memory used: ${(pythonResult.metadata.memory / 1024 / 1024).toFixed(2)} MB`);
  console.log(`   Box ID: ${pythonResult.box_id}\n`);

  // 4. Execute JavaScript code (if supported)
  if (languages.languages.includes('javascript')) {
    console.log('4. Executing JavaScript code...');
    const jsResult = await client.execute({
      language: 'javascript',
      code: `
console.log("Hello from JavaScript!");
console.log("Array example:", [1, 2, 3, 4, 5]);
`,
    });

    console.log('   JavaScript Output:');
    console.log(`   stdout: ${jsResult.stdout.trim()}`);
    console.log(`   Box ID: ${jsResult.box_id}\n`);
  }

  // 5. Execute code with error
  console.log('5. Executing code with runtime error...');
  const errorResult = await client.execute({
    language: 'python',
    code: `
print("This will run")
raise ValueError("This is an error!")
`,
  });

  console.log('   Error Output:');
  console.log(`   stdout: ${errorResult.stdout.trim()}`);
  console.log(`   stderr: ${errorResult.stderr.trim()}`);
  console.log(`   Exit code: ${errorResult.metadata.exit_code}`);
  console.log(`   Status: ${errorResult.metadata.status}\n`);

  console.log('=== Example completed successfully ===');
}

// Run the example
main().catch(console.error);

