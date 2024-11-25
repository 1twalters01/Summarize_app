package proxy

import (
    "net/http"
)

func proxyRequest(w http.ResponseWriter, r *http.Request, target string) {
    client:= &http.Client{}
}
