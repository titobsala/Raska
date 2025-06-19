Proposed Simplified Approach
Not only TUI but the CLI as well.

Our pivot it's very simple, we want a tool CLI and TUI that, we can have it read a .md plan or roadmap as we have and show in diferent views, but also allows the user to complet taks, modify or add.

My idea is, when the user init the project, with "rask init <name of the .md roadmap file>". It will creat a folder localy, with the sparsed .md, that probaly will be a .json file. and both CLI or TUI, will read from that folder. The user can have "multiple projects", this is nothing more then differets .md roadmaps.

For exemple, let's assume the user have a large .roadmpa file, well structured with phases and so on, but he wants to works in another small set of tasks, so he can switch quickly to this other set of tasks and get back to the original plan, that should be possible.

Organize, research, expand, prioritize, and ship tasks effortlessly. Enjoy permanent context, zero drift & instant clarity.
Based on your direction to make TUI local-only, here's the plan:
What to Remove:
Global Project System: Remove all ProjectsConfig, global project switching
Workspace Detection: Remove complex workspace type detection
Project Switcher View: Remove the entire project switcher from TUI navigation
Caching System: Remove all the cached workspace/project data
Complex State Management: Simplify to just local state
What to Keep (Simplified):
Local .rask/ Only: TUI only works if you're in a directory with .rask/
Direct State Access: Just read/write to .rask/state.json directly
Simple Navigation: Home, Tasks, Templates, Settings (no Projects tab)
Clean Error Handling: If no .rask/ found, show clear message to run rask init
Benefits of This Approach:
No Freezing: Eliminates complex I/O operations that cause blocking
Faster Startup: No workspace detection or config loading
Clearer UX: TUI is for local work, web dashboard for global management
Simpler Code: Removes hundreds of lines of complex logic
Better Separation: Local vs global projects have different use cases
Implementation Plan:
Strip out all global project logic
Remove project switcher navigation item
Simplify app initialization to just check for local .rask/state.json
Remove caching system entirely
Update navigation to be 4 items instead of 5
Add clear messaging when not in a .rask directory
This approach aligns with the principle that TUI = local development tool and Web Dashboard = global project management.