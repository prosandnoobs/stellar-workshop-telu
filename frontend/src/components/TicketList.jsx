import React from 'react';
import '../styles/TicketList.css';

function TicketList({ tickets, loading, onRefresh }) {
  return (
    <div className="ticket-list-container">
      <div className="ticket-list-header">
        <h2>Available Concert Tickets</h2>
        <button className="refresh-btn" onClick={onRefresh} disabled={loading}>
          {loading ? 'Loading...' : 'Refresh'}
        </button>
      </div>

      {loading ? (
        <div className="loading">Loading tickets...</div>
      ) : tickets.length === 0 ? (
        <div className="no-tickets">No tickets available at the moment.</div>
      ) : (
        <div className="tickets-grid">
          {tickets.map((ticket) => (
            <div key={ticket.id} className="ticket-card">
              <div className="ticket-header">
                <h3>{ticket.event_name}</h3>
                <span className="ticket-id">#{ticket.id}</span>
              </div>
              <div className="ticket-details">
                <p><strong>Price:</strong> {ticket.price} XLM</p>
                <p><strong>Owner:</strong> {ticket.owner.substring(0, 10)}...</p>
                <p><strong>Status:</strong> {ticket.is_valid ? '✅ Valid' : '❌ Revoked'}</p>
              </div>
              <button className="buy-btn">Buy Now</button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default TicketList;