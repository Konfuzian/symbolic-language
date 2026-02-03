#!/usr/bin/env node

/**
 * SYM Test Harness
 *
 * Runs all test cases from tests/cases/ directory.
 * Tests are organized in subdirectories, each containing:
 * - input.sym: The SYM format input
 * - expected.json: Expected JSON output (for success cases)
 * - error.json: Expected error pattern (for error cases)
 */

import { readFileSync, readdirSync, statSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { parse, ParseError } from './parser-stub.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// ANSI color codes for terminal output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  dim: '\x1b[2m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
};

// Configuration
const CASES_DIR = join(__dirname, '../../cases');
const verbose = process.argv.includes('--verbose') || process.argv.includes('-v');

/**
 * Find all test case directories
 */
function findTestCases(dir) {
  const cases = [];

  function traverse(currentDir) {
    const entries = readdirSync(currentDir);

    // Check if this directory is a test case (has input.sym)
    if (entries.includes('input.sym')) {
      cases.push(currentDir);
      return;
    }

    // Recurse into subdirectories
    for (const entry of entries) {
      const fullPath = join(currentDir, entry);
      if (statSync(fullPath).isDirectory()) {
        traverse(fullPath);
      }
    }
  }

  traverse(dir);
  return cases;
}

/**
 * Read a file or return null if it doesn't exist
 */
function readFileIfExists(path) {
  try {
    return readFileSync(path, 'utf-8');
  } catch (err) {
    return null;
  }
}

/**
 * Deep equality comparison for objects
 */
function deepEqual(a, b) {
  if (a === b) return true;
  if (a == null || b == null) return false;
  if (typeof a !== typeof b) return false;

  if (typeof a !== 'object') return false;

  if (Array.isArray(a) !== Array.isArray(b)) return false;

  const keysA = Object.keys(a);
  const keysB = Object.keys(b);

  if (keysA.length !== keysB.length) return false;

  for (const key of keysA) {
    if (!keysB.includes(key)) return false;
    if (!deepEqual(a[key], b[key])) return false;
  }

  return true;
}

/**
 * Format JSON with 2-space indentation
 */
function formatJson(obj) {
  return JSON.stringify(obj, null, 2);
}

/**
 * Run a single test case
 */
function runTestCase(testDir) {
  const testName = testDir.replace(CASES_DIR + '/', '');
  const inputPath = join(testDir, 'input.sym');
  const expectedPath = join(testDir, 'expected.json');
  const errorPath = join(testDir, 'error.json');

  const input = readFileSync(inputPath, 'utf-8');
  const hasExpected = existsSync(expectedPath);
  const hasError = existsSync(errorPath);

  const result = {
    name: testName,
    passed: false,
    message: '',
    details: null,
  };

  try {
    // Attempt to parse the input
    const parsed = parse(input);

    if (hasError) {
      // Expected an error but parsing succeeded
      result.message = 'Expected parsing to fail, but it succeeded';
      result.details = {
        parsed,
      };
      return result;
    }

    if (hasExpected) {
      // Compare with expected output
      const expected = JSON.parse(readFileSync(expectedPath, 'utf-8'));

      if (deepEqual(parsed, expected)) {
        result.passed = true;
        result.message = 'Output matches expected';
      } else {
        result.message = 'Output does not match expected';
        result.details = {
          expected,
          actual: parsed,
        };
      }
    } else {
      // No expected file - just report that parsing succeeded
      result.passed = true;
      result.message = 'Parsed successfully (no expected.json to compare)';
      result.details = { parsed };
    }
  } catch (err) {
    if (hasError) {
      // Expected an error - validate it matches the pattern
      const errorSpec = JSON.parse(readFileSync(errorPath, 'utf-8'));
      const isParseError = err instanceof ParseError || err.name === 'ParseError';

      if (!isParseError) {
        result.message = `Expected ParseError but got ${err.name}`;
        result.details = { error: err.message };
        return result;
      }

      // Check error type
      if (errorSpec.type && err.name !== errorSpec.type) {
        result.message = `Expected error type "${errorSpec.type}" but got "${err.name}"`;
        result.details = { error: err.message };
        return result;
      }

      // Check message pattern
      if (errorSpec.messagePattern) {
        const pattern = new RegExp(errorSpec.messagePattern, 'i');
        if (!pattern.test(err.message)) {
          result.message = `Error message doesn't match pattern: ${errorSpec.messagePattern}`;
          result.details = {
            pattern: errorSpec.messagePattern,
            actualMessage: err.message,
          };
          return result;
        }
      }

      // Check line number if specified
      if (errorSpec.line !== undefined && err.line !== errorSpec.line) {
        result.message = `Expected error on line ${errorSpec.line} but got line ${err.line}`;
        result.details = {
          expectedLine: errorSpec.line,
          actualLine: err.line,
          error: err.message,
        };
        return result;
      }

      result.passed = true;
      result.message = 'Error matches expected pattern';
    } else {
      // Unexpected error
      result.message = `Unexpected error: ${err.message}`;
      result.details = {
        error: err.message,
        stack: err.stack,
      };
    }
  }

  return result;
}

/**
 * Print test results
 */
function printResults(results) {
  const passed = results.filter((r) => r.passed).length;
  const failed = results.filter((r) => !r.passed).length;
  const total = results.length;

  console.log(`\n${colors.bright}Test Results${colors.reset}`);
  console.log(`${'='.repeat(60)}\n`);

  // Group results by category
  const byCategory = {};
  for (const result of results) {
    const category = result.name.split('/')[0];
    if (!byCategory[category]) {
      byCategory[category] = [];
    }
    byCategory[category].push(result);
  }

  // Print results by category
  for (const [category, tests] of Object.entries(byCategory)) {
    console.log(`${colors.cyan}${category}${colors.reset}`);

    for (const result of tests) {
      const icon = result.passed ? `${colors.green}✓${colors.reset}` : `${colors.red}✗${colors.reset}`;
      const testName = result.name.replace(category + '/', '  ');

      console.log(`${icon} ${testName}`);

      if (!result.passed || verbose) {
        console.log(`  ${colors.dim}${result.message}${colors.reset}`);

        if (result.details && verbose) {
          if (result.details.expected !== undefined) {
            console.log(`\n  ${colors.yellow}Expected:${colors.reset}`);
            console.log(
              formatJson(result.details.expected)
                .split('\n')
                .map((line) => `    ${line}`)
                .join('\n')
            );
          }
          if (result.details.actual !== undefined) {
            console.log(`\n  ${colors.yellow}Actual:${colors.reset}`);
            console.log(
              formatJson(result.details.actual)
                .split('\n')
                .map((line) => `    ${line}`)
                .join('\n')
            );
          }
          if (result.details.error) {
            console.log(`\n  ${colors.yellow}Error:${colors.reset}`);
            console.log(`    ${result.details.error}`);
          }
          if (result.details.pattern) {
            console.log(`\n  ${colors.yellow}Pattern:${colors.reset} ${result.details.pattern}`);
            console.log(`  ${colors.yellow}Message:${colors.reset} ${result.details.actualMessage}`);
          }
          console.log();
        }
      }
    }
    console.log();
  }

  // Print summary
  console.log(`${'='.repeat(60)}`);
  const passRate = ((passed / total) * 100).toFixed(1);
  const summaryColor = failed === 0 ? colors.green : colors.red;

  console.log(
    `${summaryColor}${colors.bright}${passed}/${total} tests passed (${passRate}%)${colors.reset}\n`
  );

  if (failed > 0) {
    console.log(`${colors.red}${failed} test(s) failed${colors.reset}\n`);
  }

  return failed === 0;
}

/**
 * Main test runner
 */
function main() {
  console.log(`${colors.bright}${colors.blue}SYM Test Harness${colors.reset}\n`);

  if (!existsSync(CASES_DIR)) {
    console.error(`${colors.red}Error: Test cases directory not found: ${CASES_DIR}${colors.reset}`);
    process.exit(1);
  }

  console.log(`Looking for test cases in: ${colors.dim}${CASES_DIR}${colors.reset}\n`);

  const testCases = findTestCases(CASES_DIR);
  console.log(`Found ${colors.bright}${testCases.length}${colors.reset} test case(s)\n`);

  if (testCases.length === 0) {
    console.log(`${colors.yellow}No test cases found${colors.reset}`);
    process.exit(0);
  }

  const results = testCases.map(runTestCase);
  const allPassed = printResults(results);

  process.exit(allPassed ? 0 : 1);
}

// Run the test harness
main();
