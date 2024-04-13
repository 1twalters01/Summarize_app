#include <gtk/gtk.h>

void print_input(GtkEditable *self, gchar *text, gint length, gint *position, gpointer user_Data);
void login(GtkWidget *window);

void print_input(GtkEditable *self, gchar *text, gint length, gint *position, gpointer user_Data) {
    g_print("text inserted\n");
}

void login(GtkWidget *window) {
    // GtkWindow *window;
    GtkWidget *username_input;

    // window = gtk_window_new();
    // window = gtk_application_get_active_window(app);

    username_input = gtk_entry_new();
    g_signal_connect(username_input, "insert-text", G_CALLBACK(print_input), NULL); 

    gtk_window_set_child(GTK_WINDOW(window), username_input);

    gtk_window_present(GTK_WINDOW(window));
}
