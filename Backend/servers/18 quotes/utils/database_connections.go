// go get github.com/jackc/pgx/v5
package utils

import (
	"os"
	"strings"
	"githum.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

func create_pg_pool_connection() (*pgxpool.Pool, error) {
	url := os.Getenv("PG_URL")
	pool, err := pgxpool.New(ctx, url)
	if err != nil {
		log.Fatalf("Unable to connect to database: %v", err)
	}
	return pool, nil
}

// CREATE
func createUser(ctx context.Context, pool *pgxpool.Pool, name, email string) {
	query := `INSERT INTO users (name, email) VALUES ($1, $2)`
	_, err := pool.Exec(ctx, query, name, email)
	if err != nil {
		log.Fatalf("Unable to insert user: %v", err)
	}
	fmt.Println("User created successfully")
}

// READ
func readUsers(ctx context.Context, pool *pgxpool.Pool) {
	query := `SELECT id, name, email FROM users`
	rows, err := pool.Query(ctx, query)
	if err != nil {
		log.Fatalf("Unable to read users: %v", err)
	}
	defer rows.Close()

	for rows.Next() {
		var id int
		var name, email string
		if err := rows.Scan(&id, &name, &email); err != nil {
			log.Fatalf("Unable to scan row: %v", err)
		}
		fmt.Printf("ID: %d, Name: %s, Email: %s\n", id, name, email)
	}
}

// UPDATE
func updateUser(ctx context.Context, pool *pgxpool.Pool, id int, newName string) {
	query := `UPDATE users SET name = $1 WHERE id = $2`
	_, err := pool.Exec(ctx, query, newName, id)
	if err != nil {
		log.Fatalf("Unable to update user: %v", err)
	}
	fmt.Println("User updated successfully")
}

// DELETE
func deleteUser(ctx context.Context, pool *pgxpool.Pool, id int) {
	query := `DELETE FROM users WHERE id = $1`
	_, err := pool.Exec(ctx, query, id)
	if err != nil {
		log.Fatalf("Unable to delete user: %v", err)
	}
	fmt.Println("User deleted successfully")
}