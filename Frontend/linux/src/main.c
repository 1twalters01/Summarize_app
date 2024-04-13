#include <gtk/gtk.h>

typedef struct {
    GtkWidget *window;
    GtkWidget *current_widget;
} WindowData;

static void activate(GtkApplication *app, gpointer data);
void widget1(GtkWidget *window, gpointer data);
void widget2(GtkWidget *window, gpointer data);
void widget3(GtkWidget *window, gpointer data);

void widget1(GtkWidget *window, gpointer data) {
    WindowData *window_data;
    GtkWidget *button;

    window_data = (WindowData *)data;
    button = gtk_button_new_with_label("This is widget 1. Go to widget 2");
    g_signal_connect(button, "clicked", G_CALLBACK(widget2), window_data);
    gtk_window_set_child(GTK_WINDOW(window_data->window), button);
}

void widget2(GtkWidget *window, gpointer data) {
    WindowData *window_data;
    GtkWidget *grid;
    GtkWidget *button1;
    GtkWidget *button2;

    window_data = (WindowData *)data;

    button1 = gtk_button_new_with_label("This is widget 2. Go to widget1");
    g_signal_connect(button1, "clicked", G_CALLBACK(widget1), window_data);
    button2 = gtk_button_new_with_label("This is widget 2. Go to widget3");
    g_signal_connect(button2, "clicked", G_CALLBACK(widget3), window_data);

    grid = gtk_grid_new();
    gtk_grid_attach(GTK_GRID(grid), button1, 0, 0, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), button2, 1, 0, 1, 1);

    gtk_window_set_child(GTK_WINDOW(window_data->window), grid);
}

void widget3(GtkWidget *window, gpointer data) {
    WindowData *window_data;
    GtkWidget *button;

    window_data = (WindowData *)data;
    button = gtk_button_new_with_label("This is widget 3. Go to widget 1");
    g_signal_connect(button, "clicked", G_CALLBACK(widget1), window_data);
    gtk_window_set_child(GTK_WINDOW(window_data->window), button);
}

static void activate(GtkApplication *app, gpointer data) {
    GtkWidget *window;
    WindowData *window_data;

    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Summarize");
    gtk_window_maximize(GTK_WINDOW(window));

    window_data = g_malloc(sizeof(WindowData));
    window_data->window = window;

    // Initially, add widget1 to the window
    widget1(window, window_data);

    gtk_widget_show(window);
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
