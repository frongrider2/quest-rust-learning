diesel setup
diesel migration generate init_database;

<!-- up -->
diesel migration run
diesel migration redo