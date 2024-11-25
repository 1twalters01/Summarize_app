package main

import (
    "io"
    "log"
    "net/http"
)

func main() {
    http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
        w.Write([]byte("OK"))
    })

    log.Println("Starting proxy server on :3000...")
    log.Fatal(http.ListenAndServe(":8017", nil))
}
