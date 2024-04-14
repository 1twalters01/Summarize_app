#include <gtk/gtk.h>

#include "widget_test_1_and_3/widget_1.h"
#include "activate_window/window_data.h"


void widget_3(GtkWidget *window, gpointer data) {
    WindowData *window_data;
    GtkWidget *button;

    window_data = (WindowData *)data;
    button = gtk_button_new_with_label("This is widget 3. Go to widget 1");
    g_signal_connect(button, "clicked", G_CALLBACK(widget_1), window_data);
    gtk_window_set_child(GTK_WINDOW(window_data->window), button);
}

