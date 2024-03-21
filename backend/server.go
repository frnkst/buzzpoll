package main

import (
    "encoding/json"
    "fmt"
    "net/http"
    "github.com/gorilla/mux"
)

// HelloWorldResponse represents the JSON response
type HelloWorldResponse struct {
    Message string `json:"msg"`
}

func main() {
    r := mux.NewRouter()

    // Define a route that returns "Hello, World!" as JSON
    r.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        response := HelloWorldResponse{Message: "Hello, World!"}
        w.Header().Set("Content-Type", "application/json")
        w.WriteHeader(http.StatusOK)
        json.NewEncoder(w).Encode(response)
    })

    // Start the server
    port := ":8080"
    fmt.Printf("Server listening on port %s...\n", port)
    http.ListenAndServe(port, r)
}
