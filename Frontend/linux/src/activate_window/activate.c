#include <stdio.h>
#include <gtk/gtk.h>
#include <sqlite3.h>
#include "activate_window/activate.h"
#include "widgets/widget_test_1_and_3/widget_1.h"

static void instantiate_db() {
    sqlite3 *db;
    int rc;
    sqlite3_stmt *stmt;


    rc = sqlite3_open("data/example.db", &db);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Error opening database: %s\n", sqlite3_errmsg(db));
    }

    const char *sql = "CREATE TABLE IF NOT EXISTS EntryData (id INTEGER PRIMARY KEY, name TEXT, text TEXT);";
    const char *err = "Error reading sql";
    rc = sqlite3_exec(db, sql, NULL, 0, (char**) &err);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "SQL error: %s\n", err);
        sqlite3_free((void *) err);
    } else {
        fprintf(stdout, "Table created successfully\n");
    }









    // const char *check = "SELECT name FROM sqlite_master WHERE type='table'";
    const char *check = "SELECT * FROM EntryData";
    rc = sqlite3_prepare_v2(db, check, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "SQL error: %s\n", sqlite3_errmsg(db));
        sqlite3_close(db);
    }

    printf("Tables in the database:\n");
    while (sqlite3_step(stmt) == SQLITE_ROW) {
        const unsigned char *tableName = sqlite3_column_text(stmt, 0);
        printf("%s\n", tableName);
    }
    sqlite3_close(db);
}

void activate(GtkApplication *app, gpointer data) {
    GtkWidget *window;
    GtkWidget *window_data;

    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Summarize");
    gtk_window_maximize(GTK_WINDOW(window));

    window_data = g_malloc(sizeof(GtkWidget));
    window_data = window;

    instantiate_db();

    // Initially, add widget1 to the window
    widget_1(window, window_data);

    gtk_widget_show(window);
}

