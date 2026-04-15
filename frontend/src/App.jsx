import React, { useState, useEffect } from 'react';
import './App.css';
import TicketList from './components/TicketList';
import TicketPurchase from './components/TicketPurchase';
import WalletConnect from './components/WalletConnect';
import { StellarService } from './utils/StellarService';

function App() {
  const [wallet, setWallet] = useState(null);
  const [tickets, setTickets] = useState([]);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState('browse');

  useEffect(() => {
    if (wallet) {
      loadTickets();
    }
  }, [wallet]);

  const loadTickets = async () => {
    setLoading(true);
    try {
      const ticketData = await StellarService.fetchTickets(wallet);
      setTickets(ticketData);
    } catch (error) {
      console.error('Failed to load tickets:', error);
    } finally {
      setLoading(false);
    }
  };

  const handlePurchaseTicket = async (ticketData) => {
    try {
      setLoading(true);
      await StellarService.purchaseTicket(wallet, ticketData);
      await loadTickets();
    } catch (error) {
      console.error('Purchase failed:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <header className="app-header">
        <h1>🎤 Concert Ticket Marketplace</h1>
        <p>Powered by Stellar Blockchain</p>
      </header>

      <nav className="app-nav">
        <button 
          className={activeTab === 'connect' ? 'active' : ''} 
          onClick={() => setActiveTab('connect')}
        >
          Wallet
        </button>
        <button 
          className={activeTab === 'browse' ? 'active' : ''} 
          onClick={() => setActiveTab('browse')}
        >
          Browse Tickets
        </button>
        <button 
          className={activeTab === 'purchase' ? 'active' : ''} 
          onClick={() => setActiveTab('purchase')}
        >
          Buy Tickets
        </button>
      </nav>

      <main className="app-main">
        {activeTab === 'connect' && (
          <WalletConnect wallet={wallet} setWallet={setWallet} />
        )}
        
        {activeTab === 'browse' && (
          <TicketList 
            tickets={tickets} 
            loading={loading} 
            onRefresh={loadTickets}
          />
        )}
        
        {activeTab === 'purchase' && wallet && (
          <TicketPurchase 
            wallet={wallet} 
            onPurchase={handlePurchaseTicket}
            loading={loading}
          />
        )}

        {!wallet && activeTab !== 'connect' && (
          <div className="warning-box">
            <p>Please connect your wallet first</p>
          </div>
        )}
      </main>

      <footer className="app-footer">
        <p>&copy; 2026 Concert Ticket Marketplace. Built with Stellar.</p>
      </footer>
    </div>
  );
}

export default App;