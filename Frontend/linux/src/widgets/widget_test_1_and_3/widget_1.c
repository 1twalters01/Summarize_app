#include <gtk/gtk.h>
#include "widgets/widget_test_2/widget_2.h"
#include "widgets/widget_test_1_and_3/widget_1.h"

void widget_1(GtkWidget *window, gpointer data) {
    GtkWidget *window_data;
    GtkWidget *button;

    window_data = (GtkWidget *)data;
    button = gtk_button_new_with_label("This is widget 1. Go to widget 2");
    g_signal_connect(button, "clicked", G_CALLBACK(widget_2), window_data);
    gtk_window_set_child(GTK_WINDOW(window_data), button);
}

