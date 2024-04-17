#include <gtk/gtk.h>
#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>
#include <sqlite3.h>
#include "widgets/widget_test_2/widget_2.h"
#include "widgets/widget_test_1_and_3/widget_1.h"
#include "widgets/widget_test_1_and_3/widget_3.h"

static const char *get_entry_data_from_db(void) {
    sqlite3 *db;
    int rc;
    sqlite3_stmt *stmt;
    
    const char *query = "SELECT text FROM EntryData WHERE id=1";
    
    rc = sqlite3_prepare_v2(db, check, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "SQL error: %s\n", sqlite3_errmsg(db));
        sqlite3_close(db);
    }

    if (sqlite3_step(stmt) == SQLITE_ROW) {
        // Need to change unsigned char * to char * some how
        const unsigned char *text = sqlite3_column_text(stmt, 0);
        printf("%s\n", text);
    }
    sqlite3_close(db);

    return text;
}

static void save_entry_data_to_db(GtkEditable *entry, gpointer data) {
    // perform sql query
}

static const char *widget_2_button_1_lua_content(void) {
    lua_State *L = luaL_newstate();
    luaL_openlibs(L);
    (void)luaL_dofile(L, "lua/widget_test_2/widget_2.lua");

    lua_getglobal(L, "ReadJsonFromFile");
    // lua_getglobal(L, "GetJsonFromUrl");
    // lua_getglobal(L, "PostJsonFromUrl");
    lua_pushstring(L, "This is widget 2. Go to widget1.");
    lua_pcall(L, 1, 1, 0);

    const char* content = lua_tostring(L, -1);
    lua_close(L);
    
    return content;
}

static void entry_submitted(GtkEditable *button, gpointer data) {
    const gchar *text = gtk_editable_get_text(GTK_EDITABLE(data));
    g_print("Entry contents: %s\n", text);
}

void widget_2(GtkWidget *widget, gpointer data) {
    GtkWidget *window_data;
    GtkWidget *grid;
    GtkWidget *button1;
    GtkWidget *button2;
    GtkWidget *entry;
    GtkWidget *submit;

    window_data = (GtkWidget *)data;

    const char *content = widget_2_button_1_lua_content();
    button1 = gtk_button_new_with_label(content);
    g_signal_connect(button1, "clicked", G_CALLBACK(widget_1), window_data);

    button2 = gtk_button_new_with_label("This is widget 2. Go to widget3");
    g_signal_connect(button2, "clicked", G_CALLBACK(widget_3), window_data);

    entry = gtk_entry_new();
    const char *entry_text = "Enter text here:";
    gtk_entry_set_placeholder_text(GTK_ENTRY(entry), entry_text);
    gtk_entry_set_editable(GTK_ENTRY(entry), get_entry_data_from_db());
    g_signal_connect(entry, "activate", G_CALLBACK(entry_submitted), entry);

    submit = gtk_button_new_with_label("Submit");
    g_signal_connect(submit, "clicked", G_CALLBACK(entry_submitted), entry);

    grid = gtk_grid_new();
    gtk_grid_attach(GTK_GRID(grid), button1, 0, 0, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), button2, 1, 0, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), entry, 0, 2, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), submit, 1, 2, 1, 1);

    gtk_window_set_child(GTK_WINDOW(window_data), grid);
}

