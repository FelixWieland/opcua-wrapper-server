package main

import (
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net/http"
)

type Value struct {
	NodeID string  `json:"nodeId"`
	Value  float64 `json:"value"`
}

type Schema struct {
	DisplayName string   `json:"displayName,omitempty"`
	NodeID      string   `json:"nodeId,omitempty"`
	Children    []Schema `json:"children,omitempty"`
}

var mockData = []Value{}
var schemaData = Schema{
	DisplayName: "test-data-provider",
	Children:    []Schema{},
}

func init() {
	currentFolder := Schema{}
	for i := 0; i < 20000; i++ {
		mockData = append(mockData, Value{
			NodeID: fmt.Sprintf("Node-%v", i),
			Value:  rand.Float64(),
		})
		if len(currentFolder.Children) >= 100 {
			schemaData.Children = append(schemaData.Children, currentFolder)
			currentFolder = Schema{}
		}
		currentFolder.DisplayName = fmt.Sprintf("Chunk-%v", len(schemaData.Children)+1)
		currentFolder.Children = append(currentFolder.Children, Schema{
			NodeID:      fmt.Sprintf("Node-%v", i),
			DisplayName: fmt.Sprintf("Name-%v", i),
		})
	}
}

func main() {
	log.Printf("Starting server on port 8080")
	http.HandleFunc("/mock", onMockRequest)
	http.HandleFunc("/schema", onSchemaRequest)
	http.ListenAndServe(":8080", nil)
}

func onMockRequest(w http.ResponseWriter, r *http.Request) {
	log.Printf("Mocking data")
	nmd := mockData
	for i := range nmd {
		nmd[i].Value = rand.Float64()
	}
	j, _ := json.Marshal(mockData)
	w.Write(j)
}

func onSchemaRequest(w http.ResponseWriter, r *http.Request) {
	log.Printf("Schema request")
	j, _ := json.Marshal(schemaData)
	w.Write(j)
}
