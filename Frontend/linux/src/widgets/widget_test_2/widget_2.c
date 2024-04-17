#include <gtk/gtk.h>
#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>
#include <sqlite3.h>
#include "widgets/widget_test_2/widget_2.h"
#include "widgets/widget_test_1_and_3/widget_1.h"
#include "widgets/widget_test_1_and_3/widget_3.h"

static char *get_entry_data_from_db(void) {
    sqlite3 *db;
    int rc;
    sqlite3_stmt *stmt1;
    sqlite3_stmt *stmt2;
    char *text = "";
    char *result = "";
    
    
    // text = NULL;
    rc = sqlite3_open("data/example.db", &db);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Cannot open database: %s\n", sqlite3_errmsg(db));
        return text;
    }

    const char *query = "SELECT * FROM EntryData WHERE name = ?";

    rc = sqlite3_prepare_v2(db, query, -1, &stmt1, NULL);
    if (rc != SQLITE_OK) {
        fprintf(stderr, "Failed to prepare statement: %s\n", sqlite3_errmsg(db));
        sqlite3_close(db);
        return text;
    }

    char *name = "example";
    sqlite3_bind_text(stmt1, 1, name, -1, SQLITE_STATIC);

    rc = sqlite3_step(stmt1);

    if (rc == SQLITE_ROW) {
        // Need to change unsigned char * to char * some how
        text = (char *) sqlite3_column_text(stmt1, 2);
        result = (char *) malloc(strlen(text) + 1);
        if (result == NULL) {
            fprintf(stderr, "Memory allocation failed\n");
            return "";
        }
        strcpy(result, text);
        printf("working test: %s\n", text);
        printf("working result: %s\n", result);
        strcpy(text, result);
        printf("working test: %s\n", text);
        printf("working result: %s\n", result);
    } else if (rc == SQLITE_DONE) {
        printf("No rows found with the given ID.\n");




        const char *text = "test";
        const char *insert_sql_insert = "INSERT INTO EntryData (name, text) VALUES (?, ?)";

        rc = sqlite3_prepare_v2(db, insert_sql_insert, -1, &stmt2, 0);
            
        if (rc != SQLITE_OK) {
            fprintf(stderr, "SQL error: %s\n", sqlite3_errmsg(db));
        } 

        sqlite3_bind_text(stmt2, 1, name, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt2, 2, text, -1, SQLITE_STATIC);

        rc = sqlite3_step(stmt2);
        if (rc != SQLITE_DONE) {
            fprintf(stderr, "SQL err: %s\n", sqlite3_errmsg(db));
        }

        sqlite3_finalize(stmt2);
    } else {
        fprintf(stderr, "Error retrieving data: %s\n", sqlite3_errmsg(db));
    }

    printf("text test: %s\n", text);
    printf("text result: %s\n", result);

    sqlite3_finalize(stmt1);
    printf("text test: %s\n", text);
    printf("text result: %s\n", result);
    sqlite3_close(db);
    
    printf("text test: %s\n", text);
    printf("text result: %s\n", result);

    return (char *) result;
}

static void save_entry_data_to_db(GtkEditable *entry, gpointer data) {
    // perform sql query
    sqlite3 *db;
    int rc;
    sqlite3_stmt *stmt;
    const gchar *text = gtk_editable_get_text(GTK_EDITABLE(data));


    printf("key has been pressed\n");


    rc = sqlite3_open("data/example.db", &db);

    const char *update_sql_insert = "UPDATE EntryData set text = (?) WHERE id = 1";
    rc = sqlite3_prepare_v2(db, update_sql_insert, -1, &stmt, 0);
            
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, text, -1, SQLITE_STATIC);
        rc = sqlite3_step(stmt);

        if (rc == SQLITE_DONE) {
            sqlite3_finalize(stmt);
            sqlite3_close(db);
            return;
        } else {
            fprintf(stderr, "SQL error: %s\n", sqlite3_errmsg(db));
        }

    } else { 
        fprintf(stderr, "SQL error: %s\n", sqlite3_errmsg(db)); 
    }




    const char *insert_sql_insert = "INSERT INTO EntryData (text) VALUES (?)";
    rc = sqlite3_prepare_v2(db, insert_sql_insert, -1, &stmt, 0);
            
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, text, -1, SQLITE_STATIC);
        rc = sqlite3_step(stmt);

        if (rc == SQLITE_DONE) {
            sqlite3_finalize(stmt);
        } else {
            fprintf(stderr, "SQL err: %s\n", sqlite3_errmsg(db));
        }

    } else {
        fprintf(stderr, "SQL error: %s\n", sqlite3_errmsg(db));
    }
    
    sqlite3_close(db);
    return;
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
    GtkEntryBuffer *buffer;
    GtkWidget *submit;

    window_data = (GtkWidget *)data;

    const char *content = widget_2_button_1_lua_content();
    button1 = gtk_button_new_with_label(content);
    g_signal_connect(button1, "clicked", G_CALLBACK(widget_1), window_data);

    button2 = gtk_button_new_with_label("This is widget 2. Go to widget3");
    g_signal_connect(button2, "clicked", G_CALLBACK(widget_3), window_data);

    entry = gtk_entry_new();
    buffer = gtk_entry_buffer_new(NULL, 0);

    char *buffer_text = get_entry_data_from_db();
    printf("result is this: %s\n", buffer_text);
    gtk_entry_set_buffer(GTK_ENTRY(entry), buffer);
    gtk_entry_buffer_set_text(buffer, buffer_text, -1);
    const char *entry_text = "Enter text here:";
    gtk_entry_set_placeholder_text(GTK_ENTRY(entry), entry_text);
    g_signal_connect(entry, "activate", G_CALLBACK(entry_submitted), entry);
    g_signal_connect(entry, "changed", G_CALLBACK(save_entry_data_to_db), entry);

    submit = gtk_button_new_with_label("Submit");
    g_signal_connect(submit, "clicked", G_CALLBACK(entry_submitted), entry);

    grid = gtk_grid_new();
    gtk_grid_attach(GTK_GRID(grid), button1, 0, 0, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), button2, 1, 0, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), entry, 0, 2, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), submit, 1, 2, 1, 1);

    gtk_window_set_child(GTK_WINDOW(window_data), grid);
}

