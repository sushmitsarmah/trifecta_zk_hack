package internal

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"strconv"

	"prover/circuits"

	"github.com/brevis-network/brevis-sdk/sdk"
	"github.com/gorilla/mux"
)

// Circuit is a global variable so we can update its parameters
var (
	Circuit    = &circuits.AppCircuit{}
	Token1Addr = sdk.ConstUint248("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48") // Default: USDC
	Token2Addr = sdk.ConstUint248("0xdAC17F958D2ee523a2206206994597C13D831ec7") // Default: USDT
	MinVolume  = sdk.ConstUint248(500000000)                                    // Default: 500 tokens
)

// UpdateCircuitHandler handles updating circuit parameters
func UpdateCircuitHandler(w http.ResponseWriter, r *http.Request) {
	var params CircuitParams

	err := json.NewDecoder(r.Body).Decode(&params)
	if err != nil {
		respondWithError(w, http.StatusBadRequest, "Invalid request payload")
		return
	}

	// Update token addresses
	if params.Token1Address != "" {
		Token1Addr = sdk.ConstUint248(params.Token1Address)
	}

	if params.Token2Address != "" {
		Token2Addr = sdk.ConstUint248(params.Token2Address)
	}

	// Update minimum volume if provided
	if params.MinimumVolume != "" {
		volume, err := strconv.ParseUint(params.MinimumVolume, 10, 64)
		if err != nil {
			respondWithError(w, http.StatusBadRequest, "Invalid minimum volume")
			return
		}
		MinVolume = sdk.ConstUint248(volume)
	}

	// Return current circuit configuration
	currentConfig := CircuitParams{
		Token1Address: fmt.Sprintf("%v", Token1Addr),
		Token2Address: fmt.Sprintf("%v", Token2Addr),
		MinimumVolume: fmt.Sprintf("%v", MinVolume),
	}

	RespondWithJSON(w, http.StatusOK, map[string]interface{}{
		"success": true,
		"message": "Circuit parameters updated successfully",
		"config":  currentConfig,
	})
}

// GetCircuitConfigHandler returns the current circuit configuration
func GetCircuitConfigHandler(w http.ResponseWriter, r *http.Request) {
	currentConfig := CircuitParams{
		Token1Address: fmt.Sprintf("%v", Token1Addr),
		Token2Address: fmt.Sprintf("%v", Token2Addr),
		MinimumVolume: fmt.Sprintf("%v", MinVolume),
	}

	RespondWithJSON(w, http.StatusOK, map[string]interface{}{
		"success": true,
		"config":  currentConfig,
	})
}

// Helper function to respond with an error
func respondWithError(w http.ResponseWriter, code int, message string) {
	RespondWithJSON(w, code, Response{
		Success: false,
		Error:   message,
	})
}

func main() {
	r := mux.NewRouter()

	// API routes
	r.HandleFunc("/api/config", GetCircuitConfigHandler).Methods("GET")
	r.HandleFunc("/api/config", UpdateCircuitHandler).Methods("POST")

	// Get port from environment or use default
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	log.Printf("Server starting on port %s...\n", port)
	log.Fatal(http.ListenAndServe(":"+port, r))
}
