#include <gtk/gtk.h>

// Declare the widgets
static void activate(GtkApplication *app, gpointer data);
void widget1(GtkWidget *window, gpointer data);
void widget2(GtkWidget *window, gpointer data);

// Widget 1 - a button saying next window
void widget1(GtkWidget *window, gpointer data) {
    GtkWidget *button1;

    button1 = gtk_button_new_with_label("next window");
    g_signal_connect(button1, "clicked", G_CALLBACK(widget2), window);
    gtk_window_set_child(GTK_WINDOW(window), button1);
}

// Widget 2 - a button saying previous window
void widget2(GtkWidget *window, gpointer data) {
    GtkWidget *button2;

    button2 = gtk_button_new_with_label("previous window");
    g_signal_connect(button2, "clicked", G_CALLBACK(widget1), window);
    gtk_window_set_child(GTK_WINDOW(window), button2);
}


static void activate(GtkApplication *app, gpointer data) {
    GtkWidget *window;
    
    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Summarize");
    gtk_window_maximize(GTK_WINDOW(window));

    widget1(window, data);
    gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char **argv) {
    GtkApplication *app;
    int status;

    app = gtk_application_new("uk.summarize.app", G_APPLICATION_FLAGS_NONE);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
    status = g_application_run(G_APPLICATION(app), argc, argv);
    g_object_unref(app);

    return status;
}

