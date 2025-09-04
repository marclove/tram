#!/usr/bin/env python3
"""
PostToolUse hook for checking Rust compiler warnings and errors.
Runs after file edits to provide immediate feedback on compilation issues.
"""

import json
import subprocess
import sys
import os
from pathlib import Path


def is_rust_file(file_path: str) -> bool:
    """Check if the file is a Rust source file."""
    return file_path.endswith('.rs')


def run_rust_check(project_dir: str) -> tuple[bool, str]:
    """
    Run cargo check to detect warnings and errors.
    Returns (has_issues, output).
    """
    try:
        # Use moon to run the check task for better integration
        result = subprocess.run(
            ['moon', 'run', ':lint'],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=30
        )
        
        # Clippy exits with non-zero for warnings when using -D warnings
        if result.returncode != 0:
            return True, result.stderr.strip() or result.stdout.strip()
        
        return False, ""
        
    except subprocess.TimeoutExpired:
        return True, "Rust check timed out after 30 seconds"
    except FileNotFoundError:
        # Fallback to cargo check if moon is not available
        try:
            result = subprocess.run(
                ['cargo', 'check', '--workspace', '--all-targets'],
                cwd=project_dir,
                capture_output=True,
                text=True,
                timeout=30
            )
            
            if result.returncode != 0:
                return True, result.stderr.strip()
            
            return False, ""
            
        except (subprocess.TimeoutExpired, FileNotFoundError):
            return True, "Could not run Rust compiler check"


def extract_relevant_warnings(output: str, edited_file: str) -> list[str]:
    """Extract warnings/errors relevant to the edited file."""
    if not output:
        return []
    
    lines = output.split('\n')
    relevant_warnings = []
    current_warning = []
    capturing = False
    
    # Get the relative path for matching
    edited_file_name = Path(edited_file).name
    
    for line in lines:
        # Start of a new warning/error
        if 'warning:' in line or 'error:' in line:
            # Save previous warning if it was relevant
            if capturing and current_warning:
                relevant_warnings.extend(current_warning)
            
            current_warning = [line]
            # Check if this warning is for our file
            capturing = edited_file_name in line or edited_file in line
        elif line.startswith('  -->') and capturing:
            # File reference line - double-check if it's our file
            capturing = edited_file_name in line or edited_file in line
            if capturing:
                current_warning.append(line)
        elif capturing and (line.startswith('   ') or line.startswith('  |') or line.strip() == ''):
            # Continuation of current warning
            current_warning.append(line)
        elif line.strip() == '' and capturing:
            # End of current warning
            if current_warning:
                relevant_warnings.extend(current_warning)
                relevant_warnings.append('')  # Add blank line for readability
            current_warning = []
            capturing = False
    
    # Don't forget the last warning
    if capturing and current_warning:
        relevant_warnings.extend(current_warning)
    
    return relevant_warnings


def main():
    try:
        # Read input from stdin
        input_data = json.load(sys.stdin)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON input: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Only process successful Edit, MultiEdit, or Write operations on Rust files
    tool_name = input_data.get("tool_name", "")
    tool_input = input_data.get("tool_input", {})
    tool_response = input_data.get("tool_response", {})
    
    # Check if tool was successful
    if not tool_response.get("success", False):
        sys.exit(0)  # Don't check failed operations
    
    if tool_name not in ["Edit", "MultiEdit", "Write"]:
        sys.exit(0)  # Only check file modification tools
    
    # Get the file path
    file_path = tool_input.get("file_path", "")
    if not file_path or not is_rust_file(file_path):
        sys.exit(0)  # Only check Rust files
    
    # Get project directory
    project_dir = os.environ.get("CLAUDE_PROJECT_DIR")
    if not project_dir:
        project_dir = input_data.get("cwd", ".")
    
    # Run the Rust check
    has_issues, output = run_rust_check(project_dir)
    
    if has_issues:
        # Extract warnings relevant to the edited file
        relevant_warnings = extract_relevant_warnings(output, file_path)
        
        if relevant_warnings:
            warning_text = '\n'.join(relevant_warnings)
            
            # Use JSON output to provide feedback to Claude
            feedback = {
                "decision": "block",
                "reason": f"Rust compiler issues detected in {Path(file_path).name}:\n\n{warning_text}\n\nPlease fix these issues immediately.",
                "hookSpecificOutput": {
                    "hookEventName": "PostToolUse",
                    "additionalContext": f"The file {file_path} has compilation issues that need to be addressed."
                }
            }
            
            print(json.dumps(feedback))
        elif "warning:" in output.lower() or "error:" in output.lower():
            # General warnings/errors in the workspace
            feedback = {
                "decision": "block", 
                "reason": f"Rust compiler issues detected in workspace after editing {Path(file_path).name}. Please run 'just check' to see all issues and fix them.",
                "hookSpecificOutput": {
                    "hookEventName": "PostToolUse",
                    "additionalContext": "There are compilation issues in the workspace that may be related to recent changes."
                }
            }
            
            print(json.dumps(feedback))
    
    # Exit with 0 for success (no blocking)
    sys.exit(0)


if __name__ == "__main__":
    main()