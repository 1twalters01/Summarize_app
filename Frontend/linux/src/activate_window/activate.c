#include <gtk/gtk.h>
#include "activate_window/activate.h"
#include "widgets/widget_test_1_and_3/widget_1.h"

void activate(GtkApplication *app, gpointer data) {
    GtkWidget *window;
    GtkWidget *window_data;

    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Summarize");
    gtk_window_maximize(GTK_WINDOW(window));

    window_data = g_malloc(sizeof(GtkWidget));
    window_data = window;

    // Initially, add widget1 to the window
    widget_1(window, window_data);

    gtk_widget_show(window);
}

