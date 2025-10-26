/**
 * Box Management Example
 * 
 * This example demonstrates working with sandbox boxes, including
 * listing files, reading file contents, and cleaning up boxes.
 */

import { IsolateSandboxClient } from '../src/index';

async function main() {
  const client = new IsolateSandboxClient({
    baseUrl: 'http://localhost:3000',
    // Optional: Uncomment if your API requires authentication
    // apiKey: process.env.ISOLATE_SANDBOX_API_KEY,
  });

  console.log('=== IsolateSandbox Box Management Example ===\n');

  // 1. Execute code that creates files
  console.log('1. Executing code that creates files...');
  const result = await client.execute({
    language: 'python',
    code: `
# Write to multiple files
with open('output.txt', 'w') as f:
    f.write('Hello from Python!\\n')
    f.write('This is a test file.\\n')

with open('data.json', 'w') as f:
    import json
    data = {"message": "Hello", "numbers": [1, 2, 3, 4, 5]}
    json.dump(data, f, indent=2)

with open('results.csv', 'w') as f:
    f.write('name,age,city\\n')
    f.write('Alice,30,NYC\\n')
    f.write('Bob,25,LA\\n')

print("Files created successfully!")
`,
  });

  console.log(`   stdout: ${result.stdout.trim()}`);
  console.log(`   Box ID: ${result.box_id}\n`);

  const boxId = result.box_id;

  // 2. List files in the box
  console.log(`2. Listing files in box ${boxId}...`);
  const files = await client.listBoxFiles(boxId);
  console.log(`   Files found: ${files.files.length}`);
  files.files.forEach(file => {
    console.log(`   - ${file}`);
  });
  console.log();

  // 3. Read file contents
  console.log('3. Reading file contents...\n');

  // Read text file
  const textFile = await client.getBoxFile(boxId, 'output.txt');
  console.log(`   === ${textFile.filename} ===`);
  console.log(`   ${textFile.content.trim()}\n`);

  // Read JSON file
  const jsonFile = await client.getBoxFile(boxId, 'data.json');
  console.log(`   === ${jsonFile.filename} ===`);
  console.log(`   ${jsonFile.content.trim()}\n`);

  // Read CSV file
  const csvFile = await client.getBoxFile(boxId, 'results.csv');
  console.log(`   === ${csvFile.filename} ===`);
  console.log(`   ${csvFile.content.trim()}\n`);

  // 4. Get file as raw base64 (alternative method)
  console.log('4. Getting file as raw base64...');
  const rawFile = await client.getBoxFileRaw(boxId, 'output.txt');
  console.log(`   Filename: ${rawFile.filename}`);
  console.log(`   Base64 content (first 50 chars): ${rawFile.content.substring(0, 50)}...\n`);

  // 5. Clean up the box
  console.log(`5. Cleaning up box ${boxId}...`);
  const cleanup = await client.cleanupBox(boxId);
  console.log(`   ${cleanup.message}\n`);

  console.log('=== Example completed successfully ===');
}

// Run the example
main().catch(console.error);

