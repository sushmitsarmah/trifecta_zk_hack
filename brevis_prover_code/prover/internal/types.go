package internal

// CircuitParams represents the configurable parameters for the circuit
type CircuitParams struct {
	Token1Address string `json:"token1_address"`
	Token2Address string `json:"token2_address"`
	MinimumVolume string `json:"minimum_volume,omitempty"`
}

// Response represents the API response
type Response struct {
	Success bool   `json:"success"`
	Message string `json:"message,omitempty"`
	Error   string `json:"error,omitempty"`
}
