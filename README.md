# MyTodoRust

A small fullstack project made entirelly on Rust (Or most of, at least).

As for now it only display the tasks stored on the database.

## Start server

To start the server run the following commands in the project folder:

- BackEnd: cargo run -p backend --bin backend
- FronEnd (default port 9090): microserver -a 127.0.1.1

## Compile the project

Simply run **cargo make**, and both the BackEnd and FrontEnd will be compiled.

## Task Related comamnds:

To add tasks, use the command: **cargo run -p backend --bin todo new 'WRITE_TASK_HERE'**

To show tasks, use the command: **cargo run -p backend --bin todo show**
