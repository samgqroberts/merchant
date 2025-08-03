async function initMerchant() {
  try {
    // Import the WASM module
    console.log('Importing Merchant WASM module');
    const wasm = await import('merchant-wasm');
    
    // The start function is called automatically by wasm-bindgen
    // but we can also expose additional functions if needed
    console.log('Merchant WASM module loaded successfully');
    
    // You could expose reset functionality
    (window as any).resetGame = wasm.reset_game;
    
  } catch (error) {
    console.error('Failed to load Merchant WASM module:', error);
    
    const display = document.getElementById('game-display');
    if (display) {
      display.textContent = 'Failed to load game. Please check console for errors.';
    }
  }
}

// Initialize the game when the page loads
initMerchant();