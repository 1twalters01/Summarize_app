#include <gtk/gtk.h>
#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>

#include "widget_test_2/widget_2.h"
#include "widget_test_1_and_3/widget_1.h"
#include "widget_test_1_and_3/widget_3.h"
#include "activate_window/window_data.h"

static const char *widget_2_lua_content(void) {
    lua_State *L = luaL_newstate();

    luaL_openlibs(L);
    luaL_dofile(L, "widget_test_2/widget_2.lua");
    // luaL_dofile(L, "../src/widget_test_2/widget_2.lua");

    lua_getglobal(L, "Content");
    const char* content = lua_tostring(L, -1);
    printf("function: %s", content);
    lua_close(L);
    
    return content;
}

void widget_2(GtkWidget *window, gpointer data) {
    WindowData *window_data;
    GtkWidget *grid;
    GtkWidget *button1;
    GtkWidget *button2;

    window_data = (WindowData *)data;

    const char *content = widget_2_lua_content();
    printf("widget: %s", content);
    button1 = gtk_button_new_with_label(content);
    g_signal_connect(button1, "clicked", G_CALLBACK(widget_1), window_data);
    button2 = gtk_button_new_with_label("This is widget 2. Go to widget3");
    g_signal_connect(button2, "clicked", G_CALLBACK(widget_3), window_data);

    grid = gtk_grid_new();
    gtk_grid_attach(GTK_GRID(grid), button1, 0, 0, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), button2, 1, 0, 1, 1);

    gtk_window_set_child(GTK_WINDOW(window_data->window), grid);
}

