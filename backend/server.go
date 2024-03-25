package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"github.com/google/uuid"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"log"
	"net/http"
	"time"
)

type Poll struct {
	Question string `json:"question"`
}

type PollResponse struct {
	UUID string `json:"uuid"`
}

type ServePollRequest struct {
	UUID string `json:"uuid"`
}

type ServePollResponse struct {
	Poll Poll `json:"poll"`
}

var polls = make(map[string]Poll)

var addr = flag.String("addr", ":8080", "http service address")

func savePoll(w http.ResponseWriter, r *http.Request) {
	var poll Poll

	err := json.NewDecoder(r.Body).Decode(&poll)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	var id = uuid.New()
	polls[id.String()] = poll

	pollResponse := PollResponse{UUID: id.String()}

	jsonResponse, jsonError := json.Marshal(pollResponse)

	if jsonError != nil {
		fmt.Println("Unable to encode JSON")
	}

	fmt.Println(string(jsonResponse))

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(jsonResponse)
}

func servePoll(w http.ResponseWriter, r *http.Request) {
	params := mux.Vars(r)
	var id = params["id"]

	fmt.Printf("frank: %s", id)

	var poll = polls[id]

	pollResponse := ServePollResponse{Poll: poll}

	jsonResponse, jsonError := json.Marshal(pollResponse)

	if jsonError != nil {
		fmt.Println("Unable to encode JSON")
	}

	fmt.Println(string(jsonResponse))

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(jsonResponse)
}

func serveHome(w http.ResponseWriter, r *http.Request) {
	log.Println(r.URL)
	if r.URL.Path != "/" {
		http.Error(w, "Not found", http.StatusNotFound)
		return
	}
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	http.ServeFile(w, r, "home.html")
}

func main() {
	flag.Parse()
	hub := newHub()
	go hub.run()

	r := mux.NewRouter()
	r.HandleFunc("/", serveHome)
	r.HandleFunc("/poll/{id}", servePoll).Methods("GET")
	r.HandleFunc("/poll", savePoll).Methods("POST")
	r.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
		serveWs(hub, w, r)
	})

	// headersOk := handlers.AllowedHeaders([]string{"*"})
	originsOk := handlers.AllowedOrigins([]string{"*"})
	headersOk := handlers.AllowedHeaders([]string{"content-type", "username", "password"})
	methodsOk := handlers.AllowedMethods([]string{"GET", "HEAD", "POST", "PUT", "OPTIONS"})

	server := &http.Server{
		Addr:              *addr,
		ReadHeaderTimeout: 3 * time.Second,
		Handler:           handlers.CORS(headersOk, originsOk, methodsOk)(r),
	}

	err := server.ListenAndServe()
	if err != nil {
		log.Fatal("ListenAndServe: ", err)
	}
}
