#include <stdio.h>
#include <gtk/gtk.h>
#include <sqlite3.h>
#include "activate_window/activate.h"
#include "widgets/widget_test_1_and_3/widget_1.h"

static void instantiate_db() {
    sqlite3 *db;
    int rc;

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

    sqlite3_close(db);
}

void activate(GtkApplication *app, gpointer data) {
    GtkWidget *window;
    GtkWidget *window_data;

    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Summarize");
    gtk_window_maximize(GTK_WINDOW(window));
    gtk_widget_set_size_request(window, 900, 600);

    window_data = g_malloc(sizeof(GtkWidget));
    window_data = window;

    instantiate_db();

    // Initially, add widget1 to the window
    widget_1(window, window_data);

    gtk_widget_show(window);
}

