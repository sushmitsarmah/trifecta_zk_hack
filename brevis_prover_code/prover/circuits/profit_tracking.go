package circuits

import (
	"github.com/brevis-network/brevis-sdk/sdk"
)

// Circuit is a global variable so we can update its parameters
var (
	Circuit    = &AppCircuit{}
	Token1Addr = sdk.ConstUint248("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48") // Default: USDC
	Token2Addr = sdk.ConstUint248("0xdAC17F958D2ee523a2206206994597C13D831ec7") // Default: USDT
	MinVolume  = sdk.ConstUint248(500000000)                                    // Default: 500 tokens
)

// AppCircuit is our circuit implementation
type AppCircuit struct{}

var _ sdk.AppCircuit = &AppCircuit{}

func (c *AppCircuit) Allocate() (maxReceipts, maxStorage, maxTransactions int) {
	return 32, 0, 0
}

func (c *AppCircuit) Define(api *sdk.CircuitAPI, in sdk.DataInput) error {
	receipts := sdk.NewDataStream(api, in.Receipts)

	// Get receipt for token 1 transaction (buy)
	buyReceipt := sdk.GetUnderlying(receipts, 0)
	// Get receipt for token 2 transaction (sell)
	sellReceipt := sdk.GetUnderlying(receipts, 1)

	// Verify token 1 transaction (buy)
	api.Uint248.AssertIsEqual(buyReceipt.Fields[0].Contract, Token1Addr)
	api.Uint248.AssertIsEqual(buyReceipt.Fields[0].IsTopic, sdk.ConstUint248(1))
	api.Uint248.AssertIsEqual(buyReceipt.Fields[0].Index, sdk.ConstUint248(1))
	api.Uint32.AssertIsEqual(buyReceipt.Fields[0].LogPos, buyReceipt.Fields[1].LogPos)
	api.Uint248.AssertIsEqual(buyReceipt.Fields[1].IsTopic, sdk.ConstUint248(0))
	api.Uint248.AssertIsEqual(buyReceipt.Fields[1].Index, sdk.ConstUint248(0))
	api.Uint248.AssertIsLessOrEqual(MinVolume, api.ToUint248(buyReceipt.Fields[1].Value))

	// Verify token 2 transaction (sell)
	api.Uint248.AssertIsEqual(sellReceipt.Fields[0].Contract, Token2Addr)
	api.Uint248.AssertIsEqual(sellReceipt.Fields[0].IsTopic, sdk.ConstUint248(1))
	api.Uint248.AssertIsEqual(sellReceipt.Fields[0].Index, sdk.ConstUint248(1))
	api.Uint32.AssertIsEqual(sellReceipt.Fields[0].LogPos, sellReceipt.Fields[1].LogPos)
	api.Uint248.AssertIsEqual(sellReceipt.Fields[1].IsTopic, sdk.ConstUint248(0))
	api.Uint248.AssertIsEqual(sellReceipt.Fields[1].Index, sdk.ConstUint248(0))
	api.Uint248.AssertIsLessOrEqual(MinVolume, api.ToUint248(sellReceipt.Fields[1].Value))

	// Verify the same account is involved in both transactions
	api.Uint248.AssertIsEqual(api.ToUint248(buyReceipt.Fields[0].Value), api.ToUint248(sellReceipt.Fields[0].Value))

	// Calculate profit (can be positive or negative)
	buyValue := api.ToUint248(buyReceipt.Fields[1].Value)
	sellValue := api.ToUint248(sellReceipt.Fields[1].Value)

	// Compare values to determine if there's a profit
	// Check if buyValue <= sellValue (direct assertion as requested)
	api.Uint248.AssertIsLessOrEqual(buyValue, sellValue)

	// Since we've asserted buyValue <= sellValue, we know there's a profit
	// Calculate profit amount (sellValue - buyValue)
	profitAmount := api.Uint248.Sub(sellValue, buyValue)

	// Output results
	api.OutputUint(64, api.ToUint248(buyReceipt.BlockNum))
	api.OutputUint(64, api.ToUint248(sellReceipt.BlockNum))
	api.OutputAddress(api.ToUint248(buyReceipt.Fields[0].Value))

	// Convert profitAmount (Uint248) to the appropriate type for OutputUint
	api.OutputUint(248, profitAmount)

	// Output true since we've asserted there's a profit
	api.OutputBool(sdk.Uint248(sdk.ConstUint32(1)))

	return nil
}
