# Usage

```bash
git clone https://github.com/username/repo_name.git
cd repo_name
cargo build
```

# run 

```bash
cargo run -- add <description> -c <category> -p <priority> --due <due_date> #to add task
cargo run list --c <category> #to list tasks
cargo run done <id> #to mark task as done
cargo run delete <id> #to delete task
```














🔥 Advanced Features (Level 3 – Pushing Rust Concepts)

SQLite Storage – Replace JSON with a database (rusqlite crate).

Concurrency – Support multiple commands running in parallel safely.

Custom Command Syntax – Like task add "Buy milk" --priority high --due tomorrow.

Export/Import – Export tasks to CSV/Markdown.

Plugin System – Let users define macros (like task work → auto-filters "Work" tasks).

🦀 Rust Concepts You’ll Practice

Ownership & Borrowing – Storing tasks in a Vec<Task> or HashMap<u32, Task>.

Structs & Enums – Define Task, Priority, Category.

Traits – Implement Display for pretty-printing tasks.

Error Handling – Use Result and thiserror or custom enums.

Serialization – With serde + serde_json.

Crates – clap (CLI parsing), chrono (dates), colored (terminal colors).

🏗️ Functional Flow Example

$ task add "Finish Rust project" --priority high --category Work --due 2025-09-15
Task added ✅

$ task list
[1] Finish Rust project | Priority: High | Category: Work | Due: 2025-09-15 | Status: Pending

$ task done 1
Task #1 marked as completed 🎉

$ task list
[1] Finish Rust project | Priority: High | Category: Work | Due: 2025-09-15 | Status: ✅ Done
