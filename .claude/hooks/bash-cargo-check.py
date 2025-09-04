#!/usr/bin/env python3
"""
PreToolUse hook for Bash commands to prevent direct cargo usage.
Redirects Claude to use just/moon workflow instead of bypassing task orchestration.
"""

import json
import sys
import re
import subprocess
import os


def check_cargo_command(command: str) -> tuple[bool, str]:
    """
    Check if the command uses cargo directly and suggest alternatives.
    Returns (is_cargo_command, suggestion).
    """
    # Normalize whitespace and check for cargo commands
    normalized_command = re.sub(r'\s+', ' ', command.strip())
    
    # Common cargo commands that should be avoided
    cargo_patterns = [
        (r'\bcargo\s+build\b', 'just build'),
        (r'\bcargo\s+test\b', 'just test'),
        (r'\bcargo\s+check\b', 'just check'),
        (r'\bcargo\s+clippy\b', 'just check'),
        (r'\bcargo\s+fmt\b', 'just check'),
        (r'\bcargo\s+run\b', 'just run'),
        (r'\bcargo\s+clean\b', 'just clean'),
        (r'\bcargo\s+doc\b', 'moon run :doc (if configured)'),
        (r'\bcargo\s+bench\b', 'moon run :bench (if configured)'),
        (r'\bcargo\s+publish\b', 'moon run :publish (if configured)'),
    ]
    
    for pattern, suggestion in cargo_patterns:
        if re.search(pattern, normalized_command, re.IGNORECASE):
            return True, f"Use '{suggestion}' instead of direct cargo usage"
    
    # Generic cargo command detection
    if re.search(r'\bcargo\s+\w+', normalized_command, re.IGNORECASE):
        return True, "Use 'just --list' or 'moon query tasks' to find the appropriate task"
    
    return False, ""


def get_just_tasks(project_dir: str) -> list[str]:
    """Get available just tasks."""
    try:
        result = subprocess.run(
            ['just', '--list'],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=5
        )
        
        if result.returncode == 0:
            tasks = []
            lines = result.stdout.strip().split('\n')
            for line in lines:
                # Parse just --list output format: "task-name # description"
                if line.strip() and not line.startswith('Available recipes:'):
                    # Extract task name (first word before any spaces or #)
                    task_line = line.strip()
                    if task_line:
                        # Handle both "task" and "task arg" formats
                        parts = task_line.split()
                        if parts:
                            task_name = parts[0]
                            # Get description if available
                            if '#' in task_line:
                                desc = task_line.split('#', 1)[1].strip()
                                tasks.append(f"• `just {task_name}` - {desc}")
                            else:
                                tasks.append(f"• `just {task_name}`")
            return tasks
    except (subprocess.TimeoutExpired, FileNotFoundError, Exception):
        pass
    
    return []


def get_moon_tasks(project_dir: str) -> list[str]:
    """Get available moon tasks."""
    try:
        result = subprocess.run(
            ['moon', 'query', 'tasks'],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=5
        )
        
        if result.returncode == 0:
            tasks = []
            # Parse moon output - look for task patterns like "tram:build", ":lint", etc.
            lines = result.stdout.strip().split('\n')
            for line in lines:
                line = line.strip()
                if ':' in line and not line.startswith('✓') and not line.startswith('Tasks:'):
                    # Extract task patterns
                    task_match = re.search(r'([a-zA-Z0-9_-]*):([a-zA-Z0-9_-]+)', line)
                    if task_match:
                        project, task = task_match.groups()
                        if project:
                            tasks.append(f"• `moon run {project}:{task}`")
                        else:
                            tasks.append(f"• `moon run :{task}`")
            
            # Remove duplicates while preserving order
            seen = set()
            unique_tasks = []
            for task in tasks:
                if task not in seen:
                    seen.add(task)
                    unique_tasks.append(task)
            
            return unique_tasks[:10]  # Limit to first 10 to keep message readable
    except (subprocess.TimeoutExpired, FileNotFoundError, Exception):
        pass
    
    return []


def main():
    try:
        # Read input from stdin
        input_data = json.load(sys.stdin)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON input: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Only process Bash tool calls
    tool_name = input_data.get("tool_name", "")
    if tool_name != "Bash":
        sys.exit(0)  # Not a Bash command, allow it
    
    tool_input = input_data.get("tool_input", {})
    command = tool_input.get("command", "")
    
    if not command:
        sys.exit(0)  # No command specified, allow it
    
    # Check if this is a direct cargo command
    is_cargo, suggestion = check_cargo_command(command)
    
    if is_cargo:
        # Get project directory
        project_dir = os.environ.get("CLAUDE_PROJECT_DIR")
        if not project_dir:
            project_dir = input_data.get("cwd", ".")
        
        # Get available tasks dynamically
        just_tasks = get_just_tasks(project_dir)
        moon_tasks = get_moon_tasks(project_dir)
        
        # Build available commands section
        commands_section = ""
        if just_tasks:
            commands_section += "Available just commands:\n" + "\n".join(just_tasks)
        
        if moon_tasks:
            if commands_section:
                commands_section += "\n\n"
            commands_section += "Available moon tasks:\n" + "\n".join(moon_tasks)
        
        if not commands_section:
            # Fallback if dynamic lookup fails
            commands_section = """Available commands:
• `just --list` - Show all available development commands
• `moon query tasks` - Show moon task definitions
• `just check` - Format, lint, build, test pipeline
• `just build [CRATE]` - Build workspace or specific crate
• `just test [CRATE]` - Run tests
• `just run [ARGS]` - Run the CLI application"""
        
        # Use JSON output to provide guidance to Claude
        feedback = {
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": f"""Direct cargo usage detected: `{command.strip()}`

This project uses moon task orchestration through just recipes. {suggestion}

{commands_section}

Using the task orchestration provides:
- Intelligent caching and incremental builds
- Proper dependency resolution between crates
- Parallel execution where possible
- Consistent development workflows"""
            }
        }
        
        print(json.dumps(feedback))
        sys.exit(0)
    
    # Allow the command if it's not a direct cargo usage
    sys.exit(0)


if __name__ == "__main__":
    main()